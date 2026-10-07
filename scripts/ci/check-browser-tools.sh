#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd -P)"
cd "$ROOT"
"${BLOB_BROWSER_NODE:-node}" scripts/ci/check-browser-tools.mjs
expected_npm="$("${BLOB_BROWSER_NODE:-node}" -p 'require("./tests/browser/package.json").packageManager')"
actual_npm="npm@$("${BLOB_BROWSER_NPM:-npm}" --version)"
[[ "$actual_npm" == "$expected_npm" ]] || {
    printf 'Select %s from tests/browser/package.json (found %s).\n' "$expected_npm" "$actual_npm" >&2
    exit 1
}
printf 'Browser npm verified: %s.\n' "$actual_npm"
