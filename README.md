# GSC Language Support for Zed

A Zed editor extension providing language support for GSC, CSC, and GSH files used in Call of Duty: Black Ops III modding.

## Features

- **Syntax Highlighting**: Full syntax highlighting for GSC, CSC, and GSH files using tree-sitter
- **Language Server**: Integration with [GSCode.NET](https://github.com/Blakintosh/gscode)

## Installation

### Prerequisites

- [Zed Editor](https://zed.dev/) version 0.210.4 or later
- [Rust](https://rustup.rs/) (for building the extension)
- [.NET 8 SDK](https://dotnet.microsoft.com/download/dotnet/8.0) (for the language server)

### Installing in Zed

1. In Zed, open the command palette (`Ctrl+Shift+P` on Windows/Linux, `Cmd+Shift+P` on macOS)
2. Type "Extensions: Install Dev Extension" and select it
3. Navigate to the `gsczed` directory and select it
4. The extension will be compiled and installed automatically

## Language Server Setup

The extension will automatically download and build the GSCode.NET language server on first use. This requires:

- **.NET 8 SDK** installed and available in your PATH
- An internet connection to download the language server source from GitHub

### Custom Language Server Path

If you want to use a custom build of GSCode.NET, you can configure it in your Zed settings:

```json
{
  "lsp": {
    "gscode": {
      "binary": {
        "path": "/path/to/GSCode.NET.dll",
        "arguments": []
      }
    }
  }
}
```

## Supported File Types

- **`.gsc`** - Game Script Code (server-side scripts)
- **`.csc`** - Client Script Code (client-side scripts)
- **`.gsh`** - Game Script Header (shared definitions)

### Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

#### Tree-sitter Grammar

The tree-sitter grammar for GSC is maintained in a separate repository: [echo000/tree-sitter-gsc](https://github.com/echo000/tree-sitter-gsc)

#### Language Server

The GSCode.NET language server is maintained at: [Blakintosh/gscode](https://github.com/Blakintosh/gscode)

## License

This extension is provided as-is for use with Call of Duty: Black Ops III modding.

## Credits

- **Language Server**: [Blakintosh/gscode](https://github.com/Blakintosh/gscode)


## Related Projects

- [GSCode](https://github.com/Blakintosh/gscode) - VSCode extension for GSC

## Support

For issues and questions:
- Extension issues: Open an issue in this repository
- Language server issues: [GSCode repository](https://github.com/Blakintosh/gscode/issues)
- Grammar issues: [tree-sitter-gsc repository](https://github.com/echo000/tree-sitter-gsc/issues)
