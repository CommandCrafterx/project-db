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

```bash
cargo run
```

## Data Storage

The Program currently stores the data in a data.json file in the project folder.

## Dependencies

- [serde](https://serde.rs/) / [serde_json](https://github.com/serde-rs/json): JSON conversion
- [dialoguer](https://github.com/console-rs/dialoguer): Interactive CLI prompts

## License

This project is MIT licensed.