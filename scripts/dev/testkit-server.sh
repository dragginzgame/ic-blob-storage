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
tools_root="${BLOB_PREPARED_TOOLS_ROOT:-$root}"
if [[ "$1" == setup && "$tools_root" != "$root" ]]; then
    echo 'Borrowed prepared tools are check-only.' >&2
    exit 1
fi
args=(--consumer "$tools_root" --package ic-testkit --lockfile "$root/Cargo.lock"
    --bin ic-testkit-server --profile release --features cli)
[[ "$1" != check ]] || args+=(--check)
cli="$(bash "$root/scripts/dev/install-rust-tools.sh" "${args[@]}")"
exec "$cli" "$1" --directory "$tools_root/.tools/testkit-server"
