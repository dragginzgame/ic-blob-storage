#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd -P)"
"${BLOB_BROWSER_NODE:-node}" "$ROOT/scripts/ci/check-node-tools.mjs" \
    "$ROOT/tests/browser" "${BLOB_BROWSER_NPM:-npm}"
