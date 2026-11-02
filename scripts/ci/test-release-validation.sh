#!/usr/bin/env bash
# Consumer selection and receipt integration; Shared owns generic runner cases.
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset VALIDATION_INPUT_ROOT VALIDATION_INPUT_BASELINE BLOB_RELEASE_SOURCE_ROOT
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
TEMPORARY="$(mktemp -d "${TMPDIR:-/tmp}/blob-release-selection.XXXXXX")"
complete=false
finish() {
    local status=$?
    [[ "$complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf "$TEMPORARY"
    else printf 'Release selection fixture retained: %s\n' "$TEMPORARY" >&2; fi
    exit "$status"
}
trap finish EXIT
[[ -d "$TEMPORARY" ]] || exit 1
cd "$ROOT"
# Assert the actual full-gate/group/test rosters before using substituted stages.
perl -MJSON::PP -e '
    local $/; open my $m,"<","Makefile" or die $!; my @r=<$m> =~ /^CI_TARGETS := (.*)$/mg;
    open my $p,"<","ci/release-validation.json" or die $!; my $p=decode_json(<$p>);
    @r==1 && join(" ",@{$p->{stages}}) eq $r[0] or die "full-gate roster drift\n";
    open my $g,"<","ci/cargo-release-groups.json" or die $!; my $g=decode_json(<$g>);
    join(",",sort @{$p->{groups}}) eq join(",",sort map {$_->{name}} @{$g->{groups}}) or die "group roster drift\n";
'
cargo metadata --offline --locked --no-deps --format-version 1 > "$TEMPORARY/metadata.json"
jq -r '.packages[] | select(.name == "ic-blob-storage-pocketic-tests") | .targets[] | select(.kind == ["test"]) | .name' \
    "$TEMPORARY/metadata.json" | LC_ALL=C sort > "$TEMPORARY/actual-tests"
perl -e '
    local $/; open my $f,"<","Makefile" or die $!; my $m=<$f>;
    $m =~ /^test-pocketic-client:.*?\n(.*?)(?=^\S|\z)/ms or die "missing Client partition\n"; my $c=$1;
    $m =~ /^test-pocketic-engine:.*?\n(.*?)(?=^\S|\z)/ms or die "missing Engine partition\n"; my $e=$1;
    $e =~ /--lib --bins/ && $e =~ /--doc/ && $e =~ /--skip storage_tenants::client::/ or die "incomplete Engine partition\n";
    $c =~ /--test standalone/ && $c =~ /--test storage storage_tenants::client::/ or die "incomplete Client boundary\n";
    my %seen; my @all=grep {!$seen{$_}++} ($c =~ /--test ([a-z_]+)/g, $e =~ /--test ([a-z_]+)/g);
    print "$_\n" for sort @all;
' > "$TEMPORARY/partition-tests"
cmp "$TEMPORARY/actual-tests" "$TEMPORARY/partition-tests"
# Exercise the reviewed consumer selector and canonical runner with real Git
# history, exact plans/checkpoints/results, and no Cargo builds or real Git effects.
mkdir "$TEMPORARY/source"
bash scripts/ci/prepare-script-fixture.sh "$ROOT" "$TEMPORARY/source" \
    scripts/ci/run-validation-targets.sh
mkdir -p "$TEMPORARY/source/scripts/release" "$TEMPORARY/source/ci" "$TEMPORARY/templates"
cp scripts/release/validation.pl scripts/release/validate.sh "$TEMPORARY/source/scripts/release/"
cp scripts/ci/check-release-source.sh "$TEMPORARY/source/scripts/ci/"
cp ci/cargo-release-groups.json "$TEMPORARY/source/ci/"
cd "$TEMPORARY/source"
export CARGO_HOME="$TEMPORARY/cargo-home"
mkdir "$CARGO_HOME"
# Isolate the synthetic default-build case from the caller's project overrides.
# The actual roster admission above used the caller's real locked Cargo graph.
while IFS= read -r variable; do unset "$variable"; done < <(
    perl -e 'require "./scripts/release/validation.pl"; print "$_\n" for keys %{BlobValidation::configuration()->{environment}};'
)
printf '.tmp/\ntarget/\n' > .gitignore
printf '{}\n' > ci/config.json
for package in ic-blob-storage ic-blob-storage-client ic-blob-storage-contracts; do
    mkdir -p "crates/$package/src"
    printf '[package]\nversion = "0.68.0"\n' > "crates/$package/Cargo.toml"
    printf '// initial fixture\n' > "crates/$package/src/mod.rs"
done
mkdir -p crates/ic-blob-storage-client/src/tenants
printf '// initial tenant client\n' > crates/ic-blob-storage-client/src/tenants/mod.rs
cat > Makefile <<'MAKE'
SHELL := /bin/bash
CI_TARGETS := lint client-boundary test-pocketic-engine release-metadata-check release-check
.PHONY: lint client-boundary test-pocketic-engine release-metadata-check release-check
lint client-boundary test-pocketic-engine release-metadata-check release-check:
	@echo "$@" >> "$$TEST_EVENTS"
	@test "$${TEST_FAIL:-}" != "$@"
	@if test "$@" = client-boundary && test "$${TEST_DRIFT:-0}" = 1; then echo drift >> ci/config.json; fi
MAKE
printf '%s\n' '{"groups":["contracts","client","engine"],"schema":1,"stages":["lint","client-boundary","test-pocketic-engine","release-metadata-check","release-check"]}' > ci/release-validation.json
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$TEMPORARY/templates"
git init -q
git config user.name 'Blob validation fixture'
git config user.email 'fixture@example.invalid'
git config core.hooksPath /dev/null
git add .
git commit -qm 'Admitted fixture baseline'
# The first delivery executes the entire roster; legacy receipts cannot enable
# omission. Seed tagged baselines only from actual full substituted executions.
export RELEASE_SOURCE RELEASE_GROUP TEST_EVENTS="$TEMPORARY/events"
RELEASE_SOURCE="$(git rev-parse HEAD)"
mkdir -p releases/{contracts,client,engine}
for group in contracts client engine; do
    RELEASE_GROUP="$group" bash scripts/release/validate.sh > "$TEMPORARY/$group-baseline.log" 2>&1
    jq '{gate:"release-verify-plan-v1",validation:.}' ".git/release-state/validation/$group-$RELEASE_SOURCE.json" > "$TEMPORARY/$group-receipt.json"
done
for group in contracts client engine; do cp "$TEMPORARY/$group-receipt.json" "releases/$group/release.json"; done
git add releases
git commit -qm 'Qualified fixture release receipts'
for group in contracts client engine; do git tag -a "$group-v0.68.0" -m 'Fixture baseline'; done
printf '// tenant-client-only change\n' >> crates/ic-blob-storage-client/src/tenants/mod.rs
git add .
git commit -qm 'Client implementation'
export RELEASE_SOURCE RELEASE_GROUP=client TEST_EVENTS="$TEMPORARY/events"
RELEASE_SOURCE="$(git rev-parse HEAD)"
client_source="$RELEASE_SOURCE"
: > "$TEST_EVENTS"
mkdir -p .tmp/full .tmp/invalid .tmp/contracts .tmp/client-shared .git/release-state/validation
refuse() { if "$@" > "$TEMPORARY/refusal.log" 2>&1; then echo "Unexpected success: $*" >&2; exit 1; fi; }
verify_record() {
    perl -MJSON::PP -e 'require "./scripts/release/validation.pl"; local $/; open my $f,"<",$ARGV[0] or die $!; BlobValidation::verify(decode_json(<$f>),$ENV{RELEASE_SOURCE},$ENV{RELEASE_GROUP});' "$1"
}
record=".git/release-state/validation/client-$RELEASE_SOURCE.json"
TEST_FAIL=test-pocketic-engine bash scripts/release/validate.sh > "$TEMPORARY/narrow.log" 2>&1
verify_record "$record"
jq -e '.plan.scope == "group" and (.stages | map(select(.state == "not-applicable") | .target) == ["test-pocketic-engine","release-check"])' "$record" > /dev/null
if grep -Eq '^(test-pocketic-engine|release-check)$' "$TEST_EVENTS"; then exit 1; fi
cp "$record" "$TEMPORARY/good-record.json"
for mutation in '.stages[0].state="FAIL"' '.stages[0].state="blocked"' '.stages=[]' '.checkpoint={}' '.configuration=null' '.group="engine"' '.plan.stages[2].required=true'; do
    jq "$mutation" "$TEMPORARY/good-record.json" > "$TEMPORARY/invalid.json"
    refuse verify_record "$TEMPORARY/invalid.json"
done
RUSTFLAGS='-C opt-level=2' perl scripts/release/validation.pl plan .tmp/full/plan.json
jq -e '.scope == "workspace" and all(.stages[]; .required)' .tmp/full/plan.json > /dev/null
RUSTFLAGS='-C opt-level=2' refuse perl scripts/release/validation.pl check "$record"
printf '[build]\njobs = 2\n' > "$CARGO_HOME/config.toml"
perl scripts/release/validation.pl plan .tmp/full/plan.json
jq -e '.scope == "workspace" and all(.stages[]; .required)' .tmp/full/plan.json > /dev/null
refuse perl scripts/release/validation.pl check "$record"
rm "$CARGO_HOME/config.toml"
verify_record "$record"
# Failed reruns invalidate the selectable proof without overwriting old logs.
TEST_FAIL=client-boundary refuse bash scripts/release/validate.sh
[[ ! -e "$record" ]]
TEST_DRIFT=1 refuse bash scripts/release/validate.sh
[[ ! -e "$record" ]]
git restore ci/config.json
TEST_FAIL=release-metadata-check refuse bash scripts/release/validate.sh
[[ ! -e "$record" ]]
# Missing history, shared changes and future stage-roster drift broaden/refuse.
printf 'unknown input\n' > unknown.txt
git add .
git commit -qm 'Unknown input'
RELEASE_SOURCE="$(git rev-parse HEAD)"
perl scripts/release/validation.pl plan .tmp/full/plan.json
jq -e '.scope == "workspace" and all(.stages[]; .required)' .tmp/full/plan.json > /dev/null
refuse verify_record "$TEMPORARY/good-record.json"
TEST_FAIL=test-pocketic-engine refuse bash scripts/release/validate.sh
git reset --hard -q "$client_source"
RELEASE_SOURCE="$client_source"
RELEASE_GROUP=unknown refuse perl scripts/release/validation.pl plan .tmp/invalid/plan.json
printf '\nCI_TARGETS := unreviewed-stage\n' >> Makefile
git add Makefile
git commit -qm 'Unreviewed roster'
RELEASE_SOURCE="$(git rev-parse HEAD)"
refuse perl scripts/release/validation.pl plan .tmp/invalid/plan.json
git reset --hard -q "$client_source"
RELEASE_SOURCE="$client_source"
# Other Client APIs and the CLI have wider actual consumers, so keep Engine IC
# tests even with unchanged release machinery. Do not classify the crate wholesale.
printf '// shared client transport\n' >> crates/ic-blob-storage-client/src/mod.rs
git add .
git commit -qm 'Client shared transport'
RELEASE_SOURCE="$(git rev-parse HEAD)"
perl scripts/release/validation.pl plan .tmp/client-shared/plan.json
jq -e 'any(.stages[]; .target == "test-pocketic-engine" and .required)' .tmp/client-shared/plan.json > /dev/null
git reset --hard -q "$client_source"
RELEASE_SOURCE="$client_source"
# Contracts changes retain installed reverse consumers, not merely DTO unit tests.
printf '// public DTO change\n' >> crates/ic-blob-storage-contracts/src/mod.rs
git add .
git commit -qm 'Contracts boundary'
RELEASE_SOURCE="$(git rev-parse HEAD)"
RELEASE_GROUP=contracts perl scripts/release/validation.pl plan .tmp/contracts/plan.json
jq -e '.scope == "group" and any(.stages[]; .target == "test-pocketic-engine" and .required)' .tmp/contracts/plan.json > /dev/null
complete=true
echo 'Blob release selection, fail-closed execution, source-bound receipts and test partitions passed.'
