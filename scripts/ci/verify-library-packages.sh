#!/usr/bin/env bash
set -euo pipefail
# Blob selects packages/tools and retains its evidence root; IC Build owns
# exact extracted graph admission, isolated compilation and final integrity.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
verifier="${BLOB_PACKAGE_VERIFIER:-$root/target/debug/blob-package-verify}"
[[ "$verifier" == /* && -f "$verifier" && -x "$verifier" ]] || {
    echo 'Prepare the native verifier with make package-verifier' >&2; exit 1;
}
package_root="${CARGO_TARGET_DIR:-$PWD/target}/package"
mkdir -p "$package_root"
package_verification="$(mktemp -d "$package_root/blob-verification.XXXXXX")"
echo "Package verification retained: $package_verification"
cargo metadata --offline --locked --no-deps --format-version 1 > "$package_verification.workspace.json"
versions=()
for package in ic-blob-storage-contracts ic-blob-storage-client ic-blob-storage; do
    versions+=("$(jq -er --arg name "$package" '. as $m | .packages[] | select(.name == $name and (.id as $id | $m.workspace_members | index($id))) | .version' "$package_verification.workspace.json")")
done
# Keep the original tool status and all partial evidence on failure.
status=0
"$verifier" "$PWD" "$package_root" "$package_verification" "${versions[@]}" || status=$?
if [[ "$status" != 0 ]]; then
    printf 'Package verification failed; evidence retained: %s\n' "$package_verification" >&2
fi
exit "$status"
