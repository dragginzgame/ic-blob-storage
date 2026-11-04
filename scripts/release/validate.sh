#!/usr/bin/env bash
# Blob selects obligations; Shared admits/checkpoints/executes the complete plan.
# Shared companions: scripts/ci/check-retained-state.pl
set -euo pipefail
unset BLOB_RELEASE_VALIDATION_PLAN
cd "$(dirname "${BASH_SOURCE[0]}")/../.."
bash scripts/ci/check-release-source.sh
[[ "$(git rev-parse HEAD)" == "${RELEASE_SOURCE:?}" ]]
case "${RELEASE_GROUP:-}" in contracts|client|engine) ;; *) exit 2 ;; esac
state="$(git rev-parse --git-path release-state)/validation"
[[ ! -L "$state" && ! -L "${state%/*}" ]]
perl scripts/ci/check-retained-state.pl --root "${state%/*}"
record="$state/$RELEASE_GROUP-$RELEASE_SOURCE.json"
status=0
trap 'status=$?; if ! perl scripts/ci/check-retained-state.pl --root "${state%/*}"; then rm -f "$record"; [[ "$status" != 0 ]] || status=1; fi; exit "$status"' EXIT
mkdir -p "$state" target
# A failed fresh attempt cannot revive a previous success. Old attempt logs stay.
rm -f "$record"
attempt="$(mktemp -d target/blob-release-validation.XXXXXX)"
printf 'Release validation evidence: %s\n' "$attempt"
perl scripts/release/validation.pl plan "$attempt/plan.json"
export BLOB_RELEASE_VALIDATION_PLAN="$attempt/plan.json"
jq -r '"Release validation scope: " + .scope, (.stages[] | select(.required == false) | "Not applicable: " + .target + " (" + .reason + ")")' "$attempt/plan.json"
VALIDATION_LOG_DIR="$attempt" bash scripts/ci/run-validation-targets.sh --plan "$attempt/plan.json"
bash scripts/ci/check-release-source.sh
[[ "$(git rev-parse HEAD)" == "$RELEASE_SOURCE" ]]
# shellcheck source=scripts/ci/release-receipts.sh
source scripts/ci/release-receipts.sh
release_receipt_write "$record" perl scripts/release/validation.pl capture "$attempt"
