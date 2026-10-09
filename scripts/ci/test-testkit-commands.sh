#!/usr/bin/env bash
set -euo pipefail
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-testkit-commands.XXXXXX")"
trap 'if [[ $? == 0 ]]; then rm -rf "$fixture"; else echo "Testkit command fixture retained: $fixture" >&2; fi' EXIT
consumer="$fixture/consumer with spaces"
mkdir -p "$consumer/scripts/dev" "$consumer/scripts/ci" "$consumer/.tools/testkit-server"
cp "$root/scripts/dev/testkit-server.sh" "$consumer/scripts/dev/"
cp "$root/scripts/ci/run-pocketic-test.sh" "$consumer/scripts/ci/"
cat > "$consumer/Cargo.lock" <<'LOCK'
version = 4
[[package]]
name = "ic-testkit"
version = "0.25.5"
LOCK
cp "$consumer/Cargo.lock" "$fixture/selected-lock"
export TESTKIT_COMMAND_LOG="$fixture/commands" TESTKIT_FIXTURE_ROOT="$consumer"
unset POCKET_IC_BIN
cat > "$consumer/scripts/dev/install-rust-tools.sh" <<'INSTALL'
#!/usr/bin/env bash
set -euo pipefail
printf 'install' >> "$TESTKIT_COMMAND_LOG"
printf ' <%s>' "$@" >> "$TESTKIT_COMMAND_LOG"
printf '\n' >> "$TESTKIT_COMMAND_LOG"
[[ $# == 10 || ( $# == 11 && "${11}" == --check ) ]]
[[ "$1" == --consumer && "$2" == "$TESTKIT_FIXTURE_ROOT" && "$3" == --package &&
   "$4" == ic-testkit && "$5" == --version && "$6" == 0.25.5 &&
   "$7" == --bin && "$8" == ic-testkit-server && "$9" == --profile && "${10}" == release ]]
[[ "${TESTKIT_FAIL:-}" != install ]] || exit 23
printf '%s\n' "$TESTKIT_FIXTURE_ROOT/testkit-cli"
INSTALL
cat > "$consumer/testkit-cli" <<'CLI'
#!/usr/bin/env bash
set -euo pipefail
printf 'cli <%s> <%s> <%s>\n' "$@" >> "$TESTKIT_COMMAND_LOG"
[[ $# == 3 && "$2" == --directory && "$3" == "$TESTKIT_FIXTURE_ROOT/.tools/testkit-server" ]]
[[ "${TESTKIT_FAIL:-}" != "$1" ]] || exit 31
printf '%s\n' "$TESTKIT_FIXTURE_ROOT/.tools/testkit-server/admitted-server"
CLI
chmod +x "$consumer/testkit-cli"
cat > "$consumer/scripts/ci/run-nonempty-cargo-test.sh" <<'RUN'
#!/usr/bin/env bash
set -euo pipefail
[[ "$POCKET_IC_BIN" == "${TESTKIT_EXPECTED_SERVER-$TESTKIT_FIXTURE_ROOT/.tools/testkit-server/admitted-server}" ]]
[[ $# == 3 && "$1" == --offline && "$2" == --locked && "$3" == 'selected case' ]]
printf 'test\n' >> "$TESTKIT_COMMAND_LOG"
exit "${TESTKIT_TEST_STATUS:-0}"
RUN
wrapper="$consumer/scripts/dev/testkit-server.sh"
runner="$consumer/scripts/ci/run-pocketic-test.sh"
for action in setup check; do
    : > "$TESTKIT_COMMAND_LOG"
    path="$(bash "$wrapper" "$action")"
    [[ "$path" == "$consumer/.tools/testkit-server/admitted-server" ]]
    [[ "$(wc -l < "$TESTKIT_COMMAND_LOG" | tr -d ' ')" == 2 ]]
    if [[ "$action" == check ]]; then grep -F '<--check>' "$TESTKIT_COMMAND_LOG" >/dev/null;
    elif grep -F '<--check>' "$TESTKIT_COMMAND_LOG" >/dev/null; then exit 1; fi
    cmp "$fixture/selected-lock" "$consumer/Cargo.lock"
done
# An unset selection uses the checked owner path.
: > "$TESTKIT_COMMAND_LOG"
bash "$runner" --offline --locked 'selected case'
grep -Fx 'cli <check> <--directory> <'"$consumer/.tools/testkit-server"'>' "$TESTKIT_COMMAND_LOG" >/dev/null
[[ "$(tail -1 "$TESTKIT_COMMAND_LOG")" == test ]]
# Explicit environment and Make selections retain caller-owned byte admission.
for selected in '/caller selected/server' ''; do
    : > "$TESTKIT_COMMAND_LOG"
    TESTKIT_EXPECTED_SERVER="$selected" POCKET_IC_BIN="$selected" \
        bash "$runner" --offline --locked 'selected case'
    [[ "$(cat "$TESTKIT_COMMAND_LOG")" == test ]]
done
cat > "$consumer/Makefile" <<'MAKE'
test:
	bash scripts/ci/run-pocketic-test.sh --offline --locked 'selected case'
MAKE
: > "$TESTKIT_COMMAND_LOG"
TESTKIT_EXPECTED_SERVER='/make selected/server' make --no-print-directory -C "$consumer" \
    test 'POCKET_IC_BIN=/make selected/server' > "$fixture/make-override.log" 2>&1
[[ "$(cat "$TESTKIT_COMMAND_LOG")" == test ]]
# An undefined Make selection uses the owner's checked default.
: > "$TESTKIT_COMMAND_LOG"
make --no-print-directory -C "$consumer" test > "$fixture/make-default.log" 2>&1
[[ "$(tail -1 "$TESTKIT_COMMAND_LOG")" == test ]]
[[ "$(wc -l < "$TESTKIT_COMMAND_LOG" | tr -d ' ')" == 3 ]]
for failure in install check; do
    : > "$TESTKIT_COMMAND_LOG"
    status=0
    TESTKIT_FAIL="$failure" bash "$runner" --offline --locked 'selected case' > "$fixture/refusal.log" 2>&1 || status=$?
    expected=23; [[ "$failure" != check ]] || expected=31
    [[ "$status" == "$expected" ]]
    if grep -Fx test "$TESTKIT_COMMAND_LOG" >/dev/null; then exit 1; fi
done
status=0
TESTKIT_TEST_STATUS=37 bash "$runner" --offline --locked 'selected case' || status=$?
[[ "$status" == 37 ]]
# Missing or multiple locked selections stop before CLI installation/execution.
for shape in missing duplicate; do
    cp "$fixture/selected-lock" "$consumer/Cargo.lock"
    if [[ "$shape" == missing ]]; then printf 'version = 4\n' > "$consumer/Cargo.lock";
    else cat "$fixture/selected-lock" >> "$consumer/Cargo.lock"; fi
    : > "$TESTKIT_COMMAND_LOG"
    if bash "$wrapper" check > "$fixture/refusal.log" 2>&1; then exit 1; fi
    [[ ! -s "$TESTKIT_COMMAND_LOG" ]]
done
cp "$fixture/selected-lock" "$consumer/Cargo.lock"
echo 'Locked Testkit setup/check, admitted server handoff and failure propagation passed (substitute tools)'
