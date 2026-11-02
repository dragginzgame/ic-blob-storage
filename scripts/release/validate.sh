#!/usr/bin/env bash
# Blob selects obligations; Shared admits/checkpoints/executes the complete plan.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/../.."
bash scripts/ci/check-release-source.sh
[[ "$(git rev-parse HEAD)" == "${RELEASE_SOURCE:?}" ]]
case "${RELEASE_GROUP:-}" in contracts|client|engine) ;; *) exit 2 ;; esac
state="$(git rev-parse --git-path release-state)/validation"
[[ ! -L "$state" && ! -L "${state%/*}" ]]
mkdir -p "$state" .tmp
record="$state/$RELEASE_GROUP-$RELEASE_SOURCE.json"
# A failed fresh attempt cannot revive a previous success. Old attempt logs stay.
rm -f "$record"
attempt="$(mktemp -d .tmp/blob-release-validation.XXXXXX)"
printf 'Release validation evidence: %s\n' "$attempt"
perl scripts/release/validation.pl plan "$attempt/plan.json"
jq -r '"Release validation scope: " + .scope, (.stages[] | select(.required == false) | "Not applicable: " + .target + " (" + .reason + ")")' "$attempt/plan.json"
VALIDATION_LOG_DIR="$attempt" bash scripts/ci/run-validation-targets.sh --plan "$attempt/plan.json"
bash scripts/ci/check-release-source.sh
[[ "$(git rev-parse HEAD)" == "$RELEASE_SOURCE" ]]
perl scripts/release/validation.pl capture "$attempt" "$record"
