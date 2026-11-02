#!/usr/bin/env bash
set -Eeuo pipefail
mode="${1:-full}"
case "$mode" in full|--metadata-only) ;; *) exit 2 ;; esac
groups=(contracts client engine)
if [[ "$mode" == --metadata-only ]]; then
    case "${2:-engine}" in
        contracts|client|engine) groups=("${2:-engine}") ;;
        all) ;; # Adoption qualification only; never a real all-group delivery.
        *) exit 2 ;;
    esac
fi
# Qualify Blob's actual group policy/receipt/publication adapters. Shared owns
# generic runner recovery, version arithmetic and metadata editor regressions.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset RELEASE_COMMIT RELEASE_SOURCE RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
export PATH="$ROOT/.tools/host/bin:$ROOT/.tools/rust/common/bin:$PATH"
TEMPORARY="$(mktemp -d "${TMPDIR:-/tmp}/blob-release-tests.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$fixture_complete" == true && "$status" == 0 ]]; then rm -rf "$TEMPORARY"
    else printf 'Full logs and isolated fixture retained: %s\n' "$TEMPORARY" >&2; fi
    exit "$status"
}
trap finish EXIT
[[ -d "$TEMPORARY" ]] || exit 1
export TEST_RELEASE_EDITOR
TEST_RELEASE_EDITOR="$(make --no-print-directory --silent -C "$ROOT" release-editor-check)"
# Export only visible regular source, preserving pending work without touching
# its index. No tool stores, credentials, ignored artifacts or sibling sources.
mkdir "$TEMPORARY/source" "$TEMPORARY/bin" "$TEMPORARY/templates"
(cd "$ROOT"; git ls-files -z --cached --others --exclude-standard |
    perl -0ne 'print if -f $_ && !-l $_' > "$TEMPORARY/paths"
    tar --null -T "$TEMPORARY/paths" -cf -) | tar -xf - -C "$TEMPORARY/source"
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$TEMPORARY/templates"
cd "$TEMPORARY/source"
git init -q
git config user.name 'Blob release fixture'
git config user.email 'fixture@example.invalid'
git config core.hooksPath /dev/null
# The renderer/editor/graph/tag/receipt are real; expensive compilation, tool
# setup, network and publication are substitutes recorded outside this checkout.
cat >> Makefile <<'MAKE'
.PHONY: fixture-validation
fixture-validation:
	@echo validation >> "$$TEST_EVENTS"
release-editor-check:
	@printf '%s\n' "$$TEST_RELEASE_EDITOR"
fmt-check:
	@echo formatter >> "$$TEST_EVENTS"
	@test "$${TEST_FORMAT_FAIL:-0}" != 1
deps tools-check install-tools release-tools-check compiler-check clippy:
	@echo "$@" >> "$$TEST_EVENTS"
MAKE
# Keep the fixture's admitted full roster explicit, rather than bypassing the
# actual plan/receipt path or weakening source admission with an environment flag.
perl -0pi -e 's/^CI_TARGETS := .*$/CI_TARGETS := fixture-validation/m' Makefile
printf '%s\n' '{"groups":["contracts","client","engine"],"schema":1,"stages":["fixture-validation"]}' > ci/release-validation.json
export TEST_EVENTS="$TEMPORARY/events"
: > "$TEST_EVENTS"
# Fixture-owned candidates follow actual member versions, independently of
# pending product notes. This must also work after groups have diverged.
cargo metadata --offline --locked --format-version 1 > "$TEMPORARY/initial-metadata.json"
jq '[.packages[] | select(.source == null)] | map({key:.name,value:.version}) | from_entries' \
    "$TEMPORARY/initial-metadata.json" > "$TEMPORARY/initial-versions.json"
for group in contracts client engine; do
    primary="$(jq -er --arg group "$group" '.groups[] | select(.name == $group) | .primary' ci/cargo-release-groups.json)"
    previous="$(jq -er --arg name "$primary" '.[$name]' "$TEMPORARY/initial-versions.json")"
    candidate="$(bash scripts/ci/next-release-version.sh "$previous" patch)"
    printf '%s\n' "$candidate" > "$TEMPORARY/$group.version"
    printf '# Fixture notes\n\n## [%s] - Unreleased\n\n### 🔧 Changed\n\n- Exercise group delivery.\n' "$candidate" > "releases/$group/CHANGELOG.md"
done
git add .
git commit -qm 'Group fixture source'
base="$(git rev-parse HEAD)"
export RELEASE_DELIVERY=direct RELEASE_DATE=2040-01-02
export RELEASE_SOURCE="$base"
export BLOB_RELEASE_PREPARE=0
fingerprint() {
    # Every manifest, ledger, receipt and lock is significant, including groups
    # outside the selected roster. Logs and artifacts are not release metadata.
    git ls-files -z -- '*Cargo.toml' Cargo.lock 'releases/*/CHANGELOG.md' 'releases/*/release.json' |
        xargs -0 shasum -a 256
}
refuse() { if "$@" > "$TEMPORARY/refusal.log" 2>&1; then echo "Unexpected success: $*" >&2; exit 1; fi; }
reset_source() {
    git reset --hard -q "$base"
    export RELEASE_SOURCE="$base"
    unset RELEASE_COMMIT RELEASE_PREVIOUS RELEASE_VERSION TEST_FORMAT_FAIL PUBLISH_PACKAGE
    : > "$TEST_EVENTS"
}
# Group selection must fail before the runner, metadata admission or setup.
for selection in '' unknown '%' 'client engine'; do
    for target in release-plan release-patch release-resume publish version; do
        refuse make --no-print-directory "$target" "RELEASE_GROUP=$selection"
    done
    refuse env RELEASE_GROUP="$selection" bash scripts/release/release.sh prepare
    [[ ! -s "$TEST_EVENTS" ]] || exit 1
done
for group in "${groups[@]}"; do
    reset_source
    export RELEASE_GROUP="$group"
    echo "Group adapter: $group"
    bash scripts/release/group.sh plan > "$TEMPORARY/$group-plan.json"
    jq -e --arg group "$group" '.group == $group' "$TEMPORARY/$group-plan.json" > /dev/null
    RELEASE_PREVIOUS="$(jq -er .previous "$TEMPORARY/$group-plan.json")"
    RELEASE_VERSION="$(cat "$TEMPORARY/$group.version")"
    export RELEASE_PREVIOUS RELEASE_VERSION
    refuse bash scripts/release/release.sh publish
    [[ ! -s "$TEST_EVENTS" ]] || exit 1
    make --no-print-directory release-preflight > "$TEMPORARY/$group-preflight.log" 2>&1
    [[ "$(cat "$TEST_EVENTS")" == $'tools-check\nrelease-tools-check' ]] || exit 1
    : > "$TEST_EVENTS"
    fingerprint > "$TEMPORARY/before"
    refuse bash scripts/release/release.sh prepare
    make --no-print-directory release-verify > "$TEMPORARY/$group-validation.log" 2>&1
    TEST_FORMAT_FAIL=1 refuse bash scripts/release/release.sh prepare
    fingerprint > "$TEMPORARY/after"
    cmp "$TEMPORARY/before" "$TEMPORARY/after"
    bash scripts/release/release.sh prepare > "$TEMPORARY/$group-prepare.log" 2>&1
    # Reconciliation of prepared metadata must neither render nor bump twice.
    if [[ "$mode" == full ]]; then
        bash scripts/release/release.sh prepare > "$TEMPORARY/$group-reconcile.log" 2>&1
    fi
    perl scripts/release/release-data.pl verify "$base" "$RELEASE_VERSION" 2040-01-02
    cargo metadata --offline --locked --format-version 1 > "$TEMPORARY/$group-metadata.json"
    jq -e --arg version "$RELEASE_VERSION" --slurpfile before "$TEMPORARY/initial-versions.json" --slurpfile plan "$TEMPORARY/$group-plan.json" \
        '. as $m | all(.packages[] | select(.id as $id | $m.workspace_members | index($id)); .version == (if .name as $name | $plan[0].members | index($name) then $version else $before[0][.name] end))' \
        "$TEMPORARY/$group-metadata.json" > /dev/null
    # Numeric member changes must not re-resolve or rewrite any supplier entry.
    yq -p toml -o json '.' Cargo.lock | jq -S '[.package[] | select(.source != null)]' > "$TEMPORARY/external-after"
    git show "$base:Cargo.lock" | yq -p toml -o json '.' | jq -S '[.package[] | select(.source != null)]' > "$TEMPORARY/external-before"
    cmp "$TEMPORARY/external-before" "$TEMPORARY/external-after"
    for other in contracts client engine; do
        [[ "$other" != "$group" ]] || continue
        git diff --quiet "$base" -- "releases/$other" || exit 1
    done
    if [[ "$mode" == --metadata-only ]]; then continue; fi
    # Exact real index roster, including every literal member manifest.
    paths=()
    while IFS= read -r path; do paths+=("$path"); done < <(jq -r '.files[]' "$TEMPORARY/$group-plan.json")
    git add -- "${paths[@]}"
    bash scripts/release/release.sh commit-check > "$TEMPORARY/$group-index.log" 2>&1
    printf '\n' >> README.md
    git add README.md
    refuse bash scripts/release/release.sh commit-check
    git restore --source="$base" --staged --worktree README.md
    receipt="releases/$group/release.json"
    cp "$receipt" "$TEMPORARY/receipt"
    jq '.date = "2040-01-03"' "$receipt" > "$TEMPORARY/changed-receipt"
    cp "$TEMPORARY/changed-receipt" "$receipt"
    refuse perl scripts/release/release-data.pl verify "$base" "$RELEASE_VERSION" 2040-01-02
    cp "$TEMPORARY/receipt" "$receipt"
    git commit -qm "Release $group $RELEASE_VERSION"
    release_commit="$(git rev-parse HEAD)"
    printf '%s\n' "$release_commit" > "$TEMPORARY/$group.commit"
    git tag -a "$group-v$RELEASE_VERSION" -m "Release $group $RELEASE_VERSION"
    RELEASE_COMMIT="$release_commit" bash scripts/release/release.sh tagged-check > "$TEMPORARY/$group-tag.log" 2>&1
    # A later source/other-group edit cannot stand in for the selected receipt.
    printf '\nNext source\n' >> CHANGELOG.md
    git add CHANGELOG.md
    git commit -qm 'Later implementation source'
    RELEASE_COMMIT="$release_commit" bash scripts/release/release.sh tagged-check > "$TEMPORARY/$group-descendant.log" 2>&1
    refuse bash scripts/release/release.sh tag-check
    git reset --hard -q "$release_commit"
    refuse env PUBLISH_PACKAGE=outside bash scripts/release/release.sh publish
    [[ ! -d .git/release-state/lock ]] || exit 1
    # Preserve a real tagged Contracts release for the registry boundary cases.
    if [[ "$group" == contracts ]]; then contracts_commit="$release_commit"; fi
done
if [[ "$mode" == --metadata-only ]]; then
    fixture_complete=true
    echo 'Group metadata/rollback checks passed (real editor, complete member graph, unchanged suppliers and other groups).'
    exit 0
fi
# Publication has one selected public package. The substitute registry retains
# exact checksum/source readback and exercises no external upload or network.
git reset --hard -q "$contracts_commit"
export RELEASE_GROUP=contracts
cat > "$TEMPORARY/bin/curl" <<'CURL'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == '--disable --version' ]]; then echo 'curl 8.4.0 fixture'; exit; fi
output='' url=''
while [[ $# -gt 0 ]]; do
    case "$1" in --output) output="$2"; shift ;; https://*) url="$1" ;; esac
    shift
done
case "$url" in
    https://crates.io/*)
        version="${url##*/}"; package="${url%/*}"; package="${package##*/}"
        case "$TEST_REGISTRY" in
            missing) printf 404; exit ;;
            forbidden) printf 403; exit ;;
            dependency)
                if [[ "$package" != ic-blob-storage-contracts || "$version" != "$TEST_CONTRACTS_VERSION" ]]; then printf 404; exit; fi ;;
        esac
        checksum="$(shasum -a 256 "$TEST_ARCHIVE")"; checksum="${checksum%% *}"
        [[ "$TEST_REGISTRY" != checksum ]] || checksum="$(printf '%064d' 0)"
        yanked=false; [[ "$TEST_REGISTRY" != yanked ]] || yanked=true
        printf '{"version":{"crate":"%s","num":"%s","checksum":"%s","yanked":%s}}' "$package" "$version" "$checksum" "$yanked" > "$output"
        printf 200 ;;
    https://static.crates.io/*) cp "$TEST_ARCHIVE" "$output" ;;
    *) exit 1 ;;
esac
CURL
cat > "$TEMPORARY/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == publish ]]; then
    printf '%s\n' "$*" >> "$TEST_EVENTS"
elif [[ "$1" == fetch ]]; then
    printf 'fetch\n' >> "$TEST_EVENTS"
else exec "$TEST_REAL_CARGO" "$@"; fi
CARGO
chmod +x "$TEMPORARY/bin/curl" "$TEMPORARY/bin/cargo"
TEST_REAL_CARGO="$(command -v cargo)"
export TEST_REAL_CARGO TEST_ARCHIVE="$TEMPORARY/published.crate"
export PATH="$TEMPORARY/bin:$PATH"
archive_package="ic-blob-storage-contracts-$(cat "$TEMPORARY/contracts.version")"
mkdir -p "$TEMPORARY/archive/$archive_package"
printf '{"git":{"sha1":"%s"},"path_in_vcs":"crates/ic-blob-storage-contracts"}' "$contracts_commit" > "$TEMPORARY/archive/$archive_package/.cargo_vcs_info.json"
tar -czf "$TEST_ARCHIVE" -C "$TEMPORARY/archive" "$archive_package"
for shape in forbidden checksum yanked; do
    export TEST_REGISTRY="$shape"
    : > "$TEST_EVENTS"
    refuse bash scripts/release/release.sh publish
    if grep -q '^publish ' "$TEST_EVENTS"; then exit 1; fi
done
# A correct checksum for another source still cannot authorize skipping upload.
printf '{"git":{"sha1":"%040d"},"path_in_vcs":"crates/ic-blob-storage-contracts"}' 0 > "$TEMPORARY/archive/$archive_package/.cargo_vcs_info.json"
tar -czf "$TEST_ARCHIVE" -C "$TEMPORARY/archive" "$archive_package"
export TEST_REGISTRY=present
refuse bash scripts/release/release.sh publish
printf '{"git":{"sha1":"%s"},"path_in_vcs":"crates/ic-blob-storage-contracts"}' "$contracts_commit" > "$TEMPORARY/archive/$archive_package/.cargo_vcs_info.json"
tar -czf "$TEST_ARCHIVE" -C "$TEMPORARY/archive" "$archive_package"
: > "$TEST_EVENTS"
bash scripts/release/release.sh publish > "$TEMPORARY/already-published.log" 2>&1
if grep -q '^publish ' "$TEST_EVENTS"; then exit 1; fi
export TEST_REGISTRY=missing
bash scripts/release/release.sh publish > "$TEMPORARY/new-publication.log" 2>&1
grep -Fx 'publish --locked --registry crates-io -p ic-blob-storage-contracts' "$TEST_EVENTS" > /dev/null
mkdir -p .git/release-state/lock
printf 'another owner\n' > .git/release-state/lock/owner
refuse bash scripts/release/release.sh publish
[[ "$(cat .git/release-state/lock/owner)" == 'another owner' ]] || exit 1
rm .git/release-state/lock/owner
rmdir .git/release-state/lock
for group in client engine; do
    git reset --hard -q "$(cat "$TEMPORARY/$group.commit")"
    export RELEASE_GROUP="$group" TEST_REGISTRY=dependency
    TEST_CONTRACTS_VERSION="$(perl scripts/release/release-data.pl package-version ic-blob-storage-contracts)"
    export TEST_CONTRACTS_VERSION
    : > "$TEST_EVENTS"
    bash scripts/release/release.sh publish --dry-run > "$TEMPORARY/$group-dry-run.log" 2>&1
    primary="$(jq -r .primary "$TEMPORARY/$group-plan.json")"
    grep -Fx "publish --locked --registry crates-io -p $primary --dry-run" "$TEST_EVENTS" > /dev/null
done
# One actual runner-to-Blob smoke, with a private local bare remote. No real
# branch, tag, remote or publication is touched. Generic interruption matrices
# belong to Shared; this checks Blob's group hook/namespace integration.
reset_source
export RELEASE_GROUP=engine
git symbolic-ref HEAD refs/heads/main
git reset --hard -q "$base"
engine_version="$(cat "$TEMPORARY/engine.version")"
git tag -d "engine-v$engine_version" > /dev/null
git init --bare -q "$TEMPORARY/remote.git"
git remote add origin "$TEMPORARY/remote.git"
git push -q -u origin main
make --no-print-directory release-patch RELEASE_GROUP=engine > "$TEMPORARY/runner.log" 2>&1
[[ "$(grep -c '^install-tools$' "$TEST_EVENTS")" == 1 ]] || exit 1
delivered="$(git rev-parse HEAD)"
[[ -f ".git/release-state/engine-$engine_version.plan" ]] || exit 1
[[ "$(git rev-parse "engine-v$engine_version^{commit}")" == "$delivered" ]] || exit 1
make --no-print-directory release-resume RELEASE_GROUP=engine VERSION="$engine_version" > "$TEMPORARY/resume.log" 2>&1
[[ "$(git rev-parse HEAD)" == "$delivered" ]] || exit 1
refuse make --no-print-directory release-resume RELEASE_GROUP=client VERSION="$engine_version"
printf 'Release-adapter tests: PASS (three selective groups, rollback, exact index/receipts/tags, descendant recovery and registry admission; disposable Git effects only).\n'
fixture_complete=true
