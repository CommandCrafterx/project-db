# project-db

[![CI](https://img.shields.io/github/actions/workflow/status/CommandCrafterx/project-db/rust.yml?branch=master&style=flat-square)](https://github.com/CommandCrafterx/project-db/actions/workflows/rust.yml)
[![Rust](https://img.shields.io/badge/rust-2024-dea584?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/github/license/CommandCrafterx/project-db?style=flat-square)](https://github.com/CommandCrafterx/project-db/blob/master/LICENSE)

A simple CLI project database manager written in Rust, that lets you manage your current programming projects.

## Features

- Create projects with a name and description
- List all stored projects
- Save and load application data from a JSON file
- Interactive prompt-based interface

## Usage
### Installation:
One line installation:
```bash
cargo install --git https://github.com/CommandCrafterx/project-db
```
From the project-db source code:
```bash
cargo install --path .
```

### Usage
```bash
Usage: project-db <COMMAND>

Commands:
  new     Create a new Project
  delete  Delete a Project
  list    List all Projects
  help    Print this message or the help of the given subcommand(s)
```

## Data Storage

The Program stores its data in a dedicated folder in "~/.local/share".

## Dependencies

- [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json): JSON conversion
- [dialoguer](https://github.com/console-rs/dialoguer): Interactive CLI prompts
- [clap](https://docs.rs/clap/latest/clap/): CLI argument parsing

## License

This project is MIT licensed.