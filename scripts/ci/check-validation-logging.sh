#!/usr/bin/env bash
set -Eeuo pipefail

# Consumer integration only; Shared Tooling owns logger mechanics and its suite.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
FIXTURE="$(mktemp -d "${TMPDIR:-/tmp}/blob-validation-logging.XXXXXX")"
finish() {
    local status=$?
    if [[ "$status" == 0 ]]; then rm -rf "$FIXTURE";
    else printf 'Validation logging fixture retained: %s\n' "$FIXTURE" >&2; fi
}
trap finish EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES
mkdir "$FIXTURE/logs"
cat > "$FIXTURE/Makefile" <<'MAKE'
.PHONY: pass fail
pass:
	@printf '%s\n' 'test error::passing ... ok' 'test error::skipped ... ignored'
fail:
	@printf '%s\n' 'test error::context ... ok' 'error[E0308]: actual-diagnostic' 'error:no-space-diagnostic' 'test error::broken ... FAILED'
	@exit 9
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
[[ "$status" == 1 ]] || exit 1
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
printf 'Consumer validation logging and raw failure retention passed.\n'
