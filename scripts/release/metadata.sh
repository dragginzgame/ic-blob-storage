#!/usr/bin/env bash
# Consumer policy: four product crates; independent private qualification versions.
# Adapted from the existing ic-entropy two-lock adapter; no runtime state is shared.
# Shared helpers own the transaction; IC Build owns manifest/lock rendering.
set -euo pipefail
script="$PWD/scripts/release/metadata.sh"
case "${1:-}" in
    admit)
        bash vendor/toko-tooling/scripts/ci/cargo-release-metadata.sh check-graph ic-blob-storage
        cargo metadata --locked --offline --manifest-path qualification/Cargo.toml --format-version 1 > /dev/null
        ;;
    selected)
        if [[ -z "${RELEASE_COMMIT:-}" ]]; then
            exec bash "$script" admit
        fi
        mkdir -p target
        scratch="$(mktemp -d "$PWD/target/release-selected.XXXXXX")"
        complete=false
        trap 'if [[ "$complete" == true ]]; then rm -rf -- "$scratch"; else printf "Selected-source check retained: %s\n" "$scratch" >&2; fi' EXIT
        git archive --format=tar "$RELEASE_COMMIT" | tar -xf - -C "$scratch"
        (cd "$scratch" && unset RELEASE_COMMIT && bash "$script" admit)
        complete=true
        ;;
    render)
        original="${2:?}" candidate="${3:?}"
        editor="${CARGO_RELEASE_METADATA_EDITOR:?}"
        limit="${CARGO_RELEASE_METADATA_MAX_BYTES:?}"
        evidence="${candidate%/*}"
        cargo metadata --locked --offline --format-version 1 > "$evidence/product.json"
        cargo metadata --locked --offline --manifest-path qualification/Cargo.toml --format-version 1 > "$evidence/qualification.json"
        yq -p toml -o json '.' "$original/Cargo.toml" > "$evidence/manifest.json"
        jq -n --slurpfile manifest "$evidence/manifest.json" --slurpfile metadata "$evidence/product.json" '
            {manifest: $manifest[0], members: ($metadata[0] | . as $m |
                [.packages[] | select(.id as $id | $m.workspace_members | index($id)) |
                    {name, manifest_path}])}' > "$evidence/workspace-input.json"
        jq -r '. as $m | .packages[] | select(.id as $id | $m.workspace_members | index($id)) | .name' \
            "$evidence/product.json" > "$evidence/packages"
        packages=()
        while IFS= read -r package; do packages+=("$package"); done < "$evidence/packages"
        "$editor" --max-input-bytes "$limit" manifest Cargo.toml \
            "${RELEASE_PREVIOUS:?}" "${RELEASE_VERSION:?}" "$evidence/workspace-input.json" > "$candidate/Cargo.toml"
        "$editor" --max-input-bytes "$limit" lock "$original/Cargo.lock" \
            "$RELEASE_PREVIOUS" "$RELEASE_VERSION" "${packages[@]}" > "$candidate/Cargo.lock"
        # Only two product libraries occur in the independent qualification graph.
        "$editor" --max-input-bytes "$limit" lock "$original/qualification/Cargo.lock" \
            "$RELEASE_PREVIOUS" "$RELEASE_VERSION" ic-blob-storage ic-blob-storage-reference-app \
            > "$candidate/qualification/Cargo.lock"
        awk -v version="$RELEASE_VERSION" -v previous="$RELEASE_PREVIOUS" -v date="${RELEASE_DATE:?}" \
            -f vendor/toko-tooling/scripts/ci/finalize-release-changelog.awk "$original/CHANGELOG.md" > "$candidate/CHANGELOG.md"
        ;;
    prepared)
        bash vendor/toko-tooling/scripts/ci/cargo-release-metadata.sh check ic-blob-storage
        bash "$script" admit
        ;;
    prepare)
        bash "$script" admit
        source vendor/toko-tooling/scripts/ci/release-metadata-transaction.sh
        render() { bash "$script" render "$@"; }
        check() { bash "$script" prepared; }
        state="$(git rev-parse --git-path release-state)"
        mkdir -p "$state"
        release_metadata_prepare "$state/metadata.XXXXXX" render check \
            qualification/Cargo.lock Cargo.lock CHANGELOG.md Cargo.toml
        ;;
    *) echo 'expected admit|selected|render|prepared|prepare' >&2; exit 2 ;;
esac
