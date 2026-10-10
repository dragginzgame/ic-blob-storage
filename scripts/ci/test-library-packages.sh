#!/usr/bin/env bash
set -euo pipefail
# Exercise changed same-version archives against a warm target with real Cargo.
verifier="$PWD/scripts/ci/verify-library-packages.sh"
data="$PWD/scripts/release/release-data.pl"
fixture_parent="${CARGO_TARGET_DIR:-$PWD/target}"
mkdir -p "$fixture_parent"
fixture="$(mktemp -d "$fixture_parent/package-fixture.XXXXXX")"
echo "Package regression retained: $fixture"
fixture_complete=false
finish_fixture() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" != 0 ]]; then
        printf 'Package regression failed; fixture retained: %s\n' "$fixture" >&2
    fi
    exit "$status"
}
trap finish_fixture EXIT
mkdir -p "$fixture/contracts/src" "$fixture/service/src" "$fixture/scripts/release"
cp "$data" "$fixture/scripts/release/release-data.pl"
cd "$fixture"
git init -q
export CARGO_TARGET_DIR="$fixture/target"
cat > Cargo.toml <<'TOML'
[workspace]
resolver = "3"
members = ["contracts", "service"]
[workspace.package]
version = "0.17.2"
TOML
cat > contracts/Cargo.toml <<'TOML'
[package]
name = "ic-blob-storage-contracts"
version.workspace = true
edition = "2024"
[dependencies]
cfg-if = "=1.0.4"
TOML
cat > service/Cargo.toml <<'TOML'
[package]
name = "ic-blob-storage"
version.workspace = true
edition = "2024"
[dependencies]
ic-blob-storage-contracts = { path = "../contracts", version = "0.17.2" }
[[bin]]
name = "package-value"
path = "src/main.rs"
TOML
printf 'pub fn prior() -> u32 { 1 }\n' > contracts/src/lib.rs
printf 'pub fn value() -> u32 { ic_blob_storage_contracts::prior() }\n' > service/src/lib.rs
printf 'fn main() { println!("{}", ic_blob_storage::value()); }\n' > service/src/main.rs
cargo generate-lockfile --offline > lock.log 2>&1
bash "$verifier" > first.log 2>&1
first_metadata=""
for candidate in "$CARGO_TARGET_DIR"/package/blob-verification.*/metadata.json; do first_metadata="$candidate"; done
# Deliberately seed the shared target with the first extracted contract and caller.
first_manifest="$(dirname "$first_metadata")/Cargo.toml"
value="$(cargo run --offline --locked --manifest-path "$first_manifest" -p ic-blob-storage --bin package-value 2> warm.log)"
[[ "$value" == 1 ]] || exit 1
printf 'pub fn current() -> u32 { 2 }\n' > contracts/src/lib.rs
printf 'pub fn value() -> u32 { ic_blob_storage_contracts::current() }\n' > service/src/lib.rs
bash "$verifier" > second.log 2>&1
second_metadata=""
for candidate in "$CARGO_TARGET_DIR"/package/blob-verification.*/metadata.json; do
    [[ "$candidate" == "$first_metadata" ]] || second_metadata="$candidate"
done
second_manifest="$(dirname "$second_metadata")/Cargo.toml"
# Execute the new payload using the target actually reported by its verifier.
verified_target="$(perl -MJSON::PP -0777 -ne 'print decode_json($_)->{target_directory}' "$second_metadata")"
value="$(CARGO_TARGET_DIR="$verified_target" cargo run --offline --locked --manifest-path "$second_manifest" -p ic-blob-storage --bin package-value 2> current.log)"
if [[ "$value" != 2 ]]; then
    echo "New archive executed stale contract value: $value" >&2
    exit 1
fi
echo 'Same-version archive regression passed (warm shared target; current contract executed).'
fixture_complete=true
