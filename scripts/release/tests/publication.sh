#!/usr/bin/env bash
# Test-only transport effects; the consumer's source/tag/receipt guards stay real.
setup_publication_substitutes() {
    TEST_REAL_CARGO="$(command -v cargo)"
    export TEST_REAL_CARGO TEST_ARCHIVE="$TEMPORARY/published.crate"
    cat > "$TEMPORARY/bin/curl" <<'CURL'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == '--disable --version' ]]; then echo 'curl 8.4.0 fixture'; exit; fi
output='' url=''
while [[ $# -gt 0 ]]; do
    case "$1" in --output) output="$2"; shift ;; https://*) url="$1" ;; esac
    shift
done
case "$url" in
    https://crates.io/*)
        version="${url##*/}"; package="${url%/*}"; package="${package##*/}"
        case "$TEST_REGISTRY" in
            missing) printf 404; exit ;;
            forbidden) printf 403; exit ;;
            dependency)
                if [[ "$package" != ic-blob-storage-contracts || "$version" != "$TEST_CONTRACTS_VERSION" ]]; then printf 404; exit; fi ;;
        esac
        checksum="$(shasum -a 256 "$TEST_ARCHIVE")"; checksum="${checksum%% *}"
        [[ "$TEST_REGISTRY" != checksum ]] || checksum="$(printf '%064d' 0)"
        yanked=false; [[ "$TEST_REGISTRY" != yanked ]] || yanked=true
        printf '{"version":{"crate":"%s","num":"%s","checksum":"%s","yanked":%s}}' "$package" "$version" "$checksum" "$yanked" > "$output"
        printf 200 ;;
    https://static.crates.io/*) cp "$TEST_ARCHIVE" "$output" ;;
    *) exit 1 ;;
esac
CURL
    cat > "$TEMPORARY/bin/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == publish ]]; then
    printf '%s\n' "$*" >> "$TEST_EVENTS"
    if [[ -n ${TEST_FROZEN_PUBLICATIONS:-} ]]; then
        "$TEST_REAL_CARGO" metadata --offline --locked --no-deps --format-version 1 |
            jq -e --arg root "$PWD" '.workspace_root == $root' >/dev/null
        printf '%s\t%s\n' "$RELEASE_GROUP" "$(git rev-parse HEAD)" >> "$TEST_FROZEN_PUBLICATIONS"
        [[ ! -e "$TEST_FROZEN_FAIL" ]] || exit 17
    fi
elif [[ "$1" == fetch ]]; then
    printf 'fetch\n' >> "$TEST_EVENTS"
else exec "$TEST_REAL_CARGO" "$@"; fi
CARGO
    chmod +x "$TEMPORARY/bin/curl" "$TEMPORARY/bin/cargo"
    export PATH="$TEMPORARY/bin:$PATH"
}
