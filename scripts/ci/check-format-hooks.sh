#!/usr/bin/env bash
set -Eeuo pipefail

# Select this consumer's formatter inputs; shared mechanics have one owner.
ROOT="${BASH_SOURCE[0]}"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"
ROOT="${ROOT%/.}"
TEMPORARY="$(mktemp -d "${TMPDIR:-/tmp}/blob-format-hooks.XXXXXX")"
TEMPORARY="$(cd -P "$TEMPORARY" && printf '%s/.' "$PWD")"
TEMPORARY="${TEMPORARY%/.}"
export TMPDIR="${TEMPORARY%/*}"
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
overlays=(ci/tool-versions.env scripts/ci/check-format-tools.sh scripts/ci/run-formatting.sh scripts/ci/select-rust-formatting-sources.sh scripts/ci/fixture-lifecycle.sh make/consumer.mk make/tools.mk make/release.mk make/rust-format.mk make/execution.mk Cargo.lock)
# Shared prepares the deliberate ordering defect; Blob selects its real inputs.
perl "$ROOT/scripts/ci/prepare-cargo-formatting-fixture.pl" "$ROOT/Cargo.toml" \
    workspace.dependencies ic-cdk ic-host-artifacts > "$TEMPORARY/unsorted-Cargo.toml"
bash "$ROOT/scripts/ci/check-formatting-hooks.sh" "$ROOT" "$selected" Cargo.toml \
    "$TEMPORARY/unsorted-Cargo.toml" --rust-sources "${overlays[@]}"

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
[[ ! -d target ]] || exit 1
# Shared's reviewed hook suite owns Make-mode and failed-tree-observation
# matrices. Keep this actual-consumer rollback smoke alongside roster admission.
printf 'Consumer formatter input, rollback and lock preservation checks passed.\n'
fixture_complete=true
