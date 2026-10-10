#!/usr/bin/env bash
set -euo pipefail

# Blob owns these retained roots; the shared helper owns archive admission/IO.
[[ $# == 3 ]] || { echo 'usage: collect-tooling-evidence.sh REPOSITORY TEMP-ROOT NEW-ARCHIVE' >&2; exit 2; }
script_dir="${BASH_SOURCE[0]}"
[[ "$script_dir" == /* ]] || script_dir="$PWD/$script_dir"
script_dir="$(cd -P "${script_dir%/*}" && printf '%s/.' "$PWD")"
script_dir="${script_dir%/.}"
repository="$1"
temporary="$2"
output="$3"
[[ "$repository" == /* ]] || repository="$PWD/$repository"
[[ "$temporary" == /* ]] || temporary="$PWD/$temporary"
repository="$(cd -P "$repository" && printf '%s/.' "$PWD")"
repository="${repository%/.}"
temporary="$(cd -P "$temporary" && printf '%s/.' "$PWD")"
temporary="${temporary%/.}"
inputs=()
shopt -s nullglob
for path in "$repository"/.tools/host-set.* "$repository"/.tools/ic-set.* \
    "$repository/.tools/rust/build" "$repository"/.tools/rust/ic-testkit-* \
    "$repository/.tools/testkit-server" "$repository"/target/release-tests.*; do
    [[ -e "$path" || -L "$path" ]] || continue
    inputs+=("$repository" "${path#"$repository/"}")
done
for prefix in blob-format-hooks. formatting-adoption. formatting. blob-evidence-checksums. \
    nonempty-cargo-test. file-digests. ic-tools-test. pocketic-checks. \
    pocketic-alignment. release-commands. cargo-metadata-test. \
    shared-tooling-snapshot-test. local-lock-test. format-tools-test. \
    host-tools-test. rust-tools-test. shared-tool-commands. cloc-tooling-test. \
    shared-tooling-cloc-siblings-test. shared-tooling-cloc-test. \
    blob-validation-logging. evidence-archive. tool-evidence-test. \
    blob-tooling-evidence-test. blob-tooling-evidence-proof. \
    blob-tooling-evidence-downloaded. blob-testkit-commands.; do
    for path in "$temporary/$prefix"*; do
        [[ -e "$path" || -L "$path" ]] || continue
        inputs+=("$temporary" "${path#"$temporary/"}")
    done
done
# Match the previous uploader's missing-input behavior without publishing an
# empty archive or changing the status of the command that originally failed.
[[ ${#inputs[@]} != 0 ]] || exit 0
bash "$script_dir/archive-evidence.sh" "$output" "${inputs[@]}"
