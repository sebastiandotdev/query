# Contributing

Thanks for your interest in contributing to Rusty Fetcher! Please take a moment
to review this document **before submitting a pull request**.

## Pull requests

**Ask first before you start working on any major new features.**

## Coding standards

Our code formatting rules are defined in [.rustfmt.toml] You can check your code
against these standards by running:

```sh
cargo fmt --all -- --check
```

```sh
cargo clippy --all -- -D warnings
```

## Running building

You can run the test suite using the following commands:

```sh
cargo build && cargo test
```

Please ensure that the tests are passing when submitting a pull request. If
you're adding new features to Rusty Fecther, please include tests.
