## TODO

- [x] Precise mode: ensure 100% correctness via e.g., `cargo nextest list --message-format json` / `cargo test --all-targets -- --list -Zunstable-options --format json`. This need to compile the crate.
- [ ] Automatic fix
- [ ] Quick mode: simply search `#[test]`. If not found, we mark `test = false` for all the crate's targets (lib, bin, example, bench).
    - This is less precise, because even if some tests are found, some targets e.g., bin doesn't necessarily need to be tested. However, it can be complicated to determine. e.g., when `autobins = false`, or when a binary contains submodules.
- [ ] Support also doctests and benchmarks.
