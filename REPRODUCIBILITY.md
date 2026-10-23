# Reproducibility

Use this standalone repository root. Do not place it inside an unrelated Cargo workspace without configuring that workspace.

Recorded environment: Rust/Cargo 1.98.0, `x86_64-pc-windows-gnu`; Python with UTF-8 mode. The Rust package has zero dependencies. The optional static check requires Python 3.11+ for `tomllib`.

```sh
rustc --version --verbose
cargo --version --verbose
cargo metadata --format-version 1 --locked --offline
cargo test --workspace --all-targets --locked --offline --no-fail-fast -- --test-threads=1
cargo test --doc --locked --offline
cargo build --release --workspace --all-targets --locked --offline
cargo run --release --locked --offline --bin poc007-opls
python -B -X utf8 scripts/reference_oracle.py
python -B -X utf8 scripts/static_sanity.py
```

The recorded run additionally set `RUSTFLAGS=-Dwarnings` and an external `CARGO_TARGET_DIR`. Set these using your shell's normal environment syntax. A platform linker is still required.

Expect 10 unit plus 2 integration tests, zero doctests, and the three README outcomes. The Python oracle also checks missing-negative and scope-drift controls. It rewrites `evidence/reference_oracle_results.json`; use a clean checkout when comparing publication hashes.

[evidence/RESULT.json](evidence/RESULT.json) records command outcomes. [TRANSFORMATIONS.json](evidence/TRANSFORMATIONS.json) binds original and published log hashes. Local paths were replaced with named placeholders; these logs are not byte-identical to their originals. Do not interpret elapsed time as a performance guarantee.
