#!/bin/sh

set -e

echo Checking syntax...
cargo check # native
RUSTFLAGS='--cfg=getrandom_backend="wasm_js"' cargo check --target wasm32-unknown-unknown # web

echo Running tests...
cargo test --workspace --all-features # native
RUSTFLAGS='--cfg=getrandom_backend="wasm_js"' wasm-pack test --node # web

echo Running linter check...
cargo clippy --workspace --all-targets --all-features -- -D warnings # native
RUSTFLAGS='--cfg=getrandom_backend="wasm_js"' cargo clippy --workspace --target wasm32-unknown-unknown --all-targets --all-features -- -D warnings # web

echo Running formatting check...
cargo fmt --all -- --check

echo Checking cargo doc...
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps

echo Building book...
sh -c "cd guide && mdbook build"

echo Running mdbook tests...
sh -c "cd guide && mdbook test"

echo All done!

# End of file
