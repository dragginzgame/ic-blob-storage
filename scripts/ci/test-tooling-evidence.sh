#!/usr/bin/env bash
set -euo pipefail

# An optional new directory retains a checksum-bound archive for hosted transport.
[[ $# -le 1 ]] || { echo 'usage: test-tooling-evidence.sh [NEW-PROOF-DIRECTORY]' >&2; exit 2; }
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-tooling-evidence-test.XXXXXX")"
fixture="$(cd -P "$fixture" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
fixture_complete=false
finish_fixture() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$fixture_complete" == true && "$status" == 0 ]]; then
        rm -rf "$fixture"
    else
        printf 'Tooling evidence fixture retained: %s\n' "$fixture" >&2
    fi
    exit "$status"
}
trap finish_fixture EXIT
proof="${1:-}"
if [[ -n "$proof" ]]; then
    [[ "$proof" == /* ]] || proof="$PWD/$proof"
    mkdir "$proof"
fi
collector="$root/scripts/ci/collect-tooling-evidence.sh"
repository="$fixture/repository"
temporary="$fixture/temp"
mkdir -p "$repository" "$temporary" "$fixture/unpacked"
# Store tests never select the developer's real per-user storage.
export SHARED_TOOLING_STORE="$fixture/store"
# A failure before any candidate/fixture exists still has no upload payload.
empty="$(bash "$collector" "$repository" "$temporary" "$fixture/empty.tar.gz")"
[[ -z "$empty" && ! -e "$fixture/empty.tar.gz" ]] || exit 1

# Early installer failure and late fixtures retain the producer's original facts.
mkdir -p "$repository/.tools/host-set.failed"
printf 'candidate_status=23\n' > "$repository/.tools/host-set.failed/.hidden"
early="$(bash "$collector" "$repository" "$temporary" "$fixture/early.tar.gz")"
mkdir "$fixture/early-unpacked"
tar -xzf "$early" -C "$fixture/early-unpacked"
cmp "$repository/.tools/host-set.failed/.hidden" "$fixture/early-unpacked/.tools/host-set.failed/.hidden"
mkdir -p "$repository/.tools/host-set.failed/.git" "$repository/.tools/ic-set.failed" \
    "$repository/.tools/rust/build" "$repository/.tools/rust/ic-testkit-fixture" \
    "$repository/.tools/testkit-server/failed-candidate" "$repository/target/release-tests.failed" \
    "$temporary/file-digests.failed/.git" "$temporary/host-tools-test.failed/Linux:x86_64" \
    "$temporary/blob-validation-logging.failed" "$temporary/evidence-archive.failed" \
    "$temporary/tool-evidence-test.failed" "$temporary/unrelated"
printf 'candidate_status=23\n' > "$repository/.tools/host-set.failed/.hidden"
printf 'not uploaded\n' > "$repository/.tools/host-set.failed/.git/config"
printf 'ic candidate\n' > "$repository/.tools/ic-set.failed/payload"
printf 'compiler diagnostics\n' > "$repository/.tools/rust/build/build.log"
printf 'Testkit CLI install receipt\n' > "$repository/.tools/rust/ic-testkit-fixture/receipt"
mkdir -p "$repository/.tools/rust/common-tool-slot/installed"
printf 'common Cargo selection\n' > "$repository/.tools/rust/common-tool-slot/installed/selection.json"
printf 'original server download failure\n' > "$repository/.tools/testkit-server/failed-candidate/download.stderr"
printf 'original_status=9\n' > "$repository/target/release-tests.failed/outcome"
printf 'portable bytes\n' > "$temporary/file-digests.failed/"$'line\nbreak:payload'
chmod 640 "$temporary/file-digests.failed/"$'line\nbreak:payload'
printf '#!/bin/sh\nexit 23\n' > "$temporary/host-tools-test.failed/Linux:x86_64/executable"
chmod 755 "$temporary/host-tools-test.failed/Linux:x86_64/executable"
printf 'raw stdout\n' > "$temporary/nonempty-cargo-test.failed.log"
printf 'formatter stdout\nformatter stderr\n' > "$temporary/formatting.failed"
printf 'original_status=23\n' > "$temporary/blob-validation-logging.failed/status"
printf 'not uploaded\n' > "$temporary/file-digests.failed/.git/config"
printf 'shared archive failure\n' > "$temporary/evidence-archive.failed/outcome"
printf 'tool evidence failure\n' > "$temporary/tool-evidence-test.failed/outcome"
printf 'unrelated bytes\n' > "$temporary/unrelated/payload"
printf 'outside selection\n' > "$temporary/outside"
ln -s ../outside "$temporary/file-digests.failed/link"

# Use the shared owner to create pre-effect references for failed generations.
# A different checkout's candidate in the same store must not be collected.
prepare_candidate() (
    local candidate_consumer="$1" candidate_identity="$2" candidate_kind="${3:-host}"
    consumer="$candidate_consumer"
    tool_root="$consumer/.tools"
    [[ "$candidate_kind" != rust ]] || tool_root="$tool_root/rust"
    mkdir -p "$tool_root"
    # shellcheck source=scripts/dev/tool-store.sh
    source "$root/scripts/dev/tool-store.sh"
    tool_store_init "$candidate_kind" "$candidate_identity"
    tool_store_acquire
    trap tool_store_release EXIT
    tool_store_new
    printf 'failed shared download\n' > "$stage/download.stderr"
    printf '%s\n' "$stage"
)
shared_candidate="$(prepare_candidate "$repository" this-checkout)"
rust_candidate="$(prepare_candidate "$repository" cargo-build rust)"
rust_relative="${rust_candidate#"$SHARED_TOOLING_STORE/"}"
unrelated_candidate="$(prepare_candidate "$fixture/unrelated-checkout" other-checkout)"
shared_relative="${shared_candidate#"$SHARED_TOOLING_STORE/"}"
unrelated_relative="${unrelated_candidate#"$SHARED_TOOLING_STORE/"}"
ln -s "$fixture/unrelated-checkout" "$repository/.tools/host-set.unknown"
archive="$(bash "$collector" "$repository" "$temporary" "$fixture/evidence.tar.gz")"
[[ "$archive" == "$fixture/evidence.tar.gz" ]] || exit 1
tar -xzf "$archive" -C "$fixture/unpacked"
for path in .tools/host-set.failed/.hidden .tools/ic-set.failed/payload \
    .tools/rust/build/build.log .tools/rust/ic-testkit-fixture/receipt \
    .tools/rust/common-tool-slot/installed/selection.json \
    .tools/testkit-server/failed-candidate/download.stderr target/release-tests.failed/outcome; do
    cmp "$repository/$path" "$fixture/unpacked/$path"
done
cmp "$rust_candidate/download.stderr" "$fixture/unpacked/$rust_relative/download.stderr"
cmp "$shared_candidate/download.stderr" "$fixture/unpacked/$shared_relative/download.stderr"
[[ ! -e "$fixture/unpacked/$unrelated_relative" && \
   -L "$fixture/unpacked/.tools/host-set.unknown" ]] || exit 1
[[ "$(readlink "$fixture/unpacked/.tools/host-set.unknown")" == "$fixture/unrelated-checkout" ]] || exit 1
for path in $'file-digests.failed/line\nbreak:payload' \
    host-tools-test.failed/Linux:x86_64/executable nonempty-cargo-test.failed.log formatting.failed \
    blob-validation-logging.failed/status evidence-archive.failed/outcome \
    tool-evidence-test.failed/outcome; do
    cmp "$temporary/$path" "$fixture/unpacked/$path"
done
[[ "$(perl -e 'printf "%o", (stat($ARGV[0]))[2] & 0777' \
    "$fixture/unpacked/file-digests.failed/"$'line\nbreak:payload')" == 640 ]] || exit 1
[[ -x "$fixture/unpacked/host-tools-test.failed/Linux:x86_64/executable" ]] || exit 1
[[ -L "$fixture/unpacked/file-digests.failed/link" ]] || exit 1
[[ "$(readlink "$fixture/unpacked/file-digests.failed/link")" == ../outside ]] || exit 1
[[ ! -e "$fixture/unpacked/file-digests.failed/link" ]] || exit 1
[[ ! -e "$fixture/unpacked/file-digests.failed/.git" && \
    ! -e "$fixture/unpacked/.tools/host-set.failed/.git" && \
    ! -e "$fixture/unpacked/unrelated" && ! -e "$fixture/unpacked/outside" ]] || exit 1

# A new output is mandatory; a failed write retains inputs and partial output.
before="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$archive")"
if bash "$collector" "$repository" "$temporary" "$archive" > "$fixture/occupied.log" 2>&1; then exit 1; fi
bash "$root/scripts/ci/verify-file-checksum.sh" sha256 "$before" "$archive"
mkdir "$fixture/failing-bin"
printf '#!%s\nprintf "partial evidence"\nexit 19\n' "$BASH" > "$fixture/failing-bin/tar"
chmod +x "$fixture/failing-bin/tar"
if PATH="$fixture/failing-bin:$PATH" bash "$collector" "$repository" "$temporary" \
    "$fixture/partial.tar.gz" > "$fixture/write-failure.log" 2>&1; then exit 1; fi
[[ -s "$fixture/partial.tar.gz" && -s "$fixture/write-failure.log" ]] || exit 1
cmp "$temporary/blob-validation-logging.failed/status" "$fixture/unpacked/blob-validation-logging.failed/status"

# Unsafe shared-store configuration cannot discard safe local evidence or report
# completeness. Shared references remain links; their payloads are not followed.
ln -s "$SHARED_TOOLING_STORE" "$fixture/redirected-store"
for refused_store in "$fixture/redirected-store" relative-store; do
    attempt="$(mktemp -d "$fixture/incomplete.XXXXXX")"
    status=0
    partial="$(SHARED_TOOLING_STORE="$refused_store" bash "$collector" "$repository" "$temporary" \
        "$attempt/evidence.tar.gz" 2> "$attempt/collection.log")" || status=$?
    [[ "$status" != 0 && "$partial" == "$attempt/evidence.tar.gz" ]] || exit 1
    mkdir "$attempt/unpacked"
    tar -xzf "$partial" -C "$attempt/unpacked"
    cmp "$repository/.tools/rust/build/build.log" "$attempt/unpacked/.tools/rust/build/build.log"
    cmp "$repository/.tools/testkit-server/failed-candidate/download.stderr" \
        "$attempt/unpacked/.tools/testkit-server/failed-candidate/download.stderr"
    [[ ! -e "$attempt/unpacked/$shared_relative" && ! -e "$attempt/unpacked/$rust_relative" && \
        -L "$attempt/unpacked/.tools/rust/$(basename "$(find "$repository/.tools/rust" -name 'rust-set.*')")" ]] || exit 1
    diagnostics=("$attempt/unpacked"/blob-tooling-selection.*/selection-status)
    [[ ${#diagnostics[@]} == 1 && "$(cat "${diagnostics[0]}")" != 0 ]] || exit 1
    [[ -s "${diagnostics[0]%/*}/selection.log" ]] || exit 1
done

# Exercise the actual hosted collection block: incomplete admission must expose
# the archive output before upload; the later completeness step still refuses.
mkdir -p "$repository/scripts/ci"
for helper in collect-tooling-evidence.sh select-tool-evidence.sh archive-evidence.sh verify-file-checksum.sh; do
    cp "$root/scripts/ci/$helper" "$repository/scripts/ci/"
done
mkdir -p "$repository/scripts/dev"
cp "$root/scripts/dev/tool-store.sh" "$repository/scripts/dev/"
perl -ne '
    if (/^      - name: Archive failed tooling fixtures$/) { $selected=1; next; }
    if ($selected && /^        run: \|$/) { $run=1; next; }
    last if $run && /^      - name:/;
    print substr($_,10) if $run;
' "$root/.github/workflows/tooling.yml" > "$fixture/workflow-collection.sh"
[[ -s "$fixture/workflow-collection.sh" ]] || exit 1
(cd "$repository" && SHARED_TOOLING_STORE="$fixture/redirected-store" GITHUB_WORKSPACE="$repository" \
    RUNNER_TEMP="$temporary" GITHUB_OUTPUT="$fixture/workflow-output" bash -e "$fixture/workflow-collection.sh")
grep -Fx 'collection-complete=false' "$fixture/workflow-output" >/dev/null
hosted_archive="$(sed -n 's/^archive=//p' "$fixture/workflow-output")"
[[ -s "$hosted_archive" ]] || exit 1
perl -ne '
    if (/^      - name: Check tooling evidence completeness$/) { $selected=1; next; }
    if ($selected && /^        run: \|$/) { $run=1; next; }
    print substr($_,10) if $run;
' "$root/.github/workflows/tooling.yml" > "$fixture/workflow-completeness.sh"
[[ -s "$fixture/workflow-completeness.sh" ]] || exit 1
if bash "$fixture/workflow-completeness.sh" > "$fixture/completeness.log" 2>&1; then exit 1; fi

# Redirected parents refuse, while a final selected symlink remains unfollowed.
mv "$repository/.tools/rust/build" "$fixture/build"
ln -s "$fixture/build" "$repository/.tools/rust/build"
archive_link="$(bash "$collector" "$repository" "$temporary" "$fixture/link.tar.gz")"
mkdir "$fixture/link-unpacked"
tar -xzf "$archive_link" -C "$fixture/link-unpacked"
[[ -L "$fixture/link-unpacked/.tools/rust/build" ]] || exit 1
[[ "$(readlink "$fixture/link-unpacked/.tools/rust/build")" == "$fixture/build" ]] || exit 1
mv "$repository/.tools" "$fixture/toolsets"
ln -s "$fixture/toolsets" "$repository/.tools"
if bash "$collector" "$repository" "$temporary" "$fixture/redirected.tar.gz" \
    > "$fixture/redirected.log" 2>&1; then exit 1; fi
[[ ! -e "$fixture/redirected.tar.gz" ]] || exit 1

if [[ -n "$proof" ]]; then
    cp -p "$archive" "$proof/evidence.tar.gz"
    printf '%s\n' "$before" > "$proof/archive.sha256"
fi
echo 'Tooling evidence selections, filename/mode/link round trips and failure retention passed'
fixture_complete=true
