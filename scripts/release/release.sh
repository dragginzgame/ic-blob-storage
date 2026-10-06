#!/usr/bin/env bash
set -euo pipefail

# Consumer metadata and qualification only. Shared Tooling owns every Git effect.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
cd "$ROOT"
export CARGO_TARGET_DIR="$ROOT/target"
DATA="$ROOT/scripts/release/release-data.pl"
RELEASE_FILES=(Cargo.toml Cargo.lock CHANGELOG.md docs/release.json)

fail() { echo "release metadata refused: $*" >&2; exit 1; }
version() { perl "$DATA" version; }
ensure_clean() {
    local status
    git rev-parse --verify HEAD >/dev/null || fail 'cannot resolve release HEAD'
    status="$(git status --porcelain --untracked-files=all)" || fail 'cannot read release working tree'
    [[ -z "$status" ]] || fail 'commit the implementation and notes before releasing'
}
allowed_changes() {
    local base="$1" path paths invalid=""
    paths="$(mktemp "${TMPDIR:-/tmp}/blob-release-paths.XXXXXX")"
    git diff --name-only -z "$base" -- > "$paths" || fail 'cannot read release changes'
    git ls-files --others --exclude-standard -z >> "$paths" || fail 'cannot read untracked release paths'
    while IFS= read -r -d '' path; do
        case "$path" in
            Cargo.toml|Cargo.lock|CHANGELOG.md|docs/release.json) ;;
            *) invalid="$path"; break ;;
        esac
    done < "$paths"
    rm -f "$paths"
    [[ -z "$invalid" ]] || fail "non-release path changed: $invalid"
}
preflight() {
    local head previous
    ensure_clean
    head="$(git rev-parse HEAD)" || fail 'cannot resolve release source'
    previous="$(version)" || fail 'cannot read previous version'
    [[ "$head" == "${RELEASE_SOURCE:?}" ]] || fail 'source commit does not match release intent'
    [[ "$previous" == "${RELEASE_PREVIOUS:?}" ]] || fail 'previous version does not match release intent'
    perl "$DATA" changelog-check "${RELEASE_VERSION:?}" "${RELEASE_DATE:?}"
    make --no-print-directory release-tools-check
    # The complete gate verifies the snapshot and fetches the selected lock before
    # any offline validation. Do not fetch again on post-validation preparation.
}
prepare() {
    local head previous
    head="$(git rev-parse HEAD)" || fail 'cannot resolve release source'
    [[ "$head" == "${RELEASE_SOURCE:?}" ]] || fail 'source commit does not match release intent'
    ensure_clean
    previous="$(version)" || fail 'cannot read previous version'
    [[ "$previous" == "${RELEASE_PREVIOUS:?}" ]] || fail 'previous version does not match release intent'
    perl "$DATA" changelog-check "${RELEASE_VERSION:?}" "${RELEASE_DATE:?}"
    mkdir -p "$CARGO_TARGET_DIR"
    RELEASE_BACKUP_DIR="$(mktemp -d "$CARGO_TARGET_DIR/release-backup.XXXXXX")"
    cp Cargo.toml Cargo.lock CHANGELOG.md "$RELEASE_BACKUP_DIR/"
    RELEASE_HAD_RECEIPT=no
    if [[ -f docs/release.json ]]; then
        cp docs/release.json "$RELEASE_BACKUP_DIR/release.json"
        RELEASE_HAD_RECEIPT=yes
    fi
    # The canonical runner has already persisted exact intent. Roll back a
    # failed bounded metadata transaction, retaining its input/evidence for retry.
    trap 'cp "$RELEASE_BACKUP_DIR/Cargo.toml" Cargo.toml
          cp "$RELEASE_BACKUP_DIR/Cargo.lock" Cargo.lock
          cp "$RELEASE_BACKUP_DIR/CHANGELOG.md" CHANGELOG.md
          if [[ "$RELEASE_HAD_RECEIPT" == yes ]]; then
              cp "$RELEASE_BACKUP_DIR/release.json" docs/release.json
          else
              rm -f docs/release.json
          fi
          echo "Failed preparation restored; inputs retained: $RELEASE_BACKUP_DIR" >&2' EXIT
    cargo metadata --offline --locked --no-deps --format-version 1 > "$RELEASE_BACKUP_DIR/metadata.json"
    perl "$DATA" set-version "$RELEASE_VERSION" "$RELEASE_BACKUP_DIR/metadata.json"
    cargo metadata --offline --locked --no-deps --format-version 1 >/dev/null
    perl "$DATA" finalize "$RELEASE_VERSION" "$RELEASE_DATE"
    make --no-print-directory fmt-check
    perl "$DATA" receipt "$RELEASE_SOURCE" "$RELEASE_DATE"
    perl "$DATA" verify
    trap - EXIT
    rm -rf "$RELEASE_BACKUP_DIR"
}
verify_prepared() {
    local current source head
    perl "$DATA" verify "${RELEASE_SOURCE:?}" "${RELEASE_VERSION:?}" "${RELEASE_DATE:?}"
    current="$(version)" || fail 'cannot read prepared version'
    source="$(perl "$DATA" source)" || fail 'cannot read receipt source'
    head="$(git rev-parse HEAD)" || fail 'cannot resolve release source'
    [[ "$current" == "${RELEASE_VERSION:?}" ]] || fail 'prepared version does not match release intent'
    [[ "$source" == "${RELEASE_SOURCE:?}" ]] || fail 'receipt source does not match release intent'
    [[ "$head" == "$RELEASE_SOURCE" ]] || fail 'source commit does not match release intent'
    allowed_changes "$RELEASE_SOURCE"
    cargo metadata --offline --locked --no-deps --format-version 1 >/dev/null
    make --no-print-directory fmt-check
}
commit_check() {
    verify_prepared
    git diff --quiet -- || fail 'unstaged release edits remain'
    local paths
    paths="$(mktemp "${TMPDIR:-/tmp}/blob-release-index.XXXXXX")"
    git diff --cached --name-only -z -- > "$paths"
    if ! perl "$DATA" index-check "$paths"; then
        rm -f "$paths"
        fail 'index must contain exactly the declared release files'
    fi
    rm -f "$paths"
}
committed_check() {
    ensure_clean
    local commit="${RELEASE_COMMIT:?}" parent
    [[ "$commit" =~ ^[0-9a-f]{40,64}$ ]] || fail 'release commit must be an exact Git identity'
    perl "$DATA" verify-commit "$commit" "${RELEASE_SOURCE:?}" "${RELEASE_VERSION:?}" "${RELEASE_DATE:?}"
    parent="$(git log -1 --format=%P "$commit")" || fail 'cannot read release parent'
    [[ "$parent" == "$RELEASE_SOURCE" ]] || fail 'release parent does not match validated source'
}
verify_tag() {
    bash scripts/ci/check-release-tag.sh "$1" "$2" || fail 'release tag does not match exact release identity'
}
tag_check() {
    ensure_clean
    perl "$DATA" verify
    local current source parent head
    current="$(version)" || fail 'cannot read published version'
    source="$(perl "$DATA" source)" || fail 'cannot read receipt source'
    parent="$(git log -1 --format=%P HEAD)" || fail 'cannot read release parent'
    head="$(git rev-parse HEAD)" || fail 'cannot resolve publication HEAD'
    [[ "$parent" == "$source" ]] || fail 'release parent does not match validated source'
    verify_tag "$head" "$current"
}
publish() {
    case "${1:-}" in ''|--dry-run) ;; *) fail 'expected publish [--dry-run]' ;; esac
    # Separate registry authority still shares the release lock: never publish
    # while another process may be changing this package's release metadata.
    PUBLICATION_STATE_ROOT="$(git rev-parse --git-path release-state)" || fail 'cannot resolve publication lock path'
    [[ ! -L "$PUBLICATION_STATE_ROOT" ]] || fail 'release state directory is symlinked'
    mkdir -p "$PUBLICATION_STATE_ROOT"
    mkdir "$PUBLICATION_STATE_ROOT/lock" 2>/dev/null || fail 'release/publication lock is occupied'
    printf '%s\n' "$$" > "$PUBLICATION_STATE_ROOT/lock/owner"
    trap 'rm -f "$PUBLICATION_STATE_ROOT/lock/owner"; rmdir "$PUBLICATION_STATE_ROOT/lock"' EXIT
    tag_check
    cargo publish --locked --registry crates-io -p ic-blob-storage ${1:+"$1"}
}
command="${1:-}"
shift || true
case "$command" in
    version) version ;;
    plan)
        previous="$(version)" || fail 'cannot read previous version'
        candidate="$(bash scripts/ci/next-release-version.sh "$previous" "${1:-patch}")" || fail 'cannot derive release version'
        printf 'Current: %s\nTarget: %s\nRemote: %s\nBranch: %s\n' "$previous" "$candidate" "${RELEASE_REMOTE:-origin}" "${RELEASE_BRANCH:-main}"
        echo 'Maintainer workflow: preflight, complete validation, prepare, stage, commit/tag, exact atomic push.'
        echo 'Committed unfinished releases reconcile first; newer fixes or a different increment receive fresh validation. Publication and cleanup are separate.'
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
    *) fail 'expected a metadata/check operation; use the common Make release targets for Git effects' ;;
esac
