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
overlays=(ci/tool-versions.env scripts/ci/check-format-tools.sh Cargo.lock)
# Copy formatter inputs from every actual member, including working-tree packages
# absent from HEAD. Cargo owns the roster across standard and approved layouts.
cargo metadata --offline --locked --no-deps --format-version 1 \
    --manifest-path "$ROOT/Cargo.toml" > "$TEMPORARY/metadata.json"
jq -r '. as $m | .packages[] | select(.id as $id | $m.workspace_members | index($id)) | .manifest_path' \
    "$TEMPORARY/metadata.json" > "$TEMPORARY/manifests"
while IFS= read -r manifest; do
    overlays+=("${manifest#"$ROOT/"}")
    rg --files "$(dirname "$manifest")" -g '*.rs' > "$TEMPORARY/member-rust"
    while IFS= read -r path; do overlays+=("${path#"$ROOT/"}"); done < "$TEMPORARY/member-rust"
done < "$TEMPORARY/manifests"
# Swap two adjacent dependency declarations without changing their values.
perl -0777 -pe 's/^(ic-cdk = [^\n]*\n)(ic-host-artifacts = [^\n]*\n)/$2$1/m or die "cannot prepare ordering-only manifest\n"' \
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
for path in "$selected" Makefile .githooks/pre-commit scripts/dev/install-git-hooks.sh scripts/ci/check-make-execution.sh "${overlays[@]}"; do
    mkdir -p "$(dirname "$path")"
    cp -p "$ROOT/$path" "$path"
    git --literal-pathspecs add -- "$path"
done
printf '\npub fn hook_fixture( ) { }\n' >> "$selected"
git --literal-pathspecs add -- "$selected"
# The hook formats an isolated index snapshot. Bind this fixture-only observation
# to the outer fixture so it survives that snapshot's cleanup after refusal.
export BLOB_HOOK_FIXTURE_ATTEMPT="$PWD/formatter-attempted"
cat > Makefile <<'MAKE'
.PHONY: fmt
fmt:
	@touch "$$BLOB_HOOK_FIXTURE_ATTEMPT"
	@echo changed >> crates/ic-blob-storage/src/lib.rs
	@false
MAKE
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
[[ -f formatter-attempted ]] || exit 1
rm formatter-attempted
for flags in i n q t v; do
    if MAKEFLAGS="$flags" bash .githooks/pre-commit > "mode-$flags.log" 2>&1; then
        echo "Hook accepted a Make mode without reliable formatting: $flags" >&2; exit 1
    fi
    [[ ! -e formatter-attempted && "$(git write-tree)" == "$tree" ]] || exit 1
    cmp "$selected" before-selected
    cmp Cargo.lock before-lock
done
[[ ! -d target ]] || exit 1
printf 'Consumer formatter rollback, Make-mode refusal and lock preservation checks passed.\n'
