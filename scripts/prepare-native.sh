#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]]
records="$1"
preparer="$(jq -ser '[.[] | select(.reason=="compiler-artifact" and .target.name=="prepare-native" and .executable!=null) | .executable] | if length==1 then .[0] else error("one native preparer required") end' target/blob-native-preparer.jsonl)"
raw="$(jq -ser '[.[] | select(.reason=="compiler-artifact") | .filenames[] | select(endswith(".wasm"))] | if length==1 then .[0] else error("one raw Wasm required") end' "$records")"
mkdir -p target/native-artifacts
attempt="$(mktemp -d target/native-artifacts/attempt.XXXXXX)"
"$preparer" "$raw" "$attempt/prepared.wasm"
# Preserve the raw source identity together with the admitted prepared identity.
perl -MDigest::SHA -e 'for my $path (@ARGV) { open my $f, "<", $path or die "$path: $!"; binmode $f; print Digest::SHA->new(256)->addfile($f)->hexdigest, "  $path\n"; }' "$raw" "$attempt/prepared.wasm" > "$attempt/sha256"
jq -c --arg raw "$raw" --arg path "$PWD/$attempt/prepared.wasm" 'if .reason=="compiler-artifact" then .filenames |= map(if .==$raw then $path else . end) else . end' "$records" > "$attempt/artifacts.jsonl"
cp "$records" "$attempt/raw-artifacts.jsonl"
cp "$attempt/artifacts.jsonl" "$records"
