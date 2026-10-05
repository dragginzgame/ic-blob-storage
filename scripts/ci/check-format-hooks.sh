#!/usr/bin/env bash
set -Eeuo pipefail

# Exercise the reviewed hook against this Rust consumer's actual formatter.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
TEMPORARY="$(mktemp -d "${TMPDIR:-/tmp}/blob-format-hooks.XXXXXX")"
finish() {
    local status=$?
    if [[ "$status" == 0 ]]; then rm -rf "$TEMPORARY";
    else echo "Hook fixture/evidence retained: $TEMPORARY" >&2; fi
}
trap finish EXIT
source_commit="$(git -C "$ROOT" rev-parse HEAD)"
source_objects="$(git -C "$ROOT" rev-parse --git-path objects)"
case "$source_objects" in /*) ;; *) source_objects="$ROOT/$source_objects" ;; esac
mkdir "$TEMPORARY/templates"
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$TEMPORARY/templates"
(cd "$ROOT"; rg --files -g Cargo.toml) > "$TEMPORARY/manifests"
new_fixture() {
    mkdir "$TEMPORARY/$1"
    cd "$TEMPORARY/$1"
    git init --quiet
    mkdir -p .git/objects/info .githooks scripts/dev ci
    printf '%s\n' "$source_objects" > .git/objects/info/alternates
    git update-ref HEAD "$source_commit"
    git read-tree HEAD
    git checkout-index --all
    while IFS= read -r path; do cp "$ROOT/$path" "$path"; git add -- "$path"; done < "$TEMPORARY/manifests"
    for path in Makefile .githooks/pre-commit scripts/dev/install-git-hooks.sh ci/tool-versions.env; do
        cp "$ROOT/$path" "$path"; git add -- "$path"
    done
    cp Cargo.lock "$TEMPORARY/$1.selected-lock"
    selected=crates/ic-blob-storage/src/lib.rs
    printf '\npub fn hook_fixture( ) { }\n' >> "$selected"
    git add -- "$selected"
}
expect_failure() {
    if "$@" > refusal.log 2>&1; then echo 'Hook unexpectedly accepted fixture' >&2; exit 1; fi
}
new_fixture refresh
printf '\nUnrelated working edit.\n' >> README.md
cp README.md before-readme
printf 'untracked bytes\n' > untracked.rs
bash .githooks/pre-commit > "$TEMPORARY/refresh.log" 2>&1
[[ "$(git show ":$selected" | tail -n 1)" == 'pub fn hook_fixture() {}' ]]
cmp README.md before-readme
[[ "$(cat untracked.rs)" == 'untracked bytes' && -z "$(git ls-files -- untracked.rs)" ]]
cmp Cargo.lock "$TEMPORARY/refresh.selected-lock"
tree="$(git write-tree)"
bash .githooks/pre-commit >> "$TEMPORARY/refresh.log" 2>&1
[[ "$(git write-tree)" == "$tree" && ! -d target ]]
new_fixture partial
printf '\n// Keep this unstaged.\n' >> "$selected"
cp "$selected" before-selected
tree="$(git write-tree)"
expect_failure bash .githooks/pre-commit
[[ "$(git write-tree)" == "$tree" ]]
cmp "$selected" before-selected
new_fixture failed-formatter
# A deliberately failing consumer formatter operates only inside the export.
printf '.PHONY: fmt\nfmt:\n\t@echo changed >> crates/ic-blob-storage/src/lib.rs\n\t@false\n' > Makefile
git add Makefile
tree="$(git write-tree)"
cp "$selected" before-selected
expect_failure bash .githooks/pre-commit
[[ "$(git write-tree)" == "$tree" ]]
cmp "$selected" before-selected
cmp Cargo.lock "$TEMPORARY/failed-formatter.selected-lock"
new_fixture installer
bash scripts/dev/install-git-hooks.sh > "$TEMPORARY/install.log" 2>&1
[[ "$(git config --local --get core.hooksPath)" == .githooks ]]
ln -s "$PWD" "$TEMPORARY/alias"
(cd "$TEMPORARY/alias"; bash scripts/dev/install-git-hooks.sh >> "$TEMPORARY/install.log" 2>&1)
git config --local core.hooksPath existing-hooks
expect_failure bash scripts/dev/install-git-hooks.sh
[[ "$(git config --local --get core.hooksPath)" == existing-hooks ]]
printf 'Consumer hook refresh, refusal, formatter failure, lock preservation and installation checks passed.\n'
