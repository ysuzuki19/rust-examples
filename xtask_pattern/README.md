# xtask pattern

This directory contains custom build and development tasks for this project, implemented as a separate Rust binary crate. This pattern is often referred to as the "xtask" pattern.

## Purpose

The purpose of using an xtask is to encapsulate project-specific tasks that are not part of the main application logic and to manage CLI tool dependencies within the project. This approach provides several benefits:

- **No global installation required**: CLI tools are managed within the project
- **Project-specific versioning**: Each project can use different versions of the same tools
- **Consistent CI/local environments**: Ensures the same tool versions and dependency resolution across local development and CI pipelines

This can include tasks such as:

- Code generation
- Documentation generation
- Running linters or formatters
- Custom build steps
- Deployment scripts

## Usage

To run the xtask, you can use Cargo's `run` command with the `--package` flag to specify the xtask crate. For example:

```sh
cargo run --package xtask -- <task-name> [args]
```

## Aliases

To simplify the execution of xtasks, you can create `.cargo/config.toml` aliases like so:

```toml
[alias]
xtask = "run --package xtask -- "
sqlx = "run --package xtask -- sqlx"
```

With this alias, you can run an xtask with:

```sh
cargo xtask sqlx [args]
cargo sqlx [sqlx-args]
```
