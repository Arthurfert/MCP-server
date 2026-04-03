# MCP Server

A Model Context Protocol (MCP) server collection, implemented in Rust.

## Description

This project implements a MCP server collection that allows you to expose tools, resources, or other standardized features via the MCP protocol.

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed on your machine (2021 edition or later recommended).

## Installation and Launch

1. Clone this repository.

2. Navigate to the project directory.

3. Compile and run the server with Cargo:

```bash
cargo run
```

4. VS Code integration with the Copilot extension: 
    - Select `...` > `Configure Tools`
    - `mcp` > `Command (stdio)`
    - Paste the path to mcp.exe (`C:\\Your-path\\MCP-server\\mcp\\target\\debug\\mcp.exe`)

## MCP Tools

The server currently exposes the following tools to the LLM agent:

- **git repository tools**: Runs `git status`, `git add` or `git commit` to make repository modifications.
- **`read_file`**: Reads and returns the content of a file (absolute or relative path).
- **`replace_text_in_file`**: Searches for an exact block of text in a file and replaces it. Useful for modifying or removing code snippets without rewriting the entire file.
- **`run_command`**: Executes a PowerShell command in a terminal (e.g., `ls`, `dir`).
- **`update_file`**: Completely writes or overwrites an existing file with new content.

And a lot more [incoming](Todo.md) !

## Parameters

- `--auto-approve` parameter : add this in the `mcp.json` > `args` to allow auto approval. **Be careful as it can lead to security issues**. *The vscode copilot extension overrule this parameter **just once per command type and by prompt***.

## Project Structure

```text
MCP-server/
├── LICENSE
├── README.md
└── mcp/
    ├── Cargo.lock
    ├── Cargo.toml
    ├── Makefile
    └── src/
        ├── main.rs       (Point d'entrée, configuration et routing)
        └── tools/        (Outils organisés par groupes logiques)
            ├── mod.rs    (Déclaration des modules)
            ├── cmd.rs    (Outils liés au terminal/commandes)
            ├── file.rs   (Outils liés au système de fichiers)
            └── git.rs    (Outils liés à Git)
```

### Adding a new tool

1. Create a new module file in `mcp/src/tools/` (e.g. `docker.rs` or add to an existing module).
2. Declare the new module in `mcp/src/tools/mod.rs` (e.g., `pub mod docker;`).
3. Add the tool's JSON schema description in the `tools/list` response matching in `main.rs`.
4. Route the tool call in the `tools/call` match block inside `main.rs`.

## Development

To compile the project without running it:

```bash
cargo build
```

To run tests:

```bash
cargo test
```