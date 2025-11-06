# Contributing to EDTest

Thanks for taking the time to contribute!

## Getting started

1. Fork the repository and create your branch from `main`.
2. Ensure you have a recent stable Rust toolchain (edition 2021).
3. Run the test suite at the workspace root:

   ```sh
   cargo test --all -q
   ```

4. Keep the code formatted and warnings clean:

   ```sh
   cargo fmt --all
   cargo clippy --all-targets --all-features -D warnings
   ```

## Commit and PR guidelines

- Keep commits focused; prefer small, self‑contained changes.
- Include tests for new behavior or bug fixes when possible.
- Update documentation and examples as needed.
- Open a Pull Request with a clear description of the change and motivation.

## Crate versions and MSRV

- The workspace targets stable Rust; raising MSRV should be discussed in an issue first.
- Crates in this workspace are versioned together. Changes to public API may require a semver bump.

## License

By contributing, you agree that your contributions will be licensed under the dual MIT/Apache‑2.0 license.
