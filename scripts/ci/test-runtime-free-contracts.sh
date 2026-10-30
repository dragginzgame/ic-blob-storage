#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-feature-boundary.XXXXXX")"
trap 'rm -rf "$fixture"' EXIT
cat > "$fixture/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == *'{p}|{f}'* ]]; then
    printf '%s\n' "${FIXTURE_FEATURE_LINE:-}"
elif [[ "$*" == *'wasm32-unknown-unknown'* && "$*" == *'ic-blob-storage-canister'* ]]; then
    printf '%s\n' "${FIXTURE_PACKAGE_LINE:-}"
fi
CARGO
chmod +x "$fixture/cargo"
export PATH="$fixture:$PATH"
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
for package in clap clap_builder clap_derive; do
    export FIXTURE_PACKAGE_LINE="$package v4.6.7"
    if bash "$root/scripts/ci/check-runtime-free-contracts.sh" > "$fixture/output" 2>&1; then
        echo "Deployment accepted native CLI parser $package" >&2; exit 1
    fi
    rg -q 'Deployment depends on a legacy runtime or native-only owner' "$fixture/output"
done
unset FIXTURE_PACKAGE_LINE
printf 'Deployment CLI parser refusals: PASS\n'
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
