#!/usr/bin/env bash
set -Eeuo pipefail

# Consumer integration only; Shared Tooling owns logger mechanics and its suite.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
task_tmp="${TMPDIR:-$ROOT/target/tmp}"
mkdir -p "$task_tmp"
FIXTURE="$(mktemp -d "$task_tmp/blob-validation-logging.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$fixture_complete" == true && "$status" == 0 ]]; then rm -rf "$FIXTURE";
    else printf 'Validation logging fixture retained: %s\n' "$FIXTURE" >&2; fi
    exit "$status"
}
trap finish EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES
# Synthetic targets own their evidence, never the parent gate's logs/summary.
unset VALIDATION_LOG_DIR VALIDATION_FAILURE_LOG_DIR GITHUB_STEP_SUMMARY
# Exercise the actual consumer stage loop, including a nested gate's inherited
# checkpoint. Only disposable inputs change; no product build or release runs.
for input_case in stable drift deleted; do
    gate="$FIXTURE/gate-$input_case"
    mkdir -p "$gate/make" "$gate/ci" "$gate/scripts/ci" "$gate/scripts/release"
    cp "$ROOT/Makefile" "$ROOT/Cargo.toml" "$ROOT/Cargo.lock" "$gate/"
    cp "$ROOT"/make/{consumer,tools,release,rust-format,execution}.mk "$gate/make/"
    # Keep the actual runner and its declared companion closure together.
    bash "$ROOT/scripts/ci/prepare-script-fixture.sh" "$ROOT" "$gate" \
        scripts/ci/run-validation-targets.sh scripts/ci/validation-evidence.pl
    cp "$ROOT/ci/cargo-release-groups.json" "$gate/ci/"
    cp "$ROOT/scripts/release/release-data.pl" "$ROOT/scripts/release/validation.pl" "$gate/scripts/release/"
    printf '%s\n' '{"groups":["contracts","client","engine"],"schema":1,"stages":["fixture-pass","fixture-change","fixture-late"]}' > "$gate/ci/release-validation.json"
    cat >> "$gate/Makefile" <<'MAKE'
CI_INPUTS := Cargo.toml Cargo.lock
.PHONY: fixture-pass fixture-change fixture-late
fixture-pass:
	@echo first-stage
fixture-change:
	@case "$$BLOB_INPUT_CASE" in drift) printf '\nchanged\n' >> Cargo.lock ;; deleted) rm Cargo.lock ;; stable) : ;; esac
fixture-late:
	@touch late-dispatched
MAKE
    status=0
    BLOB_INPUT_CASE="$input_case" VALIDATION_REPOSITORY_ROOT="$gate" \
        VALIDATION_LOG_DIR="$gate/logs" VALIDATION_FAILURE_LOG_DIR="$gate/failures" \
        bash "$gate/scripts/ci/run-validation-targets.sh" --fail-fast \
            --input Cargo.toml --input Cargo.lock ci > "$gate/result.log" 2>&1 || status=$?
    if [[ "$input_case" == stable ]]; then
        [[ "$status" == 0 && -f "$gate/late-dispatched" ]] || exit 1
    else
        [[ "$status" != 0 && ! -e "$gate/late-dispatched" ]] || exit 1
        # Nested runners retain diagnostics even when console excerpts are omitted.
        failure_log="$gate/failures/latest-combined.log"
        [[ -s "$failure_log" ]] || exit 1
        if [[ "$input_case" == drift ]]; then
            rg -F 'validation input changed: Cargo.lock' "$failure_log" >/dev/null
        else
            rg -F 'cannot observe validation input Cargo.lock' "$failure_log" >/dev/null
        fi
        rg -F 'Restart preparation and the full gate' "$failure_log" >/dev/null
        rg --files "$gate/logs" | rg '/inputs.json$' >/dev/null
    fi
done
echo 'Consumer CI stages reject changed/deleted inputs and retain nested checkpoints'
fixture_complete=true
