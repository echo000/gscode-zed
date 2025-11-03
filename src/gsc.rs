use std::{env, fs};
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct GscBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct GscExtension {
    cached_binary_path: Option<String>,
}

impl GscExtension {
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
        let release = zed::latest_github_release(
            "echo000/gscode",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let version_dir = format!("gscode-{}", release.version);
        let binary_path = format!("{version_dir}/GSCode.NET.dll");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            // Find the zip asset from GitHub release
            let asset_name = "GSCodeLsp.zip";
            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            // Clean up old versions
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
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
}

zed::register_extension!(GscExtension);
