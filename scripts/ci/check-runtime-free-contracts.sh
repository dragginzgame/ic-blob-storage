#!/usr/bin/env bash
set -euo pipefail
contract_graph="$(mktemp "${TMPDIR:-/tmp}/blob-contract-graph.XXXXXX")"
trap 'rm -f "$contract_graph"' EXIT
cargo tree --offline --locked --target all --edges normal --prefix none --format '{p}' \
    -p ic-blob-storage-contracts -p ic-blob-storage-cli > "$contract_graph"
if rg '^(ic-blob-storage |ic-cdk|ic-memory |ic-storage |ic-stable-structures )' "$contract_graph"; then
    echo 'Contracts/native CLI depend on a service runtime owner' >&2
    exit 1
fi
echo 'Contracts and native CLI normal graphs are runtime-free across targets'

# Replicated callers need Platform execution, but never the installed service or stores.
cargo tree --offline --locked --target all --edges normal --prefix none --format '{p}' \
    -p ic-blob-storage-client > "$contract_graph"
if rg '^(ic-blob-storage |ic-memory |ic-storage |ic-stable-structures )' "$contract_graph"; then
    echo 'Blob client depends on a service storage owner' >&2; exit 1
fi
echo 'Blob client normal graph excludes service storage across targets'

# Optional and native tooling features must not reintroduce the retired runtime.
cargo tree --offline --locked --workspace --all-features --target all \
    --edges normal,build,dev --prefix none --format '{p}' > "$contract_graph"
if rg '^(ic-platform v0\.[12345]\.|(ic-cdk|ic-cdk-macros) v)' "$contract_graph"; then
    echo 'Workspace depends on retired Platform/CDK' >&2; exit 1
fi
echo 'Workspace feature/target selections exclude retired Platform/CDK'

# Qualify the deployment selection independently of native/all-feature builds.
# Native interface inspection never enables a deployed parser or tooling runtime.
cargo tree --offline --locked --target wasm32-unknown-unknown --edges normal,build \
    --prefix none --format '{p}' -p ic-blob-storage-canister > "$contract_graph"
if rg '^((ic-cdk|ic-cdk-macros|ic-testkit|pocket-ic|ic-build|ic-agent|candid_parser) v|ic-platform v0\.[12345]\.|ic-host-)' "$contract_graph"; then
    echo 'Deployment depends on a legacy runtime or native-only owner' >&2
    exit 1
fi
if rg '^(blob-test-protocol|ic-blob-storage-pocketic-tests|blob-[a-z-]+-probe|blob-gateway-source) v' "$contract_graph"; then
    echo 'Deployment depends on private fixture packages' >&2
    exit 1
fi
echo 'Standalone deployment excludes legacy CDK/Platform and native-only owners'

cargo tree --offline --locked --target wasm32-unknown-unknown --edges normal,build \
    --prefix none --format '{p}|{f}' -p ic-blob-storage-canister > "$contract_graph"
if rg '^ic-(wire|platform|blob-storage-contracts) v[^|]*\|([^,]*,)*(typescript|wire-schema)(,| |$)' "$contract_graph"; then
    echo 'Deployment enables host-only Wire schema generation' >&2
    exit 1
fi
echo 'Standalone deployment excludes host-only Wire schema features'

if rg '^ic-(blob-storage|memory|storage) v[^|]*\|([^,]*,)*test-support(,| |$)' "$contract_graph"; then
    echo 'Deployment enables test-support access' >&2
    exit 1
fi
echo 'Standalone deployment excludes test-support access'
