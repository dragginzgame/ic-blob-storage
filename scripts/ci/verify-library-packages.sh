#!/usr/bin/env bash
set -euo pipefail
# Cargo's local-registry verifier hits rust-lang/cargo#14396. Assemble with Cargo,
# then verify the exact extracted payload together, preserving external selections.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
data="$root/scripts/release/release-data.pl"
checksum="$root/scripts/ci/verify-file-checksum.sh"
package_version="$(perl "$data" version)"
package_root="${CARGO_TARGET_DIR:-$PWD/target}/package"
mkdir -p "$package_root"
package_verification="$(mktemp -d "$package_root/blob-verification.XXXXXX")"
echo "Package verification retained: $package_verification"
verification_complete=false
finish_verification() {
    local status=$?
    [[ "$verification_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" != 0 ]]; then
        printf 'Package verification failed; evidence retained: %s\n' "$package_verification" >&2
    fi
    exit "$status"
}
trap finish_verification EXIT
cp Cargo.lock "$package_verification/entry-Cargo.lock"
cargo metadata --offline --locked --no-deps --format-version 1 \
    > "$package_verification/entry-metadata.json"
jq -je '[.packages[] | select(.name == "ic-blob-storage-contracts" or .name == "ic-blob-storage")] |
    if length != 2 then error("expected two coordinated packages")
    else .[].manifest_path + "\u0000" end' "$package_verification/entry-metadata.json" \
    > "$package_verification/entry-manifests"
manifests=(Cargo.toml)
while IFS= read -r -d '' manifest; do manifests+=("$manifest"); done < "$package_verification/entry-manifests"
for index in "${!manifests[@]}"; do
    cp -p "${manifests[$index]}" "$package_verification/entry-manifest-$index"
done
cargo package --offline --locked --allow-dirty --no-verify \
    -p ic-blob-storage-contracts -p ic-blob-storage
archive_hashes=()
for package_name in ic-blob-storage-contracts ic-blob-storage; do
    archive_hashes+=("$(bash "$checksum" --print sha256 "$package_root/$package_name-$package_version.crate")")
    tar -xzf "$package_root/$package_name-$package_version.crate" -C "$package_verification"
done
printf '%s\n' "${archive_hashes[@]}" > "$package_verification/archive.sha256"
perl "$data" package-sources "$package_verification" "$package_version" \
    > "$package_verification/entry-sources.json"
cat > "$package_verification/Cargo.toml" <<EOF
[workspace]
resolver = "3"
members = ["ic-blob-storage-contracts-$package_version", "ic-blob-storage-$package_version"]
[patch.crates-io]
ic-blob-storage-contracts = { path = "ic-blob-storage-contracts-$package_version" }
EOF
cp "$package_verification/entry-Cargo.lock" "$package_verification/Cargo.lock"
# Cargo archives carry fixed source mtimes. Reusing a target from another extracted
# payload can admit stale path-dependency artifacts with the same package identity.
# Keep compilation and metadata inside this retained, unique verification run.
export CARGO_TARGET_DIR="$package_verification/target"
cargo metadata --offline --format-version 1 --manifest-path "$package_verification/Cargo.toml" \
    > "$package_verification/metadata.json"
perl "$data" package-graph-check "$package_verification/metadata.json" "$package_verification" "$package_version"
perl "$data" package-lock-check \
    "$package_verification/entry-Cargo.lock" "$package_verification/Cargo.lock"
cp "$package_verification/Cargo.toml" "$package_verification/admitted-Cargo.toml"
cp "$package_verification/Cargo.lock" "$package_verification/admitted-Cargo.lock"
cargo test --offline --locked --manifest-path "$package_verification/Cargo.toml" \
    --workspace --all-targets
cargo test --offline --locked --manifest-path "$package_verification/Cargo.toml" \
    --workspace --doc
cmp Cargo.lock "$package_verification/entry-Cargo.lock"
for index in "${!manifests[@]}"; do
    cmp "${manifests[$index]}" "$package_verification/entry-manifest-$index"
done
index=0
for package_name in ic-blob-storage-contracts ic-blob-storage; do
    bash "$checksum" sha256 "${archive_hashes[$index]}" "$package_root/$package_name-$package_version.crate"
    index=$((index + 1))
done
perl "$data" package-sources "$package_verification" "$package_version" \
    > "$package_verification/final-sources.json"
cmp "$package_verification/entry-sources.json" "$package_verification/final-sources.json"
cmp "$package_verification/Cargo.toml" "$package_verification/admitted-Cargo.toml"
cmp "$package_verification/Cargo.lock" "$package_verification/admitted-Cargo.lock"
verification_complete=true
printf 'Coordinated archive graph, tests and final integrity checks passed.\n'
