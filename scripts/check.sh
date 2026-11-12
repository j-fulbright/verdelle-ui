#!/bin/sh
# Everything that must pass before a commit. Run from anywhere: ./scripts/check.sh
set -e
cd "$(dirname "$0")/.."

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy --workspace --target wasm32-unknown-unknown --all-targets -- -D warnings
# The gallery's real code paths only compile with its features on.
cargo clippy -p verdelle-gallery --no-default-features --features ssr --all-targets -- -D warnings
cargo clippy -p verdelle-gallery --no-default-features --features hydrate --target wasm32-unknown-unknown -- -D warnings
cargo test --workspace
cargo run -q -p verdelle-ui --example themegen -- --check
echo "all checks passed"
