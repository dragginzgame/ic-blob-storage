#!/usr/bin/env bash
# Thin Blob caller; Shared owns baseline observation, execution and evidence admission.
set -euo pipefail
mode="${1:?ci or release}"
[[ "$mode" == ci || "$mode" == release ]]
[[ -z "${RELEASE_GROUP:-}" || "$RELEASE_GROUP" == blob ]]
export RELEASE_SOURCE="${RELEASE_SOURCE:-$(git rev-parse HEAD)}" RELEASE_GROUP=blob
mkdir -p target/validation
attempt="$(mktemp -d target/validation/blob.XXXXXX)"
if [[ "$mode" == release ]]; then
    previous="${RELEASE_PREVIOUS:?release runner must select previous version}"
    [[ "$previous" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]
    perl vendor/toko-tooling/scripts/ci/validation-evidence.pl baseline "$PWD" "$RELEASE_SOURCE" "refs/tags/v$previous" > "$attempt/baseline.json"
else
    printf '%s\n' '{"available":false,"reason":"repository CI"}' > "$attempt/baseline.json"
fi
custom_cargo=false
for variable in CARGO_TARGET_DIR CARGO_BUILD_TARGET CARGO_BUILD_RUSTFLAGS RUSTFLAGS CARGO_ENCODED_RUSTFLAGS RUSTC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER RUSTDOCFLAGS; do
    [[ -z "${!variable+x}" ]] || custom_cargo=true
done
jq -cnS --arg mode "$mode" --arg host "$(uname -sm)" --argjson custom_cargo "$custom_cargo" \
    --arg compiler "$(rustc --version)" --arg cargo "$(cargo --version)" \
    '{mode:$mode,host:$host,compiler:$compiler,cargo:$cargo,custom_cargo:$custom_cargo}' > "$attempt/context.json"
perl scripts/release/select-validation.pl "$mode" "$attempt/baseline.json" "$attempt/context.json" > "$attempt/plan.json"
export BLOB_VALIDATION_PLAN="$attempt/plan.json" VALIDATION_LOG_DIR="$attempt/logs"
jq -r '.stages[] | "\(if .required then "RUN" else "OMIT" end) \(.target): \(.reason)"' "$BLOB_VALIDATION_PLAN"
printf 'Validation plan: %s\n' "$BLOB_VALIDATION_PLAN"
VALIDATION_REPOSITORY_ROOT="$PWD" bash vendor/toko-tooling/scripts/ci/run-validation-targets.sh --plan "$BLOB_VALIDATION_PLAN"
# Exactly one run is created by this invocation; never select an older successful run.
runs=("$attempt"/logs/validation.*)
[[ ${#runs[@]} == 1 && -d "${runs[0]}" ]]
if [[ "$mode" == release ]]; then
    perl vendor/toko-tooling/scripts/ci/validation-evidence.pl baseline "$PWD" "$RELEASE_SOURCE" "refs/tags/v$previous" > "$attempt/baseline-after.json"
    cmp "$attempt/baseline.json" "$attempt/baseline-after.json"
fi
perl vendor/toko-tooling/scripts/ci/validation-evidence.pl verify "$PWD" "$BLOB_VALIDATION_PLAN" \
    "$RELEASE_SOURCE" blob "${runs[0]}" "$attempt/context.json" > "$attempt/proof.json"
printf 'Validation evidence: %s/proof.json\n' "$attempt"
