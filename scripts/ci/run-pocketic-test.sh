#!/usr/bin/env bash
set -euo pipefail

# Admit the default before Cargo; explicit binary admission remains caller-owned.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
if [[ -z ${POCKET_IC_BIN+x} ]]; then
    POCKET_IC_BIN="$(bash "$root/scripts/dev/testkit-server.sh" check)"
    export POCKET_IC_BIN
fi
exec bash "$root/scripts/ci/run-nonempty-cargo-test.sh" "$@"
