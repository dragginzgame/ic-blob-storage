#!/usr/bin/env bash
set -euo pipefail

# Bounded offline orchestration: existing owners build, authenticate and test.
# Never replace Cargo output or alter the product's unoptimized build policy.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
cd "$root"
bin="$(bash scripts/dev/install-ic-tools.sh --check)"
report="$(mktemp -d "$CARGO_TARGET_DIR/blob-wasm-opt.XXXXXX")"
printf 'Blob Wasm optimizer evidence retained: %s\n' "$report"
complete=false
finish() {
    local status=$?
    [[ "$complete" == true || "$status" != 0 ]] || status=1
    exit "$status"
}
trap finish EXIT
mkdir "$report/original"
cp "$BLOB_STANDALONE_WASM" "$report/original/standalone.wasm"
cp "$BLOB_STORAGE_PROBE_WASM" "$report/original/storage.wasm"
git rev-parse HEAD > "$report/source.txt"
git diff --binary HEAD > "$report/source.diff"
cp Cargo.lock "$report/Cargo.lock"
cp scripts/ci/qualify-blob-wasm-opt.sh "$report/qualify-blob-wasm-opt.sh"
cp ci/ic-tools.tsv "$report/pins.tsv"
"$bin/wasm-opt" --version > "$report/optimizer-version.txt"
bash scripts/ci/verify-file-checksum.sh --print sha256 "$bin/wasm-opt" > "$report/optimizer.sha256"
for optimization in original O3 Os Oz; do
    if [[ "$optimization" != original ]]; then
        mkdir "$report/$optimization"
        for module in standalone storage; do
            "$bin/wasm-opt" "$report/original/$module.wasm" --enable-bulk-memory-opt "-$optimization" \
                -o "$report/$optimization/$module.wasm" > "$report/$optimization/$module.optimize.log" 2>&1
        done
    fi
    export BLOB_STANDALONE_WASM="$report/$optimization/standalone.wasm"
    export BLOB_STORAGE_PROBE_WASM="$report/$optimization/storage.wasm"
    (
        bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone \
            standalone_installation_cli::standalone_installs_cli_carrier_and_independently_rejects_wrong_actual_service -- --exact --test-threads=1
        bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test standalone \
            standalone_admission_manifest_and_restore_use_shared_authority -- --exact --test-threads=1
        bash scripts/ci/run-pocketic-test.sh --offline --locked -p ic-blob-storage-pocketic-tests --test storage \
            storage_resources::reads::restoration_reads_are_attributed_only_to_the_operator_reopen_window -- --exact --test-threads=1
    ) > "$report/$optimization/tests.log" 2>&1
    printf 'Blob Wasm %s installation/restoration passed\n' "$optimization"
done
cmp Cargo.lock "$report/Cargo.lock"
find "$report" -name '*.wasm' -type f -exec shasum -a 256 '{}' \; > "$report/modules.sha256"
printf 'Local PocketIC only; no deployed provider or Canic qualification.\n' > "$report/completed.txt"
complete=true
