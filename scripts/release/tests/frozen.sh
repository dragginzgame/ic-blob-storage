#!/usr/bin/env bash
# Consumer hooks/metadata are real; compilation and registry effects are recorded
# substitutes. Actual locked builds, installed behavior and archives remain gates
# of the first real frozen delivery, never replaced by this fixture.
run_frozen_routing_fixture() {
    # Replace only the disposable copy's delivery owner. Exercise Blob's actual
    # Makefile, including its environment boundary, without Git/registry effects.
    cat > scripts/ci/run-release-batch.pl <<'PERL'
use JSON::PP;
open my $fh, '>', $ENV{TEST_EVENTS} or die $!;
print {$fh} encode_json({args=>\@ARGV,target=>$ENV{CARGO_TARGET_DIR},tools=>$ENV{BLOB_PREPARED_TOOLS_ROOT}});
PERL
    printf '#!/usr/bin/env bash\nexit 89\n' > scripts/ci/run-release.sh
    unset RELEASE_GROUPS RELEASE_PUBLISH_TARGET RELEASE_SETUP_TARGET RELEASE_BATCH_ACTIVE
    for group in contracts client engine; do
        for kind in patch minor major; do
            CARGO_TARGET_DIR=/not-the-candidate BLOB_PREPARED_TOOLS_ROOT=/not-the-candidate \
                make --no-print-directory "release-$kind" "RELEASE_GROUP=$group" > "$TEMPORARY/$group-$kind.log" 2>&1
            jq -e --arg group "$group" --arg kind "$kind" '
                (.args | index("--groups")) as $g |
                (.args | index("--kind")) as $k |
                .target == null and .tools == null and .args[$g+1] == $group and .args[$k+1] == $kind and
                (.args | index("--publish-target")) == null' "$TEST_EVENTS" >/dev/null
        done
    done
    : > "$TEST_EVENTS"
    if make --no-print-directory release-patch RELEASE_GROUP=engine RELEASE_GROUPS= > "$TEMPORARY/empty-roster.log" 2>&1; then
        echo 'Empty roster bypassed frozen capture' >&2; return 1
    fi
    [[ ! -s "$TEST_EVENTS" ]]
    for retired in RELEASE_FROZEN_GROUPS RELEASE_FROZEN_PUBLISH_TARGET RELEASE_FROZEN_SETUP_TARGET RELEASE_FROZEN_SETUP_MODE FROZEN_RELEASE_ACTIVE; do
        if make --no-print-directory release-patch RELEASE_GROUP=engine "$retired=" > "$TEMPORARY/retired-$retired.log" 2>&1; then
            echo "Retired setting was silently accepted: $retired" >&2; return 1
        fi
        grep -F 'Retired release settings:' "$TEMPORARY/retired-$retired.log" >/dev/null
        [[ ! -s "$TEST_EVENTS" ]]
    done
    make --no-print-directory release-patch RELEASE_GROUP=contracts \
        RELEASE_GROUPS=contracts,client,engine RELEASE_PUBLISH_TARGET=publish > "$TEMPORARY/multiple-groups.log" 2>&1
    jq -e '(.args | index("--groups")) as $g | (.args | index("--publish-target")) as $p |
        .args[$g+1] == "contracts,client,engine" and .args[$p+1] == "publish"' "$TEST_EVENTS" >/dev/null
    printf 'Blob default batch routing passed: all three increments/groups, clean outer context, empty/retired-setting refusal and explicit multi-group publication (substitute runner).\n'
}

run_frozen_fixture() {
    export TEST_FROZEN_NORMAL="$PWD" TEST_FROZEN_PUBLICATIONS="$TEMPORARY/publications"
    export TEST_FROZEN_FAIL="$TEMPORARY/publication-failure" TEST_REGISTRY=missing
    cat >> Makefile <<'MAKE'
fixture-validation:
	@test "$(RELEASE_BATCH_ACTIVE)" = 1
	@test "$(CURDIR)" != "$$TEST_FROZEN_NORMAL"
	@printf 'deferred normal Cargo update\n' > "$$TEST_FROZEN_NORMAL/Cargo.lock"
	@cargo metadata --offline --locked --no-deps --format-version 1 | jq -e --arg root "$(CURDIR)" '.workspace_root == $$root' >/dev/null
MAKE
    git add Makefile
    git commit -qm 'Frozen hook qualification'
    base="$(git rev-parse HEAD)"
    git branch -M main
    git init --bare -q "$TEMPORARY/remote.git"
    git remote add origin "$TEMPORARY/remote.git"
    git push -q -u origin main
    setup_publication_substitutes
    : > "$TEST_FROZEN_FAIL"
    # This fixture owns its initial shell context. The outer Make must then
    # suppress its normal target/tool exports without weakening child setup.
    unset CARGO_TARGET_DIR CARGO_HOME BLOB_PREPARED_TOOLS_ROOT
    unset BLOB_RELEASE_SOURCE_ROOT BLOB_RELEASE_VALIDATION_PLAN BLOB_RELEASE_STAGE_ADMISSIONS
    local status=0
    make --no-print-directory release-patch RELEASE_GROUP=contracts \
        RELEASE_GROUPS=contracts,client,engine RELEASE_PUBLISH_TARGET=publish > "$TEMPORARY/frozen-first.log" 2>&1 || status=$?
    [[ "$status" != 0 ]]
    local intent id work gitdir contracts_commit archive_package delivered
    intent=$(find .git/release-state/frozen -name intent.json)
    [[ "$intent" != *$'\n'* && -f "$intent" ]]
    id="${intent%/intent.json}"; id="${id##*/}"
    work="$PWD/target/release-frozen/$id"
    gitdir="$PWD/.git/release-state/frozen/$id/repository.git"
    jq -e --arg base "$base" '.phase == "publishing" and .index == 0 and .source == $base' "$intent" >/dev/null
    [[ "$(git rev-parse HEAD)" == "$base" && "$(cat Cargo.lock)" == 'deferred normal Cargo update' ]]
    [[ "$(cut -f1 "$TEST_FROZEN_PUBLICATIONS")" == contracts ]]
    cp "$intent" "$TEMPORARY/frozen-before-status.json"
    make --no-print-directory release-batch-status "BATCH=$id" > "$TEMPORARY/frozen-status.log" 2>&1
    cmp "$intent" "$TEMPORARY/frozen-before-status.json"
    [[ "$(cat Cargo.lock)" == 'deferred normal Cargo update' ]]
    refuse make --no-print-directory release-batch-resume "BATCH=$id"
    [[ "$(wc -l < "$TEST_FROZEN_PUBLICATIONS")" == 1 ]]
    contracts_commit=$(git -C "$work" rev-parse HEAD)
    jq -e --arg commit "$contracts_commit" '.schema == 2 and .release_commit == $commit' "$intent" >/dev/null
    archive_package="ic-blob-storage-contracts-$(cat "$TEMPORARY/contracts.version")"
    mkdir -p "$TEMPORARY/archive/$archive_package"
    printf '{"git":{"sha1":"%s"},"path_in_vcs":"crates/ic-blob-storage-contracts"}' "$contracts_commit" > "$TEMPORARY/archive/$archive_package/.cargo_vcs_info.json"
    tar -czf "$TEST_ARCHIVE" -C "$TEMPORARY/archive" "$archive_package"
    export TEST_REGISTRY=dependency TEST_CONTRACTS_VERSION
    TEST_CONTRACTS_VERSION=$(cat "$TEMPORARY/contracts.version")
    # Consumer archive/checksum/source admission supplies the reconciliation
    # control; this is fixture data, not a real uploaded or compiled archive.
    make -C "$work" --no-print-directory publish RELEASE_GROUP=contracts > "$TEMPORARY/frozen-readback.log" 2>&1
    grep -F 'already published from this release; skipping upload.' "$TEMPORARY/frozen-readback.log" >/dev/null
    rm "$TEST_FROZEN_FAIL"
    make --no-print-directory release-batch-resume "BATCH=$id" PUBLICATION_RESULT=published > "$TEMPORARY/frozen-resume.log" 2>&1
    [[ "$(cut -f1 "$TEST_FROZEN_PUBLICATIONS")" == $'contracts\nclient\nengine' ]]
    jq -e '.schema == 2 and .phase == "complete" and .index == 3 and .release_commit == .commit' "$intent" >/dev/null
    delivered=$(jq -r .commit "$intent")
    [[ "$(git --git-dir "$TEMPORARY/remote.git" rev-parse refs/heads/main)" == "$delivered" ]]
    [[ "$(git --git-dir "$gitdir" rev-parse HEAD)" == "$delivered" ]]
    [[ "$(git rev-parse HEAD)" == "$base" && "$(cat Cargo.lock)" == 'deferred normal Cargo update' ]]
    for group in contracts client engine; do
        git -C "$work" show "$group-v$(cat "$TEMPORARY/$group.version"):releases/$group/release.json" > "$TEMPORARY/frozen-$group-receipt.json"
        jq -e --arg group "$group" '.status == "released" and .group == $group and .validation.evidence.scope == "workspace"' "$TEMPORARY/frozen-$group-receipt.json" >/dev/null
    done
    make --no-print-directory release-batch-resume "BATCH=$id" > "$TEMPORARY/frozen-complete-resume.log" 2>&1
    [[ "$(wc -l < "$TEST_FROZEN_PUBLICATIONS")" == 3 ]]
    # Independent checkout/configuration: all metadata and saved locked graphs
    # remain readable despite deliberately unusable normal-checkout lock bytes.
    "$TEST_REAL_CARGO" metadata --offline --locked --no-deps --format-version 1 --manifest-path "$work/Cargo.toml" > "$TEMPORARY/frozen-final-metadata.json"
    jq -e --arg root "$work" '.workspace_root == $root' "$TEMPORARY/frozen-final-metadata.json" >/dev/null
    refuse make --no-print-directory release-batch-reconcile "BATCH=$id"
    refuse make --no-print-directory release-batch-retire "BATCH=$id"
    [[ -d "$work" && -f "$intent" && "$(cat Cargo.lock)" == 'deferred normal Cargo update' ]]
    # Retain deferred fixture work on its own branch before the canonical clean
    # fast-forward. Never exercise reconciliation/retirement on real batches.
    git switch -qc deferred-lock
    git add Cargo.lock
    git commit -qm 'Preserve deferred fixture lock update'
    git switch -q main
    make --no-print-directory release-batch-reconcile "BATCH=$id" > "$TEMPORARY/frozen-reconcile.log" 2>&1
    [[ "$(git rev-parse HEAD)" == "$delivered" ]]
    [[ "$(git show deferred-lock:Cargo.lock)" == 'deferred normal Cargo update' ]]
    make --no-print-directory release-batch-retire "BATCH=$id" > "$TEMPORARY/frozen-retire.log" 2>&1
    [[ ! -e "$work" && ! -e "${intent%/intent.json}" ]]
    [[ "$(git rev-parse HEAD)" == "$delivered" ]]
    for group in contracts client engine; do
        git rev-parse --verify "$group-v$(cat "$TEMPORARY/$group.version")" >/dev/null
    done
    printf 'Blob frozen hooks passed: three groups, saved publication context, deferred lock isolation, exact receipt/archive controls, status, uncertain/idempotent resume, guarded reconciliation and fixture retirement (substitute compiler/upload effects).\n'
}
