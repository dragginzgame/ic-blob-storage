#!/usr/bin/env bash
set -euo pipefail
# Exercise changed same-version archives against a warm target with real Cargo.
verifier="$PWD/scripts/ci/verify-library-packages.sh"
fixture_parent="${CARGO_TARGET_DIR:-$PWD/target}"
mkdir -p "$fixture_parent"
fixture="$(mktemp -d "$fixture_parent/package-fixture.XXXXXX")"
echo "Package regression retained: $fixture"
fixture_complete=false
finish_fixture() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" != 0 ]]; then
        printf 'Package regression failed; fixture retained: %s\n' "$fixture" >&2
    fi
    exit "$status"
}
trap finish_fixture EXIT
mkdir -p "$fixture/contracts/src" "$fixture/service/src"
cd "$fixture"
git init -q
export CARGO_TARGET_DIR="$fixture/target"
cat > Cargo.toml <<'TOML'
[workspace]
resolver = "3"
members = ["contracts", "service"]
[workspace.package]
version = "0.17.2"
TOML
cat > contracts/Cargo.toml <<'TOML'
[package]
name = "ic-blob-storage-contracts"
version.workspace = true
edition = "2024"
[dependencies]
cfg-if = "=1.0.4"
TOML
cat > service/Cargo.toml <<'TOML'
[package]
name = "ic-blob-storage"
version.workspace = true
edition = "2024"
[dependencies]
ic-blob-storage-contracts = { path = "../contracts", version = "0.17.2" }
[[bin]]
name = "package-value"
path = "src/main.rs"
TOML
printf 'pub fn prior() -> u32 { 1 }\n' > contracts/src/lib.rs
printf 'pub fn value() -> u32 { ic_blob_storage_contracts::prior() }\n' > service/src/lib.rs
printf 'fn main() { println!("{}", ic_blob_storage::value()); }\n' > service/src/main.rs
cargo generate-lockfile --offline > lock.log 2>&1
bash "$verifier" > first.log 2>&1
first_metadata=""
for candidate in "$CARGO_TARGET_DIR"/package/blob-verification.*/metadata.json; do first_metadata="$candidate"; done
# Deliberately seed the shared target with the first extracted contract and caller.
first_manifest="$(dirname "$first_metadata")/Cargo.toml"
value="$(cargo run --offline --locked --manifest-path "$first_manifest" -p ic-blob-storage --bin package-value 2> warm.log)"
[[ "$value" == 1 ]] || exit 1
printf 'pub fn current() -> u32 { 2 }\n' > contracts/src/lib.rs
printf 'pub fn value() -> u32 { ic_blob_storage_contracts::current() }\n' > service/src/lib.rs
bash "$verifier" > second.log 2>&1
second_metadata=""
for candidate in "$CARGO_TARGET_DIR"/package/blob-verification.*/metadata.json; do
    [[ "$candidate" == "$first_metadata" ]] || second_metadata="$candidate"
done
second_manifest="$(dirname "$second_metadata")/Cargo.toml"
# Execute the new payload using the target actually reported by its verifier.
verified_target="$(perl -MJSON::PP -0777 -ne 'print decode_json($_)->{target_directory}' "$second_metadata")"
value="$(CARGO_TARGET_DIR="$verified_target" cargo run --offline --locked --manifest-path "$second_manifest" -p ic-blob-storage --bin package-value 2> current.log)"
if [[ "$value" != 2 ]]; then
    echo "New archive executed stale contract value: $value" >&2
    exit 1
fi
echo 'Same-version archive regression passed (warm shared target; current contract executed).'
real_cargo="$(command -v cargo)"
# Alter actual Cargo observations or post-build bytes in independent consumers.
# Admission must stop before tests; final drift must retain a failed attempt.
for failure in path source version member metadata-exit archive unpacked manifest lock; do
    case_root="$fixture/refusal-$failure with spaces π"
    mkdir -p "$case_root/bin"
    cp Cargo.toml Cargo.lock "$case_root/"
    cp -R contracts service "$case_root/"
    cat > "$case_root/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == metadata && " $* " != *' --no-deps '* ]]; then
    "$PACKAGE_REAL_CARGO" "$@" > "$PACKAGE_CASE_ROOT/observed.json"
    PACKAGE_CASE_ROOT="$PACKAGE_CASE_ROOT" perl -MJSON::PP -0777 -e '
        my $metadata = decode_json(<>);
        for my $package (@{$metadata->{packages}}) {
            next unless $package->{name} eq "ic-blob-storage-contracts";
            $package->{manifest_path} = "$ENV{PACKAGE_CASE_ROOT}/contracts/Cargo.toml"
                if $ENV{PACKAGE_FAILURE} eq "path";
            $package->{source} = "registry+https://github.com/rust-lang/crates.io-index"
                if $ENV{PACKAGE_FAILURE} eq "source";
            $package->{version} = "0.17.1" if $ENV{PACKAGE_FAILURE} eq "version";
        }
        push @{$metadata->{workspace_members}}, $metadata->{workspace_members}[0]
            if $ENV{PACKAGE_FAILURE} eq "member";
        print encode_json($metadata);
    ' "$PACKAGE_CASE_ROOT/observed.json"
    [[ "$PACKAGE_FAILURE" != metadata-exit ]] || exit 23
    exit 0
fi
if [[ "$1" == test ]]; then printf 'test\n' >> "$PACKAGE_CASE_ROOT/effects"; fi
"$PACKAGE_REAL_CARGO" "$@"
if [[ "$1" == test && ! -e "$PACKAGE_CASE_ROOT/injected" ]]; then
    touch "$PACKAGE_CASE_ROOT/injected"
    case "$PACKAGE_FAILURE" in
        archive) printf '\n' >> "$PACKAGE_CASE_ROOT/target/package/ic-blob-storage-contracts-0.17.2.crate" ;;
        unpacked)
            for source in "$PACKAGE_CASE_ROOT"/target/package/blob-verification.*/ic-blob-storage-contracts-0.17.2/src/lib.rs; do
                printf '\n// injected source drift\n' >> "$source"
            done ;;
        manifest) printf '\n' >> "$PACKAGE_CASE_ROOT/Cargo.toml" ;;
        lock) printf '\n' >> "$PACKAGE_CASE_ROOT/Cargo.lock" ;;
    esac
fi
CARGO
    chmod +x "$case_root/bin/cargo"
    status=0
    (cd "$case_root"; PATH="$case_root/bin:$PATH" CARGO_TARGET_DIR="$case_root/target" \
        PACKAGE_REAL_CARGO="$real_cargo" PACKAGE_CASE_ROOT="$case_root" PACKAGE_FAILURE="$failure" \
        bash "$verifier") > "$case_root/result.log" 2>&1 || status=$?
    [[ "$status" != 0 ]] || exit 1
    [[ "$failure" != metadata-exit || "$status" == 23 ]] || exit 1
    retained="$(sed -n 's/^Package verification failed; evidence retained: //p' "$case_root/result.log")"
    [[ -f "$retained/metadata.json" && -f "$retained/entry-Cargo.lock" ]] || exit 1
    if grep -F 'Coordinated archive graph, tests and final integrity checks passed.' "$case_root/result.log" >/dev/null; then exit 1; fi
    case "$failure" in
        path|source|version)
            [[ ! -e "$case_root/effects" ]] || exit 1
            grep -F 'resolved package is not the exact extracted payload: ic-blob-storage-contracts' "$case_root/result.log" >/dev/null ;;
        metadata-exit) [[ ! -e "$case_root/effects" ]] || exit 1 ;;
        member)
            [[ ! -e "$case_root/effects" ]] || exit 1
            grep -F 'unexpected extracted workspace member' "$case_root/result.log" >/dev/null ;;
        *) [[ "$(wc -l < "$case_root/effects" | tr -d ' ')" == 2 ]] || exit 1 ;;
    esac
done
echo 'Archive graph admission and final byte/source/manifest/lock drift refusals passed.'
fixture_complete=true
