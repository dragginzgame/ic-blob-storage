#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && ( "$1" == setup || "$1" == check ) ]] || {
    echo 'usage: release-tools.sh setup|check' >&2; exit 2;
}
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=ci/release-tools.env
source "$root/ci/release-tools.env"
args=(--consumer "$root" --package cargo-edit --version "$BLOB_CARGO_EDIT_VERSION"
    --bin cargo-set-version --profile release)
[[ "$1" != check ]] || args+=(--check)
exec bash "$root/scripts/dev/install-rust-tools.sh" "${args[@]}"
