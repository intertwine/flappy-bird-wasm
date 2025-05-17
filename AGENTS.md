# Contributor Guidelines

Follow these rules when modifying this repository.

## Formatting

- Use `cargo fmt` before committing if `rustfmt` is installed.
- Keep Rust code compatible with edition 2021.

## Testing

- Run integration tests with:

  ```bash
  # Using npm
  npm test              # Run tests in Chrome
  npm run test:firefox  # Run tests in Firefox
  npm run test:chrome   # Run tests in Chrome
  npm run test:node     # Run tests in Node.js

  # Or using cargo directly
  cargo test --target wasm32-unknown-unknown -- --nocapture
  ```

  Tests are located in the `tests/` directory and rely on `wasm-bindgen-test`.

- **Browser-Based Tests**: Tests are configured to run in a browser environment using `wasm_bindgen_test_configure!(run_in_browser);`. To run tests:

  - **Firefox**: Use `npm run test:firefox` or `wasm-pack test --headless --firefox`.
  - **Chrome**: Use `npm run test:chrome` or `wasm-pack test --headless --chrome`.
  - **Node.js**: Use `npm run test:node` or `wasm-pack test --node`.

- **Test Configuration**: The crate is configured with both `cdylib` and `rlib` crate types to support integration tests. Test-specific methods (e.g., `new_headless` and `set_score`) are made available for testing but hidden from public documentation.

- Add or update tests when changing functionality.

## Building

- Install dependencies:

  ```bash
  # Install Rust dependencies
  cargo build

  # Install npm dependencies and download game assets
  npm install
  ```

  The `npm install` command will automatically download required game assets to the `assets/` directory. If you need to manually download assets, run:

  ```bash
  npm run download-assets
  ```

- Build the WebAssembly package with:

  ```bash
  # Using npm
  npm run build

  # Or using wasm-pack directly
  wasm-pack build --target web
  ```

- For a local development server run:

  ```bash
  # Using npm
  npm start

  # Or using Python directly
  python3 -m http.server 8000
  ```

## Commit Messages

- Write concise commit messages in present tense.
- Summaries should stay under 72 characters.

## Documentation

- Update `README.md` when user-facing behavior or setup steps change.
- Keep `package.json` scripts in sync with documentation.
