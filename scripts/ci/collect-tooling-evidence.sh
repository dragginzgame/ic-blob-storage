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
selection="$(mktemp -d "$temporary/blob-tooling-selection.XXXXXX")"
selection_complete=false
status=0
trap 'status=$?; [[ "$selection_complete" == true || "$status" != 0 ]] || status=1;
      if [[ "$status" == 0 ]]; then rm -rf "$selection";
      else printf "Tooling evidence selection retained: %s\n" "$selection" >&2; fi;
      exit "$status"' EXIT
for path in \
    "$repository/.tools/testkit-server" "$repository"/target/release-tests.*; do
    [[ -e "$path" || -L "$path" ]] || continue
    inputs+=("$repository" "${path#"$repository/"}")
done
# shellcheck source=scripts/ci/formatting-evidence.sh
source "$script_dir/formatting-evidence.sh"
select_formatting_evidence "$temporary"
inputs+=(${formatting_evidence[@]+"${formatting_evidence[@]}"})
for prefix in blob-format-hooks. formatting-adoption. blob-evidence-checksums. \
    nonempty-cargo-test. file-digests. ic-tools-test. pocketic-checks. \
    pocketic-alignment. release-commands. cargo-metadata-test. \
    shared-tooling-snapshot-test. local-lock-test. format-tools-test. \
    host-tools-test. rust-tools-test. shared-tool-commands. cloc-tooling-test. \
    shared-tooling-cloc-siblings-test. shared-tooling-cloc-test. \
    blob-validation-logging. evidence-archive. tool-evidence-test. \
    blob-tooling-evidence-test. blob-tooling-evidence-proof. \
    blob-tooling-evidence-downloaded. blob-testkit-commands. blob-release-tests.; do
    for path in "$temporary/$prefix"*; do
        [[ -e "$path" || -L "$path" ]] || continue
        inputs+=("$temporary" "${path#"$temporary/"}")
    done
done
# Shared admits complete store selections and archives safe local references on
# failure. Blob supplies only its product-owned roots; never admit partial paths.
bash "$script_dir/collect-tool-evidence.sh" "$output" "$repository" "$selection" \
    full "$repository/ci/tool-versions.env" "$repository/ci/ic-tools.tsv" \
    ${inputs[@]+"${inputs[@]}"} || status=$?
selection_complete=true
exit "$status"
