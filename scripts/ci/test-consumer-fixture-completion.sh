#!/usr/bin/env bash
set -euo pipefail
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES

root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/blob-fixture-completion.XXXXXX")"
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$fixture_complete" == true && "$status" == 0 ]]; then rm -rf "$fixture"
    else printf 'Consumer completion fixture retained: %s\n' "$fixture" >&2; fi
    exit "$status"
}
trap finish EXIT

# Inject disposable copies of actual entrypoints before their first assertion.
# No production test hook, Git mutation, build or publication is involved.
for script in scripts/release/test-release.sh scripts/ci/test-tooling-evidence.sh scripts/ci/test-testkit-commands.sh scripts/ci/test-library-packages.sh scripts/ci/check-validation-logging.sh; do
    owner=fixture
    boundary='trap finish_fixture EXIT'
    if [[ "$script" == scripts/release/test-release.sh || "$script" == scripts/ci/check-validation-logging.sh ]]; then
        owner=FIXTURE
        [[ "$script" != scripts/release/test-release.sh ]] || owner=TEMPORARY
        boundary='trap finish EXIT'
    fi
    for failure in nounset zero nonzero command; do
        # shellcheck disable=SC2016 # Expanded only by the injected child script.
        case "$failure" in
            nounset) injection='unset BLOB_FIXTURE_UNBOUND; : "$BLOB_FIXTURE_UNBOUND"'; expected=1 ;;
            zero) injection='exit 0'; expected=1 ;;
            nonzero) injection='exit 23'; expected=23 ;;
            command) injection='false'; expected=1 ;;
        esac
        case_root="$fixture/${script##*/}-$failure"
        mkdir -p "$case_root/${script%/*}" "$case_root/tmp"
        FIXTURE_BOUNDARY="$boundary" FIXTURE_OWNER="$owner" FIXTURE_INJECTION="$injection" awk '
            { print }
            $0 == ENVIRON["FIXTURE_BOUNDARY"] {
                print "printf evidence > \"$" ENVIRON["FIXTURE_OWNER"] "/probe.txt\""
                print ENVIRON["FIXTURE_INJECTION"]
                print "exit 99"
                injected++
            }
            END { if (injected != 1) exit 1 }
        ' "$root/$script" > "$case_root/$script"
        status=0
        CARGO_TARGET_DIR="$case_root/target" TMPDIR="$case_root/tmp" bash "$case_root/$script" > "$case_root/result.log" 2>&1 || status=$?
        [[ "$status" == "$expected" ]]
        retained="$(sed -n 's/^.*fixture retained: //p; s/^Full logs and isolated fixture retained: //p' "$case_root/result.log")"
        [[ -f "$retained/probe.txt" && "$(cat "$retained/probe.txt")" == evidence ]]
        if grep -F 'passed' "$case_root/result.log" >/dev/null || grep -F 'tests: PASS' "$case_root/result.log" >/dev/null; then exit 1; fi
    done
done
echo 'Consumer fixtures reject premature exits and retain evidence (isolated injected entrypoints)'
fixture_complete=true
