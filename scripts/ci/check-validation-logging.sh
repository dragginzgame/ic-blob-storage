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
fixture_complete=true
