#!/usr/bin/env bash
# Consumer selection and receipt integration; Shared owns generic runner cases.
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset VALIDATION_INPUT_ROOT VALIDATION_INPUT_BASELINE BLOB_RELEASE_SOURCE_ROOT BLOB_RELEASE_VALIDATION_PLAN
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
mkdir -p "$ROOT/target"
TEMPORARY="$(mktemp -d "$ROOT/target/blob-release-selection.XXXXXX")"
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
printf '%s\n' '__blob_ci_roster:' $'\t@echo $(CI_TARGETS)' > "$TEMPORARY/inspect.mk"
make --no-print-directory -f Makefile -f "$TEMPORARY/inspect.mk" __blob_ci_roster > "$TEMPORARY/full-roster"
perl -MJSON::PP -e '
    local $/; open my $m,"<",$ARGV[0] or die $!; my $r=<$m>;
    open my $p,"<","ci/release-validation.json" or die $!; my $p=decode_json(<$p>);
    join(" ",@{$p->{stages}})."\n" eq $r or die "full-gate roster drift\n";
    open my $g,"<","ci/cargo-release-groups.json" or die $!; my $g=decode_json(<$g>);
    join(",",sort @{$p->{groups}}) eq join(",",sort map {$_->{name}} @{$g->{groups}}) or die "group roster drift\n";
' "$TEMPORARY/full-roster"
cargo metadata --offline --locked --no-deps --format-version 1 > "$TEMPORARY/metadata.json"
# Every runtime/build/dev/optional/target-specific local edge must be covered by
# the reviewed reverse-consumer closure. Extra integration consumers are allowed.
perl -MJSON::PP -e '
    sub load { local $/; open my $f,"<",$_[0] or die $!; decode_json(<$f>) }
    my $meta=load($ARGV[0]); my $groups=load("ci/cargo-release-groups.json");
    my $scope=load("ci/release-package-scopes.json"); my %owner;
    for my $g (@{$groups->{groups}}) { $owner{$_}=$g->{name} for @{$g->{members}} }
    join(",",sort keys %owner) eq join(",",sort map {$_->{name}} @{$meta->{packages}}) or die "unowned workspace member\n";
    my %reverse;
    for my $p (@{$meta->{packages}}) {
        for my $d (@{$p->{dependencies}}) {
            next unless exists $owner{$d->{name}};
            $reverse{$owner{$d->{name}}}{$owner{$p->{name}}}=1;
        }
    }
    for my $g (keys %{$scope->{consumers}}) {
        my %required; my @queue=($g);
        while (@queue) { my $next=shift @queue; next if $required{$next}++; push @queue,keys %{$reverse{$next}//{}} }
        my %declared=map {$_=>1} @{$scope->{consumers}{$g}};
        !grep {!$declared{$_}} keys %required or die "missing reverse consumer for $g\n";
    }
' "$TEMPORARY/metadata.json"
yq -o=json '.' .github/workflows/tooling.yml > "$TEMPORARY/workflow.json"
jq -e '
    .on.workflow_dispatch.inputs.release_group.default == "workspace" and
    .on.workflow_dispatch.inputs.release_group.options == ["workspace","contracts","client","engine"] and
    .jobs.tooling.strategy.matrix.os == ["ubuntu-24.04","macos-15","macos-15-intel"] and
    (.jobs.tooling.env.BLOB_GROUP_CI | contains("workflow_dispatch")) and
    (.jobs.tooling.steps | any(.name == "Run the selected source-bound group plan on this native host" and .env.RELEASE_GROUP == "${{ inputs.release_group }}" and (.run | contains("make release-verify")))) and
    (.jobs.tooling.steps | any(.name == "Retain group plans, package scopes and stage execution evidence" and .with["include-hidden-files"] == true)) and
    (.jobs.tooling.steps | any(.name == "Qualify native CLI, installation and Metrics restoration probe" and .if == null)) and
    .jobs["wire-schema"].if == null and .jobs["library-tests"].if == null
' "$TEMPORARY/workflow.json" > /dev/null
jq -r '.packages[] | select(.name == "ic-blob-storage-pocketic-tests") | .targets[] | select(.kind == ["test"]) | .name' \
    "$TEMPORARY/metadata.json" | LC_ALL=C sort > "$TEMPORARY/actual-tests"
perl -e '
    local $/; open my $f,"<","Makefile" or die $!; my $m=<$f>;
    $m =~ /^test-pocketic-client:.*?\n(.*?)(?=^\S|\z)/ms or die "missing Client partition\n"; my $c=$1;
    $m =~ /^test-pocketic-engine:.*?\n(.*?)(?=^\S|\z)/ms or die "missing Engine partition\n"; my $e=$1;
    $e =~ /--lib/ && $e =~ /--doc/ && $e =~ /--skip storage_tenants::client::/ or die "incomplete Engine partition\n";
    $c =~ /--test standalone/ && $c =~ /--test storage storage_tenants::client::/ or die "incomplete Client boundary\n";
    my %seen; my @all=grep {!$seen{$_}++} ($c =~ /--test ([a-z_]+)/g, $e =~ /--test ([a-z_]+)/g);
    print "$_\n" for sort @all;
' > "$TEMPORARY/partition-tests"
cmp "$TEMPORARY/actual-tests" "$TEMPORARY/partition-tests"
# Real Cargo owns member discovery, including globbed members, aliases and table
# dependency declarations. Release-version preparation must not change path scope.
mkdir -p "$TEMPORARY/cargo-layout/ci"
cat > "$TEMPORARY/cargo-layout/Cargo.toml" <<'CARGO'
[workspace]
members = ["products/*"]
resolver = "3"
[workspace.dependencies.aliased-engine]
package = "ic-blob-storage"
path = "products/engine"
version = "0.68.0"
CARGO
for pair in contracts:ic-blob-storage-contracts client:ic-blob-storage-client engine:ic-blob-storage; do
    directory="$TEMPORARY/cargo-layout/products/${pair%%:*}"
    mkdir -p "$directory/src"
    printf '[package]\nname = "%s"\nversion = "0.68.0"\nedition = "2024"\nrust-version = "1.99.1"\n' "${pair#*:}" > "$directory/Cargo.toml"
    : > "$directory/src/lib.rs"
done
printf '\n[dependencies]\naliased-engine.workspace = true\n' >> "$TEMPORARY/cargo-layout/products/client/Cargo.toml"
jq '.groups |= map(.members=[.primary] | .dependencies=[.primary])' ci/cargo-release-groups.json > "$TEMPORARY/cargo-layout/ci/cargo-release-groups.json"
jq -cS '.stages |= map_values("members")' ci/release-package-scopes.json > "$TEMPORARY/cargo-layout/ci/release-package-scopes.json"
(
    cd "$TEMPORARY/cargo-layout"
    cargo generate-lockfile --offline > "$TEMPORARY/layout-lock.log" 2>&1
    perl -e 'require $ARGV[0]; print BlobValidation::encode(BlobValidation::workspace_members());' "$ROOT/scripts/release/validation.pl" > "$TEMPORARY/layout-before.json"
    jq -e '.["ic-blob-storage"] == "products/engine/Cargo.toml" and length == 3' "$TEMPORARY/layout-before.json" > /dev/null
    [[ "$(perl "$ROOT/scripts/release/validation.pl" packages clippy)" == --workspace ]]
    perl -pi -e 's/version = "0.68.0"/version = "0.68.1"/' products/engine/Cargo.toml
    cargo generate-lockfile --offline > "$TEMPORARY/layout-prepared-lock.log" 2>&1
    perl -e 'require $ARGV[0]; print BlobValidation::encode(BlobValidation::workspace_members());' "$ROOT/scripts/release/validation.pl" > "$TEMPORARY/layout-after.json"
    cmp "$TEMPORARY/layout-before.json" "$TEMPORARY/layout-after.json"
)
# Exercise the reviewed consumer selector and canonical runner with real Git
# history, exact plans/checkpoints/results, and no Cargo builds or real Git effects.
mkdir "$TEMPORARY/source"
bash scripts/ci/prepare-script-fixture.sh "$ROOT" "$TEMPORARY/source" \
    scripts/ci/run-validation-targets.sh scripts/ci/validation-evidence.pl \
    scripts/ci/release-receipts.sh
mkdir -p "$TEMPORARY/source/scripts/release" "$TEMPORARY/source/ci" "$TEMPORARY/templates"
cp scripts/release/validation.pl scripts/release/validate.sh "$TEMPORARY/source/scripts/release/"
cp scripts/ci/check-release-source.sh "$TEMPORARY/source/scripts/ci/"
cp ci/cargo-release-groups.json ci/release-package-scopes.json "$TEMPORARY/source/ci/"
cp Cargo.toml "$TEMPORARY/source/"
cp scripts/ci/run-nonempty-cargo-test.sh "$TEMPORARY/source/scripts/ci/"
jq --arg root "$TEMPORARY/source" '
    .workspace_root as $old | .workspace_root=$root |
    .packages |= map(.manifest_path = ($root + (.manifest_path | ltrimstr($old))))
' "$TEMPORARY/metadata.json" > "$TEMPORARY/fixture-metadata.json"
export TEST_METADATA="$TEMPORARY/fixture-metadata.json"
# Exercise the actual five Cargo recipes, substituting only Cargo itself.
perl -0777 -ne 'for my $t (qw(clippy check docs-check test-native wasm-check ci)) { /^\Q$t\E:\n(.*?)(?=^\S|\z)/ms or die "missing $t recipe"; print "$t:\n$1"; }' Makefile > "$TEMPORARY/package-recipes.mk"
perl -0777 -ne '/^override CI_TARGETS = [^\n]+\n/m or die "missing roster caller"; print $&;' Makefile > "$TEMPORARY/roster.mk"
mkdir "$TEMPORARY/bin"
cat > "$TEMPORARY/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == metadata ]]; then
    cat "$TEST_METADATA"
    [[ "${TEST_FAIL_METADATA:-0}" != 1 ]]
    exit
fi
printf '%s\n' "$*" >> "$TEST_COMMANDS"
[[ "${TEST_FAIL:-}" != "$1" ]] || exit 1
if [[ "$1" == test ]]; then echo 'test result: ok. 1 passed; 0 failed;'; fi
CARGO
chmod +x "$TEMPORARY/bin/cargo"
export PATH="$TEMPORARY/bin:$PATH" TEST_COMMANDS="$TEMPORARY/commands"
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
while IFS=$'\t' read -r package manifest; do
    mkdir -p "${manifest%/*}/src"
    printf '[package]\nname = "%s"\nversion = "0.68.0"\n' "$package" > "$manifest"
    printf '// initial fixture\n' > "${manifest%/*}/src/mod.rs"
done < <(jq -r --arg root "$PWD/" '.packages[] | [.name,(.manifest_path | ltrimstr($root))] | @tsv' "$TEST_METADATA")
mkdir -p crates/ic-blob-storage-client/src/tenants
printf '// initial tenant client\n' > crates/ic-blob-storage-client/src/tenants/mod.rs
cat > Makefile <<'MAKE'
SHELL := /bin/bash
.PHONY: lint clippy check docs-check test-native wasm-check ci client-boundary test-pocketic-engine release-metadata-check release-check
lint client-boundary test-pocketic-engine release-metadata-check release-check:
	@echo "$@" >> "$$TEST_EVENTS"
	@test "$${TEST_FAIL:-}" != "$@"
	@if test "$@" = client-boundary && test "$${TEST_DRIFT:-0}" = 1; then echo drift >> ci/config.json; fi
MAKE
cat "$TEMPORARY/roster.mk" >> Makefile
cat "$TEMPORARY/package-recipes.mk" >> Makefile
printf '%s\n' '{"groups":["contracts","client","engine"],"schema":1,"stages":["lint","clippy","check","docs-check","test-native","wasm-check","client-boundary","test-pocketic-engine","release-metadata-check","release-check"]}' > ci/release-validation.json
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$TEMPORARY/templates"
: > "$TEST_COMMANDS"
make clippy check docs-check test-native wasm-check > "$TEMPORARY/full-commands.log" 2>&1
cp "$TEST_COMMANDS" "$TEMPORARY/full-commands"
# No plan means the original full workspace/native/doc roots, regardless of group.
[[ "$(sed -n '1p' "$TEST_COMMANDS")" == 'clippy --offline --locked --workspace --all-targets --all-features -- -D warnings' ]]
[[ "$(sed -n '2p' "$TEST_COMMANDS")" == 'check --offline --locked --workspace --all-targets --all-features' ]]
[[ "$(sed -n '3p' "$TEST_COMMANDS")" == 'doc --offline --locked -p ic-blob-storage -p ic-blob-storage-client -p ic-blob-storage-contracts -p ic-blob-storage-canister --all-features --no-deps' ]]
[[ "$(sed -n '4p' "$TEST_COMMANDS")" == 'test --offline --locked -p ic-blob-storage -p ic-blob-storage-client -p ic-blob-storage-contracts -p blob-consumer-probe -p ic-blob-storage-canister -p ic-blob-storage-cli --all-features' ]]
[[ "$(sed -n '5p' "$TEST_COMMANDS")" == 'check --offline --locked --workspace --all-features --target wasm32-unknown-unknown' ]]
git init -q
git config user.name 'Blob validation fixture'
git config user.email 'fixture@example.invalid'
git config core.hooksPath /dev/null
git add .
git commit -qm 'Admitted fixture baseline'
# Execute the actual normal-ci recipe with an invalid ambient plan. All five
# original full Cargo commands must still execute through the canonical runner.
: > "$TEST_COMMANDS"
BLOB_RELEASE_VALIDATION_PLAN=invalid TEST_EVENTS="$TEMPORARY/events" make ci CI_TARGETS=lint > "$TEMPORARY/normal-ci.log" 2>&1
cmp "$TEMPORARY/full-commands" "$TEST_COMMANDS"
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
mkdir -p .tmp/blob-release-validation.historical
printf 'original historical evidence\n' > .tmp/blob-release-validation.historical/plan.json
cp .tmp/blob-release-validation.historical/plan.json "$TEMPORARY/historical-plan.json"
TEST_FAIL_METADATA=1 refuse perl scripts/release/validation.pl packages clippy
verify_record() {
    perl -MJSON::PP -e 'require "./scripts/release/validation.pl"; local $/; open my $f,"<",$ARGV[0] or die $!; BlobValidation::verify(decode_json(<$f>),$ENV{RELEASE_SOURCE},$ENV{RELEASE_GROUP});' "$1"
}
record=".git/release-state/validation/client-$RELEASE_SOURCE.json"
TEST_FAIL=test-pocketic-engine bash scripts/release/validate.sh > "$TEMPORARY/narrow.log" 2>&1
verify_record "$record"
jq -e '.plan.scope == "group" and (.evidence.stages | map(select(.state == "not-applicable") | .target) == ["test-pocketic-engine","release-check"])' "$record" > /dev/null
if grep -Eq '^(test-pocketic-engine|release-check)$' "$TEST_EVENTS"; then exit 1; fi
cp "$record" "$TEMPORARY/good-record.json"
# Shared re-admits the actual retained run; Blob's self-contained receipt must
# also work in a fresh publication checkout without rewriting absolute log paths.
run="$(jq -r .evidence.run "$record")"
active_plan="$(jq -r .plan_path "$record")"
context="${active_plan%/*}/configuration.json"
proof="${active_plan%/*}/proof.json"
jq -cS '.evidence' "$record" > "$proof"
perl scripts/ci/validation-evidence.pl check "$PWD" "$active_plan" "$RELEASE_SOURCE" "$RELEASE_GROUP" "$run" "$context" "$proof" > "$TEMPORARY/shared-proof.json"
cmp "$proof" "$TEMPORARY/shared-proof.json"
log="$run/$(jq -r '.evidence.stages[] | select(.state == "PASS") | .log' "$record" | sed -n '1p')"
cp "$log" "$TEMPORARY/original-log"
printf '\nchanged retained output\n' >> "$log"
refuse perl scripts/ci/validation-evidence.pl check "$PWD" "$active_plan" "$RELEASE_SOURCE" "$RELEASE_GROUP" "$run" "$context" "$proof"
cmp "$record" "$TEMPORARY/good-record.json"
cp "$TEMPORARY/original-log" "$log"
# A malformed execution cannot replace a valid receipt through the production
# writer, including when the selector itself and the source remain unchanged.
cp "$run/plan-events.tsv" "$TEMPORARY/original-events"
printf 'invalid execution\n' > "$run/plan-events.tsv"
source scripts/ci/release-receipts.sh
refuse release_receipt_write "$record" perl scripts/release/validation.pl capture "${active_plan%/*}"
cmp "$record" "$TEMPORARY/good-record.json"
cp "$TEMPORARY/original-events" "$run/plan-events.tsv"
git clone -q --no-hardlinks . "$TEMPORARY/publication"
(
    cd "$TEMPORARY/publication"
    [[ ! -e "$active_plan" ]]
    verify_record "$TEMPORARY/good-record.json"
)
# Client implementation includes Engine reverse consumers, but not Contracts roots.
jq -e 'all(.plan.stages[] | select(.target == "clippy" or .target == "check" or .target == "wasm-check"); (.reason | contains("-p ic-blob-storage-pocketic-tests")) and (.reason | contains("-p ic-blob-storage-client")) and (.reason | contains("-p ic-blob-storage-contracts") | not))' "$record" > /dev/null
active_plan="$(jq -r .plan_path "$record")"
BLOB_RELEASE_VALIDATION_PLAN="$active_plan" TEST_FAIL=clippy refuse make clippy
cp "$active_plan" "$TEMPORARY/active-plan.json"
jq '.stages[1].reason += " changed"' "$active_plan" > "$TEMPORARY/tampered-plan.json"
cp "$TEMPORARY/tampered-plan.json" "$active_plan"
BLOB_RELEASE_VALIDATION_PLAN="$active_plan" refuse make clippy
cp "$TEMPORARY/active-plan.json" "$active_plan"
BLOB_RELEASE_VALIDATION_PLAN="$active_plan" RUSTFLAGS=changed refuse make clippy
cp "$TEST_METADATA" "$TEMPORARY/original-metadata.json"
jq '.packages[0].manifest_path="/outside/Cargo.toml"' "$TEMPORARY/original-metadata.json" > "$TEST_METADATA"
BLOB_RELEASE_VALIDATION_PLAN="$active_plan" refuse make clippy
cp "$TEMPORARY/original-metadata.json" "$TEST_METADATA"
for mutation in '.evidence.stages[0].state="FAIL"' '.evidence.stages[0].state="blocked"' '.evidence.stages=[]' '.evidence.inputs={}' '.evidence=null' '.evidence.context_sha256="changed"' '.configuration=null' '.configuration.workspace_members={}' '.group="engine"' '.plan.stages[7].required=true'; do
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
jq -cS '.stages += ["unreviewed-stage"]' ci/release-validation.json > "$TEMPORARY/roster.json"
cp "$TEMPORARY/roster.json" ci/release-validation.json
git add ci/release-validation.json
git commit -qm 'Unreviewed roster'
RELEASE_SOURCE="$(git rev-parse HEAD)"
perl scripts/release/validation.pl plan .tmp/invalid/plan.json
jq -e '.scope == "workspace" and any(.stages[]; .target == "unreviewed-stage" and .required)' .tmp/invalid/plan.json > /dev/null
refuse bash scripts/release/validate.sh
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
# Contracts changes select all public identities and reverse consumers.
jq -e 'all(.stages[] | select(.target == "clippy" or .target == "check"); (.reason | contains("-p ic-blob-storage-contracts")) and (.reason | contains("-p ic-blob-storage-client")) and (.reason | contains("-p ic-blob-storage")))' .tmp/contracts/plan.json > /dev/null
git reset --hard -q "$client_source"
# The complete delta includes an earlier Engine change even when the last commit
# is tenant-only. Only Engine edits select Engine roots; both IC partitions stay.
git reset --hard -q 'client-v0.68.0^{commit}'
printf '// Engine implementation\n' >> crates/ic-blob-storage/src/mod.rs
git add .
git commit -qm 'Engine implementation'
RELEASE_SOURCE="$(git rev-parse HEAD)"
RELEASE_GROUP=engine perl scripts/release/validation.pl plan .tmp/full/plan.json
jq -e '.scope == "group" and all(.stages[] | select(.target == "clippy" or .target == "check"); (.reason | contains("-p ic-blob-storage ")) and (.reason | contains("-p ic-blob-storage-client") | not) and (.reason | contains("-p ic-blob-storage-contracts") | not))' .tmp/full/plan.json > /dev/null
: > "$TEST_COMMANDS"
RELEASE_GROUP=engine bash scripts/release/validate.sh > "$TEMPORARY/engine.log" 2>&1
[[ "$(wc -l < "$TEST_COMMANDS")" == 5 ]]
if grep -Eq -- '-p ic-blob-storage-(client|contracts)( |$)' "$TEST_COMMANDS"; then exit 1; fi
printf '// later tenant change\n' >> crates/ic-blob-storage-client/src/tenants/mod.rs
git add .
git commit -qm 'Later tenant implementation'
RELEASE_SOURCE="$(git rev-parse HEAD)"
perl scripts/release/validation.pl plan .tmp/full/plan.json
jq -e '.scope == "group" and any(.stages[]; .target == "test-pocketic-engine" and .required)' .tmp/full/plan.json > /dev/null
# Dependencies, generated input, tests outside reviewed roots and renames from
# unclassified paths must broaden. Never bless an old baseline after policy drift.
for input in Cargo.toml Cargo.lock ci/config.json scripts/generator.pl tests/input.rs; do
    git reset --hard -q "$client_source"
    mkdir -p "$(dirname "$input")"
    printf '\n// changed\n' >> "$input"
    git add .
    git commit -qm 'Shared or unknown input'
    RELEASE_SOURCE="$(git rev-parse HEAD)"
    perl scripts/release/validation.pl plan .tmp/full/plan.json
    jq -e '.scope == "workspace" and all(.stages[]; .required)' .tmp/full/plan.json > /dev/null
done
git reset --hard -q "$client_source"
# A pre-mapping receipt cannot enable narrower package commands.
jq 'del(.validation.evidence.inputs["ci/release-package-scopes.json"])' releases/client/release.json > "$TEMPORARY/legacy.json"
cp "$TEMPORARY/legacy.json" releases/client/release.json
git add .
git commit -qm 'Legacy baseline receipt'
git tag -fa client-v0.68.0 -m 'Legacy fixture baseline' > /dev/null
printf '// tenant change\n' >> crates/ic-blob-storage-client/src/tenants/mod.rs
git add .
git commit -qm 'Post legacy implementation'
RELEASE_SOURCE="$(git rev-parse HEAD)"
perl scripts/release/validation.pl plan .tmp/full/plan.json
jq -e '.scope == "workspace" and all(.stages[]; .required)' .tmp/full/plan.json > /dev/null
# Incomplete ownership/roots cannot supply even ordinary full command flags.
cp ci/release-package-scopes.json "$TEMPORARY/package-policy.json"
for mutation in '.consumers.engine=["client"]' '.consumers.client += ["client"]' '.stages.clippy=["not-a-member"]' 'del(.stages.check)' '.stages["test-native"]=[]'; do
    jq -cS "$mutation" "$TEMPORARY/package-policy.json" > ci/release-package-scopes.json
    refuse perl scripts/release/validation.pl packages clippy
done
cp "$TEMPORARY/package-policy.json" ci/release-package-scopes.json
cmp "$TEMPORARY/historical-plan.json" .tmp/blob-release-validation.historical/plan.json
# An empty/missing policy cannot turn normal full CI into an empty success.
cp ci/release-validation.json "$TEMPORARY/validation-policy.json"
jq -cS '.stages=[]' "$TEMPORARY/validation-policy.json" > ci/release-validation.json
refuse make ci
rm ci/release-validation.json
refuse make ci
TEST_EVENTS="$TEMPORARY/events" make lint > "$TEMPORARY/primitive-without-policy.log" 2>&1
cp "$TEMPORARY/validation-policy.json" ci/release-validation.json
complete=true
echo 'Blob release selection, fail-closed execution, source-bound receipts and test partitions passed.'
