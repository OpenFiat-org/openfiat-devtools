# fuzzing

Fuzz targets for OpenFiat, using [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz).

```bash
cargo install cargo-fuzz
cargo +nightly fuzz run parse_stub
```

`src/lib.rs` holds stand-in functions; real parsers/serializers from
`openfiat-core` will replace them here as they land. `cargo-fuzz` requires a
nightly toolchain and was not run as part of this repository's CI bootstrap
(not installed in the environment that generated this scaffold) — verify
locally before relying on it.
