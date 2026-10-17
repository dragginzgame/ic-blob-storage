#!/usr/bin/env bash
# The checked-in Shared Tooling installer owns executable preparation and admission.
set -euo pipefail

# Explicit native qualification, separate from the offline portable suite.
# Cargo may prepare registry inputs unless CARGO_NET_OFFLINE=true is selected.
# Retain every install, build, receipt and failed attempt in a new owned root.
[[ $# == 1 && ! -e "$1" && ! -L "$1" ]] || {
    echo 'usage: qualify-published-example-install.sh <new-evidence-directory>' >&2; exit 2;
}
ROOT="$0"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"
ROOT="${ROOT%/.}"
selected="$1"
[[ "$selected" == /* ]] || selected="$PWD/$selected"
mkdir "$selected"
fixture="$(cd -P "$selected" && printf '%s/.' "$PWD")"
fixture="${fixture%/.}"
trap 'printf "Cargo installation evidence retained: %s\n" "$fixture" >&2' EXIT
export RUSTUP_AUTO_INSTALL=0
export SHARED_TOOLING_STORE="$fixture/store"
checksum="$ROOT/scripts/ci/verify-file-checksum.sh"
rustc -vV > "$fixture/rustc.txt"
cargo -V > "$fixture/cargo.txt"
cat "$fixture/rustc.txt" "$fixture/cargo.txt"
host="$(sed -n 's/^host: //p' "$fixture/rustc.txt")"
[[ -n "$host" ]] || exit 1
{
    git -C "$ROOT" rev-parse HEAD
    for file in scripts/ci/qualify-published-example-install.sh scripts/dev/install-rust-tools.sh scripts/dev/cargo-tool-build.sh scripts/dev/tool-store.sh scripts/ci/verify-file-checksum.sh .shared-tooling.snapshot; do
        printf '%s  %s\n' "$(bash "$checksum" --print sha256 "$ROOT/$file")" "$file"
    done
} > "$fixture/source.txt"

# Fail only actual compiler dispatch, allowing Cargo's compiler/target discovery.
cat > "$fixture/reject-compile" <<'WRAPPER'
#!/usr/bin/env bash
set -euo pipefail
for argument in "$@"; do
    case "$argument" in --print|--print=*) exec "$@" ;; esac
done
for argument in "$@"; do
    if [[ "$argument" == --crate-name ]]; then
        printf 'compiler dispatch refused\n' >> "$CARGO_QUALIFICATION_REJECTION"
        exit 23
    fi
done
exec "$@"
WRAPPER
chmod +x "$fixture/reject-compile"

install_at() {
    local install_root="$1" build_root="$2"; shift 2
    cargo install "$package" --version "=$version" "--$kind" "$target" \
        --registry crates-io --locked --debug --root "$install_root" \
        --target-dir "$build_root" -j 2 "$@"
}
admit_receipt() {
    # Cargo records --debug as dev on older toolchains and debug on newer ones.
    jq -e --arg identity "$package $version (registry+https://github.com/rust-lang/crates.io-index)" \
        --arg version "=$version" --arg target "$target" --arg host "$host" '
        (.installs | keys) == [$identity] and
        (.installs[$identity] | .version_req == $version and .bins == [$target] and
          (.profile == "dev" or .profile == "debug") and
          .target == $host and (.rustc | type == "string"))
    ' "$1/.crates2.json" > /dev/null || {
        printf 'Cargo receipt does not match the selected package, version, target, debug profile or host: %s\n' "$1/.crates2.json" >&2
        return 1
    }
}
assert_preserved() {
    cmp "$case_root/binary.before" "$case_root/install/bin/$target"
    cmp "$case_root/crates.before.toml" "$case_root/install/.crates.toml"
    cmp "$case_root/crates2.before.json" "$case_root/install/.crates2.json"
}
exercise() {
    package="$1" version="$2" kind="$3" target="$4"
    case_root="$fixture/$target"
    mkdir "$case_root"
    printf 'package=%s\nversion=%s\nkind=%s\ntarget=%s\nprofile=debug\nregistry=crates-io\n' \
        "$package" "$version" "$kind" "$target" > "$case_root/selection.txt"
    install_at "$case_root/install" "$case_root/build" > "$case_root/install.log" 2>&1
    admit_receipt "$case_root/install"
    [[ -f "$case_root/install/bin/$target" && ! -L "$case_root/install/bin/$target" && -x "$case_root/install/bin/$target" ]] || exit 1
    cp "$case_root/install/bin/$target" "$case_root/binary.before"
    cp "$case_root/install/.crates.toml" "$case_root/crates.before.toml"
    cp "$case_root/install/.crates2.json" "$case_root/crates2.before.json"
    digest="$(bash "$checksum" --print sha256 "$case_root/binary.before")"
    printf '%s\n' "$digest" > "$case_root/observed.sha256"
    CARGO_NET_OFFLINE=true install_at "$case_root/install" "$case_root/build" > "$case_root/reuse.log" 2>&1
    admit_receipt "$case_root/install"
    assert_preserved

    if CARGO_NET_OFFLINE=true install_at "$case_root/install" "$case_root/build" \
        "--$kind" shared_tooling_missing_target > "$case_root/missing-target.log" 2>&1; then
        echo 'missing Cargo target was admitted' >&2; exit 1
    fi
    assert_preserved

    # A real failed Cargo compilation must retain the usable installation.
    status=0
    CARGO_NET_OFFLINE=true RUSTC_WRAPPER="$fixture/reject-compile" \
        CARGO_QUALIFICATION_REJECTION="$case_root/compile-rejected.txt" \
        install_at "$case_root/install" "$case_root/failed-build" --force \
        > "$case_root/failed-build.log" 2>&1 || status=$?
    [[ "$status" != 0 && -s "$case_root/compile-rejected.txt" ]] || exit 1
    assert_preserved

    # Concurrent offline requests share Cargo's lock and recheck installation.
    CARGO_NET_OFFLINE=true install_at "$case_root/concurrent" "$case_root/build" > "$case_root/concurrent-1.log" 2>&1 &
    first=$!
    CARGO_NET_OFFLINE=true install_at "$case_root/concurrent" "$case_root/build" > "$case_root/concurrent-2.log" 2>&1 &
    second=$!
    first_status=0 second_status=0
    wait "$first" || first_status=$?
    wait "$second" || second_status=$?
    [[ "$first_status" == 0 && "$second_status" == 0 ]] || exit 1
    admit_receipt "$case_root/concurrent"
    bash "$checksum" sha256 "$digest" "$case_root/concurrent/bin/$target"
    assert_preserved

    # A locally observed digest detects changed bytes; it is not authentication
    # by the publisher, and Cargo's receipt alone is not a byte-integrity check.
    cp "$case_root/binary.before" "$case_root/damaged-binary"
    printf '\nchanged qualification bytes\n' >> "$case_root/damaged-binary"
    if bash "$checksum" sha256 "$digest" "$case_root/damaged-binary" > "$case_root/damaged-check.log" 2>&1; then
        echo 'changed binary was admitted' >&2; exit 1
    fi

    # Exercise the production entry point separately from Cargo's own contract.
    # Include release-profile binary selection; the example retains debug.
    local profile=debug installed
    [[ "$kind" != bin ]] || profile=release
    mkdir "$case_root/consumer"
    local installer_args=(--consumer "$case_root/consumer" --package "$package"
        --version "$version" "--$kind" "$target" --profile "$profile")
    bash "$ROOT/scripts/dev/install-rust-tools.sh" "${installer_args[@]}" > "$case_root/selected-install.txt" 2> "$case_root/selected-install.log"
    installed="$(cat "$case_root/selected-install.txt")"
    [[ -x "$installed" && -f "$installed" && ! -L "$installed" ]] || exit 1
    CARGO_NET_OFFLINE=true bash "$ROOT/scripts/dev/install-rust-tools.sh" "${installer_args[@]}" --check > "$case_root/selected-check.txt"
    CARGO_NET_OFFLINE=true bash "$ROOT/scripts/dev/install-rust-tools.sh" "${installer_args[@]}" > "$case_root/selected-reuse.txt"
    cmp "$case_root/selected-install.txt" "$case_root/selected-check.txt"
    cmp "$case_root/selected-install.txt" "$case_root/selected-reuse.txt"
    printf '\nchanged qualification bytes\n' >> "$installed"
    if CARGO_NET_OFFLINE=true bash "$ROOT/scripts/dev/install-rust-tools.sh" "${installer_args[@]}" --check > "$case_root/selected-corruption.log" 2>&1; then
        echo 'production check admitted changed executable' >&2; exit 1
    fi
    printf 'Native Cargo %s installation qualified: %s %s (%s); observed sha256 %s\n' "$kind" "$package" "$version" "$target" "$digest"
}

# Frozen assessment fixture matching the published example reviewed in #65.
# This is not a default package selection for consuming applications.
exercise ic-blob-storage 0.15.1 example prepare_upload
echo 'Published Blob example installation qualification passed'
