#!/usr/bin/env bash
set -Eeuo pipefail

# Consumer integration only; Shared Tooling owns logger mechanics and its suite.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
FIXTURE="$(mktemp -d "${TMPDIR:-/tmp}/blob-validation-logging.XXXXXX")"
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
mkdir "$FIXTURE/logs"
cat > "$FIXTURE/Makefile" <<'MAKE'
.PHONY: pass fail second
pass:
	@printf '%s\n' 'test error::passing ... ok' 'test error::skipped ... ignored'
fail:
	@printf '%s\n' 'test error::context ... ok' 'error[E0308]: actual-diagnostic' 'error:no-space-diagnostic' 'test error::broken ... FAILED'
	@exit 9
second:
	@printf '%s\n' 'error:second-target-diagnostic'
	@exit 13
MAKE
export VALIDATION_REPOSITORY_ROOT="$FIXTURE" VALIDATION_RUNNER_DEPTH=0
export VALIDATION_RUNNER_SNAPSHOT_PATH='' VALIDATION_FAILURE_LOG_DIR="$FIXTURE/logs"
(
    cd "$FIXTURE"
    bash "$ROOT/scripts/ci/run-validation-targets.sh" pass
) > "$FIXTURE/pass.log" 2>&1
if rg -F '[ERR:' "$FIXTURE/pass.log" >/dev/null; then
    echo 'Passing Rust paths were highlighted as errors' >&2; exit 1
fi
status=0
(
    cd "$FIXTURE"
    bash "$ROOT/scripts/ci/run-validation-targets.sh" pass fail
) > "$FIXTURE/fail.log" 2>&1 || status=$?
[[ "$status" == 2 ]] || exit 1
for log in "$FIXTURE/fail.log" "$FIXTURE/logs/latest-errors.log"; do
    for diagnostic in \
        '[ERR:fail] error[E0308]: actual-diagnostic' \
        '[ERR:fail] error:no-space-diagnostic' \
        '[ERR:fail] test error::broken ... FAILED' \
        '[fail] test error::context ... ok'; do
        rg -F "$diagnostic" "$log" >/dev/null || exit 1
    done
    if rg -F '[ERR:fail] test error::context ... ok' "$log" >/dev/null; then exit 1; fi
done
rg -F 'test error::context ... ok' "$FIXTURE/logs/latest.log" >/dev/null
rg -F 'error[E0308]: actual-diagnostic' "$FIXTURE/logs/latest.log" >/dev/null
if rg -F '[ERR:' "$FIXTURE/logs/latest.log" >/dev/null; then exit 1; fi
status=0
(
    cd "$FIXTURE"
    VALIDATION_FAILURE_LOG_DIR="$FIXTURE/combined-logs" \
        bash "$ROOT/scripts/ci/run-validation-targets.sh" fail pass second
) > "$FIXTURE/combined.log" 2>&1 || status=$?
[[ "$status" == 2 ]] || exit 1
# Consumer-selected batches retain every failed target in order; the existing
# latest.log still owns only the last failure. Passing-target output is excluded.
first_logs=("$FIXTURE/combined-logs/"*-fail.log)
second_logs=("$FIXTURE/combined-logs/"*-second.log)
[[ ${#first_logs[@]} == 1 && ${#second_logs[@]} == 1 ]] || exit 1
cat "${first_logs[0]}" "${second_logs[0]}" > "$FIXTURE/expected-combined.log"
cmp "$FIXTURE/expected-combined.log" "$FIXTURE/combined-logs/latest-combined.log"
rg -F 'error:second-target-diagnostic' "$FIXTURE/combined-logs/latest.log" >/dev/null
if rg -F 'actual-diagnostic' "$FIXTURE/combined-logs/latest.log" >/dev/null; then exit 1; fi
printf 'Consumer validation logging and raw failure retention passed.\n'
# Verify the consumer's actual runner rejects malformed inherited metadata.
for depth in SHARED_DEPTH_UNDEFINED 01 -1 '1+1' '1/0' 18446744073709551616; do
    status=0
    VALIDATION_RUNNER_DEPTH="$depth" bash "$ROOT/scripts/ci/run-validation-targets.sh" pass \
        > "$FIXTURE/depth-refusal.log" 2>&1 || status=$?
    [[ "$status" == 2 ]] || exit 1
    rg -F 'VALIDATION_RUNNER_DEPTH must be' "$FIXTURE/depth-refusal.log" >/dev/null
    if rg -F '==> pass' "$FIXTURE/depth-refusal.log" >/dev/null; then exit 1; fi
done
# Exercise the actual consumer stage loop, including a nested gate's inherited
# checkpoint. Only disposable inputs change; no product build or release runs.
for input_case in stable drift deleted; do
    gate="$FIXTURE/gate-$input_case"
    mkdir -p "$gate/make" "$gate/scripts/ci" "$gate/scripts/release"
    cp "$ROOT/Makefile" "$ROOT/Cargo.toml" "$ROOT/Cargo.lock" "$gate/"
    cp "$ROOT"/make/{consumer,tools,release,rust-format,execution}.mk "$gate/make/"
    cp "$ROOT/scripts/ci/run-validation-targets.sh" \
        "$ROOT/scripts/ci/check-make-execution.sh" \
        "$ROOT/scripts/ci/verify-file-checksum.sh" "$gate/scripts/ci/"
    cp "$ROOT/scripts/release/release-data.pl" "$gate/scripts/release/"
    cat >> "$gate/Makefile" <<'MAKE'
CI_INPUTS := Cargo.toml Cargo.lock
CI_TARGETS := fixture-pass fixture-change fixture-late
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
        if [[ "$input_case" == drift ]]; then
            rg -F 'validation input changed: Cargo.lock' "$gate/result.log" >/dev/null
        else
            rg -F 'cannot observe validation input Cargo.lock' "$gate/result.log" >/dev/null
        fi
        rg -F 'Restart preparation and the full gate' "$gate/result.log" >/dev/null
        [[ -s "$gate/failures/latest-combined.log" ]] || exit 1
        rg --files "$gate/logs" | rg '/inputs.json$' >/dev/null
    fi
done
echo 'Consumer CI stages reject changed/deleted inputs and retain nested checkpoints'
fixture_complete=true
