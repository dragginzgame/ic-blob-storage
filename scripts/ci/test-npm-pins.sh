#!/usr/bin/env bash
# Shared companions: scripts/ci/check-dependency-pins.sh scripts/ci/dependency-pins.jq
set -euo pipefail
ROOT="$0"
[[ "$ROOT" == /* ]] || ROOT="$PWD/$ROOT"
ROOT="$(cd -P "${ROOT%/*}/../.." && printf '%s/.' "$PWD")"
ROOT="${ROOT%/.}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/npm-pins-test.XXXXXX")"
# Bash 3.2 can enter EXIT with status zero after nounset; require completion too.
fixture_complete=false
finish() {
    local status=$?
    [[ "$fixture_complete" == true || "$status" != 0 ]] || status=1
    if [[ "$status" == 0 ]]; then rm -rf "$fixture"
    else echo "npm pin fixtures retained: $fixture" >&2; fi
    exit "$status"
}
trap finish EXIT
consumer="$fixture/consumer"
mkdir -p "$consumer/web/local" "$consumer/ci" "$fixture/sibling" "$fixture/bin"
git init -q "$consumer"
printf 'Approved development sibling; freeze its source before release.\n' > "$consumer/AGENTS.md"
cat > "$fixture/package.json" <<'JSON'
{"name":"fixture","version":"1.0.0","packageManager":"npm@11.19.0","engines":{"node":"24.21.0","npm":"11.19.0"},"dependencies":{"registry":"^1.0.0"},"devDependencies":{"alias":"npm:other@^2.0.0"}}
JSON
# These commands must never execute, even when metadata contains lifecycle hooks.
for command in node npm npx curl cargo; do
    cat > "$fixture/bin/$command" <<'STUB'
#!/usr/bin/env bash
echo 'forbidden execution during declaration check' >&2
exit 98
STUB
    chmod +x "$fixture/bin/$command"
done
export PATH="$fixture/bin:$PATH"
manifest="$consumer/web/package.json" lock="$consumer/web/package-lock.json"
reset_manifest() { cp "$fixture/package.json" "$manifest"; }
edit_manifest() { jq "$1" "$manifest" > "$fixture/edited"; cp "$fixture/edited" "$manifest"; }
sync_lock() { jq '{name,version,lockfileVersion:3,packages:{"":.}}' "$manifest" > "$lock"; }
check() {
    local result=0
    cp "$manifest" "$fixture/manifest-before"
    cp "$lock" "$fixture/lock-before"
    cp "$consumer/.git/index" "$fixture/index-before"
    bash "$ROOT/scripts/ci/check-dependency-pins.sh" --consumer "$consumer" \
        --npm-root web --node-version 24.21.0 --npm-version 11.19.0 "$@" > "$fixture/output" 2>&1 || result=$?
    cmp "$manifest" "$fixture/manifest-before"
    cmp "$lock" "$fixture/lock-before"
    cmp "$consumer/.git/index" "$fixture/index-before"
    if rg -F 'forbidden execution' "$fixture/output" >/dev/null; then cat "$fixture/output" >&2; exit 1; fi
    return "$result"
}
pass() { check || { cat "$fixture/output" >&2; exit 1; }; }
reject() {
    local expected="$1"; shift
    if check "$@"; then echo "unexpected npm pin admission: $expected" >&2; exit 1; fi
    rg -F "$expected" "$fixture/output" >/dev/null || { cat "$fixture/output" >&2; exit 1; }
}
reset_manifest
sync_lock
git -C "$consumer" add -- web/package.json web/package-lock.json AGENTS.md
pass
for schema in 2 3; do
    jq --argjson schema "$schema" '.lockfileVersion=$schema' "$lock" > "$fixture/edited"
    cp "$fixture/edited" "$lock"
    pass
done
for mutation in '.lockfileVersion=1' 'del(.packages)' '.packages[""]=null' '.version="2.0.0"' '.packages[""].name="other"' '.packages[""].dependencies.registry="^2.0.0"'; do
    sync_lock
    jq "$mutation" "$lock" > "$fixture/edited"; cp "$fixture/edited" "$lock"
    reject npm-lock
done
sync_lock
git -C "$consumer" rm --cached -q -- web/package-lock.json
reject 'npm input must be tracked'
git -C "$consumer" add -- web/package-lock.json
mv "$lock" "$fixture/saved-lock"
if bash "$ROOT/scripts/ci/check-dependency-pins.sh" --consumer "$consumer" --npm-root web \
    --node-version 24.21.0 --npm-version 11.19.0 > "$fixture/output" 2>&1; then exit 1; fi
rg -F 'npm input must be a regular file' "$fixture/output" >/dev/null
mv "$fixture/saved-lock" "$lock"
for path in "$manifest" "$lock"; do
    for invalid in '{broken' '{} {}' '[]'; do
        printf '%s\n' "$invalid" > "$path"
        reject 'JSON'
        reset_manifest; sync_lock
    done
done

# Explicit tool ownership; ranges remain npm's compatibility check.
reject 'exact stable' --node-version 24
reject 'exact stable' --npm-version latest
for mutation in '.packageManager="npm@10.0.0"' '.packageManager="pnpm@11.19.0"' '.engines.node="22.0.0"' '.engines.npm="10.0.0"' '.engines=[]'; do
    edit_manifest "$mutation"; sync_lock; reject npm-toolchain; reset_manifest
done
edit_manifest '.engines={node:">=24",npm:"11.x"} | .scripts={install:"exit 99"}'
sync_lock; pass
edit_manifest 'del(.packageManager)'; sync_lock; pass
reset_manifest

# Cover each direct dependency table, immutable Git identities and redaction.
for section in dependencies devDependencies optionalDependencies peerDependencies; do
    for commit in aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb; do
        edit_manifest ".$section.git=\"git+https://example.invalid/repository.git#$commit\""
        sync_lock; pass
    done
    for selector in 'github:owner/repo#main' 'owner/repo#abcdef0' 'git+https://token-secret@example.invalid/repo.git#v1'; do
        edit_manifest ".$section.git=\"$selector\""; sync_lock; reject npm-git
        if rg -F 'token-secret' "$fixture/output" >/dev/null; then exit 1; fi
    done
    reset_manifest
done
for mutation in '.dependencies=[]' '.dependencies.registry=null' '.dependencies.registry=""' '.dependencies.registry="workspace:*"'; do
    edit_manifest "$mutation"; sync_lock; reject npm-declaration; reset_manifest
done

# File inputs reuse the existing exact exception/evidence owner, including escapes.
edit_manifest '.dependencies.local="file:./local"'; sync_lock; pass
edit_manifest '.dependencies.local="file:../../sibling"'; sync_lock; reject npm-external-path
cat > "$consumer/ci/dependency-pinning-exceptions.json" <<'JSON'
[{"rule":"npm-external-path","file":"web/package.json","subject":"local","value":"file:../../sibling","reason":"Development sibling with separately frozen release inputs.","evidence":"AGENTS.md"}]
JSON
pass
edit_manifest '.dependencies.local="../../sibling"'; sync_lock; reject npm-external-path
ln -s "$fixture/sibling" "$consumer/web/escape"
edit_manifest '.dependencies.local="file:./escape"'; sync_lock; reject npm-external-path
edit_manifest '.dependencies.local="file:"'; sync_lock; reject 'requires a directory'
edit_manifest '.dependencies.local="file:./missing"'; sync_lock; reject 'directory is unavailable'
rm "$consumer/ci/dependency-pinning-exceptions.json"
reset_manifest; sync_lock
printf '{}\n' > "$consumer/web/npm-shrinkwrap.json"
reject 'takes precedence'
rm "$consumer/web/npm-shrinkwrap.json"
mv "$lock" "$fixture/saved-lock"
ln -s "$fixture/saved-lock" "$lock"
reject 'must be a regular file'
rm "$lock"; mv "$fixture/saved-lock" "$lock"

# Different roots own independent selections; no implicit discovery or association.
mkdir "$consumer/publish"
jq '.engines={node:"22.13.0",npm:"10.9.2"} | .packageManager="npm@10.9.2"' "$manifest" > "$consumer/publish/package.json"
jq '{name,version,lockfileVersion:3,packages:{"":.}}' "$consumer/publish/package.json" > "$consumer/publish/package-lock.json"
git -C "$consumer" add -- publish
pass
bash "$ROOT/scripts/ci/check-dependency-pins.sh" --consumer "$consumer" --npm-root publish \
    --node-version 22.13.0 --npm-version 10.9.2 > "$fixture/output" 2>&1
printf '{bad JSON\n' > "$manifest"
bash "$ROOT/scripts/ci/check-dependency-pins.sh" --consumer "$consumer" > "$fixture/output" 2>&1
echo 'npm declaration, selection, exception, preservation and offline tests passed'
fixture_complete=true
