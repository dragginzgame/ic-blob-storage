#!/usr/bin/env bash
# Artifact and lint closure of the selected installed consumers, not a test runner.
set -euo pipefail
plan="${BLOB_VALIDATION_PLAN:?}"
goals=()
tests=()
while IFS= read -r suite; do
    tests+=(--test "${suite#installed-}")
    case "$suite" in
        installed-standalone|installed-capacity|installed-signed) goals+=(build-host) ;;
        installed-endpoints) goals+=(build-host build-reply-probe) ;;
        installed-application) goals+=(build-host build-reference-app artifact-cli) ;;
        installed-enrollment) goals+=(build-enrollment-probe build-host) ;;
        installed-callbacks) goals+=(build-callback-probe artifact-cli build-reference-app) ;;
        installed-management) goals+=(build-host artifact-cli) ;;
        *) exit 2 ;;
    esac
done < <(jq -r '.stages[] | select(.required and (.target | startswith("installed-"))) | .target' "$plan")
[[ ${#goals[@]} -gt 0 ]]
# Full qualification lint already covers these targets with the same features.
if ! jq -e '.stages[] | select(.target == "check-qualification") | .required' "$plan" >/dev/null; then
    CARGO_BUILD_WARNINGS=deny cargo clippy --locked --manifest-path qualification/Cargo.toml \
        -p ic-blob-storage-tests --example prepare-server "${tests[@]}" -- -D warnings
    if printf '%s\n' "${tests[@]}" | grep -qx enrollment; then
        CARGO_BUILD_WARNINGS=deny cargo clippy --locked --manifest-path qualification/Cargo.toml \
            -p ic-blob-storage-enrollment-probe --lib --tests --examples -- -D warnings
    fi
fi
make --no-print-directory prepare-test-server "${goals[@]}"
