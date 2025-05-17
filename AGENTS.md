# Contributor Guidelines

Follow these rules when modifying this repository.

## Formatting

* Use `cargo fmt` before committing if `rustfmt` is installed.
* Keep Rust code compatible with edition 2021.

## Testing

* Run integration tests with:

  ```bash
  cargo test --target wasm32-unknown-unknown -- --nocapture
  ```

  Tests are located in the `tests/` directory and rely on `wasm-bindgen-test`.
* Add or update tests when changing functionality.

## Building

* Build the WebAssembly package with:

  ```bash
  wasm-pack build --target web
  ```

* For a local development server run:

  ```bash
  python3 -m http.server 8000
  ```

## Commit Messages

* Write concise commit messages in present tense.
* Summaries should stay under 72 characters.

## Documentation

* Update `README.md` when user-facing behavior or setup steps change.
