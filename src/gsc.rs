use std::{env, fs, path::Path};
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

const MARKETPLACE_EXTENSION_QUERY_URL: &str =
    "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

struct GscBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct GscExtension {
    cached_binary_path: Option<String>,
}

impl GscExtension {
    fn latest_marketplace_package() -> Result<(String, String)> {
        let request_body = zed::serde_json::json!({
            "filters": [{
                "criteria": [{
                    "filterType": 7,
                    "value": "blakintosh.gscode"
                }]
            }],
            "flags": 103
        })
        .to_string()
        .into_bytes();

        let request = zed::http_client::HttpRequest::builder()
            .method(zed::http_client::HttpMethod::Post)
            .url(MARKETPLACE_EXTENSION_QUERY_URL)
            .header("User-Agent", "gsczed")
            .header("Accept", "application/json;api-version=7.2-preview.1")
            .header("Content-Type", "application/json")
            .body(request_body)
            .redirect_policy(zed::http_client::RedirectPolicy::FollowAll)
            .build()?;
        let response = request.fetch()?;
        let response_body = String::from_utf8(response.body)
            .map_err(|e| format!("Marketplace returned invalid UTF-8: {e}"))?;
        let response_json: zed::serde_json::Value = zed::serde_json::from_str(&response_body)
            .map_err(|e| format!("failed to parse Marketplace response: {e}"))?;

        let latest_version = &response_json["results"][0]["extensions"][0]["versions"][0];
        let version = latest_version["version"]
            .as_str()
            .map(str::to_owned)
            .ok_or_else(|| "Marketplace response did not contain a GSCode version".to_string())?;
        let package_url = latest_version["files"]
            .as_array()
            .and_then(|files| {
                files.iter().find_map(|file| {
                    (file["assetType"].as_str()
                        == Some("Microsoft.VisualStudio.Services.VSIXPackage"))
                    .then(|| file["source"].as_str().map(str::to_owned))
                    .flatten()
                })
            })
            .ok_or_else(|| {
                "Marketplace response did not contain a raw GSCode VSIX asset".to_string()
            })?;

        Ok((version, package_url))
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<GscBinary> {
        let binary_settings = LspSettings::for_worktree("gscode", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(GscBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if path == "dotnet" || !path.ends_with(".dll") {
                self.cached_binary_path = None;
            } else if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(GscBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let (version, package_url) = Self::latest_marketplace_package()?;
        let version_dir = format!("gscode-{version}");
        let binary_path = Path::new(&version_dir)
            .join("extension")
            .join("service")
            .join("GSCode.NET.dll");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            // The upstream GitHub releases no longer attach server binaries. The
            // Marketplace VSIX is the official distribution and contains the
            // server plus all of its managed .NET dependencies.
            if fs::metadata(&version_dir).is_ok() {
                fs::remove_dir_all(&version_dir)
                    .map_err(|e| format!("failed to remove incomplete {version_dir}: {e}"))?;
            }

            zed::download_file(&package_url, &version_dir, zed::DownloadedFileType::Zip)
                .map_err(|e| format!("failed to download GSCode Marketplace package: {e}"))?;

            if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
                return Err(format!(
                    "GSCode Marketplace package did not contain {}",
                    binary_path.display()
                ));
            }

            // Clean up only older GSCode packages; leave unrelated extension data alone.
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
                    continue;
                };
                if name.starts_with("gscode-")
                    && name != version_dir
                    && entry
                        .file_type()
                        .map_err(|e| format!("failed to inspect {name}: {e}"))?
                        .is_dir()
                {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        // Convert to absolute path since we're in the extension work directory
        let absolute_path = env::current_dir()
            .map_err(|e| format!("failed to get current directory: {e}"))?
            .join(&binary_path)
            .to_string_lossy()
            .to_string();

        self.cached_binary_path = Some(absolute_path.clone());
        Ok(GscBinary {
            path: absolute_path,
            args: binary_args,
        })
    }
}

impl zed::Extension for GscExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let gsc_binary = self.language_server_binary(language_server_id, worktree)?;

        let mut args = vec![gsc_binary.path.clone()];
        if let Some(extra_args) = gsc_binary.args {
            args.extend(extra_args);
        }

        // Get the absolute path to dotnet
        let dotnet_path = worktree
            .which("dotnet")
            .ok_or_else(|| "dotnet runtime not found in PATH".to_string())?;

        Ok(zed::Command {
            command: dotnet_path,
            args,
            env: Default::default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree("gscode", worktree)
            .ok()
            .and_then(|settings| settings.initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree("gscode", worktree)
            .ok()
            .and_then(|settings| settings.settings))
    }
}

zed::register_extension!(GscExtension);
