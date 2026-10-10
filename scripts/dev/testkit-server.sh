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
args=(--consumer "$root" --package ic-testkit --lockfile Cargo.lock
    --bin ic-testkit-server --profile release)
[[ "$1" != check ]] || args+=(--check)
cli="$(bash "$root/scripts/dev/install-rust-tools.sh" "${args[@]}")"
exec "$cli" "$1" --directory "$root/.tools/testkit-server"
