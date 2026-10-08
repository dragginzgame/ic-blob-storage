#!/usr/bin/env bash
set -euo pipefail
contract_graph="$(mktemp "${TMPDIR:-/tmp}/blob-contract-graph.XXXXXX")"
trap 'rm -f "$contract_graph"' EXIT
cargo tree --offline --locked --target all --edges normal --prefix none --format '{p}' \
    -p ic-blob-storage-contracts -p ic-blob-storage-cli > "$contract_graph"
if rg '^(ic-blob-storage |ic-cdk|ic-memory |ic-stable-structures )' "$contract_graph"; then
    echo 'Contracts/native CLI depend on a service runtime owner' >&2
    exit 1
fi
echo 'Contracts and native CLI normal graphs are runtime-free across targets'
