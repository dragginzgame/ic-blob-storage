#!/usr/bin/env bash
set -euo pipefail

ROOT="$0"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"
ROOT="${ROOT%/.}"
export PATH="$ROOT/.tools/host/bin:$PATH"

usage() {
    cat <<'EOF'
Usage: cloc.sh [--subcrates] [--json] [--manifest PATH] [directory]
       cloc.sh --check-tools

Report Rust runtime/test lines and test-function totals per workspace.
The directory defaults to the current working directory. A parent directory
selects its immediate Git checkouts; --subcrates also displays member rows.
Counts always include all members. Independent excluded workspaces require
--manifest. --json is a single-workspace report and always includes crate rows.
--manifest selects one independent Cargo workspace explicitly; its own Cargo
configuration and target directory apply. The default is the Git root workspace.
Prepared host tools beside this script's checkout take precedence over PATH.
--check-tools checks prerequisites without inspecting a workspace or installing.
--json emits one complete document with crate rows and totals on success.
Each selected source path counts once, including files with identical contents.
EOF
}

check_tools() {
    local tool missing=""
    for tool in cargo cloc jq; do
        if ! command -v "$tool" >/dev/null 2>&1; then missing="$missing $tool"; fi
    done
    if [[ -n "$missing" ]]; then
        echo "error: missing LOC tools:$missing" >&2
        echo "Run make install-host-tools in $ROOT; prepare the Cargo toolchain separately if missing." >&2
        return 1
    fi
}

report_repositories() {
local parent_root="$1"
export LC_ALL=C
shopt -s nullglob dotglob
repos=()
needs_cargo=false
column_width=24
for repo in "${parent_root%/}"/*; do
    [[ -d "$repo" && ! -L "$repo" && -e "$repo/.git" ]] || continue
    name="${repo##*/}"
    if [[ "$name" == *$'\n'* || "$name" == *$'\r'* || "$name" == *$'\t'* ]]; then
        echo 'error: repository names containing tabs or newlines cannot be displayed' >&2
        exit 1
    fi
    repos+=("$repo")
    if [[ -f "$repo/Cargo.toml" ]]; then needs_cargo=true; fi
    if [[ ${#name} -gt $column_width ]]; then column_width=${#name}; fi
done
if [[ ${#repos[@]} == 0 ]]; then
    echo "error: no Git checkouts found in $parent_root" >&2
    exit 1
fi
if [[ "$needs_cargo" == true ]]; then
    bash "$ROOT/scripts/dev/cloc.sh" --check-tools
fi

rows=()
# Buffer the counted rows so every repository and indented crate shares one
# width. Counting remains a single pass, even when a later crate is longest.
print_row() {
    local row
    if [[ ${#1} -gt $column_width ]]; then column_width=${#1}; fi
    printf -v row '%s\t%s\t%s\t%s\t%s\t%s' "$@"
    rows+=("$row")
}
print_row repository runtime_loc test_loc test_% test_fns inline_fns
rows+=(divider)

status=0
counted_repos=0
total_runtime_loc=0
total_test_loc=0
total_test_fns=0
total_inline_fns=0
for repo in "${repos[@]}"; do
    name="${repo##*/}"
    if ! git_root="$(git -C "$repo" rev-parse --show-toplevel 2>/dev/null)" || [[ "$git_root" != "$repo" ]]; then
        print_row "$name" ERROR ERROR ERROR ERROR ERROR
        echo "error: not a Git checkout root: $repo" >&2
        status=1
        continue
    fi
    if [[ ! -f "$repo/Cargo.toml" ]]; then
        print_row "$name" N/A N/A N/A N/A N/A
        continue
    fi
    # Enter the checkout so Cargo reads its own .cargo/config.toml. Reuse the
    # complete workspace report, publishing only its final totals on success.
    if ! report="$(cd "$repo" && bash "$ROOT/scripts/dev/cloc.sh" --json "$repo")"; then
        print_row "$name" ERROR ERROR ERROR ERROR ERROR
        echo "error: unable to count $repo" >&2
        status=1
        continue
    fi
    if ! totals="$(jq -er -s '
        if length != 1 then error("expected one LOC report") else .[0].totals end
        | [.runtime_loc, .test_loc, .test_fns, .inline_fns]
        | if all(.[]; type == "number" and . >= 0 and floor == .)
          then @tsv else error("invalid LOC counts") end
    ' <<< "$report")"; then
        print_row "$name" ERROR ERROR ERROR ERROR ERROR
        echo "error: invalid workspace totals for $repo" >&2
        status=1
        continue
    fi
    IFS=$'\t' read -r runtime_loc test_loc test_fns inline_fns <<< "$totals"
    test_pct="$(awk -v runtime="$runtime_loc" -v tests="$test_loc" \
        'BEGIN { printf "%.1f%%", (runtime + tests > 0 ? 100 * tests / (runtime + tests) : 0) }')"
    print_row "$name" "$runtime_loc" "$test_loc" "$test_pct" "$test_fns" "$inline_fns"
    if [[ "$subcrates" == true ]]; then
        while IFS=$'\t' read -r crate runtime tests pct fns inline; do
            print_row "  $crate" "$runtime" "$tests" "$pct%" "$fns" "$inline"
        done < <(jq -r '.crates[] | [.crate,.runtime_loc,.test_loc,.test_pct,.test_fns,.inline_fns] | @tsv' <<< "$report")
    fi
    counted_repos=$((counted_repos + 1))
    total_runtime_loc=$((total_runtime_loc + runtime_loc))
    total_test_loc=$((total_test_loc + test_loc))
    total_test_fns=$((total_test_fns + test_fns))
    total_inline_fns=$((total_inline_fns + inline_fns))
done
rows+=(divider)
total_label=TOTAL
if [[ "$status" != 0 ]]; then total_label='TOTAL (partial)'; fi
if [[ "$counted_repos" == 0 ]]; then
    print_row "$total_label" N/A N/A N/A N/A N/A
else
    total_pct=0.0%
    if [[ $((total_runtime_loc + total_test_loc)) -gt 0 ]]; then
        total_pct="$(awk -v runtime="$total_runtime_loc" -v tests="$total_test_loc" \
            'BEGIN { printf "%.1f%%", 100 * tests / (runtime + tests) }')"
    fi
    print_row "$total_label" "$total_runtime_loc" "$total_test_loc" "$total_pct" "$total_test_fns" "$total_inline_fns"
fi
printf -v divider '%*s' "$column_width" ''
for row in "${rows[@]}"; do
    if [[ "$row" == divider ]]; then
        printf '%-*s %12s %12s %10s %9s %10s\n' "$column_width" \
            "${divider// /-}" ------------ ------------ ---------- --------- ----------
    else
        IFS=$'\t' read -r name runtime tests pct fns inline <<< "$row"
        printf '%-*s %12s %12s %10s %9s %10s\n' "$column_width" \
            "$name" "$runtime" "$tests" "$pct" "$fns" "$inline"
    fi
done
return "$status"

}


subcrates=false
selected_manifest=''
json=false
while [[ $# -gt 0 ]]; do
    case "$1" in
        --subcrates) subcrates=true; shift ;;
        --json) json=true; shift ;;
        --manifest)
            [[ $# -ge 2 && -n "$2" && -z "$selected_manifest" ]] || { usage >&2; exit 2; }
            selected_manifest="$2"; shift 2 ;;
        --help|-h|--check-tools) break ;;
        --) shift; break ;;
        -*) usage >&2; exit 2 ;;
        *) break ;;
    esac
done
[[ $# -le 1 ]] || { usage >&2; exit 2; }

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi
if [[ "${1:-}" == --check-tools ]]; then
    check_tools
    exit 0
fi

resolve_directory() (
    directory="$1"
    [[ "$directory" == /* ]] || directory="$PWD/$directory"
    case "$directory" in
        *$'\n'*|*$'\r'*) echo 'error: directory contains line breaks' >&2; exit 1 ;;
    esac
    cd -P "$directory" || exit 1
    case "$PWD" in
        *$'\n'*|*$'\r'*) echo 'error: resolved directory contains line breaks' >&2; exit 1 ;;
    esac
    printf '%s/.' "$PWD"
)

if [[ -n "$selected_manifest" ]]; then
    case "$selected_manifest" in
        *$'\n'*|*$'\r'*) echo 'error: manifest path contains line breaks' >&2; exit 1 ;;
    esac
    [[ -f "$selected_manifest" && ! -L "$selected_manifest" ]] || {
        echo 'error: selected manifest must be a regular file' >&2; exit 1;
    }
    selected_directory="$(resolve_directory "$(dirname "$selected_manifest")")"
    selected_directory="${selected_directory%/.}"
    selected_manifest="$selected_directory/$(basename "$selected_manifest")"
fi
requested_root="$(resolve_directory "${1:-${selected_directory:-$PWD}}")"
requested_root="${requested_root%/.}"
has_checkouts=false
for child in "$requested_root"/* "$requested_root"/.[!.]*; do
    if [[ -d "$child" && ! -L "$child" && -e "$child/.git" ]]; then has_checkouts=true; break; fi
done
if [[ -z "$selected_manifest" && ! -f "$requested_root/Cargo.toml" && ! -e "$requested_root/.git" ]] &&
   { [[ "$has_checkouts" == true ]] || ! git -C "$requested_root" rev-parse --show-toplevel >/dev/null 2>&1; }; then
    [[ "$json" == false ]] || { echo 'error: --json requires one workspace' >&2; exit 2; }
    report_repositories "$requested_root"
    exit $?
fi
if repo_root="$(git -C "${requested_root}" rev-parse --show-toplevel 2>/dev/null)"; then
    :
else
    repo_root="$requested_root"
fi

repo_root="$(resolve_directory "$repo_root")"
repo_root="${repo_root%/.}"
manifest_path="${selected_manifest:-${repo_root}/Cargo.toml}"
case "$manifest_path" in
    "$repo_root/"*) ;;
    *) echo 'error: selected workspace must belong to the selected checkout' >&2; exit 1 ;;
esac
if [[ ! -f "${manifest_path}" ]]; then
    echo "error: Cargo workspace manifest not found: ${manifest_path}" >&2
    exit 1
fi

check_tools

if ! metadata="$(
    cd "$(dirname "$manifest_path")" &&
    RUSTUP_AUTO_INSTALL=0 cargo metadata \
        --format-version 1 \
        --manifest-path "${manifest_path}" \
        --locked --offline \
        --no-deps
)"; then
    echo "error: unable to resolve Cargo workspace metadata" >&2
    exit 1
fi

if ! crate_rows="$(
    jq -r '
        .workspace_members as $members
        | .packages[]
        | select(.id as $id | $members | index($id))
        | [.name, (.manifest_path | sub("/Cargo.toml$"; ""))]
        | @tsv
    ' <<<"${metadata}"
)"; then
    echo "error: unable to parse Cargo workspace metadata" >&2
    exit 1
fi

if [[ -z "${crate_rows}" ]]; then
    echo "error: Cargo workspace contains no packages" >&2
    exit 1
fi

if ! target_path="$(jq -er '.target_directory | select(type == "string" and startswith("/"))' <<<"$metadata")"; then
    echo 'error: Cargo metadata has no absolute target directory' >&2
    exit 1
fi
target_paths=("$target_path")
# Cargo preserves a configured alias. find does not follow source symlinks, but
# can reach the existing output through its physical path elsewhere in a member.
# A target that has never been built need not exist.
if [[ -d "$target_path" ]]; then
    target_paths+=("$(cd "$target_path" && pwd -P)")
fi

FILE_LIST_DIR="$(mktemp -d "${TMPDIR:-/tmp}/shared-tooling-cloc.XXXXXX")"
trap 'rm -rf "$FILE_LIST_DIR"' EXIT

crates=()
while IFS= read -r crate_row; do
    crates[${#crates[@]}]="${crate_row}"
done < <(printf '%s\n' "${crate_rows}" | sort)

tests_pattern='(^|/)(tests/|[^/]*tests\.rs$)'
test_attr_pattern='^[[:space:]]*#\[(tokio::)?test([[:space:]]|\(|\])'
crate_column_width=24
total_runtime_loc=0
total_test_loc=0
total_test_fns=0
total_inline_test_fns=0

for crate_row in "${crates[@]}"; do
    IFS=$'\t' read -r crate_name _ <<<"${crate_row}"
    if [[ "${#crate_name}" -gt "${crate_column_width}" ]]; then
        crate_column_width="${#crate_name}"
    fi
done

printf -v crate_divider "%*s" "${crate_column_width}" ""
crate_divider="${crate_divider// /-}"

print_divider() {
    printf "%-*s %12s %12s %10s %9s %10s\n" \
        "${crate_column_width}" \
        "${crate_divider}" \
        "------------" \
        "------------" \
        "--------" \
        "---------" \
        "----------"
}

if [[ "$json" == false ]]; then
    printf "%-*s %12s %12s %10s %9s %10s\n" \
        "${crate_column_width}" \
        "workspace/crate" \
        "runtime_loc" \
        "test_loc" \
        "test_%" \
        "test_fns" \
        "inline_fns"
    print_divider
fi

# Both projections use the same counted rows, including the final totals.
print_counts() {
    if [[ "$json" == true ]]; then
        jq -cn --arg crate "$1" --argjson runtime_loc "$2" --argjson test_loc "$3" \
            --argjson test_pct "$4" --argjson test_fns "$5" --argjson inline_fns "$6" \
            '{crate:$crate,runtime_loc:$runtime_loc,test_loc:$test_loc,test_pct:$test_pct,
              test_fns:$test_fns,inline_fns:$inline_fns}' >> "$FILE_LIST_DIR/rows.jsonl"
    elif [[ "$subcrates" == true || "$1" == TOTAL ]]; then
        printf '%-*s %12d %12d %9s%% %9d %10d\n' "$crate_column_width" "$@"
    fi
}

# Use the same file lists for LOC and test-attribute counts.
count_test_fns() {
    local crate_root="$1" file_list="$2" total=0 inline=0 rust_file file_count status
    while IFS= read -r rust_file; do
        if file_count=$(grep -Ec "${test_attr_pattern}" "${rust_file}"); then :; else
            status=$?
            [[ "$status" == 1 ]] || return "$status"
        fi
        total=$((total + file_count))
        if [[ ! "${rust_file#"$crate_root/"}" =~ ${tests_pattern} ]]; then
            inline=$((inline + file_count))
        fi
    done <"$file_list"
    printf '%s\t%s\n' "$total" "$inline"
}

for crate_row in "${crates[@]}"; do
    IFS=$'\t' read -r crate_name crate_path <<<"${crate_row}"

    # Cargo owns the build-output identity, including configured target paths.
    # Prune it before either LOC or test counting, even inside a package.
    find_args=("$crate_path")
    excluded_paths=("${target_paths[@]}")
    for member_row in "${crates[@]}"; do
        IFS=$'\t' read -r _ member_path <<<"$member_row"
        if [[ "$member_path" == "$crate_path/"* ]]; then
            excluded_paths+=("$member_path")
        fi
    done
    for excluded_path in "${excluded_paths[@]}"; do
        # find's -path takes a glob; escape literal path metacharacters.
        excluded_pattern="${excluded_path//\\/\\\\}"
        excluded_pattern="${excluded_pattern//\*/\\*}"
        excluded_pattern="${excluded_pattern//\?/\\?}"
        excluded_pattern="${excluded_pattern//\[/\\[}"
        find_args+=(-path "$excluded_pattern" -prune -o)
    done
    find "${find_args[@]}" -type f -name '*.rs' -print0 >"$FILE_LIST_DIR/files"
    : >"$FILE_LIST_DIR/list"
    while IFS= read -r -d '' rust_file; do
        # cloc's --list-file format has one literal path per line.
        if [[ "$rust_file" == *$'\n'* ]]; then
            echo "error: cloc file lists cannot represent paths containing newlines" >&2
            exit 1
        fi
        printf '%s\n' "$rust_file" >>"$FILE_LIST_DIR/list"
    done <"$FILE_LIST_DIR/files"

    # Count every admitted path once. cloc must not choose a representative from
    # identical files in different runtime/test partitions.
    cloc --list-file="$FILE_LIST_DIR/list" --include-lang=Rust \
        --skip-uniqueness --by-file --json > "$FILE_LIST_DIR/counts.json"
    counts="$(jq -er --arg prefix "$crate_path/" \
        --arg tests "$tests_pattern" '
        to_entries | map(select(.value | type == "object" and .language == "Rust"))
        | reduce .[] as $file ({runtime:0, tests:0};
            if ($file.value.code | type != "number" or . < 0 or floor != .)
            then error("invalid cloc count")
            elif ($file.key | ltrimstr($prefix) | test($tests))
            then .tests += $file.value.code else .runtime += $file.value.code end)
        | [.runtime, .tests] | @tsv' "$FILE_LIST_DIR/counts.json")"
    IFS=$'\t' read -r runtime_loc test_loc <<< "$counts"
    functions="$(count_test_fns "$crate_path" "$FILE_LIST_DIR/list")"
    IFS=$'\t' read -r test_fns inline_test_fns <<< "$functions"
    crate_loc=$((runtime_loc + test_loc))

    if [[ "${crate_loc}" -gt 0 ]]; then
        test_pct=$(awk "BEGIN { printf \"%.1f\", (${test_loc}/${crate_loc})*100 }")
    else
        test_pct="0.0"
    fi

    print_counts \
        "${crate_name}" \
        "${runtime_loc}" \
        "${test_loc}" \
        "${test_pct}" \
        "${test_fns}" \
        "${inline_test_fns}"

    total_runtime_loc=$((total_runtime_loc + runtime_loc))
    total_test_loc=$((total_test_loc + test_loc))
    total_test_fns=$((total_test_fns + test_fns))
    total_inline_test_fns=$((total_inline_test_fns + inline_test_fns))
done

total_loc=$((total_runtime_loc + total_test_loc))
if [[ "${total_loc}" -gt 0 ]]; then
    total_test_pct=$(awk "BEGIN { printf \"%.1f\", (${total_test_loc}/${total_loc})*100 }")
else
    total_test_pct="0.0"
fi

if [[ "$json" == false ]]; then print_divider; fi
print_counts \
    "TOTAL" \
    "${total_runtime_loc}" \
    "${total_test_loc}" \
    "${total_test_pct}" \
    "${total_test_fns}" \
    "${total_inline_test_fns}"
if [[ "$json" == true ]]; then
    jq -s --arg manifest "$manifest_path" \
        '{manifest:$manifest,crates:.[0:-1],totals:(.[-1] | del(.crate))}' "$FILE_LIST_DIR/rows.jsonl"
fi
