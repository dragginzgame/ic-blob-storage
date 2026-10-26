#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd -P)"
"${BLOB_BROWSER_NODE:-node}" "$ROOT/scripts/ci/check-node-tools.mjs" \
    "$ROOT/tests/browser" "${BLOB_BROWSER_NPM:-npm}"
"${BLOB_BROWSER_NODE:-node}" "$ROOT/scripts/ci/check-browser-tools.mjs"
"${BLOB_BROWSER_NODE:-node}" --test "$ROOT/tests/browser/dependency-boundary.test.mjs"
