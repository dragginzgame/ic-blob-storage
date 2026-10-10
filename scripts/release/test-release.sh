#!/usr/bin/env bash
set -Eeuo pipefail
# Independent fixtures own their Make selections. Parent command-line variables
# otherwise override the environment selections exercised by refusal cases.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
export RELEASE_DELIVERY=direct

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
TEST_REAL_MAKE="$(command -v make)"
TEST_REAL_GIT="$(command -v git)"
TEST_REAL_PERL="$(command -v perl)"
TEST_REAL_AWK="$(command -v awk)"
export TEST_REAL_MAKE TEST_REAL_GIT TEST_REAL_PERL TEST_REAL_AWK
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
    mkdir -p "$FIXTURE/scripts/release" "$FIXTURE/scripts/ci" "$FIXTURE/ci" "$FIXTURE/docs" "$FIXTURE/make" "$FIXTURE/target/debug"
    cp "$ROOT/scripts/release/release.sh" "$ROOT/scripts/release/release-data.pl" "$FIXTURE/scripts/release/"
    cp "$ROOT/scripts/ci/run-release.sh" "$ROOT/scripts/ci/next-release-version.sh" \
        "$ROOT/scripts/ci/run-validation-targets.sh" "$ROOT/scripts/ci/check-release-tag.sh" \
        "$ROOT/scripts/ci/rewrite-local-lock-versions.pl" \
        "$ROOT/scripts/ci/check-make-execution.sh" \
        "$ROOT/scripts/ci/check-release-source.sh" \
        "$ROOT/scripts/ci/finalize-release-changelog.awk" \
        "$ROOT/scripts/ci/check-format-tools.sh" "$ROOT/scripts/ci/run-formatting.sh" "$FIXTURE/scripts/ci/"
    cp "$ROOT/ci/tool-versions.env" "$FIXTURE/ci/"
    cp "$ROOT/Makefile" "$FIXTURE/Makefile"
    cp "$ROOT/make/tools.mk" "$ROOT/make/release.mk" "$ROOT/make/rust-format.mk" "$ROOT/make/execution.mk" "$FIXTURE/make/"
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

[workspace.dependencies]
ic-blob-storage-contracts = { path = "crates/ic-blob-storage-contracts", version = "0.1.0" }
TOML
    cat > Cargo.lock <<'LOCK'
version = 4

[[package]]
name = "ic-blob-storage"
version = "0.1.0"
dependencies = [
 "ic-blob-storage-contracts 0.1.0",
 "external 0.1.0",
]

[[package]]
name = "ic-blob-storage-contracts"
version = "0.1.0"

[[package]]
name = "external"
version = "0.1.0"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "unchanged-external-checksum"
LOCK
    printf '# Changelog\n\n## [0.1.1]\n\n- Test notes.\n\n## [0.1.0]\n\n- Imported undated history.\n' > CHANGELOG.md
    : > "$TEST_LOG"; : > "$TEST_EFFECTS"
    echo retained > target/debug/cache-sentinel
    unset PUBLISH_PACKAGE TEST_PUBLISH_FAIL TEST_PUBLISH_LOST_REPLY TEST_REGISTRY_FAILURE TEST_REGISTRY_FOREIGN TEST_REGISTRY_YANKED
    mkdir -p target/registry
    unset TEST_DIRTY TEST_GATE_FAIL TEST_GATE_DIRTY TEST_METADATA_FAIL TEST_PREPARE_FAIL TEST_INDEX_EXTRA TEST_PUSH_FAIL TEST_FETCH_FAIL TEST_SNAPSHOT_FAIL TEST_TAG_TYPE
    unset TEST_GIT_FAIL_CALL TEST_DATA_FAIL_COMMAND
    unset TEST_FINALIZER_FAIL
    unset TEST_FMT_FAIL
    unset TEST_RELEASE_SETUP_FAIL TEST_RELEASE_SETUP_CHANGE
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

# Preparation is a substitute too: fixtures never provision real tools or fetch.
cat > "$TEMPORARY/bin/make" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
case "$*" in
    '--no-print-directory deps'|'--no-print-directory install-tools'|'--no-print-directory tools-check')
        selection="${*: -1}"
        echo "$selection" >> target/preparation.log
        [[ "${TEST_RELEASE_SETUP_FAIL:-}" != "$selection" ]] || exit 1
        if [[ "$selection" == install-tools ]]; then
            case "${TEST_RELEASE_SETUP_CHANGE:-}" in
                selection) perl -pi -e 's/version = "0\.1\.0"/version = "0.1.9"/' Cargo.toml ;;
                head) echo 3333333333333333333333333333333333333333 > target/mock-head ;;
            esac
        fi
        ;;
    *) exec "$TEST_REAL_MAKE" "$@" ;;
esac
MOCK

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
            --show-prefix) ;;
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
        observed=''
        if [[ "${TEST_DIRTY:-0}" == 1 ]]; then observed=' M src/lib.rs';
        elif [[ ! -f target/mock-head && "$(perl scripts/release/release-data.pl version)" != 0.1.0 ]]; then observed=' M Cargo.toml'; fi
        if [[ -n "$observed" ]]; then
            case "$*" in
                *' -z '*) printf '%s\0' "$observed" ;;
                *) printf '%s\n' "$observed" ;;
            esac
        fi ;;
    diff)
        case "$2" in
            --binary) cat Cargo.toml Cargo.lock CHANGELOG.md ;;
            --name-only) printf 'Cargo.toml\0Cargo.lock\0CHANGELOG.md\0docs/release.json\0'; [[ "${TEST_DIRTY:-0}" != 1 ]] || printf 'src/lib.rs\0' ;;
            --cached)
                case "$3" in
                    --name-only) printf 'Cargo.toml\0Cargo.lock\0CHANGELOG.md\0docs/release.json\0'; [[ "${TEST_INDEX_EXTRA:-0}" != 1 ]] || printf 'src/lib.rs\0' ;;
                    --quiet) [[ "${TEST_INDEX_EXTRA:-0}" != 1 ]] ;;
                    *) exit 97 ;;
                esac ;;
            --quiet) [[ "${TEST_DIRTY:-0}" != 1 ]] ;;
            *) exit 97 ;;
        esac ;;
    ls-files) ;;
    # Tracking-ref mutation is qualified by the canonical real-Git fixture.
    # This metadata/publication fixture has no configured upstream mapping.
    for-each-ref) ;;
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
        [[ "$#" == 7 && "$2" == --no-follow-tags && "$3" == --atomic && "$4" == -- && "$5" == https://invalid.example/fixture && "$6" == *:refs/heads/main && "$7" == refs/tags/v*:refs/tags/v* ]] || exit 97
        tag="${7%%:*}"; tag="${tag#refs/tags/}"
        [[ "$7" == "refs/tags/$tag:refs/tags/$tag" ]] || exit 97
        mkdir -p target/remote-tags
        resolve "${6%:refs/heads/main}" > target/remote-head
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
        printf '{"workspace_members":["local","contracts"],"packages":[{"id":"local","name":"ic-blob-storage","version":"%s","source":null},{"id":"contracts","name":"ic-blob-storage-contracts","version":"%s","source":null}]}\n' "$current" "$current" ;;
    'sort --version') [[ "${TEST_SORT_FAIL:-0}" != 1 ]] || exit 1; echo "cargo-sort ${TEST_SORT_VERSION:-2.1.4}" ;;
    'fmt --version') echo rustfmt; [[ "${TEST_FMT_FAIL:-0}" != 1 ]] ;;
    'sort --workspace --check') [[ "${TEST_PREPARE_FAIL:-0}" != 1 ]] ;;
    'fmt --all -- --check') ;;
    'fetch --locked') echo fetch >> "$TEST_EFFECTS"; [[ "${TEST_FETCH_FAIL:-0}" != 1 ]] || exit 1; touch target/mock-cargo-cache ;;
    'check --offline --locked --workspace --all-targets --all-features') [[ -f target/mock-cargo-cache ]] || exit 1; echo check >> "$TEST_EFFECTS" ;;
     'publish --locked --registry crates-io -p ic-blob-storage'|'publish --locked --registry crates-io -p ic-blob-storage --dry-run'|'publish --locked --registry crates-io -p ic-blob-storage-contracts'|'publish --locked --registry crates-io -p ic-blob-storage-contracts --dry-run')
        package="$6"
        echo "publish:$package" >> "$TEST_EFFECTS"
        [[ "${TEST_PUBLISH_FAIL:-}" != "$package" ]] || exit 1
        if [[ "${7:-}" != --dry-run ]]; then
            current="$(perl scripts/release/release-data.pl version)"
            mkdir -p "target/registry/$package-$current"
            printf '{"git":{"sha1":"%s"},"path_in_vcs":"crates/%s"}\n' "$(git rev-parse HEAD)" "$package" > "target/registry/$package-$current/.cargo_vcs_info.json"
            tar -czf "target/registry/$package.crate" -C target/registry "$package-$current"
        fi
        [[ "${TEST_PUBLISH_LOST_REPLY:-}" != "$package" ]] || exit 1 ;;

    *) echo "unsupported Cargo substitute: $*" >&2; exit 97 ;;
esac
MOCK
cat > "$TEMPORARY/bin/curl" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "curl $*" >> "$TEST_LOG"
output= url= user_agent=
[[ "${1:-}" == --disable ]] || exit 97
while [[ $# -gt 0 ]]; do
    case "$1" in
        --output) output="$2"; shift 2 ;;
        --user-agent) user_agent="$2"; shift 2 ;;
        --connect-timeout|--max-time|--max-filesize|--write-out) shift 2 ;;
        --disable|--silent|--show-error|--fail) shift ;;
        https://*) url="$1"; shift ;;
        *) exit 97 ;;
    esac
done
[[ -n "$output" && -n "$url" ]] || exit 97
# Model registry admission: unidentified application reads fail before lookup.
if [[ "$user_agent" != "ic-blob-storage-release/$(perl scripts/release/release-data.pl version) (+https://github.com/dragginzgame/ic-blob-storage)" ]]; then
    : > "$output"; printf 403; exit 0
fi
[[ "${TEST_REGISTRY_FAILURE:-}" != transport ]] || exit 1
case "$url" in
    https://crates.io/api/v1/crates/*)
        suffix="${url#https://crates.io/api/v1/crates/}"
        package="${suffix%/*}"; current="${suffix##*/}"
        if [[ "${TEST_REGISTRY_FAILURE:-}" == http ]]; then
            echo '{}' > "$output"; printf 503
        elif [[ "${TEST_REGISTRY_FAILURE:-}" == forbidden ]]; then
            : > "$output"; printf 403
        elif [[ "${TEST_REGISTRY_FAILURE:-}" == malformed ]]; then
            echo '{' > "$output"; printf 200
        elif [[ -f "target/registry/$package.crate" ]]; then
            checksum="$(shasum -a 256 "target/registry/$package.crate")"; checksum="${checksum%% *}"
            [[ "${TEST_REGISTRY_FAILURE:-}" != checksum ]] || checksum="$(printf '%064d' 0)"
            echo "{\"version\":{\"crate\":\"${TEST_REGISTRY_FOREIGN:-$package}\",\"num\":\"$current\",\"yanked\":${TEST_REGISTRY_YANKED:-false},\"checksum\":\"$checksum\"}}" > "$output"
            printf 200
        else echo '{}' > "$output"; printf 404; fi ;;
    https://static.crates.io/crates/*)
        suffix="${url#https://static.crates.io/crates/}"; package="${suffix%%/*}"
        cp "target/registry/$package.crate" "$output" ;;
    *) exit 97 ;;
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
cat > "$TEMPORARY/bin/awk" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *scripts/ci/finalize-release-changelog.awk* ]]; then
    if [[ "${TEST_FINALIZER_FAIL:-}" == preflight ]] ||
       { [[ "${TEST_FINALIZER_FAIL:-}" == after-bump ]] &&
         [[ "$("$TEST_REAL_PERL" scripts/release/release-data.pl version)" == 0.1.1 ]]; }; then
        "$TEST_REAL_AWK" "$@" | tee target/failed-finalizer-candidate
        echo 'Controlled finalizer failure after complete candidate output' >&2
        exit 1
    fi
fi
exec "$TEST_REAL_AWK" "$@"
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
    '--no-print-directory -f - MAKEFILES=') exec "$TEST_REAL_MAKE" "$@" ;;
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
        rustfmt-broken) export TEST_FMT_FAIL=1 ;;
    esac
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    [[ ! -s "$TEST_EFFECTS" && ! -e target/release-state/0.1.1.plan ]] || exit 1
    case "$1" in
        shellcheck-*) rg -F 'SHELLCHECK=/absolute/path/to/shellcheck' target/rejection.log >/dev/null ;;
    esac
    unset SHELLCHECK TEST_SHELLCHECK_FAIL TEST_SORT_FAIL TEST_SORT_VERSION TEST_FMT_FAIL
    # A selected executable with spaces in its path must survive Make dispatch.
    cp "$TEMPORARY/bin/shellcheck" 'target/selected shellcheck'
    export SHELLCHECK="$PWD/target/selected shellcheck"
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]] || exit 1
    assert_cache_retained
}
test_selected_tool_preparation() {
    before="$(fingerprint)"
    export TEST_RELEASE_SETUP_FAIL="$1"
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    [[ ! -s "$TEST_EFFECTS" && ! -e target/gate-ran && ! -e docs/release.json ]] || exit 1
    case "$1" in
        deps) expected=deps ;;
        install-tools) expected=$'deps\ninstall-tools' ;;
        tools-check) expected=$'deps\ninstall-tools\ntools-check' ;;
    esac
    [[ "$(cat target/preparation.log)" == "$expected" ]] || exit 1
    unset TEST_RELEASE_SETUP_FAIL
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]] || exit 1
}
test_tool_preparation_source_change() {
    local shape="$1"
    export TEST_RELEASE_SETUP_CHANGE="$shape"
    cp Cargo.lock target/before-lock
    cp CHANGELOG.md target/before-notes
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(cat target/preparation.log)" == $'deps\ninstall-tools\ntools-check' ]] || exit 1
    [[ ! -s "$TEST_EFFECTS" && ! -e target/gate-ran && ! -e docs/release.json ]] || exit 1
    if [[ "$shape" == selection ]]; then
        [[ "$(perl scripts/release/release-data.pl version)" == 0.1.9 ]] || exit 1
        rg -F 'unstaged: Cargo.toml' target/rejection.log >/dev/null
    else
        rg -F 'source commit changed during tool preparation' target/rejection.log >/dev/null
    fi
    cmp Cargo.lock target/before-lock
    cmp CHANGELOG.md target/before-notes
    assert_cache_retained
}
test_directory_admission() {
    local shape="$1" regular invalid selected path
    regular="$PWD/target/directory-source"
    case "$shape" in
        lf|alias) invalid="$regular"$'\n' ;;
        cr) invalid="$regular"$'\r' ;;
    esac
    for path in "$regular" "$invalid"; do
        mkdir -p "$path/scripts/release"
        cp scripts/release/release.sh scripts/release/release-data.pl "$path/scripts/release/"
    done
    printf '[workspace]\n[workspace.package]\nversion = "1.2.3"\n' > "$regular/Cargo.toml"
    printf '[workspace]\n[workspace.package]\nversion = "9.9.9"\n' > "$invalid/Cargo.toml"
    selected="$invalid"
    if [[ "$shape" == alias ]]; then
        selected="$PWD/target/directory-alias"
        ln -s "$invalid" "$selected"
    fi
    expect_failure bash "$selected/scripts/release/release.sh" version
    rg -F 'release directory must not contain LF or CR' target/rejection.log >/dev/null
    [[ "$(bash "$regular/scripts/release/release.sh" version)" == 1.2.3 ]] || exit 1
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
    assert_cache_retained
}
test_source_diagnostics() {
    before="$(fingerprint)"
    export TEST_DIRTY=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    rg -F 'unstaged: src/lib.rs' target/rejection.log >/dev/null
    rg -F 'preflight refused; this attempt has not started validation or version preparation' target/rejection.log >/dev/null
    [[ ! -s "$TEST_EFFECTS" && ! -e target/gate-ran && ! -e docs/release.json ]] || exit 1
    assert_unchanged; assert_cache_retained
}
test_invalid_changelog() {
    case "$1" in
        duplicate) printf '\n## [0.1.1]\n- Duplicate.\n' >> CHANGELOG.md ;;
        historical-duplicate) printf '\n## [0.1.0]\n- Duplicate history.\n' >> CHANGELOG.md ;;
        competing) printf '\n## [0.2.0]\n- Competing.\n' >> CHANGELOG.md ;;
    esac
    before="$(fingerprint)"
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    [[ ! -e target/release-state/0.1.1.plan ]] || exit 1
}
test_changelog_content() {
    case "$1" in
        empty) printf '# Changelog\n\n## [0.1.1]\n\n## [0.1.0]\n\n- Imported undated history.\n' > CHANGELOG.md ;;
        missing) printf '# Changelog\n\n## [0.1.0]\n\n- Imported undated history.\n' > CHANGELOG.md ;;
    esac
    before="$(fingerprint)"
    perl scripts/release/release-data.pl changelog-check 0.1.1 0.1.0 2040-01-02
    assert_unchanged
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    perl scripts/release/release-data.pl verify
    rg -q '^## \[0.1.0\]$' CHANGELOG.md
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]] || exit 1
    assert_cache_retained
}
test_changelog_intent() {
    # Preparation has already bumped package metadata; historical classification
    # must still use the previous version saved in the release intent.
    sed 's/version = "0.1.0"/version = "0.1.1"/' Cargo.toml > target/manifest
    cp target/manifest Cargo.toml
    before="$(fingerprint)"
    perl scripts/release/release-data.pl changelog-check 0.1.1 0.1.0 2040-01-02
    assert_unchanged
    perl scripts/release/release-data.pl finalize 0.1.1 0.1.0 2040-01-02
    printf '# Changelog\n\n## [0.1.1] - 2040-01-02\n\n- Test notes.\n\n## [0.1.0]\n\n- Imported undated history.\n' > target/expected-changelog
    cmp CHANGELOG.md target/expected-changelog
    before="$(fingerprint)"
    # Prepared-state recovery verifies its frozen receipt; it does not call this
    # finalizer again, even for the same date.
    expect_failure perl scripts/release/release-data.pl finalize 0.1.1 0.1.0 2040-01-02
    assert_unchanged
    expect_failure perl scripts/release/release-data.pl finalize 0.1.1 0.1.0 2040-01-03
    assert_unchanged
    printf '# Changelog\n\n## [0.2.0]\n- Target.\n\n## [0.1.1]\n- Competing.\n\n## [0.1.0]\n- Historical.\n' > CHANGELOG.md
    sed 's/version = "0.1.1"/version = "0.2.0"/' Cargo.toml > target/manifest
    cp target/manifest Cargo.toml
    before="$(fingerprint)"
    expect_failure perl scripts/release/release-data.pl finalize 0.2.0 0.1.0 2040-01-02
    assert_unchanged
}
test_finalizer_failure() {
    export TEST_FINALIZER_FAIL="$1"
    before="$(fingerprint)"
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    assert_unchanged; assert_cache_retained
    [[ -s target/failed-finalizer-candidate ]] || exit 1
    if [[ "$1" == after-bump ]]; then
        [[ "$(tail -n 1 target/release-state/0.1.1.plan)" == prepare ]] || exit 1
        local backup
        backup="$(find target -maxdepth 1 -type d -name 'release-backup.*')"
        [[ -n "$backup" && -f "$backup/metadata.json" ]] || exit 1
        cmp "$backup/CHANGELOG.md" CHANGELOG.md
    else
        [[ ! -s "$TEST_EFFECTS" && ! -e target/release-state/0.1.1.plan ]] || exit 1
    fi
    unset TEST_FINALIZER_FAIL
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    perl scripts/release/release-data.pl verify
    [[ "$(awk '$0 == "commit" { n++ } END { print n }' "$TEST_EFFECTS")" == 1 ]] || exit 1
}
test_release() {
    local expected=0.1.1
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(perl scripts/release/release-data.pl version)" == "$expected" ]] || exit 1
    perl scripts/release/release-data.pl verify
    [[ "$(cat "$TEST_EFFECTS")" == $'validate\nstage\ncommit\ntag\npush' ]] || exit 1
    [[ "$(tail -n 1 "target/release-state/$expected.plan")" == complete ]] || exit 1
    rg -q '^ic-blob-storage-contracts = \{ path = "crates/ic-blob-storage-contracts", version = "0.1.1" \}$' Cargo.toml
    rg -q '^ "ic-blob-storage-contracts 0.1.1",$' Cargo.lock
    # External package selection and initial imported history are unchanged.
    sed -n '/^name = "external"/,$p' Cargo.lock > target/external
    printf 'name = "external"\nversion = "0.1.0"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\nchecksum = "unchanged-external-checksum"\n' > target/expected
    cmp target/external target/expected
    rg -q '^## \[0.1.0\]$' CHANGELOG.md
    assert_cache_retained
}
prepare_tagged_release() { test_release; : > "$TEST_EFFECTS"; }
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
        prefix) export TEST_GIT_FAIL_CALL='rev-parse --show-prefix' ;;
        status) export TEST_GIT_FAIL_CALL='status --porcelain=v1 -z --untracked-files=all' ;;
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
    if [[ "${2:-}" != '' ]]; then export PUBLISH_PACKAGE="$2"; fi
    # Dry-run core needs a real indexed contracts dependency, unlike local paired verification.
    if [[ "$1" == dry-run && "${2:-}" != ic-blob-storage-contracts ]]; then
        "$TEST_REAL_MAKE" --no-print-directory publish PUBLISH_PACKAGE=ic-blob-storage-contracts
        : > "$TEST_EFFECTS"
    fi
    if [[ "$1" == dry-run ]]; then "$TEST_REAL_MAKE" --no-print-directory publish-dry-run;
    else "$TEST_REAL_MAKE" --no-print-directory publish; fi
    if [[ "${2:-}" == '' ]]; then
        [[ "$(cat "$TEST_EFFECTS")" == $'publish:ic-blob-storage-contracts\npublish:ic-blob-storage' ]] || exit 1
    else [[ "$(cat "$TEST_EFFECTS")" == "publish:$2" ]] || exit 1; fi
    assert_cache_retained
}
test_publication_retry() {
    prepare_tagged_release
    case "$1" in
        contracts-failure) export TEST_PUBLISH_FAIL=ic-blob-storage-contracts ;;
        core-failure) export TEST_PUBLISH_FAIL=ic-blob-storage ;;
        contracts-lost-reply) export TEST_PUBLISH_LOST_REPLY=ic-blob-storage-contracts ;;
        core-lost-reply) export TEST_PUBLISH_LOST_REPLY=ic-blob-storage ;;
    esac
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    if [[ "$1" == contracts-* ]]; then
        [[ "$(cat "$TEST_EFFECTS")" == publish:ic-blob-storage-contracts ]] || exit 1
    else
        [[ "$(cat "$TEST_EFFECTS")" == $'publish:ic-blob-storage-contracts\npublish:ic-blob-storage' ]] || exit 1
    fi
    unset TEST_PUBLISH_FAIL TEST_PUBLISH_LOST_REPLY
    : > "$TEST_EFFECTS"
    "$TEST_REAL_MAKE" --no-print-directory publish
    case "$1" in
        contracts-failure) [[ "$(cat "$TEST_EFFECTS")" == $'publish:ic-blob-storage-contracts\npublish:ic-blob-storage' ]] || exit 1 ;;
        core-lost-reply) [[ ! -s "$TEST_EFFECTS" ]] || exit 1 ;;
        *) [[ "$(cat "$TEST_EFFECTS")" == publish:ic-blob-storage ]] || exit 1 ;;
    esac
    : > "$TEST_EFFECTS"
    "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
    assert_cache_retained
}
test_registry_refusal() {
    prepare_tagged_release
    if [[ "$1" == checksum || "$1" == foreign || "$1" == source || "$1" == yanked || "$1" == dirty || "$1" == path ]]; then
        "$TEST_REAL_MAKE" --no-print-directory publish
        : > "$TEST_EFFECTS"
    fi
    case "$1" in
        foreign) export TEST_REGISTRY_FOREIGN=foreign-package ;;
        yanked) export TEST_REGISTRY_YANKED=true ;;
        source|dirty|path)
            package=ic-blob-storage-contracts
            case "$1" in
                source) vcs="{\"git\":{\"sha1\":\"$TEST_SOURCE\"},\"path_in_vcs\":\"crates/$package\"}" ;;
                dirty) vcs="{\"git\":{\"sha1\":\"$(git rev-parse HEAD)\",\"dirty\":true},\"path_in_vcs\":\"crates/$package\"}" ;;
                path) vcs="{\"git\":{\"sha1\":\"$(git rev-parse HEAD)\"},\"path_in_vcs\":\"crates/foreign\"}" ;;
            esac
            printf '%s\n' "$vcs" > "target/registry/$package-0.1.1/.cargo_vcs_info.json"
            tar -czf "target/registry/$package.crate" -C target/registry "$package-0.1.1" ;;
        *) export TEST_REGISTRY_FAILURE="$1" ;;
    esac
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
    assert_cache_retained
}
test_unpublished_dry_run() {
    prepare_tagged_release
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish-dry-run
    [[ "$(cat "$TEST_EFFECTS")" == publish:ic-blob-storage-contracts ]] || exit 1
    [[ ! -f target/registry/ic-blob-storage-contracts.crate ]] || exit 1
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
# One actual Make-to-adapter recovery proof; the shared suite owns the matrix.
test_recovery_integration() {
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
    export TEST_GATE_FAIL=1
    expect_failure "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(tail -n 1 target/release-state/0.2.0.plan)" == complete && "$(perl scripts/release/release-data.pl version)" == 0.2.0 ]] || exit 1
    [[ "$(cat target/remote-head)" == 3333333333333333333333333333333333333333 ]] || exit 1
    # The real validation adapter retains its failed log across the retry.
    local retained
    retained="$(rg --files target/release-state/validation-failures -g '*-ci.log')"
    [[ -n "$retained" ]] || exit 1
    cp "$retained" target/first-failed-validation.log
    unset TEST_GATE_FAIL
    "$TEST_REAL_MAKE" --no-print-directory release-patch
    [[ "$(perl scripts/release/release-data.pl version)" == 0.2.1 && "$(tail -n 1 target/release-state/0.2.1.plan)" == complete ]] || exit 1
    [[ "$(cat target/remote-head)" == 7777777777777777777777777777777777777777 ]] || exit 1
    [[ "$(cat target/validation.1.log)" == "validation source: $TEST_SOURCE" ]] || exit 1
    [[ "$(cat target/validation.2.log)" == 'validation source: 6666666666666666666666666666666666666666' ]] || exit 1
    perl scripts/release/release-data.pl verify 6666666666666666666666666666666666666666 0.2.1 "$(date -u +%F)"
    cmp "$retained" target/first-failed-validation.log
    [[ "$(tail -n 1 target/release-state/0.2.0.plan)" == complete ]] || exit 1
    cmp target/older-receipt target/history/3333333333333333333333333333333333333333.files/docs/release.json
    cmp target/older-tag target/tags/v0.2.0.sha
    assert_cache_retained
}
test_local_catalog_refusal() {
    local shape="$1"
    cargo metadata --offline --locked --no-deps --format-version 1 > target/catalog-metadata.json
    if [[ "$shape" == stale ]]; then
        perl -pi -e 's/version = "0.1.0" \}/version = "0.9.0" }/' Cargo.toml
    else
        printf '\nforeign-package = { path = "foreign", version = "0.1.0" }\n' >> Cargo.toml
    fi
    cp Cargo.toml target/catalog-before.toml
    cp Cargo.lock target/catalog-before.lock
    expect_failure perl scripts/release/release-data.pl set-version 0.1.1 target/catalog-metadata.json
    cmp target/catalog-before.toml Cargo.toml
    cmp target/catalog-before.lock Cargo.lock
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
}

test_invalid_publish_selection() {
    prepare_tagged_release
    export PUBLISH_PACKAGE=foreign-package
    expect_failure "$TEST_REAL_MAKE" --no-print-directory publish
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
}

test_package_lock_selection() {
    local shape="$1"
    perl -pi -e 's/unchanged-external-checksum/"a" x 64/e' Cargo.lock
    cp Cargo.lock target/package-candidate.lock
    case "$shape" in
        valid) ;;
        checksum) perl -pi -e 's/a{64}/"b" x 64/e' target/package-candidate.lock ;;
        version) perl -0777 -pi -e 's/(name = "external"\nversion = ")0\.1\.0/${1}0.1.1/' target/package-candidate.lock ;;
        source) perl -pi -e 's!registry\+https://github.com/rust-lang/crates.io-index!git+https://example.invalid/source#abc!' target/package-candidate.lock ;;
    esac
    if [[ "$shape" == valid ]]; then
        perl scripts/release/release-data.pl package-lock-check Cargo.lock target/package-candidate.lock
    else
        expect_failure perl scripts/release/release-data.pl package-lock-check Cargo.lock target/package-candidate.lock
    fi
    [[ ! -s "$TEST_EFFECTS" ]] || exit 1
}

test_make_execution_refusal() {
    before="$(fingerprint)"
    for target in release-patch release-minor release-major release-resume fmt fmt-check; do
        for mode in -i --ignore-errors -n -t -q; do
            for selection in direct inherited; do
                status=0
                if [[ "$selection" == direct ]]; then
                    "$TEST_REAL_MAKE" "$mode" "$target" > target/rejection.log 2>&1 || status=$?
                else
                    MAKEFLAGS="$mode" _shared_make_execution_checked=yes \
                        "$TEST_REAL_MAKE" "$target" > target/rejection.log 2>&1 || status=$?
                fi
                [[ "$status" == 2 ]] || exit 1
                assert_unchanged
                [[ ! -s "$TEST_LOG" && ! -s "$TEST_EFFECTS" && ! -e target/release-state ]] || exit 1
            done
        done
    done
}

test_delivery_refusal() {
    local target="$1" selection="$2" before
    before="$(fingerprint)"
    # Exercise both command-line and inherited selection at the actual Make
    # boundary, before the shared runner or any metadata/effect adapter runs.
    if [[ "$selection" == inherited ]]; then
        expect_failure env RELEASE_DELIVERY=pr "$TEST_REAL_MAKE" --no-print-directory "$target" VERSION=0.1.1
    else
        expect_failure "$TEST_REAL_MAKE" --no-print-directory "$target" VERSION=0.1.1 "RELEASE_DELIVERY=$selection"
    fi
    assert_unchanged
    [[ ! -s "$TEST_LOG" && ! -s "$TEST_EFFECTS" && ! -e target/release-state ]] || exit 1
}

run_case make-execution-refusal test_make_execution_refusal
for target in release-patch release-minor release-major release-resume; do
    for selection in pr inherited invalid; do
        run_case "delivery-$target-$selection" test_delivery_refusal "$target" "$selection"
    done
done
run_case bootstrap-success test_dependency_bootstrap success
run_case bootstrap-snapshot test_dependency_bootstrap snapshot
run_case bootstrap-tools test_dependency_bootstrap tools
run_case bootstrap-pins test_dependency_bootstrap pins
run_case bootstrap-fetch test_dependency_bootstrap fetch
run_case bootstrap-fetch-cached test_dependency_bootstrap fetch-cached
for tool in shellcheck-missing shellcheck-broken sort-missing sort-wrong rustfmt-broken; do
    run_case "preflight-$tool" test_preflight_tools "$tool"
done
for phase in deps install-tools tools-check; do
    run_case "selected-tools-$phase" test_selected_tool_preparation "$phase"
done
for shape in selection head; do
    run_case "selected-tools-change-$shape" test_tool_preparation_source_change "$shape"
done
for shape in lf cr alias; do
    run_case "directory-$shape" test_directory_admission "$shape"
done
run_case source-diagnostics test_source_diagnostics
for shape in duplicate historical-duplicate competing; do run_case "notes-$shape" test_invalid_changelog "$shape"; done
for shape in empty missing; do run_case "notes-$shape" test_changelog_content "$shape"; done
run_case notes-saved-intent test_changelog_intent
for phase in preflight after-bump; do run_case "finalizer-$phase" test_finalizer_failure "$phase"; done
for shape in valid checksum version source; do run_case "package-lock-$shape" test_package_lock_selection "$shape"; done
run_case adapter-release test_release
for shape in stale foreign; do run_case "catalog-$shape" test_local_catalog_refusal "$shape"; done
for failure in TEST_GATE_FAIL TEST_METADATA_FAIL TEST_PREPARE_FAIL; do run_case "$failure" test_rejected_preparation "$failure"; done
run_case exact-index test_preparation
run_case receipt-identity test_receipt_identity
run_case publish test_publish normal
run_case publish-dry-run test_publish dry-run
run_case publish-contracts test_publish normal ic-blob-storage-contracts
run_case publish-contracts-dry-run test_publish dry-run ic-blob-storage-contracts
run_case publish-core test_publish normal ic-blob-storage
run_case publish-core-dry-run test_publish dry-run ic-blob-storage
run_case publish-invalid-selection test_invalid_publish_selection
run_case publish-unindexed-dry-run test_unpublished_dry_run
for shape in contracts-failure core-failure contracts-lost-reply core-lost-reply; do
    run_case "publication-retry-$shape" test_publication_retry "$shape"
done
for shape in transport http forbidden malformed checksum foreign source dirty path yanked; do
    run_case "registry-refusal-$shape" test_registry_refusal "$shape"
done
for invalid in tag receipt parent merge; do run_case "bad-$invalid" test_invalid_publish "$invalid"; done
run_case publication-lock test_publication_lock
run_case real-index test_actual_index
for shape in valid missing invalid source parent merge receipt absent tag tag-type dirty publication; do
    run_case "selected-$shape" test_selected_commit "$shape"
done
run_case recovery-integration test_recovery_integration
for operation in preflight prepare prepared-check committed-check publish plan; do
    case "$operation" in
        preflight) reads='version head verified-head prefix status' ;;
        prepare) reads='version head prefix status' ;;
        prepared-check) reads='version source head prefix status' ;;
        committed-check) reads='parent' ;;
        publish) reads='version source head parent prefix status' ;;
        plan) reads='version' ;;
    esac
    for read in $reads; do
        run_case "failed-read-$operation-$read" test_failed_guard_read "$operation" "$read"
    done
done
printf 'Release-adapter tests: PASS (isolated Git/Cargo effects; no real commits, pushes or publication).\n'
