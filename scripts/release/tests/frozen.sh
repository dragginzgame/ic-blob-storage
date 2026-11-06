#!/usr/bin/env bash
# Consumer hooks/metadata are real; compilation and registry effects are recorded
# substitutes. Actual locked builds, installed behavior and archives remain gates
# of the first real frozen delivery, never replaced by this fixture.
run_frozen_fixture() {
    export TEST_FROZEN_NORMAL="$PWD" TEST_FROZEN_PUBLICATIONS="$TEMPORARY/publications"
    export TEST_FROZEN_FAIL="$TEMPORARY/publication-failure" TEST_REGISTRY=missing
    cat >> Makefile <<'MAKE'
fixture-validation:
	@test "$(FROZEN_RELEASE_ACTIVE)" = 1
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
    # This fixture owns its environment. Real frozen delivery uses the ordinary
    # caller shell; Make's exported target/context must not escape into it.
    unset CARGO_TARGET_DIR CARGO_HOME BLOB_PREPARED_TOOLS_ROOT
    unset BLOB_RELEASE_SOURCE_ROOT BLOB_RELEASE_VALIDATION_PLAN BLOB_RELEASE_STAGE_ADMISSIONS
    local status=0
    perl scripts/ci/run-frozen-release.pl --root "$PWD" --groups contracts,client,engine --publish-target publish > "$TEMPORARY/frozen-first.log" 2>&1 || status=$?
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
    refuse perl scripts/ci/run-frozen-release.pl --root "$PWD" --resume "$id"
    [[ "$(wc -l < "$TEST_FROZEN_PUBLICATIONS")" == 1 ]]
    contracts_commit=$(git -C "$work" rev-parse HEAD)
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
    perl scripts/ci/run-frozen-release.pl --root "$PWD" --resume "$id" --publication-result published > "$TEMPORARY/frozen-resume.log" 2>&1
    [[ "$(cut -f1 "$TEST_FROZEN_PUBLICATIONS")" == $'contracts\nclient\nengine' ]]
    jq -e '.phase == "complete" and .index == 3' "$intent" >/dev/null
    delivered=$(jq -r .commit "$intent")
    [[ "$(git --git-dir "$TEMPORARY/remote.git" rev-parse refs/heads/main)" == "$delivered" ]]
    [[ "$(git --git-dir "$gitdir" rev-parse HEAD)" == "$delivered" ]]
    [[ "$(git rev-parse HEAD)" == "$base" && "$(cat Cargo.lock)" == 'deferred normal Cargo update' ]]
    for group in contracts client engine; do
        git -C "$work" show "$group-v$(cat "$TEMPORARY/$group.version"):releases/$group/release.json" > "$TEMPORARY/frozen-$group-receipt.json"
        jq -e --arg group "$group" '.status == "released" and .group == $group and .validation.evidence.scope == "workspace"' "$TEMPORARY/frozen-$group-receipt.json" >/dev/null
    done
    perl scripts/ci/run-frozen-release.pl --root "$PWD" --resume "$id" > "$TEMPORARY/frozen-complete-resume.log" 2>&1
    [[ "$(wc -l < "$TEST_FROZEN_PUBLICATIONS")" == 3 ]]
    # Independent checkout/configuration: all metadata and saved locked graphs
    # remain readable despite deliberately unusable normal-checkout lock bytes.
    "$TEST_REAL_CARGO" metadata --offline --locked --no-deps --format-version 1 --manifest-path "$work/Cargo.toml" > "$TEMPORARY/frozen-final-metadata.json"
    jq -e --arg root "$work" '.workspace_root == $root' "$TEMPORARY/frozen-final-metadata.json" >/dev/null
    printf 'Blob frozen hooks passed: three groups, deferred lock isolation, exact receipt/archive controls, uncertain publication and idempotent resume (substitute compiler/upload effects).\n'
}
