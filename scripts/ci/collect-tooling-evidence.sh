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
trap 'status=$?; [[ "$selection_complete" == true || "$status" != 0 ]] || status=1;
      if [[ "$status" == 0 ]]; then rm -rf "$selection";
      else printf "Tooling evidence selection retained: %s\n" "$selection" >&2; fi;
      exit "$status"' EXIT
# The canonical owner admits shared references and emits their physical payloads.
# Never traverse arbitrary links or scan unrelated entries in the per-user store.
if bash "$script_dir/select-tool-evidence.sh" full "$repository" "$selection" \
    > "$selection/inputs.nul" 2> "$selection/selection.log"; then
    while IFS= read -r -d '' selected_root && IFS= read -r -d '' selected_path; do
        inputs+=("$selected_root" "$selected_path")
    done < "$selection/inputs.nul"
else
    status=$?
    cat "$selection/selection.log" >&2
    exit "$status"
fi
for path in "$repository/.tools/rust" \
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
if [[ ${#inputs[@]} == 0 ]]; then selection_complete=true; exit 0; fi
bash "$script_dir/archive-evidence.sh" "$output" "${inputs[@]}"
selection_complete=true
