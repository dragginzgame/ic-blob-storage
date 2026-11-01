#!/usr/bin/env bash
# Consumer selection only; Shared owns group admission and version rendering.
set -euo pipefail
case "${RELEASE_GROUP:-}" in contracts|client|engine) ;; *) echo 'Select RELEASE_GROUP=contracts, client or engine' >&2; exit 2 ;; esac
[[ $# == 1 ]] || exit 2
case "$1" in
    version|files|plan) exec bash scripts/ci/cargo-release-group.sh "$1" "$RELEASE_GROUP" ;;
    primary|changelog)
        plan="$(bash scripts/ci/cargo-release-group.sh plan "$RELEASE_GROUP")"
        jq -er --arg field "$1" '.[$field]' <<< "$plan" ;;
    *) exit 2 ;;
esac
