#!/usr/bin/env bash
set -Eeuo pipefail

# Select this consumer's formatter inputs; shared mechanics have one owner.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
TEMPORARY="$(mktemp -d "${TMPDIR:-/tmp}/blob-format-hooks.XXXXXX")"
finish() {
    local status=$?
    if [[ "$status" == 0 ]]; then rm -rf "$TEMPORARY";
    else echo "Hook fixture/evidence retained: $TEMPORARY" >&2; fi
}
trap finish EXIT
selected=crates/ic-blob-storage/src/lib.rs
(cd "$ROOT"; rg --files -g Cargo.toml) > "$TEMPORARY/manifests"
overlays=(ci/tool-versions.env Cargo.lock)
while IFS= read -r path; do overlays+=("$path"); done < "$TEMPORARY/manifests"
# Swap two adjacent dependency declarations without changing their values.
perl -0777 -pe 's/^(ic-cdk = [^\n]*\n)(ic-host-tools = [^\n]*\n)/$2$1/m or die "cannot prepare ordering-only manifest\n"' \
    "$ROOT/Cargo.toml" > "$TEMPORARY/unsorted-Cargo.toml"
bash "$ROOT/scripts/ci/check-formatting-hooks.sh" "$ROOT" "$selected" Cargo.toml \
    "$TEMPORARY/unsorted-Cargo.toml" "${overlays[@]}"

# Keep the additional consumer regression: a formatter changes selected bytes
# before failing. The hook must restore those bytes and leave the index intact.
source_commit="$(git -C "$ROOT" rev-parse HEAD)"
source_objects="$(git -C "$ROOT" rev-parse --git-path objects)"
case "$source_objects" in /*) ;; *) source_objects="$ROOT/$source_objects" ;; esac
mkdir "$TEMPORARY/templates" "$TEMPORARY/failed-formatter"
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$TEMPORARY/templates"
cd "$TEMPORARY/failed-formatter"
git init --quiet
printf '%s\n' "$source_objects" > .git/objects/info/alternates
git update-ref HEAD "$source_commit"
git read-tree HEAD
git checkout-index --all
for path in "$selected" Makefile .githooks/pre-commit scripts/dev/install-git-hooks.sh "${overlays[@]}"; do
    cp -p "$ROOT/$path" "$path"
    git --literal-pathspecs add -- "$path"
done
printf '\npub fn hook_fixture( ) { }\n' >> "$selected"
git --literal-pathspecs add -- "$selected"
printf '.PHONY: fmt\nfmt:\n\t@echo changed >> crates/ic-blob-storage/src/lib.rs\n\t@false\n' > Makefile
git add Makefile
tree="$(git write-tree)"
cp "$selected" before-selected
cp Cargo.lock before-lock
if bash .githooks/pre-commit > refusal.log 2>&1; then
    echo 'Hook unexpectedly accepted a mutating failed formatter' >&2; exit 1
fi
[[ "$(git write-tree)" == "$tree" ]] || exit 1
cmp "$selected" before-selected
cmp Cargo.lock before-lock
[[ ! -d target ]] || exit 1
printf 'Consumer mutating-formatter rollback and lock preservation checks passed.\n'
