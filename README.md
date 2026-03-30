# MCP Server

A Model Context Protocol (MCP) server implemented in Rust.

## Description

This project implements an MCP server that allows you to expose tools, resources, or other standardized features via the MCP protocol.

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

## Project Structure

- `src/main.rs`: Main entry point of the MCP server.
- `Cargo.toml`: Project configuration and Rust dependencies.

## Development

To compile the project without running it:

```bash
cargo build
```

To run tests:

```bash
cargo test
```