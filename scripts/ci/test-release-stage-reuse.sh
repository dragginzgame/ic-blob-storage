#!/usr/bin/env bash
# Actual Blob native/Clippy/installed stages across disposable group releases. No network,
# live release, upload or provider effect. Run explicitly for mapping changes.
set -euo pipefail
mode=${1:-full}
case "$mode" in full|--publication-only) ;; *) exit 2 ;; esac
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
[[ $(uname -s)/$(uname -m) == Linux/x86_64 ]] || {
    echo 'Candidate reuse is not qualified on this host; ordinary stages remain required.'
    exit 0
}
stage_tmp="$root/target/tmp"
[[ ${#stage_tmp} -le 60 ]] || {
    echo 'Candidate reuse is not qualified at this long temp root; ordinary stages remain required.'
    exit 0
}
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset BLOB_RELEASE_VALIDATION_PLAN BLOB_RELEASE_SOURCE_ROOT BLOB_RELEASE_STAGE_ADMISSIONS POCKET_IC_BIN
unset RELEASE_SOURCE RELEASE_COMMIT RELEASE_VERSION RELEASE_PREVIOUS RELEASE_DATE
mkdir -p "$root/target"
fixture=$(mktemp -d "$root/target/blob-stage-reuse.XXXXXX")
echo "Blob native candidate reuse qualification retained: $fixture"
mkdir "$fixture/source" "$fixture/templates"
mkdir -p "$stage_tmp"
# Keep PocketIC Unix sockets short even in nested disposable source.
export TMPDIR="$stage_tmp"
(cd "$root"; git ls-files -z --cached --others --exclude-standard |
    perl -0ne 'print if -f $_ && !-l $_' > "$fixture/paths";
    tar --null -T "$fixture/paths" -cf -) | tar -xf - -C "$fixture/source"
cd "$fixture/source"
export BLOB_PREPARED_TOOLS_ROOT="$root"
export PATH="$root/.tools/host/bin:$root/.tools/rust/common/bin:$root/.tools/ic/bin:$PATH"
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$fixture/templates"
# This limited fixture roster qualifies the expensive cached consumer commands. It
# is never an authorized production gate or a substitute for archive acceptance.
printf '%s\n' '{"groups":["contracts","client","engine"],"schema":1,"stages":["clippy","test-native","test-pocketic"]}' > ci/release-validation.json
printf '%s\n' '{"contract":"blob-final-candidate/1","hosts":["Linux/x86_64"],"schema":1,"stages":["clippy","test-native","test-pocketic"]}' > ci/release-stage-reuse.json
groups=(contracts client engine)
if [[ "$mode" == --publication-only ]]; then
    groups=(contracts)
    # Qualify the publication wrapper against an authentic completed candidate
    # proof without executing unrelated native/installed suites. This fixture's
    # real Cargo lint is deliberately limited to the selected Contracts library.
    printf '%s\n' '{"groups":["contracts","client","engine"],"schema":1,"stages":["clippy"]}' > ci/release-validation.json
    printf '%s\n' '{"contract":"blob-final-candidate/1","hosts":["Linux/x86_64"],"schema":1,"stages":["clippy"]}' > ci/release-stage-reuse.json
    cat >> Makefile <<'MAKE'
clippy-run:
	cargo clippy --offline --locked -p ic-blob-storage-contracts --lib --all-features -- -D warnings
MAKE
fi
for group in "${groups[@]}"; do
    previous=$(RELEASE_GROUP="$group" bash scripts/release/group.sh version)
    next=$(bash scripts/ci/next-release-version.sh "$previous" patch)
    printf '# Fixture notes\n\n## [%s] - Unreleased\n\n### 🔧 Changed\n\n- Qualify independent group delivery.\n' "$next" > "releases/$group/CHANGELOG.md"
done
git init -q
git config user.name 'Blob stage qualification'
git config user.email 'stage@example.invalid'
git config core.hooksPath /dev/null
git add .
git -c commit.gpgsign=false commit -qm 'Reproducible native stage fixture'
export RELEASE_GROUP RELEASE_SOURCE RELEASE_PREVIOUS RELEASE_VERSION RELEASE_DATE=2040-01-02
for group in "${groups[@]}"; do
    RELEASE_GROUP="$group"
    RELEASE_SOURCE=$(git rev-parse HEAD)
    RELEASE_PREVIOUS=$(bash scripts/release/group.sh version)
    RELEASE_VERSION=$(bash scripts/ci/next-release-version.sh "$RELEASE_PREVIOUS" patch)
    echo "Qualifying actual group stages: $group"
    bash scripts/release/validate.sh > "$fixture/$group-gate.log" 2>&1
    record=".git/release-state/validation/$group-$RELEASE_SOURCE.json"
    cp "$record" "$fixture/$group-record.json"
    expected=reused; [[ "$group" != contracts ]] || expected=executed
    jq -e --arg expected "$expected" --arg group "$group" --arg source "$RELEASE_SOURCE" '
        .kind == "blob-release-stages-v1" and .group == $group and .source == $source and
        all(.stage_admissions[]; .admission.mode == $expected and .admission.source == $source and .admission.group == $group)
    ' "$record" > /dev/null
    if [[ "$group" == contracts && "$mode" == full ]]; then
        index=0
        for mutation in '.stage_admissions.clippy.files={}' 'del(.stage_admissions["test-native"])' '.stage_admissions.clippy.admission.mode="failed"' '.stage_admissions.clippy.admission.execution_proof_sha256=("0" * 64)' '.stage_admissions.clippy.cwd="/wrong-candidate"'; do
            index=$((index + 1))
            jq "$mutation" "$record" > "$fixture/bad-custody-$index.json"
            if perl scripts/release/validation.pl check "$fixture/bad-custody-$index.json" > "$fixture/bad-custody-$index.log" 2>&1; then
                echo 'Damaged stage custody unexpectedly admitted' >&2; exit 1
            fi
            grep -Eq 'sealed stage custody|stage admission roster|candidate stage identity differs' "$fixture/bad-custody-$index.log"
        done
    fi
    bash scripts/release/release.sh prepare > "$fixture/$group-prepare.log" 2>&1
    bash scripts/release/group.sh plan > "$fixture/$group-plan.json"
    paths=()
    while IFS= read -r path; do paths+=("$path"); done < <(jq -r '.files[]' "$fixture/$group-plan.json")
    git add -- "${paths[@]}"
    git -c commit.gpgsign=false commit -qm "Fixture release $group $RELEASE_VERSION"
    git tag -a "$group-v$RELEASE_VERSION" -m "Fixture release $group $RELEASE_VERSION"
    mkdir -p "target/fixture-publication-$group"
    primary=$(jq -er .primary "$fixture/$group-plan.json")
    perl scripts/release/stage.pl --publication-clippy "$primary" "target/fixture-publication-$group" > "$fixture/$group-publication-lint.log" 2>&1
    jq -e '.kind == "blob-publication-stage-qualification" and .qualification.admission.stage == "clippy"' "target/fixture-publication-$group/stages/clippy.json" > /dev/null
done
if [[ "$mode" == full ]]; then
jq -s '{groups:map({group,source,candidate:.candidate.identity,stages:(.stage_admissions|map_values({mode:.admission.mode,origin:.origin,seconds:.elapsed_seconds}))}),native_executions:([.[].stage_admissions["test-native"].admission|select(.mode=="executed")]|length),native_reuses:([.[].stage_admissions["test-native"].admission|select(.mode=="reused")]|length)}' \
    "$fixture/contracts-record.json" "$fixture/client-record.json" "$fixture/engine-record.json" > "$fixture/summary.json"
jq -e '.native_executions == 1 and .native_reuses == 2 and ([.groups[].candidate]|unique|length)==1' "$fixture/summary.json" > /dev/null
fi
# Exercise the standard publication wrapper, including its isolated archive
# target. Registry reads and Cargo upload are substitutes; source/tag/receipt,
# candidate inventory, sealed lint and primary closure checks remain real.
mkdir "$fixture/bin"
export TEST_REAL_CARGO TEST_PUBLISH_TARGETS="$fixture/publication-targets"
TEST_REAL_CARGO=$(command -v cargo)
cat > "$fixture/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in
    publish) printf '%s\n' "$CARGO_TARGET_DIR" >> "$TEST_PUBLISH_TARGETS" ;;
    fetch) ;; # The fixture's locked source cache was already admitted offline.
    *) exec "$TEST_REAL_CARGO" "$@" ;;
esac
CARGO
cat > "$fixture/bin/curl" <<'CURL'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == '--disable --version' ]]; then echo 'curl 8.4.0 fixture'; exit; fi
printf 404
CURL
chmod +x "$fixture/bin/cargo" "$fixture/bin/curl"
export PATH="$fixture/bin:$PATH"
bash scripts/release/release.sh publish > "$fixture/wrapper-publication.log" 2>&1
publication=$(sed -n 's/^Publication readback retained: //p' "$fixture/wrapper-publication.log")
[[ "$publication" == "$PWD"/target/publication.* && -d "$publication" ]]
jq -e '.kind == "blob-publication-stage-qualification" and .qualification.admission.stage == "clippy"' "$publication/stages/clippy.json" > /dev/null
[[ $(cat "$TEST_PUBLISH_TARGETS") == "$publication/target" ]]
if [[ "$mode" == --publication-only ]]; then
    # The exception admits only this wrapper's own run/target. A neighboring
    # run, caller-selected output, or compiler override still executes lint.
    for reason in foreign-target flags; do
        run=$(mktemp -d "$PWD/target/publication.XXXXXX")
        if [[ "$reason" == foreign-target ]]; then
            CARGO_TARGET_DIR="$PWD/target/unrelated" perl scripts/release/stage.pl --publication-clippy "$primary" "$run" > "$fixture/$reason.log" 2>&1
        else
            RUSTFLAGS='-C debuginfo=0' CARGO_TARGET_DIR="$run/target" perl scripts/release/stage.pl --publication-clippy "$primary" "$run" > "$fixture/$reason.log" 2>&1
        fi
        jq -e '.kind == "blob-direct-stage-execution" and .mode == "executed"' "$run/stages/clippy.json" > /dev/null
    done
fi
# Retain a missing-log fixture without erasing the original capture. Publication
# must run actual current-source lint rather than trust an incomplete local proof.
log=$(jq -er '.stage_admissions.clippy.files|keys[]|select(endswith("command.stdout"))' "$fixture/$group-record.json")
mv "$log" "$log.fixture-retained"
bash scripts/release/release.sh publish > "$fixture/publication-missing-log.log" 2>&1
fallback=$(sed -n 's/^Publication readback retained: //p' "$fixture/publication-missing-log.log")
jq -e '.kind == "blob-direct-stage-execution" and .mode == "executed"' "$fallback/stages/clippy.json" > /dev/null
[[ $(tail -n 1 "$TEST_PUBLISH_TARGETS") == "$fallback/target" ]]
jq -s 'map({group,seconds:.elapsed_seconds,kind})' target/fixture-publication-*/stages/clippy.json "$publication/stages/clippy.json" "$fallback/stages/clippy.json" > "$fixture/publication-summary.json"
echo 'Blob publication wrapper admits authentic candidate lint, preserves archive isolation and falls back after proof loss; no real release/upload.'
echo "Publication results: $fixture/publication-summary.json"
if [[ "$mode" == full ]]; then echo "Group results: $fixture/summary.json"; fi
