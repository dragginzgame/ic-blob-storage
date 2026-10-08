#!/usr/bin/env bash
set -euo pipefail
# Cargo's local-registry verifier hits rust-lang/cargo#14396. Assemble with Cargo,
# then verify the exact extracted payload together, preserving external selections.
package_version="$(perl scripts/release/release-data.pl version)"
package_root="${CARGO_TARGET_DIR:-$PWD/target}/package"
mkdir -p "$package_root"
package_verification="$(mktemp -d "$package_root/blob-verification.XXXXXX")"
echo "Package verification retained: $package_verification"
cp Cargo.lock "$package_verification/entry-Cargo.lock"
cargo package --offline --locked --allow-dirty --no-verify \
    -p ic-blob-storage-contracts -p ic-blob-storage
for package_name in ic-blob-storage-contracts ic-blob-storage; do
    tar -xzf "$package_root/$package_name-$package_version.crate" -C "$package_verification"
done
cat > "$package_verification/Cargo.toml" <<EOF
[workspace]
resolver = "3"
members = ["ic-blob-storage-contracts-$package_version", "ic-blob-storage-$package_version"]
[patch.crates-io]
ic-blob-storage-contracts = { path = "ic-blob-storage-contracts-$package_version" }
EOF
cp "$package_verification/entry-Cargo.lock" "$package_verification/Cargo.lock"
cargo metadata --offline --format-version 1 --manifest-path "$package_verification/Cargo.toml" \
    > "$package_verification/metadata.json"
perl scripts/release/release-data.pl package-lock-check \
    "$package_verification/entry-Cargo.lock" "$package_verification/Cargo.lock"
cargo test --offline --locked --manifest-path "$package_verification/Cargo.toml" \
    --workspace --all-targets
cargo test --offline --locked --manifest-path "$package_verification/Cargo.toml" \
    --workspace --doc
cmp Cargo.lock "$package_verification/entry-Cargo.lock"
