#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-testkit-commands.XXXXXX")"
fixture_complete=false
finish_fixture() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$fixture_complete" == true && "$status" == 0 ]]; then
        rm -rf "$fixture"
    else
        printf 'Testkit command fixture retained: %s\n' "$fixture" >&2
    fi
    exit "$status"
}
trap finish_fixture EXIT
consumer="$fixture/consumer with spaces"
mkdir -p "$consumer/scripts/dev" "$consumer/scripts/ci" "$consumer/.tools/testkit-server"
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$consumer" \
    Makefile make/consumer.mk ci/release-delivery-policy.txt \
    scripts/dev/testkit-server.sh scripts/ci/run-pocketic-test.sh
cat > "$consumer/Cargo.lock" <<'LOCK'
version = 4
[[package]]
name = "ic-testkit"
version = "0.25.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
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
if [[ $# == 4 || ( $# == 5 && ( "$5" == --check || "$5" == --preflight ) ) ]]; then
    [[ "$1" == --consumer && "$2" == "$TESTKIT_FIXTURE_ROOT" && "$3" == --versions ]] || exit 1
    failure=common-rust
    [[ "${5:-}" != --preflight ]] || failure=preflight-rust
    [[ "${TESTKIT_FAIL:-}" != "$failure" ]] || exit 23
    exit 0
fi
[[ $# == 12 || ( $# == 13 && "${13}" == --check ) ]] || exit 1
[[ "$1" == --consumer && "$2" == "$TESTKIT_FIXTURE_ROOT" && "$3" == --package &&
   "$5" == --lockfile && "$6" == Cargo.lock && "$7" == --bin && "$9" == --profile && "${11}" == --features && "${12}" == cli ]] || exit 1
case "$4:$8:${10}" in
    ic-testkit:ic-testkit-server:release) failure=install; binary=testkit-cli ;;
    ic-build:ic-build-release-metadata:debug) failure=release-editor; binary=release-editor ;;
    *) exit 1 ;;
esac
[[ "${TESTKIT_FAIL:-}" != "$failure" ]] || exit 23
printf '%s\n' "$TESTKIT_FIXTURE_ROOT/$binary"
INSTALL
cat > "$consumer/testkit-cli" <<'CLI'
#!/usr/bin/env bash
set -euo pipefail
printf 'cli <%s> <%s> <%s>\n' "$@" >> "$TESTKIT_COMMAND_LOG"
[[ $# == 3 && "$2" == --directory && "$3" == "$TESTKIT_FIXTURE_ROOT/.tools/testkit-server" ]] || exit 1
[[ "${TESTKIT_FAIL:-}" != "$1" ]] || exit 31
printf '%s\n' "$TESTKIT_FIXTURE_ROOT/.tools/testkit-server/admitted-server"
CLI
chmod +x "$consumer/testkit-cli"
cat > "$consumer/scripts/ci/run-nonempty-cargo-test.sh" <<'RUN'
#!/usr/bin/env bash
set -euo pipefail
[[ "$POCKET_IC_BIN" == "${TESTKIT_EXPECTED_SERVER-$TESTKIT_FIXTURE_ROOT/.tools/testkit-server/admitted-server}" ]] || exit 1
[[ $# == 3 && "$1" == --offline && "$2" == --locked && "$3" == 'selected case' ]] || exit 1
printf 'test\n' >> "$TESTKIT_COMMAND_LOG"
exit "${TESTKIT_TEST_STATUS:-0}"
RUN
wrapper="$consumer/scripts/dev/testkit-server.sh"
runner="$consumer/scripts/ci/run-pocketic-test.sh"
for action in setup check; do
    : > "$TESTKIT_COMMAND_LOG"
    path="$(bash "$wrapper" "$action")"
    [[ "$path" == "$consumer/.tools/testkit-server/admitted-server" ]] || exit 1
    [[ "$(wc -l < "$TESTKIT_COMMAND_LOG" | tr -d ' ')" == 2 ]] || exit 1
    if [[ "$action" == check ]]; then grep -F '<--check>' "$TESTKIT_COMMAND_LOG" >/dev/null;
    elif grep -F '<--check>' "$TESTKIT_COMMAND_LOG" >/dev/null; then exit 1; fi
    cmp "$fixture/selected-lock" "$consumer/Cargo.lock"
done
# An unset selection uses the checked owner path.
: > "$TESTKIT_COMMAND_LOG"
bash "$runner" --offline --locked 'selected case'
grep -Fx 'cli <check> <--directory> <'"$consumer/.tools/testkit-server"'>' "$TESTKIT_COMMAND_LOG" >/dev/null
[[ "$(tail -1 "$TESTKIT_COMMAND_LOG")" == test ]] || exit 1
# Explicit environment and Make selections retain caller-owned byte admission.
for selected in '/caller selected/server' ''; do
    : > "$TESTKIT_COMMAND_LOG"
    TESTKIT_EXPECTED_SERVER="$selected" POCKET_IC_BIN="$selected" \
        bash "$runner" --offline --locked 'selected case'
    [[ "$(cat "$TESTKIT_COMMAND_LOG")" == test ]] || exit 1
done
cat > "$consumer/Makefile" <<'MAKE'
test:
	bash scripts/ci/run-pocketic-test.sh --offline --locked 'selected case'
MAKE
: > "$TESTKIT_COMMAND_LOG"
TESTKIT_EXPECTED_SERVER='/make selected/server' make --no-print-directory -C "$consumer" \
    test 'POCKET_IC_BIN=/make selected/server' > "$fixture/make-override.log" 2>&1
[[ "$(cat "$TESTKIT_COMMAND_LOG")" == test ]] || exit 1
# An undefined Make selection uses the owner's checked default.
: > "$TESTKIT_COMMAND_LOG"
make --no-print-directory -C "$consumer" test > "$fixture/make-default.log" 2>&1
[[ "$(tail -1 "$TESTKIT_COMMAND_LOG")" == test ]] || exit 1
[[ "$(wc -l < "$TESTKIT_COMMAND_LOG" | tr -d ' ')" == 3 ]] || exit 1
for failure in install check; do
    : > "$TESTKIT_COMMAND_LOG"
    status=0
    TESTKIT_FAIL="$failure" bash "$runner" --offline --locked 'selected case' > "$fixture/refusal.log" 2>&1 || status=$?
    expected=23; [[ "$failure" != check ]] || expected=31
    [[ "$status" == "$expected" ]] || exit 1
    if grep -Fx test "$TESTKIT_COMMAND_LOG" >/dev/null; then exit 1; fi
done
status=0
TESTKIT_TEST_STATUS=37 bash "$runner" --offline --locked 'selected case' || status=$?
[[ "$status" == 37 ]] || exit 1
# Exercise this consumer's real aggregate and extension under parallel Make.
mkdir -p "$consumer/make" "$consumer/scripts/release" "$consumer/ci" "$fixture/bin"
cp "$root/Makefile" "$consumer/Makefile"
printf 'print "0.22.0\\n";\n' > "$consumer/scripts/release/release-data.pl"
for kind in host ic; do
    cat > "$consumer/scripts/dev/install-$kind-tools.sh" <<'COMMON'
#!/usr/bin/env bash
set -euo pipefail
kind="${0##*/install-}"
kind="${kind%-tools.sh}"
if [[ "$kind" == ic ]]; then
    [[ ( $# == 6 || $# == 7 ) && "$3" == --pins && "$5" == --tool && "$6" == wasm-opt ]] || exit 1
fi
stage=common
[[ "${!#}" != --preflight ]] || stage=preflight
printf '%s-%s\n' "$stage" "$kind" >> "$TESTKIT_COMMAND_LOG"
[[ "${TESTKIT_FAIL:-}" != "$stage-$kind" ]] || exit 23
COMMON
done
# No compiler or build may run after failed admission.
cat > "$fixture/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
printf 'unexpected-build\n' >> "$TESTKIT_COMMAND_LOG"
exit 98
CARGO
chmod +x "$fixture/bin/cargo"
# Both commands belong to the real contracts-boundary-check prerequisite.
# This fixture qualifies tool admission, not their independent Cargo fixtures.
for script in check-runtime-free-contracts test-runtime-free-contracts; do
    printf '# Substitute boundary check; no build.\n' > "$consumer/scripts/ci/$script.sh"
done
for target in install-tools tools-check; do
    action=setup; [[ "$target" != tools-check ]] || action=check
    : > "$TESTKIT_COMMAND_LOG"
    make --no-print-directory -j4 -C "$consumer" "$target" > "$fixture/$target.log" 2>&1
    offset=0
    if [[ "$target" == install-tools ]]; then
        offset=2
        [[ "$(sed -n '1p' "$TESTKIT_COMMAND_LOG")" == preflight-ic ]] || exit 1
        [[ "$(sed -n '2p' "$TESTKIT_COMMAND_LOG")" == 'install <--consumer>'*'<--preflight>' ]] || exit 1
    fi
    [[ "$(wc -l < "$TESTKIT_COMMAND_LOG" | tr -d ' ')" == $((6 + offset)) ]] || exit 1
    [[ "$(sed -n "$((offset + 1))p" "$TESTKIT_COMMAND_LOG")" == common-host ]] || exit 1
    [[ "$(sed -n "$((offset + 2))p" "$TESTKIT_COMMAND_LOG")" == 'install <--consumer>'*'<--versions>'* ]] || exit 1
    [[ "$(sed -n "$((offset + 3))p" "$TESTKIT_COMMAND_LOG")" == common-ic ]] || exit 1
    [[ "$(sed -n "$((offset + 4))p" "$TESTKIT_COMMAND_LOG")" == 'install <--consumer>'*'<--package> <ic-testkit>'* ]] || exit 1
    [[ "$(sed -n "$((offset + 5))p" "$TESTKIT_COMMAND_LOG")" == "cli <$action> <--directory> <$consumer/.tools/testkit-server>" ]] || exit 1
    [[ "$(tail -1 "$TESTKIT_COMMAND_LOG")" == 'install <--consumer>'*'<--package> <ic-build> <--lockfile> <Cargo.lock> <--bin> <ic-build-release-metadata> <--profile> <debug>'* ]] || exit 1
    if [[ "$action" == check ]]; then
        [[ "$(grep -c '<--check>' "$TESTKIT_COMMAND_LOG")" == 3 ]] || exit 1
    elif grep -F '<--check>' "$TESTKIT_COMMAND_LOG" >/dev/null; then exit 1; fi
    cp "$TESTKIT_COMMAND_LOG" "$fixture/ordered-expected"
    admitted_targets=("$target")
    [[ "$target" != tools-check ]] || admitted_targets+=(test-native-host test-wasm-opt)
    count=0
    failures=(common-host common-rust common-ic install "$action" release-editor)
    [[ "$target" != install-tools ]] || failures=(preflight-ic preflight-rust "${failures[@]}")
    for failure in "${failures[@]}"; do
        count=$((count + 1))
        for admitted_target in "${admitted_targets[@]}"; do
            : > "$TESTKIT_COMMAND_LOG"
            if TESTKIT_FAIL="$failure" PATH="$fixture/bin:$PATH" \
                make --no-print-directory -j4 -C "$consumer" "$admitted_target" \
                > "$fixture/$admitted_target-$failure.log" 2>&1; then exit 1; fi
            head -"$count" "$fixture/ordered-expected" > "$fixture/refused-expected"
            cmp "$fixture/refused-expected" "$TESTKIT_COMMAND_LOG"
        done
    done
    cmp "$fixture/selected-lock" "$consumer/Cargo.lock"
done
# Optimizer qualification must not unify the probe's test-support into production.
# Keep the real Make routing, substituting only compilation and qualification.
cat > "$fixture/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf 'cargo' >> "$TESTKIT_COMMAND_LOG"
printf ' <%s>' "$@" >> "$TESTKIT_COMMAND_LOG"
printf '\n' >> "$TESTKIT_COMMAND_LOG"
CARGO
cat > "$consumer/scripts/ci/qualify-blob-wasm-opt.sh" <<'QUALIFY'
#!/usr/bin/env bash
set -euo pipefail
printf 'qualify\n' >> "$TESTKIT_COMMAND_LOG"
QUALIFY
: > "$TESTKIT_COMMAND_LOG"
PATH="$fixture/bin:$PATH" make --no-print-directory -j4 -C "$consumer" test-wasm-opt \
    > "$fixture/optimizer.log" 2>&1
cat > "$fixture/optimizer-expected" <<'EXPECTED'
cargo <build> <--offline> <--locked> <--release> <--target> <wasm32-unknown-unknown> <-p> <ic-blob-storage-canister> <--lib>
cargo <build> <--offline> <--locked> <--release> <--target> <wasm32-unknown-unknown> <-p> <blob-storage-probe> <--lib>
cargo <build> <--offline> <--locked> <-p> <ic-blob-storage-cli>
qualify
EXPECTED
tail -4 "$TESTKIT_COMMAND_LOG" > "$fixture/optimizer-actual"
cmp "$fixture/optimizer-expected" "$fixture/optimizer-actual"
# Native CI must build both real participants and propagate the client cases.
cat > "$consumer/scripts/ci/run-nonempty-cargo-test.sh" <<'RUN'
#!/usr/bin/env bash
set -euo pipefail
printf 'test' >> "$TESTKIT_COMMAND_LOG"
printf ' <%s>' "$@" >> "$TESTKIT_COMMAND_LOG"
printf '\n' >> "$TESTKIT_COMMAND_LOG"
for argument; do
    if [[ "$argument" == storage_tenants::client:: ]]; then
        exit "${TESTKIT_CLIENT_TEST_STATUS:-0}"
    fi
done
RUN
: > "$TESTKIT_COMMAND_LOG"
PATH="$fixture/bin:$PATH" make --no-print-directory -C "$consumer" test-native-host \
    > "$fixture/native-host.log" 2>&1
grep -Fx 'cargo <build> <--offline> <--locked> <--release> <--target> <wasm32-unknown-unknown> <-p> <blob-storage-probe> <-p> <blob-consumer-probe> <--lib>' "$TESTKIT_COMMAND_LOG" >/dev/null
client_command='test <--offline> <--locked> <-p> <ic-blob-storage-pocketic-tests> <--test> <storage> <storage_tenants::client::> <--> <--test-threads=1>'
grep -Fx "$client_command" "$TESTKIT_COMMAND_LOG" >/dev/null
: > "$TESTKIT_COMMAND_LOG"
if TESTKIT_CLIENT_TEST_STATUS=37 PATH="$fixture/bin:$PATH" \
    make --no-print-directory -C "$consumer" test-native-host \
    > "$fixture/native-client-refusal.log" 2>&1; then exit 1; fi
grep -Fx "$client_command" "$TESTKIT_COMMAND_LOG" >/dev/null
# Shared's Cargo installer fixture owns registry selection, source identity,
# interrupted builds and retained generations. This fixture owns Blob's adapter
# forwarding, server handoff and ordered refusal; explicit real setup qualifies
# their composition without maintaining another Cargo installation substitute.
echo 'Locked Testkit/release-editor forwarding, admitted server handoff and ordered failure propagation passed (substitute tools)'
fixture_complete=true
