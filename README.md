# archdiff

The utility calculates the difference of _architecture_ changes between two git branches. This is especially useful when
analyzing large merge requests.

These languages are supported:

[![](https://img.shields.io/badge/Java-EA7100?style=for-the-badge&logo=openjdk)](#archdiff)

## Install

You need [Rust](https://rustup.rs) installed.

```shell
cargo install --git https://github.com/demidko/archdiff
```

## Usage

```shell
archdiff [old_branch] [new_branch]
```
