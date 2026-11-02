#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-compiler-admission.XXXXXX")"
fixture_complete=false
finish_fixture() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf "$fixture"
    else printf 'Compiler fixture retained: %s\n' "$fixture" >&2; fi
    exit "$status"
}
trap finish_fixture EXIT
mkdir -p "$fixture/consumer/scripts/ci" "$fixture/bin"
cp "$root/scripts/ci/check-rust-compiler.sh" "$fixture/consumer/scripts/ci/"
printf '[toolchain]\nchannel = "1.99.1"\n' > "$fixture/consumer/rust-toolchain.toml"
export COMPILER_FIXTURE="$fixture"
cat > "$fixture/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == -vV ]]; then
    printf 'cargo fixture\nrelease: %s\n' "${TEST_CARGO_VERSION:-1.99.1}"
else
    [[ "$*" == 'metadata --offline --locked --no-deps --format-version 1' ]] || exit 1
    printf 'metadata\n' >> "$COMPILER_FIXTURE/commands"
    printf '{"workspace_members":["contracts","engine"],"packages":[{"id":"contracts","name":"contracts","rust_version":"1.99.1"},{"id":"engine","name":"engine","rust_version":"%s"}]}' "${TEST_MEMBER_VERSION:-1.99.1}"
fi
CARGO
cat > "$fixture/bin/rustc" <<'RUSTC'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == -vV ]] || exit 1
printf 'rustc fixture\nrelease: %s\n' "${TEST_RUSTC_VERSION:-1.99.1}"
RUSTC
chmod +x "$fixture/bin/cargo" "$fixture/bin/rustc"
export PATH="$fixture/bin:$PATH"
unset RUSTC
check=(bash "$fixture/consumer/scripts/ci/check-rust-compiler.sh")
"${check[@]}" > "$fixture/valid.log" 2>&1
[[ "$(cat "$fixture/commands")" == metadata ]] || exit 1
refuse() {
    if "$@" > "$fixture/refusal.log" 2>&1; then echo 'Unexpected compiler admission' >&2; exit 1; fi
}
: > "$fixture/commands"
refuse env TEST_CARGO_VERSION=1.98.0 "${check[@]}"
refuse env TEST_RUSTC_VERSION=1.98.0 "${check[@]}"
[[ ! -s "$fixture/commands" ]] || exit 1
refuse env TEST_MEMBER_VERSION=1.91.0 "${check[@]}"
[[ "$(cat "$fixture/commands")" == metadata ]] || exit 1
: > "$fixture/commands"
printf '[toolchain]\nchannel = "stable"\n' > "$fixture/consumer/rust-toolchain.toml"
refuse "${check[@]}"
printf '[toolchain]\nchannel = "1.99.1"\nchannel = "1.99.1"\n' > "$fixture/consumer/rust-toolchain.toml"
refuse "${check[@]}"
[[ ! -s "$fixture/commands" ]] || exit 1
printf 'Compiler admission rejects mismatched Cargo/rustc/member floors and moving/duplicate pins.\n'
fixture_complete=true
