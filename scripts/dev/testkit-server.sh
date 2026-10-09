#!/usr/bin/env bash
set -euo pipefail

# Blob selects the CLI from its lock; Shared installs it and Testkit owns servers.
[[ $# == 1 && ( "$1" == setup || "$1" == check ) ]] || {
    echo 'usage: testkit-server.sh setup|check' >&2; exit 2;
}
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# Cargo-generated lockfiles use one flat package record per selected identity.
# Refuse missing/ambiguous selections rather than inventing another version pin.
version="$(awk -F '"' '
    /^\[\[package\]\]$/ { selected = 0 }
    /^name = / { selected = ($2 == "ic-testkit") }
    selected && /^version = / { print $2; count++ }
    END { if (count != 1) exit 1 }
' "$root/Cargo.lock")" || { echo 'Select exactly one ic-testkit in Cargo.lock' >&2; exit 1; }
args=(--consumer "$root" --package ic-testkit --version "$version"
    --bin ic-testkit-server --profile release)
[[ "$1" != check ]] || args+=(--check)
cli="$(bash "$root/scripts/dev/install-rust-tools.sh" "${args[@]}")"
exec "$cli" "$1" --directory "$root/.tools/testkit-server"
