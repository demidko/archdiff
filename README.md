# javadiff

The utility calculates the difference of _architecture_ changes between two git branches. This is especially useful when
analyzing large merge requests.

# Install

You need [Rust](https://rustup.rs) installed.

```shell
cargo install --git https://github.com/demidko/javadiff
```

## Usage example

```shell
javadiff master dev
```