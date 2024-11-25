# Welcome to XRA contributing Guide

Thank you for investing your time in helping out with XRA's development.

If you notice you're missing any particular piece of information here, please [open an issue](https://github.com/Proximie/x-result-analyzer/issues/new) or pull request.

## Bird's-eye view of XRA

XRA is made up of two crates - let's go over them:

- [`xra_core`](/xra_core) The core of XRA it contains the logic and data access layer
- [`xra`](/xra) The frontend of the tool, it parses the command line args then uses the underlying core crate

## Running XRA

You could use the usual `cargo run ...` command, but we prefer to use

```sh
cargo xra <command>
```
