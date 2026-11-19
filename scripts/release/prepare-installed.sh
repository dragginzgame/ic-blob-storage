#!/usr/bin/env bash
# Artifact and lint closure of the selected installed consumers, not a test runner.
set -euo pipefail
plan="${BLOB_VALIDATION_PLAN:?}"
# Finish parsing before selecting any work. Process substitution hides jq failures.
required=$(jq -ers '
    if length != 1 then error("expected one validation plan") else
        .[0].stages[] | select(.required) | .target
        | select(. == "check-qualification" or startswith("installed-")) | [.] | @tsv
    end
' "$plan")
goals=()
tests=()
full_lint=false
enrollment=false
while IFS= read -r suite; do
    case "$suite" in
        check-qualification) full_lint=true; continue ;;
        installed-standalone|installed-capacity|installed-signed) goals+=(build-host) ;;
        installed-endpoints) goals+=(build-host build-reply-probe) ;;
        installed-application) goals+=(build-host build-reference-app artifact-cli) ;;
        installed-enrollment) goals+=(build-enrollment-probe build-host); enrollment=true ;;
        installed-callbacks) goals+=(build-callback-probe artifact-cli build-reference-app) ;;
        installed-management) goals+=(build-host artifact-cli) ;;
        *) exit 2 ;;
    esac
    tests+=(--test "${suite#installed-}")
done <<< "$required"
[[ ${#goals[@]} -gt 0 ]]
# Full qualification lint already covers these targets with the same features.
if [[ "$full_lint" == false ]]; then
    CARGO_BUILD_WARNINGS=deny cargo clippy --locked --manifest-path qualification/Cargo.toml \
        -p ic-blob-storage-tests --example prepare-server "${tests[@]}" -- -D warnings
    if [[ "$enrollment" == true ]]; then
        CARGO_BUILD_WARNINGS=deny cargo clippy --locked --manifest-path qualification/Cargo.toml \
            -p ic-blob-storage-enrollment-probe --lib --tests --examples -- -D warnings
    fi
fi
make --no-print-directory prepare-test-server "${goals[@]}"
