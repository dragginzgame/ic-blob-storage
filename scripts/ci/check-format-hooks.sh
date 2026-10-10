#!/usr/bin/env bash
set -Eeuo pipefail

# Select this consumer's formatter inputs; shared mechanics have one owner.
ROOT="${BASH_SOURCE[0]}"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"
ROOT="${ROOT%/.}"
TEMPORARY="$(mktemp -d "${TMPDIR:-/tmp}/blob-format-hooks.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$fixture_complete" == true && "$status" == 0 ]]; then rm -rf "$TEMPORARY";
    else echo "Hook fixture/evidence retained: $TEMPORARY" >&2; fi
    exit "$status"
}
trap finish EXIT
selected=crates/ic-blob-storage/src/lib.rs
overlays=(ci/tool-versions.env scripts/ci/check-format-tools.sh scripts/ci/run-formatting.sh make/tools.mk make/release.mk make/rust-format.mk make/execution.mk Cargo.lock)
# Copy formatter inputs from every actual member, including working-tree packages
# absent from HEAD. Cargo owns the roster across standard and approved layouts.
cargo metadata --offline --locked --no-deps --format-version 1 \
    --manifest-path "$ROOT/Cargo.toml" > "$TEMPORARY/metadata.json"
jq -r --arg root "$ROOT/" '. as $m | .packages[] | select(.id as $id | $m.workspace_members | index($id)) | .manifest_path | if startswith($root) then ltrimstr($root) else error("workspace manifest outside checkout") end' \
    "$TEMPORARY/metadata.json" > "$TEMPORARY/manifests"
while IFS= read -r manifest; do
    overlays+=("$manifest")
    (cd "$ROOT" && rg --files "${manifest%/*}" -g '*.rs') > "$TEMPORARY/member-rust"
    while IFS= read -r path; do overlays+=("$path"); done < "$TEMPORARY/member-rust"
done < "$TEMPORARY/manifests"
# Swap two adjacent dependency declarations without changing their values.
perl -0777 -pe 's/^(ic-cdk = [^\n]*\n)(ic-host-artifacts = [^\n]*\n)/$2$1/m or die "cannot prepare ordering-only manifest\n"' \
    "$ROOT/Cargo.toml" > "$TEMPORARY/unsorted-Cargo.toml"
bash "$ROOT/scripts/ci/check-formatting-hooks.sh" "$ROOT" "$selected" Cargo.toml \
    "$TEMPORARY/unsorted-Cargo.toml" "${overlays[@]}"

# Keep the additional consumer regression: a formatter changes selected bytes
# before failing. The hook must restore those bytes and leave the index intact.
source_commit="$(git -C "$ROOT" rev-parse HEAD)"
source_objects="$(git -C "$ROOT" rev-parse --git-path objects && printf '.')"
source_objects="${source_objects%$'\n.'}"
case "$source_objects" in /*) ;; *) source_objects="$ROOT/$source_objects" ;; esac
mkdir "$TEMPORARY/templates" "$TEMPORARY/failed-formatter"
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null GIT_TEMPLATE_DIR="$TEMPORARY/templates"
cd "$TEMPORARY/failed-formatter"
git init --quiet
perl -e '$p = $ARGV[0]; $p =~ s/([\\"])/\\$1/g; $p =~ s/\n/\\n/g;
    $p =~ s/\r/\\r/g; $p =~ s/\t/\\t/g; print "\"$p\"\n"' \
    "$source_objects" > .git/objects/info/alternates
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

# Matching tree output must not hide a failed Git observation. Exercise the
# selected canonical hook against this consumer's real index and staged inputs.
cat > Makefile <<'MAKE'
.PHONY: fmt
fmt:
	@touch "$$BLOB_HOOK_FIXTURE_ATTEMPT"
	@echo changed >> crates/ic-blob-storage/src/lib.rs
MAKE
git add Makefile
tree="$(git write-tree)"
cp .git/index before-index
printf '\nUnrelated working edit.\n' >> README.md
cp README.md before-readme
real_git="$(command -v git)"
mkdir mock-bin
cat > mock-bin/git <<'GIT'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$BLOB_HOOK_STATE/commands"
if [[ "$*" == write-tree ]]; then
    count=0
    [[ ! -f "$BLOB_HOOK_STATE/count" ]] || read -r count < "$BLOB_HOOK_STATE/count"
    count=$((count + 1))
    printf '%s\n' "$count" > "$BLOB_HOOK_STATE/count"
    "$BLOB_HOOK_REAL_GIT" "$@"
    if [[ "$count" == "$BLOB_HOOK_OBSERVATION" ]]; then
        echo 'injected tree observation failure' >&2
        exit 23
    fi
    exit 0
fi
exec "$BLOB_HOOK_REAL_GIT" "$@"
GIT
chmod +x mock-bin/git
for observation in 1 2 3; do
    rm -f mock-bin/count mock-bin/commands formatter-attempted
    status=0
    PATH="$PWD/mock-bin:$PATH" BLOB_HOOK_REAL_GIT="$real_git" \
        BLOB_HOOK_STATE="$PWD/mock-bin" BLOB_HOOK_OBSERVATION="$observation" \
        bash .githooks/pre-commit > "tree-$observation.log" 2>&1 || status=$?
    [[ "$status" == 23 && "$(tail -1 mock-bin/commands)" == write-tree ]] || exit 1
    grep -F 'injected tree observation failure' "tree-$observation.log" >/dev/null
    [[ "$(git write-tree)" == "$tree" ]] || exit 1
    cmp .git/index before-index
    cmp "$selected" before-selected
    cmp Cargo.lock before-lock
    cmp README.md before-readme
    if [[ "$observation" == 3 ]]; then [[ -f formatter-attempted ]] || exit 1;
    else [[ ! -e formatter-attempted ]] || exit 1; fi
done
printf 'Consumer formatter rollback, Make-mode/tree-observation refusal and lock preservation checks passed.\n'
fixture_complete=true
