#!/usr/bin/env bash
# Substitute Cargo only: qualify the local assessment adapter, not live registry builds.
set -euo pipefail
ROOT="${BASH_SOURCE[0]}"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"
ROOT="${ROOT%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/cargo-qualification-paths.XXXXXX")"
fixture="$(CDPATH="" cd -P -- "$fixture" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$ROOT/scripts/ci/fixture-lifecycle.sh"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Cargo qualification path fixtures retained"' EXIT
mkdir "$fixture/bin"
export QUALIFICATION_CALLS="$fixture/calls"
cat > "$fixture/bin/rustc" <<'SCRIPT'
#!/usr/bin/env bash
printf 'rustc\n' >> "$QUALIFICATION_CALLS"
printf 'host: x86_64-unknown-linux-gnu\n'
SCRIPT
cat > "$fixture/bin/cargo" <<'SCRIPT'
#!/usr/bin/env bash
printf 'cargo\n' >> "$QUALIFICATION_CALLS"
if [[ "$1" == -V ]]; then echo 'cargo fixture'; exit 0; fi
printf '%s\0' "$@" > "$QUALIFICATION_CALLS.arguments"
if [[ -n "${QUALIFICATION_PROFILE:-}" && ! -e "$QUALIFICATION_CALLS.installed" ]]; then
    while [[ $# -gt 0 ]]; do
        if [[ "$1" == --root ]]; then install_root="$2"; break; fi
        shift
    done
    mkdir -p "$install_root/bin"
    printf '#!/usr/bin/env bash\nexit 0\n' > "$install_root/bin/prepare_upload"
    chmod +x "$install_root/bin/prepare_upload"
    printf '[v1]\n' > "$install_root/.crates.toml"
    jq -n --arg profile "$QUALIFICATION_PROFILE" '{installs: {
      "ic-blob-storage 0.15.1 (registry+https://github.com/rust-lang/crates.io-index)": {
        version_req: "=0.15.1", bins: ["prepare_upload"], profile: $profile,
        target: "x86_64-unknown-linux-gnu", rustc: "rustc fixture"
      }
    }}' > "$install_root/.crates2.json"
    touch "$QUALIFICATION_CALLS.installed"
    exit 0
fi
# Stop at the first install boundary: no Cargo, compilation or network effects.
exit 97
SCRIPT
chmod +x "$fixture/bin/"*
export PATH="$fixture/bin:$PATH"
cd "$fixture"
physical="$(pwd -P)"
for selected in relative 'with spaces' -leading "$fixture/absolute" "$fixture/newline"$'\n'; do
    expected="$physical/${selected##*/}"
    status=0
    CDPATH="$fixture" bash "$ROOT/scripts/ci/qualify-published-example-install.sh" "$selected" > "$fixture/run.log" 2>&1 || status=$?
    # Bash 3.2 writes the EXIT retention message inside the failed function's
    # redirected log; newer Bash writes it to run.log. Either retains evidence.
    [[ "$status" == 97 && -s "$expected/rustc.txt" && -s "$expected/source.txt" &&
       -d "$expected/prepare_upload" && -f "$expected/prepare_upload/install.log" ]] || exit 1
    found=false
    while IFS= read -r -d '' argument; do
        [[ "$argument" != "$expected/prepare_upload/install" ]] || found=true
    done < "$QUALIFICATION_CALLS.arguments"
    [[ "$found" == true ]] || exit 1
    # Retrying must refuse the owned evidence root before even probing tools.
    : > "$QUALIFICATION_CALLS"
    status=0
    CDPATH="$fixture" bash "$ROOT/scripts/ci/qualify-published-example-install.sh" "$selected" > "$fixture/refusal.log" 2>&1 || status=$?
    [[ "$status" == 2 && ! -s "$QUALIFICATION_CALLS" && -s "$expected/source.txt" ]] || exit 1
done
ln -s missing "$fixture/dangling"
status=0
bash "$ROOT/scripts/ci/qualify-published-example-install.sh" "$fixture/dangling" > "$fixture/refusal.log" 2>&1 || status=$?
[[ "$status" == 2 && ! -s "$QUALIFICATION_CALLS" && -L "$fixture/dangling" ]] || exit 1

# Observed Cargo --debug receipts use dev or debug; release remains a mismatch.
for profile in dev debug release; do
    rm -f "$QUALIFICATION_CALLS.installed"
    status=0
    QUALIFICATION_PROFILE="$profile" bash "$ROOT/scripts/ci/qualify-published-example-install.sh" "$fixture/$profile" > "$fixture/profile.log" 2>&1 || status=$?
    if [[ "$profile" == release ]]; then
        [[ "$status" == 1 && ! -e "$fixture/$profile/prepare_upload/binary.before" ]] || exit 1
        grep -F 'Cargo receipt does not match' "$fixture/profile.log" > /dev/null
    else
        [[ "$status" == 97 && -s "$fixture/$profile/prepare_upload/observed.sha256" ]] || exit 1
    fi
done
echo 'Cargo qualification paths, receipt profiles, retained failure roots and early refusal passed (substitute Cargo)'
fixture_complete=true
