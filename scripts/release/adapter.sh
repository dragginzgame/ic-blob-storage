#!/usr/bin/env bash
# Blob policy only; Toko owns release ordering, effect custody and Git delivery.
set -euo pipefail
export PATH="$PWD/.tools/host/bin:$PWD/.tools/rust/common/bin:$PATH"
[[ $# == 1 ]] || exit 2
metadata=(Cargo.toml Cargo.lock CHANGELOG.md qualification/Cargo.lock)
case "$1" in
    version)
        exec bash vendor/toko-tooling/scripts/ci/read-cargo-workspace-version.sh --stable Cargo.toml ;;
    preflight)
        bash vendor/toko-tooling/scripts/ci/check-release-source.sh
        bash vendor/toko-tooling/scripts/ci/check-release-changelog.sh "${RELEASE_VERSION:?}" "${RELEASE_PREVIOUS:?}" "${RELEASE_DATE:?}" CHANGELOG.md
        make --no-print-directory deps install-tools release-editor-setup
        ;;
    prepare)
        CARGO_RELEASE_METADATA_EDITOR="$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$PWD" --package ic-build --version 0.73.0 --bin ic-build-release-metadata --profile debug --features cli --check)"
        export CARGO_RELEASE_METADATA_EDITOR CARGO_RELEASE_METADATA_MAX_BYTES=8388608
        exec bash scripts/release/metadata.sh prepare ;;
    check|commit-check)
        allowed=()
        for path in "${metadata[@]}"; do allowed+=(--allow "$path"); done
        bash vendor/toko-tooling/scripts/ci/check-release-source.sh "${allowed[@]}"
        if [[ "$1" == commit-check ]]; then git diff --exit-code; fi
        bash vendor/toko-tooling/scripts/ci/cargo-release-metadata.sh check ic-blob-storage
        bash scripts/release/metadata.sh selected
        ;;
    publication)
        bash vendor/toko-tooling/scripts/ci/check-release-source.sh
        version="$(bash "$0" version)"
        [[ "$(git cat-file -t "v$version")" == tag ]]
        [[ "$(git rev-parse "v$version^{commit}")" == "$(git rev-parse HEAD)" ]]
        RELEASE_VERSION="$version" RELEASE_DATE="$(git show -s --format=%cs HEAD)" bash "$0" check
        ;;
    *) echo 'expected version|preflight|prepare|check|commit-check|publication' >&2; exit 2 ;;
esac
