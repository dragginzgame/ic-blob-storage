#!/usr/bin/env bash
# Shared companions: scripts/release/group.sh scripts/release/release-data.pl ci/release-delivery-policy.txt scripts/ci/check-release-delivery.sh scripts/ci/check-release-source.sh scripts/ci/cargo-release-metadata.sh scripts/ci/check-release-tag.sh scripts/ci/check-crates-io-version.sh scripts/ci/next-release-version.sh scripts/ci/check-release-changelog.sh scripts/ci/release-metadata-transaction.sh scripts/ci/cargo-release-preflight.sh scripts/ci/check-retained-state.pl
set -euo pipefail
root_file="${BASH_SOURCE[0]}"
[[ "$root_file" == /* ]] || root_file="$PWD/$root_file"
case "$root_file" in *$'\n'*|*$'\r'*) echo 'release directory must not contain LF or CR' >&2; exit 2 ;; esac
ROOT="$(cd -P "${root_file%/*}/../.." && pwd -P)"
case "$ROOT" in *$'\n'*|*$'\r'*) echo 'release directory must not contain LF or CR' >&2; exit 2 ;; esac
cd "$ROOT"
export CARGO_TARGET_DIR="$ROOT/target"
unset BLOB_RELEASE_SOURCE_ROOT BLOB_RELEASE_VALIDATION_RECORD
export RELEASE_GROUP
case "${RELEASE_GROUP:-}" in contracts|client|engine) ;; *) echo 'Select RELEASE_GROUP=contracts, client or engine' >&2; exit 2 ;; esac
DATA="$ROOT/scripts/release/release-data.pl"
fail() { echo "release metadata refused: $*" >&2; exit 1; }
command="${1:-}"
shift || true
bash scripts/ci/check-release-delivery.sh ci/release-delivery-policy.txt "${RELEASE_DELIVERY-direct}"
plan="$(bash scripts/release/group.sh plan)"
primary="$(jq -er .primary <<< "$plan")"
changelog="$(jq -er .changelog <<< "$plan")"
paths="$(jq -er '.files[]' <<< "$plan")"
RELEASE_FILES=()
while IFS= read -r path; do RELEASE_FILES+=("$path"); done <<< "$paths"
version() { bash scripts/release/group.sh version; }
ensure_clean() {
    git rev-parse --verify HEAD >/dev/null || fail 'cannot resolve release HEAD'
    bash scripts/ci/check-release-source.sh || fail 'commit the implementation and group notes before releasing'
}
allowed_changes() {
    local path allowed=()
    for path in "${RELEASE_FILES[@]}"; do allowed+=(--allow "$path"); done
    bash scripts/ci/check-release-source.sh "${allowed[@]}" || fail 'non-release paths changed'
}
preflight() {
    local head previous state tool_prepare="${BLOB_RELEASE_PREPARE:-0}"
    ensure_clean
    head="$(git rev-parse HEAD)" || fail 'cannot resolve release source'
    previous="$(version)" || fail 'cannot read previous version'
    [[ "$head" == "${RELEASE_SOURCE:?}" && "$previous" == "${RELEASE_PREVIOUS:?}" ]] || fail 'source/version does not match release intent'
    bash scripts/ci/check-release-changelog.sh "${RELEASE_VERSION:?}" "$previous" "${RELEASE_DATE:?}" "$changelog"
    # The runner repeats preflight after saving preparation intent. Setup belongs
    # only to fresh admission, never that second check or saved-attempt recovery.
    state="$(git rev-parse --git-path release-state)" || fail 'cannot resolve saved intent'
    if [[ -e "$state/$RELEASE_GROUP-$RELEASE_VERSION.plan" || -L "$state/$RELEASE_GROUP-$RELEASE_VERSION.plan" ]]; then tool_prepare=0; fi
    bash scripts/ci/cargo-release-preflight.sh "$ROOT" "$tool_prepare" \
        --pin Cargo.toml --pin Cargo.lock --pin ci/tool-versions.env --pin ci/ic-tools.tsv \
        --check tools-check --check release-tools-check \
        --prepare install-tools --prepare tools-check --prepare release-tools-check
    [[ "$(git rev-parse HEAD)" == "$RELEASE_SOURCE" ]] || fail 'source changed during tool preparation'
    ensure_clean
}
# Shared owns installation/rollback of the exact selected roster. Render the
# group and its source-bound receipt in disposable committed source first.
# shellcheck source=scripts/ci/release-metadata-transaction.sh
source scripts/ci/release-metadata-transaction.sh
# shellcheck disable=SC2329 # Invoked by the shared metadata transaction.
render_metadata() {
    local original="$1" candidate="$2" workspace path validation_record
    validation_record="$(git rev-parse --git-path release-state)/validation/$RELEASE_GROUP-$RELEASE_SOURCE.json"
    [[ "$validation_record" == /* ]] || validation_record="$ROOT/$validation_record"
    workspace="${original%/original}/workspace"
    mkdir "$workspace" || return
    git archive HEAD | tar -xf - -C "$workspace" || return
    for path in "${RELEASE_FILES[@]}"; do
        mkdir -p "$workspace/$(dirname "$path")" || return
        cp -p "$original/$path" "$workspace/$path" || return
    done
    (
        cd "$workspace" || exit
        export BLOB_RELEASE_SOURCE_ROOT="$ROOT" BLOB_RELEASE_VALIDATION_RECORD="$validation_record"
        bash scripts/ci/cargo-release-metadata.sh prepare "$primary" || exit
        make --no-print-directory fmt-check || exit
        perl scripts/release/release-data.pl receipt "$RELEASE_SOURCE" "$RELEASE_DATE" || exit
        perl scripts/release/release-data.pl verify "$RELEASE_SOURCE" "$RELEASE_VERSION" "$RELEASE_DATE"
    ) || return
    for path in "${RELEASE_FILES[@]}"; do cp -p "$workspace/$path" "$candidate/$path" || return; done
}
# shellcheck disable=SC2329 # Invoked by the shared metadata transaction.
check_metadata() {
    perl "$DATA" verify "$RELEASE_SOURCE" "$RELEASE_VERSION" "$RELEASE_DATE" || return
    bash scripts/ci/cargo-release-metadata.sh check "$primary" --manifest-order-grouped
}
prepare() {
    [[ "$(git rev-parse HEAD)" == "${RELEASE_SOURCE:?}" ]] || fail 'source commit does not match release intent'
    if [[ "$(version)" == "${RELEASE_VERSION:?}" ]]; then verify_prepared; return; fi
    ensure_clean
    [[ "$(version)" == "${RELEASE_PREVIOUS:?}" ]] || fail 'previous version does not match release intent'
    perl scripts/release/validation.pl check "$(git rev-parse --git-path release-state)/validation/$RELEASE_GROUP-$RELEASE_SOURCE.json" || fail 'complete the current source-bound release validation first'
    CARGO_RELEASE_METADATA_EDITOR="$(make --no-print-directory --silent release-editor-check)" || fail 'prepare the selected editor with make install-release-editor'
    export CARGO_RELEASE_METADATA_EDITOR CARGO_RELEASE_METADATA_MAX_BYTES=8388608
    release_metadata_prepare "${TMPDIR:-/tmp}/blob-group-metadata.XXXXXX" render_metadata check_metadata "${RELEASE_FILES[@]}"
}
verify_prepared() {
    [[ "$(git rev-parse HEAD)" == "${RELEASE_SOURCE:?}" ]] || fail 'source commit does not match release intent'
    allowed_changes
    check_metadata
    make --no-print-directory fmt-check
}
commit_check() {
    verify_prepared
    git diff --quiet -- || fail 'unstaged release edits remain'
    local paths
    paths="$(mktemp "${TMPDIR:-/tmp}/blob-release-index.XXXXXX")"
    git diff --cached --name-only -z -- > "$paths"
    if ! perl "$DATA" index-check "$paths"; then rm -f "$paths"; fail 'index must contain exactly the selected group release files'; fi
    rm -f "$paths"
}
committed_check() {
    ensure_clean
    local commit="${RELEASE_COMMIT:?}" parent
    [[ "$commit" =~ ^[0-9a-f]{40,64}$ ]] || fail 'release commit must be exact'
    perl "$DATA" verify-commit "$commit" "${RELEASE_SOURCE:?}" "${RELEASE_VERSION:?}" "${RELEASE_DATE:?}"
    parent="$(git log -1 --format=%P "$commit")"
    [[ "$parent" == "$RELEASE_SOURCE" ]] || fail 'release parent does not match validated source'
    RELEASE_COMMIT="$commit" bash scripts/ci/cargo-release-metadata.sh check "$primary" --manifest-order-grouped
}
verify_tag() { bash scripts/ci/check-release-tag.sh "$1" "$2" "$RELEASE_GROUP" || fail 'tag does not match selected group'; }
tag_check() {
    ensure_clean
    # source verifies the complete receipt as well as returning its identity.
    local current source parent head changed
    current="$(version)"; source="$(perl "$DATA" source)"
    parent="$(git log -1 --format=%P HEAD)"; head="$(git rev-parse HEAD)"
    [[ "$parent" == "$source" ]] || fail 'release parent does not match validated source'
    changed="$(mktemp "${TMPDIR:-/tmp}/blob-release-commit.XXXXXX")"
    git diff --name-only -z "$source" "$head" -- > "$changed"
    if ! perl "$DATA" index-check "$changed"; then rm -f "$changed"; fail 'release commit changed paths outside its exact roster'; fi
    rm -f "$changed"
    verify_tag "$head" "$current"
}
publication_checksum() {
    local metadata status=0
    metadata="$(bash scripts/ci/check-crates-io-version.sh --metadata "$1" "$2" "$3")" || status=$?
    case "$status" in
        0) jq -er 'select(.yanked == false) | .checksum' <<< "$metadata" || fail 'published version is yanked' ;;
        1) printf 'missing\n' ;;
        *) fail 'registry state is inconclusive; no upload authorized' ;;
    esac
}
publish() {
    case "${1:-}" in ''|--dry-run) ;; *) fail 'expected publish [--dry-run]' ;; esac
    [[ -z "${PUBLISH_PACKAGE:-}" || "$PUBLISH_PACKAGE" == "$primary" ]] || fail 'publication package is outside selected group'
    # Separate registry authority still shares the release lock: never publish
    # while another process may be changing this package's release metadata.
    PUBLICATION_STATE_ROOT="$(git rev-parse --git-path release-state)" || fail 'cannot resolve publication lock path'
    [[ ! -L "$PUBLICATION_STATE_ROOT" ]] || fail 'release state directory is symlinked'
    perl scripts/ci/check-retained-state.pl --root "$PUBLICATION_STATE_ROOT"
    mkdir -p "$PUBLICATION_STATE_ROOT"
    mkdir "$PUBLICATION_STATE_ROOT/lock" 2>/dev/null || fail 'release/publication lock is occupied'
    printf '%s\n' "$$" > "$PUBLICATION_STATE_ROOT/lock/owner"
    trap 'status=$?; rm -f "$PUBLICATION_STATE_ROOT/lock/owner"; rmdir "$PUBLICATION_STATE_ROOT/lock"; if ! perl scripts/ci/check-retained-state.pl --root "$PUBLICATION_STATE_ROOT"; then [[ "$status" != 0 ]] || status=1; fi; exit "$status"' EXIT
    tag_check
    local packages=("$primary")
    local current head package checksum run registry_user_agent contracts_version
    current="$(version)" || fail 'cannot read publication version'
    registry_user_agent="ic-blob-storage-release/$current (+https://github.com/dragginzgame/ic-blob-storage)"
    head="$(git rev-parse HEAD)" || fail 'cannot resolve publication source'
    mkdir -p "$CARGO_TARGET_DIR"
    run="$(mktemp -d "$CARGO_TARGET_DIR/publication.XXXXXX")"
    printf 'Publication readback retained: %s\n' "$run"
    # Fixed archive mtimes must not admit artifacts from a prior payload verification.
    export CARGO_TARGET_DIR="$run/target"
    for package in "${packages[@]}"; do
        # Only a successful exact registry read can authorize skipping an upload.
        # Cargo owns upload and index polling; no local progress journal is needed.
        checksum="$(publication_checksum "$run/$package-observation" "$package" "$current")" || fail 'registry readback refused'
        if [[ "$checksum" != missing ]]; then
            curl --disable --fail --silent --show-error --connect-timeout 10 --max-time 30 \
                --user-agent "$registry_user_agent" \
                --max-filesize 8388608 --output "$run/$package.crate" \
                "https://static.crates.io/crates/$package/$package-$current.crate" || fail 'cannot authenticate published archive'
            tar -xOf "$run/$package.crate" "$package-$current/.cargo_vcs_info.json" > "$run/$package-vcs.json" || fail 'cannot read published source identity'
            perl "$DATA" publication-archive "$run/$package.crate" "$checksum" "$run/$package-vcs.json" "$head" "$package" || fail 'published package differs from selected release'
            if [[ "${1:-}" != --dry-run ]]; then
                printf '%s %s already published from this release; skipping upload.\n' "$package" "$current"
                continue
            fi
        fi
        if [[ "${1:-}" == --dry-run && "$package" != ic-blob-storage-contracts ]]; then
            contracts_version="$(perl "$DATA" package-version ic-blob-storage-contracts)"
            checksum="$(publication_checksum "$run/$package-contracts-dependency-observation" ic-blob-storage-contracts "$contracts_version")" || fail 'contracts dependency readback refused'
            [[ "$checksum" != missing ]] || fail 'dry-run requires the selected Contracts dependency on crates.io; run make package for local payload verification'
        fi
        # Read-only retries above authenticate the existing publication without
        # compiling. New uploads and dry runs require admitted completed lint
        # or current-source Clippy before Cargo archive verification/upload.
        make --no-print-directory deps
        make --no-print-directory compiler-check
        perl scripts/release/stage.pl --publication-clippy "$primary" "$run"
        cargo publish --locked --registry crates-io -p "$package" ${1:+"$1"}
    done
}
case "$command" in
    version) version ;;
    plan)
        previous="$(version)"
        candidate="$(bash scripts/ci/next-release-version.sh "$previous" "${1:-patch}")"
        printf 'Group: %s\nCurrent: %s\nTarget: %s\nRemote: %s\nBranch: %s\n' "$RELEASE_GROUP" "$previous" "$candidate" "${RELEASE_REMOTE:-origin}" "${RELEASE_BRANCH:-main}"
        ;;
    ensure-clean) ensure_clean ;;
    preflight) preflight ;;
    prepare) prepare ;;
    prepared-check) verify_prepared ;;
    files) printf '%s\0' "${RELEASE_FILES[@]}" ;;
    commit-check) commit_check ;;
    committed-check) committed_check ;;
    tagged-check) committed_check; verify_tag "$RELEASE_COMMIT" "$RELEASE_VERSION" ;;
    tag-check) tag_check ;;
    publish) publish "${1:-}" ;;
    *) fail 'expected a metadata/check operation; use the standard release targets for Git effects' ;;
esac
