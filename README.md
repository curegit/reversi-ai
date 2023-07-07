# Reversi AI

Heuristic Reversi AI implemented as a Rust "cdylib" library

It is a ported version of the original [Reversi AI DLL for Windows](https://github.com/curegit/reversi-ai-dll).

For a detailed explanation of the project, including documentation and API usage, please refer to the original project. This ported version aims to maintain the same functionality and performance as the original, while taking advantage of the portability that the Rust programming language offers.

## Build

```sh
cargo build --release
```

## Build Docs (Japanese)

```sh
cargo doc --no-deps
```

## Run Tests

```sh
cargo test
```

## License

[Apache License 2.0](LICENSE)
