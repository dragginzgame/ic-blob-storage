#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
TEST_REAL_MAKE="$(command -v make)"
TEST_REAL_GIT="$(command -v git)"
TEST_REAL_PERL="$(command -v perl)"
export TEST_REAL_MAKE TEST_REAL_GIT TEST_REAL_PERL
mkdir -p "$ROOT/target"
TEMPORARY="$(mktemp -d "$ROOT/target/release-tests.XXXXXX")"
mkdir "$TEMPORARY/bin"
CASE_NAME=setup
CASE_LOG=/dev/null
export TEST_LOG=/dev/null TEST_EFFECTS=/dev/null
export TEST_SOURCE=1111111111111111111111111111111111111111
finish() {
    local status=$?
    [[ "$BASH_SUBSHELL" == 0 ]] || return "$status"
    if [[ "$status" == 0 ]]; then
        rm -rf "$TEMPORARY"
    else
        printf 'Release-adapter tests: FAIL [%s]\n' "$CASE_NAME" >&2
        tail -n 80 "$CASE_LOG" >&2
        tail -n 40 "$TEST_LOG" >&2
        printf 'Full logs and isolated fixture retained: %s\n' "$TEMPORARY" >&2
    fi
}
trap finish EXIT
run_case() {
    CASE_NAME="$1"; shift
    CASE_LOG="$TEMPORARY/$CASE_NAME.log"
    FIXTURE="$TEMPORARY/$CASE_NAME"
    TEST_LOG="$FIXTURE/commands.log"
    TEST_EFFECTS="$FIXTURE/effects.log"
    (
        trap 'printf "Failed at line %s: %s\n" "$LINENO" "$BASH_COMMAND" >&2' ERR
        create_fixture
        "$@"
    ) > "$CASE_LOG" 2>&1
}
create_fixture() {
    mkdir -p "$FIXTURE/scripts/release" "$FIXTURE/scripts/ci" "$FIXTURE/ci" "$FIXTURE/docs" "$FIXTURE/target/debug"
    cp "$ROOT/scripts/release/release.sh" "$ROOT/scripts/release/release-data.pl" "$FIXTURE/scripts/release/"
    cp "$ROOT/scripts/ci/run-release.sh" "$ROOT/scripts/ci/next-release-version.sh" \
        "$ROOT/scripts/ci/run-validation-targets.sh" "$ROOT/scripts/ci/check-release-tag.sh" \
        "$ROOT/scripts/ci/rewrite-local-lock-versions.pl" "$FIXTURE/scripts/ci/"
    cp "$ROOT/ci/tool-versions.env" "$FIXTURE/ci/"
    cp "$ROOT/Makefile" "$FIXTURE/Makefile"
    cd "$FIXTURE"
    cat >> Makefile <<'MAKE'
CI_TARGETS := fixture-verify
fixture-verify:
	@bash target/verify-fixture
MAKE
    cat > target/verify-fixture <<'VERIFY'
set -euo pipefail
echo validate >> "$TEST_EFFECTS"
printf 'validation source: %s\n' "$RELEASE_SOURCE" > "target/validation.$(awk '$0 == "validate" { n++ } END { print n }' "$TEST_EFFECTS").log"
[[ "${TEST_GATE_FAIL:-0}" != 1 ]] || exit 1
touch target/gate-ran
[[ "${TEST_GATE_DIRTY:-0}" != 1 ]] || echo unvalidated >> CHANGELOG.md
VERIFY
    cat > Cargo.toml <<'TOML'
[workspace]
members = []
[workspace.package]
version = "0.1.0"
edition = "2024"
publish = ["crates-io"]
TOML
    cat > Cargo.lock <<'LOCK'
version = 4

[[package]]
name = "ic-blob-storage"
version = "0.1.0"
dependencies = [
 "external 0.1.0",
]

[[package]]
name = "external"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "unchanged-external-checksum"
LOCK
    printf '# Changelog\n\n## [0.1.1]\n\n- Test notes.\n\n## [0.1.0]\n\n- Imported undated history.\n' > CHANGELOG.md
    : > "$TEST_LOG"; : > "$TEST_EFFECTS"
    echo retained > target/debug/cache-sentinel
    unset TEST_DIRTY TEST_GATE_FAIL TEST_GATE_DIRTY TEST_METADATA_FAIL TEST_PREPARE_FAIL TEST_INDEX_EXTRA TEST_PUSH_FAIL TEST_FETCH_FAIL TEST_SNAPSHOT_FAIL TEST_TAG_TYPE
    unset TEST_GIT_FAIL_CALL TEST_DATA_FAIL_COMMAND
}
expect_failure() {
    local status
    if "$@" > target/rejection.log 2>&1; then
        echo "expected rejection: $*" >&2; exit 1
    else status=$?; fi
    cat target/rejection.log
    case "$status" in 1|2|255) ;; *) echo "broken substitute: $status" >&2; exit 1 ;; esac
}
fingerprint() { shasum -a 256 Cargo.toml Cargo.lock CHANGELOG.md; }
# Failed conditional builtins need explicit exits on Bash 3.2, including tests.
assert_unchanged() { [[ "$(fingerprint)" == "$before" && ! -e docs/release.json ]] || exit 1; }
assert_cache_retained() { [[ "$(cat target/debug/cache-sentinel)" == retained ]] || exit 1; }

# Git effects are substitutes: no commits, tags, real push or publication.
cat > "$TEMPORARY/bin/git" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "git $*" >> "$TEST_LOG"
if [[ "${TEST_GIT_FAIL_CALL:-}" == "$*" ]]; then
    unset TEST_GIT_FAIL_CALL
    "$0" "$@"
    exit 1
fi
release_head=3333333333333333333333333333333333333333
tag_sha=4444444444444444444444444444444444444444
tree=5555555555555555555555555555555555555555
resolve() { if [[ "$1" == HEAD ]]; then if [[ -f target/mock-head ]]; then cat target/mock-head; else echo "$TEST_SOURCE"; fi; else echo "$1"; fi; }
ancestor() {
    local cursor
    cursor="$(resolve "$2")"
    while [[ "$cursor" != "$1" ]]; do
        [[ -f "target/history/$cursor.parent" ]] || return 1
        cursor="$(cat "target/history/$cursor.parent")"
    done
}
case "$1" in
    check-ref-format) [[ "$2" == refs/heads/main ]] ;;
    symbolic-ref) echo main ;;
    remote) echo https://invalid.example/fixture ;;
    hash-object) exec "$TEST_REAL_GIT" hash-object --stdin ;;
    rev-parse)
        case "${*: -1}" in
            --show-toplevel) pwd ;;
            release-state) echo target/release-state ;;
            HEAD) resolve HEAD ;;
            HEAD^) cat "target/history/$(resolve HEAD).parent" ;;
            *'^{tree}') echo "$tree" ;;
            refs/tags/*'^{commit}') ref="${*: -1}"; ref="${ref%%^*}"; cat "target/tags/${ref#refs/tags/}.commit" ;;
            *'^{commit}') ref="${*: -1}"; resolve "${ref%%^*}" ;;
            refs/tags/*) ref="${*: -1}"; cat "target/tags/${ref#refs/tags/}.sha" ;;
            *) exit 97 ;;
        esac ;;
    status)
        if [[ "${TEST_DIRTY:-0}" == 1 ]]; then echo ' M src/lib.rs';
        elif [[ ! -f target/mock-head && "$(perl scripts/release/release-data.pl version)" != 0.1.0 ]]; then echo ' M Cargo.toml'; fi ;;
    diff)
        case "$2" in
            --binary) cat Cargo.toml Cargo.lock CHANGELOG.md ;;
            --name-only) printf 'Cargo.toml\0Cargo.lock\0CHANGELOG.md\0docs/release.json\0'; [[ "${TEST_DIRTY:-0}" != 1 ]] || printf 'src/lib.rs\0' ;;
            --cached) [[ "$3" == --name-only ]] || exit 97; printf 'Cargo.toml\0Cargo.lock\0CHANGELOG.md\0docs/release.json\0'; [[ "${TEST_INDEX_EXTRA:-0}" != 1 ]] || printf 'src/lib.rs\0' ;;
            --quiet) [[ "${TEST_DIRTY:-0}" != 1 ]] ;;
            *) exit 97 ;;
        esac ;;
    ls-files) ;;
    write-tree) echo "$tree" ;;
    log)
        commit="$(resolve "$4")"
        case "$3" in
            --format=%P) cat "target/history/$commit.parent" ;;
            --format=%s) cat "target/history/$commit.subject" ;;
            *) exit 97 ;;
        esac ;;
    rev-list)
        range="${*: -1}"; cursor="$(resolve HEAD)"; history=""
        while [[ "$cursor" != "${range%%..*}" ]]; do
            history="$cursor${history:+$'\n'$history}"
            cursor="$(cat "target/history/$cursor.parent")"
        done
        printf '%s\n' "$history" ;;
    merge-base) [[ "$2" == --is-ancestor ]] || exit 97; ancestor "$3" "$4" ;;
    show) ref="$2"; cat "target/history/${ref%%:*}.files/${ref#*:}" ;;
    add) [[ "$*" == 'add -- Cargo.toml Cargo.lock CHANGELOG.md docs/release.json' ]] || exit 97; echo stage >> "$TEST_EFFECTS" ;;
    commit)
        if [[ -d "target/history/$release_head.files" ]]; then release_head=7777777777777777777777777777777777777777; fi
        mkdir -p "target/history/$release_head.files/docs"
        cp Cargo.toml Cargo.lock CHANGELOG.md "target/history/$release_head.files/"
        cp docs/release.json "target/history/$release_head.files/docs/"
        resolve HEAD > "target/history/$release_head.parent"
        printf '%s\n' "$3" > "target/history/$release_head.subject"
        echo commit >> "$TEST_EFFECTS"; echo "$release_head" > target/mock-head ;;
    tag)
        if [[ "$2" == -a ]]; then
            mkdir -p target/tags
            printf '%s\n' "$4" > "target/tags/$3.commit"
            if [[ "$4" != "$release_head" ]]; then tag_sha=8888888888888888888888888888888888888888; fi
            echo "$tag_sha" > "target/tags/$3.sha"
            echo tag >> "$TEST_EFFECTS"
        elif [[ -f "target/tags/$3.commit" ]]; then echo "$3"; fi ;;
    cat-file) echo "${TEST_TAG_TYPE:-tag}" ;;
    ls-remote)
        for ref in "$@"; do
            case "$ref" in
                refs/heads/main) [[ ! -f target/remote-head ]] || printf '%s\t%s\n' "$(cat target/remote-head)" "$ref" ;;
                refs/tags/*) [[ ! -f "target/remote-tags/${ref#refs/tags/}" ]] || printf '%s\t%s\n' "$(cat "target/remote-tags/${ref#refs/tags/}")" "$ref" ;;
            esac
        done ;;
    push)
        [[ "$#" == 6 && "$2" == --no-follow-tags && "$3" == --atomic && "$4" == origin && "$5" == *:refs/heads/main && "$6" == refs/tags/v*:refs/tags/v* ]] || exit 97
        tag="${6%%:*}"; tag="${tag#refs/tags/}"
        [[ "$6" == "refs/tags/$tag:refs/tags/$tag" ]] || exit 97
        mkdir -p target/remote-tags
        resolve "${5%:refs/heads/main}" > target/remote-head
        cp "target/tags/$tag.sha" "target/remote-tags/$tag"
        echo push >> "$TEST_EFFECTS"
        [[ "${TEST_PUSH_FAIL:-0}" != 1 ]] || exit 1 ;;
    *) echo "unsupported substitute: $*" >&2; exit 97 ;;
esac
MOCK
cat > "$TEMPORARY/bin/cargo" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "cargo $*" >> "$TEST_LOG"
case "$*" in
    'metadata --offline --locked --no-deps --format-version 1')
        [[ "${TEST_METADATA_FAIL:-0}" != 1 ]] || exit 1
        current="$(perl scripts/release/release-data.pl version)"
        printf '{"workspace_members":["local"],"packages":[{"id":"local","name":"ic-blob-storage","version":"%s","source":null}]}\n' "$current" ;;
    'sort --version') [[ "${TEST_SORT_FAIL:-0}" != 1 ]] || exit 1; echo "cargo-sort ${TEST_SORT_VERSION:-2.1.4}" ;;
    'sort --workspace --check') [[ "${TEST_PREPARE_FAIL:-0}" != 1 ]] ;;
    'fmt --all -- --check') ;;
    'fetch --locked') echo fetch >> "$TEST_EFFECTS"; [[ "${TEST_FETCH_FAIL:-0}" != 1 ]] || exit 1; touch target/mock-cargo-cache ;;
    'check --offline --locked --workspace --all-targets --all-features') [[ -f target/mock-cargo-cache ]] || exit 1; echo check >> "$TEST_EFFECTS" ;;
    'publish --locked --registry crates-io -p ic-blob-storage'|'publish --locked --registry crates-io -p ic-blob-storage --dry-run') echo publish >> "$TEST_EFFECTS" ;;
    *) echo "unsupported Cargo substitute: $*" >&2; exit 97 ;;
esac
MOCK
cat > "$TEMPORARY/bin/perl" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
if [[ $# -ge 2 && "${1##*/}" == release-data.pl && "$2" == "${TEST_DATA_FAIL_COMMAND:-}" ]]; then
    "$TEST_REAL_PERL" "$@"
    exit 1
fi
exec "$TEST_REAL_PERL" "$@"
MOCK
cat > "$TEMPORARY/bin/shellcheck" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
[[ "$*" == --version && "${TEST_SHELLCHECK_FAIL:-0}" != 1 ]] || exit 1
echo 'ShellCheck version: 0.11.0'
MOCK
chmod +x "$TEMPORARY/bin/"*
export PATH="$TEMPORARY/bin:$PATH"

test_dependency_bootstrap() {
    # Use the owning gate with a finite fixture target set, without compiling.
    cat > target/gate-make <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    '--no-print-directory shared-tooling-check') echo snapshot >> "$TEST_EFFECTS"; [[ "${TEST_SNAPSHOT_FAIL:-0}" != 1 ]] ;;
    '--no-print-directory tools-check') echo tools >> "$TEST_EFFECTS"; [[ "${TEST_TOOLS_FAIL:-0}" != 1 ]] ;;
    '--no-print-directory dependency-pins-check') echo pins >> "$TEST_EFFECTS"; [[ "${TEST_PINS_FAIL:-0}" != 1 ]] ;;
    '--no-print-directory deps'|'--no-print-directory check') exec "$TEST_REAL_MAKE" "$@" ;;
    *) exit 97 ;;
esac
MOCK
    chmod +x target/gate-make
    before="$(fingerprint)"
    case "$1" in
        snapshot) export TEST_SNAPSHOT_FAIL=1; expected=snapshot ;;
        tools) export TEST_TOOLS_FAIL=1; expected=$'snapshot\ntools' ;;
        pins) export TEST_PINS_FAIL=1; expected=$'snapshot\ntools\npins' ;;
        fetch) export TEST_FETCH_FAIL=1; expected=$'snapshot\ntools\npins\nfetch' ;;
        fetch-cached)
            export TEST_FETCH_FAIL=1; expected=$'snapshot\ntools\npins\nfetch'
            echo retained-cache > target/mock-cargo-cache
            ;;
        success) expected=$'snapshot\ntools\npins\nfetch\ncheck' ;;
    esac
    if [[ "$1" == success ]]; then
        "$TEST_REAL_MAKE" --no-print-directory ci 'CI_TARGETS=shared-tooling-check tools-check dependency-pins-check deps check' "MAKE=$PWD/target/gate-make"
    else
        expect_failure "$TEST_REAL_MAKE" --no-print-directory ci 'CI_TARGETS=shared-tooling-check tools-check dependency-pins-check deps check' "MAKE=$PWD/target/gate-make"
        if [[ "$1" == fetch-cached ]]; then
            [[ "$(cat target/mock-cargo-cache)" == retained-cache ]] || exit 1
        else [[ ! -f target/mock-cargo-cache ]] || exit 1; fi
        # Failed bootstrap must stop before Cargo checks even with an old cache.
        if rg -q '^cargo check ' "$TEST_LOG"; then
            echo 'Cargo check ran after failed bootstrap' >&2; exit 1
        fi
    fi
    [[ "$(cat "$TEST_EFFECTS")" == "$expected" ]] || exit 1
    assert_unchanged; assert_cache_retained
}
test_preflight_tools() {
    before="$(fingerprint)"
    case "$1" in
        shellcheck-missing) export SHELLCHECK="$PWD/target/missing-shellcheck" ;;
        shellcheck-broken) export TEST_SHELLCHECK_FAIL=1 ;;
        sort-missing) export TEST_SORT_FAIL=1 ;;
        sort-wrong) export TEST_SORT_VERSION=0.0.0 ;;
    esac
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    [[ ! -s "$TEST_EFFECTS" && ! -e target/release-state/0.1.1.plan ]] || exit 1
    case "$1" in
        shellcheck-*) rg -F 'SHELLCHECK=/absolute/path/to/shellcheck' target/rejection.log >/dev/null ;;
        sort-*) rg -F 'cargo install cargo-sort --version 2.1.4 --locked' target/rejection.log >/dev/null ;;
    esac
    unset SHELLCHECK TEST_SHELLCHECK_FAIL TEST_SORT_FAIL TEST_SORT_VERSION
    # A selected executable with spaces in its path must survive Make dispatch.
    cp "$TEMPORARY/bin/shellcheck" 'target/selected shellcheck'
    export SHELLCHECK="$PWD/target/selected shellcheck"
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]] || exit 1
    assert_cache_retained
}
test_versions() {
    local kind expected
    for kind in patch minor major; do
        case "$kind" in patch) expected=0.1.1 ;; minor) expected=0.2.0 ;; major) expected=1.0.0 ;; esac
        [[ "$(bash scripts/ci/next-release-version.sh 0.1.0 "$kind")" == "$expected" ]] || exit 1
    done
}
test_invalid_changelog() {
    case "$1" in
        duplicate) printf '\n## [0.1.1]\n- Duplicate.\n' >> CHANGELOG.md ;;
        empty) printf '# Changelog\n\n## [0.1.1]\n' > CHANGELOG.md ;;
        competing) printf '\n## [0.2.0]\n- Competing.\n' >> CHANGELOG.md ;;
    esac
    before="$(fingerprint)"
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    [[ ! -e target/release-state/0.1.1.plan ]] || exit 1
}
test_release() {
    local kind="$1" expected
    case "$kind" in patch) expected=0.1.1 ;; minor) expected=0.2.0 ;; major) expected=1.0.0 ;; esac
    sed "s/\[0.1.1\]/[$expected]/" CHANGELOG.md > target/notes
    cp target/notes CHANGELOG.md
    "$TEST_REAL_MAKE" --no-print-directory "release-$kind"
    [[ "$(perl scripts/release/release-data.pl version)" == "$expected" ]] || exit 1
    perl scripts/release/release-data.pl verify
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]] || exit 1
    [[ "$(tail -n 1 "target/release-state/$expected.plan")" == complete ]] || exit 1
    # External package selection and initial imported history are unchanged.
    sed -n '/^name = "external"/,$p' Cargo.lock > target/external
    printf 'name = "external"\nversion = "0.1.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "unchanged-external-checksum"\n' > target/expected
    cmp target/external target/expected
    rg -q '^## \[0.1.0\]$' CHANGELOG.md
    assert_cache_retained
}
prepare_tagged_release() { test_release patch; : > "$TEST_EFFECTS"; }
test_failed_guard_read() {
    local operation="$1" read="$2"
    case "$operation" in
        prepared-check)
            export TEST_INDEX_EXTRA=1
            expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
            unset TEST_INDEX_EXTRA
            : > "$TEST_EFFECTS"
            ;;
        committed-check|publish) prepare_tagged_release ;;
    esac
    : > "$TEST_LOG"
    before="$(fingerprint)"
    if [[ -f docs/release.json ]]; then cp docs/release.json target/before-receipt; fi
    case "$read" in
        version|source) export TEST_DATA_FAIL_COMMAND="$read" ;;
        head) export TEST_GIT_FAIL_CALL='rev-parse HEAD' ;;
        verified-head) export TEST_GIT_FAIL_CALL='rev-parse --verify HEAD' ;;
        status) export TEST_GIT_FAIL_CALL='status --porcelain --untracked-files=all' ;;
        parent)
            if [[ "$operation" == committed-check ]]; then
                export TEST_GIT_FAIL_CALL='log -1 --format=%P 3333333333333333333333333333333333333333'
            else export TEST_GIT_FAIL_CALL='log -1 --format=%P HEAD'; fi
            ;;
    esac
    export RELEASE_SOURCE="$TEST_SOURCE" RELEASE_PREVIOUS=0.1.0
    export RELEASE_VERSION=0.1.1 RELEASE_COMMIT=3333333333333333333333333333333333333333
    RELEASE_DATE="$(date -u +%F)"
    export RELEASE_DATE
    expect_failure bash scripts/release/release.sh "$operation"
    [[ "$(fingerprint)" == "$before" && ! -s "$TEST_EFFECTS" ]] || exit 1
    if rg -q '^cargo ' "$TEST_LOG"; then
        echo 'Cargo dispatched after a failed admission read' >&2; exit 1
    fi
    if [[ -f target/before-receipt ]]; then cmp target/before-receipt docs/release.json;
    else [[ ! -e docs/release.json ]] || exit 1; fi
    [[ ! -d target/release-state/lock ]] || exit 1
    assert_cache_retained
}
test_rejected_preparation() {
    before="$(fingerprint)"
    export "$1=1"
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    case "$1" in TEST_METADATA_FAIL|TEST_PREPARE_FAIL) [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == prepare ]] || exit 1 ;; *) [[ ! -e target/release-state/0.1.1.plan ]] || exit 1 ;; esac
    unset "$1"
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    perl scripts/release/release-data.pl verify
    [[ "$(awk '$0 == "commit" { n++ } END { print n }' "$TEST_EFFECTS")" == 1 ]] || exit 1
}
test_preparation() {
    export TEST_INDEX_EXTRA=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == commit ]] || exit 1
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage' ]] || exit 1
    unset TEST_INDEX_EXTRA
    "$TEST_REAL_MAKE" --no-print-directory release-resume VERSION=0.1.1
    assert_cache_retained
}
test_publish() {
    prepare_tagged_release
    if [[ "$1" == dry-run ]]; then "$TEST_REAL_MAKE" --no-print-directory publish-dry-run;
    else "$TEST_REAL_MAKE" --no-print-directory publish; fi
    [[ "$(cat "$TEST_EFFECTS")" == publish ]] || exit 1
    assert_cache_retained
}
test_receipt_identity() {
    export TEST_INDEX_EXTRA=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    unset TEST_INDEX_EXTRA
    # A self-consistent receipt/notes pair still cannot change the saved date.
    sed "s/$(date -u +%F)/2000-01-01/" CHANGELOG.md > target/notes
    cp target/notes CHANGELOG.md
    perl scripts/release/release-data.pl receipt "$TEST_SOURCE" 2000-01-01
    perl scripts/release/release-data.pl verify
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-prepared-check \
        "RELEASE_SOURCE=$TEST_SOURCE" RELEASE_VERSION=0.1.1 "RELEASE_DATE=$(date -u +%F)"
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage' ]] || exit 1
    assert_cache_retained
}
test_invalid_publish() {
    prepare_tagged_release
    case "$1" in
        tag) export TEST_TAG_TYPE=commit ;;
        receipt) echo changed >> CHANGELOG.md ;;
        parent)
            perl -MJSON::PP -e 'local $/; open my $in,"<","docs/release.json" or die $!; my $data=decode_json(<$in>); close $in; $data->{source}="2" x 40; open my $out,">","docs/release.json" or die $!; print {$out} encode_json($data); close $out;'
            ;;
        merge) printf '%s %s\n' "$TEST_SOURCE" "$TEST_SOURCE" > target/history/3333333333333333333333333333333333333333.parent ;;
    esac
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
    [[ ! -d target/release-state/lock ]] || exit 1
    assert_cache_retained
}
test_publication_lock() {
    prepare_tagged_release
    mkdir target/release-state/lock
    echo another-owner > target/release-state/lock/owner
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" && "$(cat target/release-state/lock/owner)" == another-owner ]] || exit 1
    assert_cache_retained
}
test_push_retry() {
    export TEST_PUSH_FAIL=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == push ]] || exit 1
    unset TEST_PUSH_FAIL
    before="$(cat "$TEST_EFFECTS")"
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(cat "$TEST_EFFECTS")" == "$before" ]] || exit 1
    [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == complete ]] || exit 1
    assert_cache_retained
}
commit_fixture_fix() {
    local fix=6666666666666666666666666666666666666666
    mkdir -p target/history
    cat target/mock-head > "target/history/$fix.parent"
    echo 'Committed release callback fix' > "target/history/$fix.subject"
    echo "$fix" > target/mock-head
}
test_selected_commit() {
    prepare_tagged_release
    commit_fixture_fix
    # HEAD now has new draft notes; recovery must check the older frozen bytes.
    printf '\n## [0.1.2]\n\n- New fix.\n' >> CHANGELOG.md
    local commit=3333333333333333333333333333333333333333
    local target=release-push-check selected="$commit" source="$TEST_SOURCE"
    case "$1" in
        valid) ;;
        missing) selected='' ;;
        invalid) selected='HEAD' ;;
        source) source=2222222222222222222222222222222222222222 ;;
        parent) echo 2222222222222222222222222222222222222222 > "target/history/$commit.parent" ;;
        merge) printf '%s %s\n' "$TEST_SOURCE" "$TEST_SOURCE" > "target/history/$commit.parent" ;;
        receipt) echo changed >> "target/history/$commit.files/CHANGELOG.md" ;;
        absent) rm "target/history/$commit.files/Cargo.lock" ;;
        tag) echo 6666666666666666666666666666666666666666 > target/tags/v0.1.1.commit ;;
        tag-type) export TEST_TAG_TYPE=commit ;;
        dirty) export TEST_DIRTY=1 ;;
        publication) target=publish ;;
    esac
    if [[ "$1" == valid ]]; then
        for target in release-committed-check release-tagged-check release-push-check; do
            "$TEST_REAL_MAKE" --no-print-directory "$target" "RELEASE_COMMIT=$selected" \
                "RELEASE_SOURCE=$source" RELEASE_VERSION=0.1.1 "RELEASE_DATE=$(date -u +%F)"
        done
    else
        expect_failure "$TEST_REAL_MAKE" --no-print-directory "$target" "RELEASE_COMMIT=$selected" \
            "RELEASE_SOURCE=$source" RELEASE_VERSION=0.1.1 "RELEASE_DATE=$(date -u +%F)"
    fi
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
    assert_cache_retained
}
test_actual_index() {
    # Reuse committed history without creating a fixture commit. Qualify the
    # real index inventory, including staged bytes hidden by a restored worktree.
    local source objects path
    source="$("$TEST_REAL_GIT" -C "$ROOT" rev-parse HEAD)"
    objects="$("$TEST_REAL_GIT" -C "$ROOT" rev-parse --git-path objects)"
    case "$objects" in /*) ;; *) objects="$ROOT/$objects" ;; esac
    mkdir target/real-index
    cd target/real-index
    "$TEST_REAL_GIT" init --quiet
    mkdir -p .git/objects/info
    printf '%s\n' "$objects" > .git/objects/info/alternates
    "$TEST_REAL_GIT" update-ref HEAD "$source"
    "$TEST_REAL_GIT" read-tree HEAD
    "$TEST_REAL_GIT" checkout-index --all
    mkdir -p target
    for path in Cargo.toml Cargo.lock CHANGELOG.md docs/release.json; do printf '\n' >> "$path"; done
    "$TEST_REAL_GIT" add -- Cargo.toml Cargo.lock CHANGELOG.md docs/release.json
    "$TEST_REAL_GIT" diff --cached --name-only -z -- > target-index-paths
    perl "$ROOT/scripts/release/release-data.pl" index-check target-index-paths
    printf '\nUnrelated staged content.\n' >> README.md
    "$TEST_REAL_GIT" add README.md
    "$TEST_REAL_GIT" show HEAD:README.md > README.md
    "$TEST_REAL_GIT" diff --quiet HEAD -- README.md
    "$TEST_REAL_GIT" diff --cached --name-only -z -- > target-index-paths
    expect_failure perl "$ROOT/scripts/release/release-data.pl" index-check target-index-paths
}
test_followup_release() {
    sed 's/\[0.1.1\]/[0.2.0]/' CHANGELOG.md > target/notes
    cp target/notes CHANGELOG.md
    export TEST_PUSH_FAIL=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-minor
    unset TEST_PUSH_FAIL
    [[ "$(tail -n 1 target/release-state/0.2.0.plan)" == push ]] || exit 1
    cp target/history/3333333333333333333333333333333333333333.files/docs/release.json target/older-receipt
    cp target/tags/v0.2.0.sha target/older-tag
    commit_fixture_fix
    printf '\n## [0.2.1]\n\n- Committed fix.\n' >> CHANGELOG.md
    if [[ "$1" == resume ]]; then
        before="$(cat "$TEST_EFFECTS")"
        "$TEST_REAL_MAKE" --no-print-directory release-resume VERSION=0.2.0
        [[ "$(cat "$TEST_EFFECTS")" == "$before" && "$(perl scripts/release/release-data.pl version)" == 0.2.0 ]] || exit 1
    else
        case "$1" in
            gate-fail|missing-tag) export TEST_GATE_FAIL=1 ;;
        esac
        if [[ "$1" == missing-tag ]]; then rm target/remote-tags/v0.2.0; fi
        if [[ "${TEST_GATE_FAIL:-0}" == 1 ]]; then
            expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
            [[ "$(tail -n 1 target/release-state/0.2.0.plan)" == complete && "$(perl scripts/release/release-data.pl version)" == 0.2.0 ]] || exit 1
            [[ "$(cat target/remote-head)" == 3333333333333333333333333333333333333333 ]] || exit 1
            if [[ "$1" == missing-tag ]]; then
                rg -q '^git push --no-follow-tags --atomic origin 3333333333333333333333333333333333333333:refs/heads/main refs/tags/v0.2.0:refs/tags/v0.2.0$' "$TEST_LOG"
            fi
            # The real Make adapter retains the failed gate before a retry.
            local retained
            retained="$(rg --files target/release-state/validation-failures -g '*-ci.log')"
            [[ -n "$retained" ]] || exit 1
            cp "$retained" target/first-failed-validation.log
            unset TEST_GATE_FAIL
        fi
        "$TEST_REAL_MAKE" --no-print-directory release-patch
        [[ "$(perl scripts/release/release-data.pl version)" == 0.2.1 && "$(tail -n 1 target/release-state/0.2.1.plan)" == complete ]] || exit 1
        [[ "$(cat target/remote-head)" == 7777777777777777777777777777777777777777 ]] || exit 1
        [[ "$(cat target/validation.1.log)" == "validation source: $TEST_SOURCE" ]] || exit 1
        [[ "$(cat target/validation.2.log)" == 'validation source: 6666666666666666666666666666666666666666' ]] || exit 1
        perl scripts/release/release-data.pl verify 6666666666666666666666666666666666666666 0.2.1 "$(date -u +%F)"
        if [[ -f target/first-failed-validation.log ]]; then
            cmp "$retained" target/first-failed-validation.log
        fi
    fi
    [[ "$(tail -n 1 target/release-state/0.2.0.plan)" == complete ]] || exit 1
    cmp target/older-receipt target/history/3333333333333333333333333333333333333333.files/docs/release.json
    cmp target/older-tag target/tags/v0.2.0.sha
    assert_cache_retained
}
run_case bootstrap-success test_dependency_bootstrap success
run_case bootstrap-snapshot test_dependency_bootstrap snapshot
run_case bootstrap-tools test_dependency_bootstrap tools
run_case bootstrap-pins test_dependency_bootstrap pins
run_case bootstrap-fetch test_dependency_bootstrap fetch
run_case bootstrap-fetch-cached test_dependency_bootstrap fetch-cached
for tool in shellcheck-missing shellcheck-broken sort-missing sort-wrong; do
    run_case "preflight-$tool" test_preflight_tools "$tool"
done
run_case versions test_versions
for shape in duplicate empty competing; do run_case "notes-$shape" test_invalid_changelog "$shape"; done
for kind in patch minor major; do run_case "$kind" test_release "$kind"; done
for failure in TEST_GATE_FAIL TEST_METADATA_FAIL TEST_PREPARE_FAIL; do run_case "$failure" test_rejected_preparation "$failure"; done
run_case exact-index test_preparation
run_case receipt-identity test_receipt_identity
run_case publish test_publish normal
run_case publish-dry-run test_publish dry-run
for invalid in tag receipt parent merge; do run_case "bad-$invalid" test_invalid_publish "$invalid"; done
run_case publication-lock test_publication_lock
run_case lost-push-reply test_push_retry
run_case real-index test_actual_index
for shape in valid missing invalid source parent merge receipt absent tag tag-type dirty publication; do
    run_case "selected-$shape" test_selected_commit "$shape"
done
for shape in success gate-fail missing-tag resume; do run_case "followup-$shape" test_followup_release "$shape"; done
for operation in preflight prepare prepared-check committed-check publish plan; do
    case "$operation" in
        preflight) reads='version head verified-head status' ;;
        prepare) reads='version head status' ;;
        prepared-check) reads='version source head' ;;
        committed-check) reads='parent' ;;
        publish) reads='version source head parent status' ;;
        plan) reads='version' ;;
    esac
    for read in $reads; do
        run_case "failed-read-$operation-$read" test_failed_guard_read "$operation" "$read"
    done
done
printf 'Release-adapter tests: PASS (isolated Git/Cargo effects; no real commits, pushes or publication).\n'
