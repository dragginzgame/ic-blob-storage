#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
mkdir -p "$ROOT/target"
TEMPORARY="$(mktemp -d "$ROOT/target/release-tests.XXXXXX")"
trap 'rm -rf "$TEMPORARY"' EXIT
FIXTURE="$TEMPORARY/fixture"
mkdir -p "$FIXTURE/scripts/release" "$FIXTURE/docs" "$FIXTURE/target" "$TEMPORARY/bin"
cp "$ROOT/scripts/release/release.sh" "$ROOT/scripts/release/release-data.pl" "$FIXTURE/scripts/release/"
cd "$FIXTURE"
DATA="$FIXTURE/scripts/release/release-data.pl"
export TEST_LOG="$TEMPORARY/commands.log"
export TEST_SOURCE=1111111111111111111111111111111111111111

reset_fixture() {
    cat > Cargo.toml <<'TOML'
[workspace]
members = []
[workspace.package]
version = "0.1.0"
edition = "2024"
publish = false
TOML
    cat > Cargo.lock <<'LOCK'
version = 4

[[package]]
name = "ic-blob-storage"
version = "0.1.0"
LOCK
    cat > CHANGELOG.md <<'NOTES'
# Changelog

## [Unreleased]

- Test release notes.
NOTES
    rm -f docs/release.json target/mock-head target/mock-tag target/mock-staged target/gate-ran
    : > "$TEST_LOG"
    unset TEST_DIRTY TEST_TAG_EXISTS TEST_GATE_FAIL TEST_UPDATE_FAIL TEST_GATE_DIRTY TEST_GATE_HEAD TEST_METADATA_FAIL TEST_PUSH_FAIL
}

expect_failure() {
    if "$@" >"$TEMPORARY/failure.log" 2>&1; then
        echo "expected rejection: $*" >&2
        exit 1
    fi
}

fingerprint() {
    sha256sum Cargo.toml Cargo.lock CHANGELOG.md
}

assert_unchanged() {
    [[ "$(fingerprint)" == "$before" ]]
    [[ ! -e docs/release.json ]]
}

# These substitutes exercise local sequencing and failure handling without
# creating commits, tags, network requests, or registry publications.
cat > "$TEMPORARY/bin/git" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "git $*" >> "$TEST_LOG"
case "$*" in
    'rev-parse --verify HEAD' | 'rev-parse HEAD')
        if [[ -f target/mock-head ]]; then
            cat target/mock-head
        elif [[ -n "${TEST_GATE_HEAD:-}" && -f target/gate-ran ]]; then
            echo 2222222222222222222222222222222222222222
        else
            echo "$TEST_SOURCE"
        fi
        ;;
    'rev-parse --verify --quiet refs/tags/'*) [[ "${TEST_TAG_EXISTS:-0}" == 1 ]] ;;
    'status --porcelain --untracked-files=all')
        if [[ "${TEST_DIRTY:-0}" == 1 ]] ||
            [[ "${TEST_GATE_DIRTY:-0}" == 1 && -f target/gate-ran ]]; then
            echo ' M src/lib.rs'
        fi
        ;;
    'tag --list v*') ;;
    'merge-base --is-ancestor '*) ;;
    'diff --name-only '*) printf '%s\n' Cargo.toml Cargo.lock CHANGELOG.md ;;
    'ls-files --others --exclude-standard') echo docs/release.json ;;
    'add -- Cargo.toml Cargo.lock CHANGELOG.md docs/release.json') touch target/mock-staged ;;
    'diff --quiet') ;;
    'diff --cached --quiet') [[ ! -f target/mock-staged ]] ;;
    'commit -m Release '*)
        [[ -f target/mock-staged ]]
        echo 3333333333333333333333333333333333333333 > target/mock-head
        rm target/mock-staged
        ;;
    'tag -a v'*)
        [[ -f target/mock-head ]]
        cp target/mock-head target/mock-tag
        ;;
    'rev-parse --verify refs/tags/'*) cat target/mock-tag ;;
    'cat-file -t refs/tags/'*) echo tag ;;
    'rev-parse HEAD^') echo "$TEST_SOURCE" ;;
    'symbolic-ref --quiet --short HEAD') echo main ;;
    'remote get-url origin') echo 'https://invalid.example/no-network-fixture.git' ;;
    'push --atomic origin HEAD:refs/heads/main refs/tags/v'*)
        [[ "${TEST_PUSH_FAIL:-0}" != 1 ]]
        ;;
    *) echo "unexpected Git command: $*" >&2; exit 1 ;;
esac
MOCK
cat > "$TEMPORARY/bin/make" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "make $*" >> "$TEST_LOG"
[[ "$*" == '--no-print-directory release-verify' ]]
[[ "${TEST_GATE_FAIL:-0}" != 1 ]]
touch target/gate-ran
MOCK
cat > "$TEMPORARY/bin/cargo" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
echo "cargo $*" >> "$TEST_LOG"
case "$*" in
    'update --offline -p ic-blob-storage')
        [[ "${TEST_UPDATE_FAIL:-0}" != 1 ]]
        next="$(perl scripts/release/release-data.pl version)"
        sed -i "s/^version = \"0.1.0\"$/version = \"$next\"/" Cargo.lock
        ;;
    'metadata --offline --locked --no-deps --format-version 1')
        [[ "${TEST_METADATA_FAIL:-0}" != 1 ]]
        echo '{}'
        ;;
    clean) ;;
    *) echo "unexpected Cargo command: $*" >&2; exit 1 ;;
esac
MOCK
chmod +x "$TEMPORARY/bin/"*
export PATH="$TEMPORARY/bin:$PATH"

reset_fixture
[[ "$(perl "$DATA" next patch)" == 0.1.1 ]]
[[ "$(perl "$DATA" next minor)" == 0.2.0 ]]
[[ "$(perl "$DATA" next major)" == 1.0.0 ]]
[[ "$(perl "$DATA" next 0.1.0)" == 0.1.0 ]]
expect_failure perl "$DATA" next 0.0.9
expect_failure perl "$DATA" next 01.2.3
expect_failure perl "$DATA" next 0.1.1-extra
before="$(fingerprint)"
perl "$DATA" changelog-check 0.1.1 2026-09-25
assert_unchanged
echo "PASS version selection and non-mutating changelog preflight"

reset_fixture
cat > CHANGELOG.md <<'NOTES'
# Changelog

## [Unreleased]

## [0.2.0]

- Named release.
NOTES
expect_failure perl "$DATA" changelog-check 0.1.1 2026-09-25
perl "$DATA" finalize 0.2.0 2026-09-25
rg -q '^## \[0.2.0\] - 2026-09-25$' CHANGELOG.md
expect_failure perl "$DATA" finalize 0.2.0 2026-09-25
reset_fixture
printf '\n## [Unreleased]\n' >> CHANGELOG.md
expect_failure perl "$DATA" changelog-check 0.1.1 2026-09-25
reset_fixture
printf '# Changelog\n\n## [Unreleased]\n' > CHANGELOG.md
expect_failure perl "$DATA" changelog-check 0.1.1 2026-09-25
echo "PASS named drafts, empty notes, duplicate headings and repeat finalization"

for failure in TEST_DIRTY TEST_TAG_EXISTS TEST_GATE_FAIL TEST_UPDATE_FAIL TEST_GATE_DIRTY TEST_GATE_HEAD TEST_METADATA_FAIL; do
    reset_fixture
    rm -f target/gate-ran
    before="$(fingerprint)"
    export "$failure=1"
    expect_failure bash scripts/release/release.sh bump patch
    assert_unchanged
done
echo "PASS rejected source, failed validation, concurrent changes and mutation rollback"

reset_fixture
bash scripts/release/release.sh bump patch
[[ "$(perl "$DATA" version)" == 0.1.1 ]]
rg -q '^version = "0.1.1"$' Cargo.lock
perl "$DATA" verify
[[ "$(perl "$DATA" source)" == "$TEST_SOURCE" ]]
expect_failure bash scripts/release/release.sh bump 0.1.1
bash scripts/release/release.sh stage
rg -q '^git add -- Cargo.toml Cargo.lock CHANGELOG.md docs/release.json$' "$TEST_LOG"
printf '\n- Unvalidated change.\n' >> CHANGELOG.md
expect_failure perl "$DATA" verify
echo "PASS prepared release, exact staging, repeat rejection and tamper detection"

reset_fixture
bash scripts/release/release.sh bump 0.1.0
[[ "$(perl "$DATA" version)" == 0.1.0 ]]
perl "$DATA" verify
echo "PASS first release at the scaffold version"

# No command under test may accidentally reach the publication boundary.
if rg -q '^git (commit|tag -a|push)|^cargo publish' "$TEST_LOG"; then
    echo "unexpected publication command in preparation tests" >&2
    exit 1
fi

reset_fixture
bash scripts/release/release.sh release minor
[[ "$(perl "$DATA" version)" == 0.2.0 ]]
perl -e '
    local $/; my $log = <>;
    die "incorrect one-shot order\n" unless
        $log =~ /make --no-print-directory release-verify.*git add --.*git commit -m Release 0\.2\.0.*git tag -a v0\.2\.0.*git push --atomic origin HEAD:refs\/heads\/main refs\/tags\/v0\.2\.0.*cargo clean/s;
' "$TEST_LOG"
echo "PASS one-shot ordering, exact atomic push and post-release cleanup (substitutes)"

reset_fixture
export TEST_PUSH_FAIL=1
expect_failure bash scripts/release/release.sh release patch
[[ -f target/mock-head && -f target/mock-tag ]]
perl "$DATA" verify
if rg -q '^cargo clean$' "$TEST_LOG"; then
    echo "cleanup ran after failed push" >&2
    exit 1
fi
unset TEST_PUSH_FAIL
bash scripts/release/release.sh push
echo "PASS failed-push recovery retains the prepared release (substitutes)"
echo "Release helper tests passed (substituted Git/Cargo/validation commands)."
