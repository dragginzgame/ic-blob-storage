#!/usr/bin/env bash
set -euo pipefail
mode="${1:-full}"
[[ $# -le 1 && ( "$mode" == full || "$mode" == --metrics-only ) ]] || exit 2
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-feature-boundary.XXXXXX")"
trap 'rm -rf "$fixture"' EXIT
cat > "$fixture/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *'{p}|{f}'* ]]; then
    printf '%s\n' "${FIXTURE_FEATURE_LINE:-}"
elif [[ "$*" == *'ic-blob-storage-contracts'* ]]; then
    printf '%s\n' "${FIXTURE_CONTRACTS_LINE:-}"
elif [[ "$*" == *'wasm32-unknown-unknown'* && "$*" == *'ic-blob-storage-client'* ]]; then
    printf '%s\n' "${FIXTURE_NATIVE_CLIENT_LINE:-}"
elif [[ "$*" == *'ic-blob-storage-client'* ]]; then
    printf '%s\n' "${FIXTURE_DEFAULT_CLIENT_LINE:-}"
elif [[ "$*" == *'wasm32-unknown-unknown'* && "$*" == *'ic-blob-storage-canister'* ]]; then
    printf '%s\n' "${FIXTURE_PACKAGE_LINE:-}"
fi
CARGO
chmod +x "$fixture/cargo"
export PATH="$fixture:$PATH"
for features in '' default native-wire 'default,native-wire' 'native-wire,recording'; do
    export FIXTURE_FEATURE_LINE="ic-metrics v1.0.0|$features (*)"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Contracts accepted instrumentation recorders: $features" >&2; exit 1
    fi
    rg -q 'Contracts schema depends on instrumentation recorders' "$fixture/output"
done
unset FIXTURE_FEATURE_LINE
printf 'Contracts instrumentation dependency refusals: PASS\n'
[[ "$mode" != --metrics-only ]] || exit 0
for package in ic-platform candid candid_derive ic-agent ic-wire-derive; do
    export FIXTURE_CONTRACTS_LINE="$package v1.0.0"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Passive Contracts accepted $package" >&2; exit 1
    fi
    rg -q 'Passive Contracts depend on Candid, facade, execution or schema derive' "$fixture/output"
done
unset FIXTURE_CONTRACTS_LINE
printf 'Passive Contracts Candid/facade refusals: PASS\n'
for package in ic-blob-storage ic-memory ic-storage; do
    for features in test-support 'default,test-support' 'test-support,other'; do
        export FIXTURE_FEATURE_LINE="$package v1.0.0|$features (*)"
        if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
            echo "Deployment accepted $package/$features" >&2; exit 1
        fi
        rg -q 'Deployment enables test-support access' "$fixture/output"
    done
    export FIXTURE_FEATURE_LINE="$package v1.0.0|other-test-support"
    bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1
done
printf 'Deployment fixture feature refusals: PASS\n'
unset FIXTURE_FEATURE_LINE
for package in ic-metrics ic-memory ic-stable-structures ic-wire-derive clap clap_builder clap_derive ic0 ic-cdk-executor ic-management-canister-types ic-platform-pocket-ic; do
    export FIXTURE_PACKAGE_LINE="$package v4.6.7"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Deployment accepted forbidden package $package" >&2; exit 1
    fi
    rg -q 'Deployment depends on a legacy runtime or native-only owner' "$fixture/output"
done
unset FIXTURE_PACKAGE_LINE
printf 'Deployment CLI and retired runtime refusals: PASS\n'
for package in blob-test-protocol ic-blob-storage-pocketic-tests blob-authority-probe blob-admission-probe blob-storage-probe blob-consumer-probe blob-funding-probe blob-gateway-source; do
    export FIXTURE_PACKAGE_LINE="$package v1.0.0"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Deployment accepted $package" >&2; exit 1
    fi
    rg -q 'Deployment depends on private fixture packages' "$fixture/output"
done
printf 'Deployment private package refusals: PASS\n'

unset FIXTURE_PACKAGE_LINE
for features in provider-json 'default,provider-json' 'provider-json,other'; do
    export FIXTURE_FEATURE_LINE="ic-blob-storage-contracts v1.0.0|$features (*)"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Deployment accepted provider JSON tooling: $features" >&2; exit 1
    fi
    rg -q 'Deployment enables host-only schema or provider JSON tooling' "$fixture/output"
done
export FIXTURE_FEATURE_LINE='ic-blob-storage-contracts v1.0.0|other-provider-json'
bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1
printf 'Deployment provider JSON feature refusals: PASS\n'
for features in platform-types 'platform-types,typescript' 'default,platform-types'; do
    export FIXTURE_FEATURE_LINE="ic-wire v0.10.0|$features (*)"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/result" 2>&1; then
        echo "Deployment accepted Platform identity schema tooling: $features" >&2; exit 1
    fi
    rg -F 'Deployment enables host-only schema or provider JSON tooling' "$fixture/result" >/dev/null
done
printf 'Deployment Platform identity schema feature refusals: PASS\n'
for features in candid-service 'candid,candid-service' 'candid-service,stable-structures'; do
    export FIXTURE_FEATURE_LINE="ic-platform v0.9.0|$features (*)"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Deployment accepted native Candid service export: $features" >&2; exit 1
    fi
    rg -q 'Deployment enables host-only schema or provider JSON tooling' "$fixture/output"
done
export FIXTURE_FEATURE_LINE='ic-platform v0.9.0|candid,management-canister-types'
bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1
printf 'Deployment native Candid service feature refusals: PASS\n'

unset FIXTURE_FEATURE_LINE FIXTURE_PACKAGE_LINE
for package in reqwest tokio ic-host-fs; do
    export FIXTURE_NATIVE_CLIENT_LINE="$package v1.0.0"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Wasm client accepted native transport: $package" >&2; exit 1
    fi
    rg -q 'Wasm client depends on native download transport' "$fixture/output"
done
unset FIXTURE_NATIVE_CLIENT_LINE
printf 'Native download Wasm dependency refusals: PASS\n'

for package in reqwest tokio ic-host-fs; do
    export FIXTURE_DEFAULT_CLIENT_LINE="$package v1.0.0"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Default client accepted optional transport: $package" >&2; exit 1
    fi
    rg -q 'Client default graph depends on optional native download transport' "$fixture/output"
done
unset FIXTURE_DEFAULT_CLIENT_LINE
printf 'Default client optional dependency refusals: PASS\n'

for package in ic-platform-runtime ic-platform-runtime-guard ic-platform-macros; do
    export FIXTURE_CONTRACTS_LINE="$package v0.17.1"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Contracts accepted runtime owner: $package" >&2; exit 1
    fi
    rg -q 'Contracts depend on a Platform execution owner' "$fixture/output"
done
unset FIXTURE_CONTRACTS_LINE
printf 'Independent Runtime contract exclusions: PASS\n'
