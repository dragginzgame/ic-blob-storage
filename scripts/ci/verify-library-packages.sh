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
package_version="$(perl "$root/scripts/release/release-data.pl" version)"
package_root="${CARGO_TARGET_DIR:-$PWD/target}/package"
mkdir -p "$package_root"
package_verification="$(mktemp -d "$package_root/blob-verification.XXXXXX")"
echo "Package verification retained: $package_verification"
# Keep the original tool status and all partial evidence on failure.
status=0
"$verifier" "$PWD" "$package_root" "$package_verification" "$package_version" || status=$?
if [[ "$status" != 0 ]]; then
    printf 'Package verification failed; evidence retained: %s\n' "$package_verification" >&2
fi
exit "$status"
