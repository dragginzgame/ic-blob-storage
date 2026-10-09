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
trap 'if [[ $? == 0 ]]; then rm -rf "$fixture"; else printf "Tooling evidence fixture retained: %s\n" "$fixture" >&2; fi' EXIT
proof="${1:-}"
if [[ -n "$proof" ]]; then
    [[ "$proof" == /* ]] || proof="$PWD/$proof"
    mkdir "$proof"
fi
collector="$root/scripts/ci/collect-tooling-evidence.sh"
repository="$fixture/repository"
temporary="$fixture/temp"
mkdir -p "$repository" "$temporary" "$fixture/unpacked"
# A failure before any candidate/fixture exists still has no upload payload.
empty="$(bash "$collector" "$repository" "$temporary" "$fixture/empty.tar.gz")"
[[ -z "$empty" && ! -e "$fixture/empty.tar.gz" ]]

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
printf 'original server download failure\n' > "$repository/.tools/testkit-server/failed-candidate/download.stderr"
printf 'original_status=9\n' > "$repository/target/release-tests.failed/outcome"
printf 'portable bytes\n' > "$temporary/file-digests.failed/"$'line\nbreak:payload'
chmod 640 "$temporary/file-digests.failed/"$'line\nbreak:payload'
printf '#!/bin/sh\nexit 23\n' > "$temporary/host-tools-test.failed/Linux:x86_64/executable"
chmod 755 "$temporary/host-tools-test.failed/Linux:x86_64/executable"
printf 'raw stdout\n' > "$temporary/nonempty-cargo-test.failed.log"
printf 'original_status=23\n' > "$temporary/blob-validation-logging.failed/status"
printf 'not uploaded\n' > "$temporary/file-digests.failed/.git/config"
printf 'shared archive failure\n' > "$temporary/evidence-archive.failed/outcome"
printf 'tool evidence failure\n' > "$temporary/tool-evidence-test.failed/outcome"
printf 'unrelated bytes\n' > "$temporary/unrelated/payload"
printf 'outside selection\n' > "$temporary/outside"
ln -s ../outside "$temporary/file-digests.failed/link"
archive="$(bash "$collector" "$repository" "$temporary" "$fixture/evidence.tar.gz")"
[[ "$archive" == "$fixture/evidence.tar.gz" ]]
tar -xzf "$archive" -C "$fixture/unpacked"
for path in .tools/host-set.failed/.hidden .tools/ic-set.failed/payload \
    .tools/rust/build/build.log .tools/rust/ic-testkit-fixture/receipt \
    .tools/testkit-server/failed-candidate/download.stderr target/release-tests.failed/outcome; do
    cmp "$repository/$path" "$fixture/unpacked/$path"
done
for path in $'file-digests.failed/line\nbreak:payload' \
    host-tools-test.failed/Linux:x86_64/executable nonempty-cargo-test.failed.log \
    blob-validation-logging.failed/status evidence-archive.failed/outcome \
    tool-evidence-test.failed/outcome; do
    cmp "$temporary/$path" "$fixture/unpacked/$path"
done
[[ "$(perl -e 'printf "%o", (stat($ARGV[0]))[2] & 0777' \
    "$fixture/unpacked/file-digests.failed/"$'line\nbreak:payload')" == 640 ]]
[[ -x "$fixture/unpacked/host-tools-test.failed/Linux:x86_64/executable" ]]
[[ -L "$fixture/unpacked/file-digests.failed/link" ]]
[[ "$(readlink "$fixture/unpacked/file-digests.failed/link")" == ../outside ]]
[[ ! -e "$fixture/unpacked/file-digests.failed/link" ]]
[[ ! -e "$fixture/unpacked/file-digests.failed/.git" && \
    ! -e "$fixture/unpacked/.tools/host-set.failed/.git" && \
    ! -e "$fixture/unpacked/unrelated" && ! -e "$fixture/unpacked/outside" ]]

# A new output is mandatory; a failed write retains inputs and partial output.
before="$(bash "$root/scripts/ci/verify-file-checksum.sh" --print sha256 "$archive")"
if bash "$collector" "$repository" "$temporary" "$archive" > "$fixture/occupied.log" 2>&1; then exit 1; fi
bash "$root/scripts/ci/verify-file-checksum.sh" sha256 "$before" "$archive"
mkdir "$fixture/failing-bin"
printf '#!%s\nprintf "partial evidence"\nexit 19\n' "$BASH" > "$fixture/failing-bin/tar"
chmod +x "$fixture/failing-bin/tar"
if PATH="$fixture/failing-bin:$PATH" bash "$collector" "$repository" "$temporary" \
    "$fixture/partial.tar.gz" > "$fixture/write-failure.log" 2>&1; then exit 1; fi
[[ -s "$fixture/partial.tar.gz" && -s "$fixture/write-failure.log" ]]
cmp "$temporary/blob-validation-logging.failed/status" "$fixture/unpacked/blob-validation-logging.failed/status"

# Redirected parents refuse, while a final selected symlink remains unfollowed.
mv "$repository/.tools/rust/build" "$fixture/build"
ln -s "$fixture/build" "$repository/.tools/rust/build"
archive_link="$(bash "$collector" "$repository" "$temporary" "$fixture/link.tar.gz")"
mkdir "$fixture/link-unpacked"
tar -xzf "$archive_link" -C "$fixture/link-unpacked"
[[ -L "$fixture/link-unpacked/.tools/rust/build" ]]
[[ "$(readlink "$fixture/link-unpacked/.tools/rust/build")" == "$fixture/build" ]]
mv "$repository/.tools" "$fixture/toolsets"
ln -s "$fixture/toolsets" "$repository/.tools"
if bash "$collector" "$repository" "$temporary" "$fixture/redirected.tar.gz" \
    > "$fixture/redirected.log" 2>&1; then exit 1; fi
[[ ! -e "$fixture/redirected.tar.gz" ]]

if [[ -n "$proof" ]]; then
    cp -p "$archive" "$proof/evidence.tar.gz"
    printf '%s\n' "$before" > "$proof/archive.sha256"
fi
echo 'Tooling evidence selections, filename/mode/link round trips and failure retention passed'
