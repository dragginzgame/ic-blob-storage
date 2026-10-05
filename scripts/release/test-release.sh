#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
TEST_REAL_MAKE="$(command -v make)"
TEST_REAL_GIT="$(command -v git)"
export TEST_REAL_MAKE TEST_REAL_GIT
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
    cp "$ROOT/scripts/ci/run-release.sh" "$ROOT/scripts/ci/next-release-version.sh" "$FIXTURE/scripts/ci/"
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
[[ "${TEST_GATE_FAIL:-0}" != 1 ]]
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
assert_unchanged() { [[ "$(fingerprint)" == "$before" && ! -e docs/release.json ]]; }
assert_cache_retained() { [[ "$(cat target/debug/cache-sentinel)" == retained ]]; }

# Git effects are substitutes: no commits, tags, real push or publication.
cat > "$TEMPORARY/bin/git" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "git $*" >> "$TEST_LOG"
release_head=3333333333333333333333333333333333333333
tag_sha=4444444444444444444444444444444444444444
tree=5555555555555555555555555555555555555555
case "$1" in
    check-ref-format) [[ "$2" == refs/heads/main ]] ;;
    symbolic-ref) echo main ;;
    remote) echo https://invalid.example/fixture ;;
    hash-object) exec "$TEST_REAL_GIT" hash-object --stdin ;;
    rev-parse)
        case "${*: -1}" in
            --show-toplevel) pwd ;;
            release-state) echo target/release-state ;;
            HEAD) if [[ -f target/mock-head ]]; then cat target/mock-head; else echo "$TEST_SOURCE"; fi ;;
            HEAD^) echo "$TEST_SOURCE" ;;
            'HEAD^{tree}') echo "$tree" ;;
            refs/tags/*'^{commit}') [[ -f target/mock-tag ]]; echo "$release_head" ;;
            refs/tags/*) [[ -f target/mock-tag ]]; echo "$tag_sha" ;;
            *) exit 97 ;;
        esac ;;
    status)
        if [[ "${TEST_DIRTY:-0}" == 1 ]]; then echo ' M src/lib.rs';
        elif [[ ! -f target/mock-head && "$(perl scripts/release/release-data.pl version)" != 0.1.0 ]]; then echo ' M Cargo.toml'; fi ;;
    diff)
        case "$2" in
            --binary) cat Cargo.toml Cargo.lock CHANGELOG.md ;;
            --name-only) printf 'Cargo.toml\0Cargo.lock\0CHANGELOG.md\0docs/release.json\0'; [[ "${TEST_DIRTY:-0}" != 1 ]] || printf 'src/lib.rs\0' ;;
            --cached) [[ "$3" == --name-only ]]; printf 'Cargo.toml\0Cargo.lock\0CHANGELOG.md\0docs/release.json\0'; [[ "${TEST_INDEX_EXTRA:-0}" != 1 ]] || printf 'src/lib.rs\0' ;;
            --quiet) [[ "${TEST_DIRTY:-0}" != 1 ]] ;;
            *) exit 97 ;;
        esac ;;
    ls-files) ;;
    write-tree) echo "$tree" ;;
    log) printf 'Release %s\n' "$(perl scripts/release/release-data.pl version)" ;;
    add) [[ "$*" == 'add -- Cargo.toml Cargo.lock CHANGELOG.md docs/release.json' ]]; echo stage >> "$TEST_EFFECTS" ;;
    commit) echo commit >> "$TEST_EFFECTS"; echo "$release_head" > target/mock-head ;;
    tag)
        if [[ "$2" == -a ]]; then echo tag >> "$TEST_EFFECTS"; echo "${3#v}" > target/mock-tag;
        elif [[ -f target/mock-tag && "$(cat target/mock-tag)" == "${3#v}" ]]; then echo "$3"; fi ;;
    cat-file) echo "${TEST_TAG_TYPE:-tag}" ;;
    ls-remote)
        for ref in "$@"; do
            case "$ref" in
                refs/heads/main) [[ ! -f target/remote-head ]] || printf '%s\t%s\n' "$release_head" "$ref" ;;
                refs/tags/*) [[ ! -f target/remote-tag ]] || printf '%s\t%s\n' "$tag_sha" "$ref" ;;
            esac
        done ;;
    push)
        current="$(perl scripts/release/release-data.pl version)"
        [[ "$*" == "push --no-follow-tags --atomic origin HEAD:refs/heads/main refs/tags/v$current:refs/tags/v$current" ]]
        echo push >> "$TEST_EFFECTS"
        touch target/remote-head target/remote-tag
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
        [[ "${TEST_METADATA_FAIL:-0}" != 1 ]]
        current="$(perl scripts/release/release-data.pl version)"
        printf '{"workspace_members":["local"],"packages":[{"id":"local","name":"ic-blob-storage","version":"%s","source":null}]}\n' "$current" ;;
    'sort --version') echo 'cargo-sort 2.1.4' ;;
    'sort --workspace --check') [[ "${TEST_PREPARE_FAIL:-0}" != 1 ]] ;;
    'fmt --all -- --check') ;;
    'fetch --locked') echo fetch >> "$TEST_EFFECTS"; [[ "${TEST_FETCH_FAIL:-0}" != 1 ]]; touch target/mock-cargo-cache ;;
    'check --offline --locked --workspace --all-targets --all-features') [[ -f target/mock-cargo-cache ]]; echo check >> "$TEST_EFFECTS" ;;
    'publish --locked --registry crates-io -p ic-blob-storage'|'publish --locked --registry crates-io -p ic-blob-storage --dry-run') echo publish >> "$TEST_EFFECTS" ;;
    *) echo "unsupported Cargo substitute: $*" >&2; exit 97 ;;
esac
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
    '--no-print-directory deps'|'--no-print-directory check') exec "$TEST_REAL_MAKE" "$@" ;;
    *) exit 97 ;;
esac
MOCK
    chmod +x target/gate-make
    before="$(fingerprint)"
    case "$1" in
        snapshot) export TEST_SNAPSHOT_FAIL=1; expected=snapshot ;;
        fetch) export TEST_FETCH_FAIL=1; expected=$'snapshot\nfetch' ;;
        success) expected=$'snapshot\nfetch\ncheck' ;;
    esac
    if [[ "$1" == success ]]; then
        "$TEST_REAL_MAKE" --no-print-directory ci 'CI_TARGETS=shared-tooling-check deps check' "MAKE=$PWD/target/gate-make"
    else
        expect_failure "$TEST_REAL_MAKE" --no-print-directory ci 'CI_TARGETS=shared-tooling-check deps check' "MAKE=$PWD/target/gate-make"
        [[ ! -f target/mock-cargo-cache ]]
    fi
    [[ "$(cat "$TEST_EFFECTS")" == "$expected" ]]
    assert_unchanged; assert_cache_retained
}
test_versions() {
    local kind expected
    for kind in patch minor major; do
        case "$kind" in patch) expected=0.1.1 ;; minor) expected=0.2.0 ;; major) expected=1.0.0 ;; esac
        [[ "$(bash scripts/ci/next-release-version.sh 0.1.0 "$kind")" == "$expected" ]]
    done
    expect_failure "$TEST_REAL_MAKE" release-patch release-minor
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
    [[ ! -e target/release-state/0.1.1.plan ]]
}
test_release() {
    local kind="$1" expected
    case "$kind" in patch) expected=0.1.1 ;; minor) expected=0.2.0 ;; major) expected=1.0.0 ;; esac
    sed "s/\[0.1.1\]/[$expected]/" CHANGELOG.md > target/notes
    cp target/notes CHANGELOG.md
    "$TEST_REAL_MAKE" --no-print-directory "release-$kind"
    [[ "$(perl scripts/release/release-data.pl version)" == "$expected" ]]
    perl scripts/release/release-data.pl verify
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]]
    [[ "$(tail -n 1 "target/release-state/$expected.plan")" == complete ]]
    # External package selection and initial imported history are unchanged.
    sed -n '/^name = "external"/,$p' Cargo.lock > target/external
    printf 'name = "external"\nversion = "0.1.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "unchanged-external-checksum"\n' > target/expected
    cmp target/external target/expected
    rg -q '^## \[0.1.0\]$' CHANGELOG.md
    assert_cache_retained
}
prepare_tagged_release() { test_release patch; : > "$TEST_EFFECTS"; }
test_rejected_preparation() {
    before="$(fingerprint)"
    export "$1=1"
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    case "$1" in TEST_METADATA_FAIL|TEST_PREPARE_FAIL) [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == prepare ]] ;; *) [[ ! -e target/release-state/0.1.1.plan ]] ;; esac
    unset "$1"
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    perl scripts/release/release-data.pl verify
    [[ "$(awk '$0 == "commit" { n++ } END { print n }' "$TEST_EFFECTS")" == 1 ]]
}
test_preparation() {
    export TEST_INDEX_EXTRA=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == commit ]]
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage' ]]
    unset TEST_INDEX_EXTRA
    "$TEST_REAL_MAKE" --no-print-directory release-resume VERSION=0.1.1
    assert_cache_retained
}
test_publish() {
    prepare_tagged_release
    if [[ "$1" == dry-run ]]; then "$TEST_REAL_MAKE" --no-print-directory publish-dry-run;
    else "$TEST_REAL_MAKE" --no-print-directory publish; fi
    [[ "$(cat "$TEST_EFFECTS")" == publish ]]
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
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage' ]]
    assert_cache_retained
}
test_invalid_publish() {
    prepare_tagged_release
    export TEST_TAG_TYPE=commit
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" ]]
}
test_publication_lock() {
    prepare_tagged_release
    mkdir target/release-state/lock
    echo another-owner > target/release-state/lock/owner
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" && "$(cat target/release-state/lock/owner)" == another-owner ]]
    assert_cache_retained
}
test_push_retry() {
    export TEST_PUSH_FAIL=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == push ]]
    unset TEST_PUSH_FAIL
    before="$(cat "$TEST_EFFECTS")"
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(cat "$TEST_EFFECTS")" == "$before" ]]
    [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == complete ]]
    assert_cache_retained
}
run_case bootstrap-success test_dependency_bootstrap success
run_case bootstrap-snapshot test_dependency_bootstrap snapshot
run_case bootstrap-fetch test_dependency_bootstrap fetch
run_case versions test_versions
for shape in duplicate empty competing; do run_case "notes-$shape" test_invalid_changelog "$shape"; done
for kind in patch minor major; do run_case "$kind" test_release "$kind"; done
for failure in TEST_GATE_FAIL TEST_METADATA_FAIL TEST_PREPARE_FAIL; do run_case "$failure" test_rejected_preparation "$failure"; done
run_case exact-index test_preparation
run_case receipt-identity test_receipt_identity
run_case publish test_publish normal
run_case publish-dry-run test_publish dry-run
run_case bad-tag test_invalid_publish
run_case publication-lock test_publication_lock
run_case lost-push-reply test_push_retry
printf 'Release-adapter tests: PASS (isolated Git/Cargo effects; no real commits, pushes or publication).\n'
