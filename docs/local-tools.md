<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Local preparation and PocketIC tools

These examples run from the repository root. Offline preparation reads local
files. PocketIC qualification uses typed calls with explicit simulated callers; it does
not provide production authentication. The six private fixture CLI frontends
have been retired; use the maintained native CLI for supported operator commands.
See [dependency setup](dependencies.md) for the local server.

| Task | Tool |
| --- | --- |
| Build a file declaration or inventory | `prepare_upload` example |
| Save exactly the bytes that were hashed | `prepare_upload --snapshot` |
| Check downloaded bytes against a trusted claim | `verify_download` example |
| Prepare and inspect exact reference intent | Native `reference-inputs`, `reference-receipt` |

Fixture outcomes are local test evidence. Live provider behavior and operational
recovery remain separately qualified in the [acceptance plan](acceptance-plan.md).

## Install the native and browser tools

The supported source path uses one explicitly selected checkout for the native
CLI, browser modules and patched SDK build. Select a clean release tag, retain
its commit identity and keep that checkout with the installed tools. The CLI and
private browser package are not registry distributions; the contracts library’s
packaged `prepare_upload` example is not the publication driver.

From that checkout, select a fresh absolute installation prefix. This recipe
uses the development profile and the repository's own build cache:

```sh
set -e
BLOB_TOOLS_ROOT="$PWD/.tmp/selected-tools"
test ! -e "$BLOB_TOOLS_ROOT"
cargo fetch --locked
CARGO_TARGET_DIR="$PWD/target" cargo install --offline --locked \
  --path crates/ic-blob-storage-cli --bin blob-storage --debug \
  --root "$BLOB_TOOLS_ROOT"
"$BLOB_TOOLS_ROOT/bin/blob-storage" --version > "$BLOB_TOOLS_ROOT/version.json"
cat "$BLOB_TOOLS_ROOT/version.json"
rustc --version > "$BLOB_TOOLS_ROOT/rust-version.txt"
printf '%s\n' debug > "$BLOB_TOOLS_ROOT/build-profile.txt"
```

`--version` returns JSON with `tool: "blob-storage"` and the compiled contract
`version`, without reading identities or making requests. Compare that value
with the selected checkout's workspace release. It does not authenticate an
installed canister. An optimized build may omit `--debug`; retain that binary's
own hash rather than substituting a hash from another build profile.

Use Node 24 and the pinned browser dependencies from the same checkout:

```sh
npm ci --prefix tests/browser --ignore-scripts --no-audit --no-fund
node tests/browser/build.mjs
node --version > "$BLOB_TOOLS_ROOT/node-version.txt"
git rev-parse HEAD > "$BLOB_TOOLS_ROOT/source-commit.txt"
git status --porcelain > "$BLOB_TOOLS_ROOT/source-status.txt"
shasum -a 256 Cargo.lock tests/browser/package-lock.json \
  "$BLOB_TOOLS_ROOT/version.json" "$BLOB_TOOLS_ROOT/rust-version.txt" \
  "$BLOB_TOOLS_ROOT/node-version.txt" "$BLOB_TOOLS_ROOT/build-profile.txt" \
  "$BLOB_TOOLS_ROOT/bin/blob-storage" \
  target/browser/publication-host.js target/browser/publication-worker.js \
  clients/browser/launcher.mjs clients/browser/native.mjs \
  clients/browser/session.mjs clients/browser/package.json \
  clients/browser/patches/caffeine-1.1.2.patch \
  > "$BLOB_TOOLS_ROOT/SHA256SUMS"
```

The existing bundler verifies the published Platform dependency, one SDK identity,
original Caffeine hashes and its peer version, applies the maintained Caffeine
patch to a build copy, and emits the host/worker bundles. Installed npm
files stay unchanged. A consumer imports `launcher.mjs` and `native.mjs` from this
retained checkout and selects the installed binary and those bundle paths
explicitly. See the [callable driver](../clients/browser/README.md#follow-native-phase-guidance)
and [native process contract](../clients/browser/README.md#launch-chromium-from-a-native-parent).
Provision Chromium separately as described in [dependency setup](dependencies.md#browser-certificate-evidence).

The recipe records Node/Rust versions and the selected build profile with these
hashes. If you omit `--debug`, record `release` in `build-profile.txt` instead.
Keep original binary, bundles, profile and signed/effect history together for
recovery. Installing new tools never authorizes reopening old release artifacts,
replacing their identities, retrying uncertain effects, or registering assets.
The [isolated-source check](evidence/caffeine-probes/README.md#isolated-source-tool-installation--2026-10-04)
installs into a fresh prefix with fresh npm dependencies and builds without Git
metadata. The exact installed CLI and matching browser/Wasm tools complete real
local media and recover a lost reply without another PUT. That check captures
unreleased source and reuses the Cargo cache and provisioned Chromium/PocketIC;
it is not a cold machine, released-tag acceptance or consumer adoption. A later
workspace dependency update cannot relabel the frozen tools or their results.

The separate [clean-release check](evidence/caffeine-probes/README.md#clean-released-source-tool-installation--2026-10-04)
now verifies this recipe from `v0.14.3`, with a fresh native prefix and pinned npm
directory, no Git discovery and matching 0.14.3 CLI/Wasm. Original cached
GLB/WebP completes and PNG/JPEG recovers a lost reply without another PUT.
The prefix/build cache belong to this repository; downstream installation and
application registration still require consumer acceptance.

## Prepare one file

Local headless preparation computes the declaration needed for upload admission:

```sh
cargo run --offline --locked -p ic-blob-storage-contracts --example prepare_upload -- \
  declaration.json 10485760 10 < body.bin > prepared.json
```

`declaration.json` supplies `bytes` and original `headers` (`name`/`value` entries),
including canonical `Content-Length` equal to `bytes`. The two numeric arguments
bound content bytes and retained leaves. The example checks the shared service
metadata rules with local limits of 16 headers / 4 KiB framed metadata, reads a
16 KiB maximum declaration and uses a 64 KiB body buffer. Deployment limits still
need independent selection. Only a successful invocation yields a complete JSON
result: `claim`, `chunk_hashes` and `computed_content_digest`. The `claim` object
has the input shape used by `verify_download` below.

This is offline preparation, with no enrollment, certificate, provider upload or
completion claim. Preserve the exact source bytes and metadata for later upload;
the single-file mode does not copy the source or persist resumable operation intent. The
computed raw digest is diagnostic and is not required by service admission.

## Prepare an inventory

For multiple files, use `prepare_upload --inventory inventory.json`. Source paths
resolve relative to the inventory file. For example:

```json
{
  "limits": {
    "files": 100,
    "file_bytes": 10485760,
    "file_chunks": 10,
    "total_bytes": 104857600,
    "total_chunks": 100
  },
  "files": [{
    "asset": "example",
    "source": "body.bin",
    "declaration": {
      "bytes": 3,
      "headers": [{"name": "Content-Length", "value": "3"}]
    }
  }]
}
```

The inventory is capped at 1 MiB of JSON. All limits are required positive maxima;
aggregate budgets include duplicate sources. The tool reads files sequentially
and emits `totals`, separate `assets` mappings and one prepared entry per distinct
root in `blobs`, only after all succeed. Asset IDs must be unique; source paths
must be relative, without parent traversal or symlinks, and name regular files.
Use a controlled, unchanged source tree: these checks do not provide a filesystem
sandbox or freeze a snapshot. Distinct-root totals do not establish service
capacity, cross-tenant sharing, fresh-upload eligibility or provider charges.
Existing references, lifetime history and billing obligations require service
inspection; no actual service/tenant/namespace or operation identity is selected.

## Inspect fixture inventory and capacity

The shared `blob_upload_capacity` query supplies the next planning input:
tenant-scoped lifetime object, concurrent upload, manifest-leaf and byte headroom,
plus enrollment, per-object limits and the restore fence. The standalone host and
both admission fixtures reuse the same handler. It includes reservations
and continuing billing; freed logical quota alone cannot make those obligations
disappear. These independent counts reserve nothing. Existing blobs still require
the shared `blob_lookup_content` and `blob_reference_capacity` queries. Discovery
returns complete original identities and local lifecycle, with exact request echoes
and restore fencing even for absence. Reference capacity binds
the tenant and root, preserves cleanup receipt reservations and reports its own
restore fence. Typed qualification in
[the inventory scenario](../tests/pocketic/tests/admission_content/planning/inventory/mod.rs)
checks pending, live and retired content, reference headroom and tenant isolation.
These observations reserve no capacity and never authorize a retry.

## Prepare and inspect reference intent

Use the maintained native [reference commands](operator-guide.md#generate-reference-inputs-offline)
for original permission-bound inputs, signed submission and query-only receipt
recovery. Tests use genuine signed local tenant queries for those same commands.
The local multi-entry save/lock journal contract is retired; retain existing
artifact files and original identities rather than converting or redispatching them.
Historical success still does not prove current reference liveness or retry authority.

## Save a content snapshot

To save the exact hashed bytes for later use, add an existing, caller-controlled
destination directory:

```sh
cargo run --offline --locked -p ic-blob-storage-contracts --example prepare_upload -- \
  --inventory inventory.json --snapshot ./snapshots > snapshot.json
```

Success returns `directory` (a fresh absolute path) and `inventory`. The directory
contains `inventory.json` and `bodies/<root hex without sha256:>`, one file per
distinct root. Asset `source` paths in the report remain provenance; use the saved
bodies for later upload. Copying and hashing use the same buffers, so later changes
to the originals cannot change the saved content. Duplicate sources still count
against work limits and are fully checked. Disk use includes saved distinct bodies,
one in-progress body, and the report.

Normal failure removes the current attempt. Each repeat creates a separate
directory and preserves earlier snapshots. On Unix the directory is private (0700)
and body files are 0600. Files are synced, but this is not a portable crash-durable
transaction: interruption can leave incomplete residue or a complete snapshot
without a stdout receipt. No old directory is automatically resumed or deleted.
Keep the saved directory controlled and reverify its files before future effects;
this local copy is mutable and does not persist service operation identities.

## Verify downloaded bytes

The local streaming verification example can save the exact checked bytes:

```sh
cargo run --offline --locked -p ic-blob-storage-contracts --example verify_download -- \
  claim.json 10485760 verified.bin < body.bin
```

`claim.json` supplies `root`, `bytes` and `headers` (`name`/`value` entries) from
trusted application data; the second argument is the caller's maximum byte
budget. The example bounds the claim to 16 KiB and uses a 64 KiB receive buffer.
Omit `verified.bin` to verify without retaining the body. With an output path,
the example stages at most the declared length under the existing destination
directory, verifies clean EOF and the root, syncs the file, then publishes without
overwriting a file or symlink. Computed identities print only after success.
Use a caller-controlled directory with no concurrent path replacement or temporary
cleaner; staging uses owner-only directory/file permissions on Unix. Normal
failures clean staging, but interruption can leave residue. A missing stdout receipt
does not prove no file was published: reverify the existing output before using it.
The example does not authenticate the claim, resume interrupted downloads or
promise crash-durable directory updates. Persistence follows
[tempfile's platform guarantees](https://docs.rs/tempfile/3.27.0/tempfile/struct.NamedTempFile.html#method.persist_noclobber).
See the
[consumer download direction](roadmap.md#consumer-download-verification).

## Inspect production-path PocketIC fixtures

[Storage scenarios](../tests/pocketic/tests/storage.rs) and
[standalone account scenarios](../tests/pocketic/tests/standalone_account/mod.rs)
exercise actual operator/tenant endpoints, preserving absent economic observations
and comparing durable state across passive reads. The former authority probe's
private balance cache, preview commands and alternate restoration archives are
retired; they are not part of the production service contract.

## Synchronize fixture gateways

[Storage gateway transport scenarios](../tests/pocketic/tests/storage_gateways/transport/mod.rs)
use the production durable registry and explicit caller/source bindings. Held
replies cannot override revocation, cancellation or a newer attempt. Source
restoration keeps its captured reply fenced. The
[independent source readers](../tests/pocketic/tests/provider.rs) retain decoder,
caller and journal checks without another service implementation.

## Qualify production funding with a local receiver

[Storage funding scenarios](../tests/pocketic/tests/storage_funding/mod.rs) exercise
production preparation, assessment, lookup, dispatch, callback accounting, credit
confirmation and fenced recovery. The [transport cases](../tests/pocketic/tests/storage_funding/transport/mod.rs)
check exact Cashier arguments, call-specific refunds, advertised failures and
callback rollback. Query-only assessment remains separate from dispatch.

The funding probe is only a bounded receiver substitute, installed with
`(peer, driver)`. The driver configures exact expected argument bytes, the offered
and accepted cycles, and a canned reply. Only the configured peer can transfer;
receipts retain actual acceptance, never provider credit. Restored receivers are
inspection-only. The fixture has no outgoing payment, budget, preview or recovery
engine: the production Storage owner is the sole source of those semantics.
These local tests do not qualify deployed Cashier credit, billing or recovery.

## Admission workload measurements

`make test-admission-resources` uses the same Storage probe and production
`StableUploads` owner as rollback/recovery tests. Its explicit resource inputs
preserve Small, RetainedHistory, ReleaseHistory and ReferenceHistory workloads.
The private `fixture_admission_measurement` command measures bounded metadata
operations; it has one 16 KiB Wire envelope and a 32-entry diagnostic window.
Service ingress remains on the actual Blob endpoints and their existing bounds.
Private descriptor observations reuse Storage's existing controls. Resource
reports describe this fixture build, never deployed provider throughput or costs.

## Opt-in receipt-history resources

With the selected locked caches, Rust Wasm target and PocketIC binary prepared,
run `BLOB_FUNDING_RECEIPT_PROFILE="$PWD/.tmp/receipt-profile-unique"
make test-funding-receipt-resources`. The report directory must not already exist.
This scoped target builds the storage fixture and measures 100/1,000/10,000
synthetic credited lifetime intents through maintained journal operations. It
retains bounded population requests/results, selected confirmation/reuse refusal,
replay, synchronous same-image restore, fences and resource counters.

Each tier has a 180-second population budget and existing calls have 30-second
bounds. Failures and incomplete tiers remain intact; select another fresh report
for an explicitly chosen retry. The observer includes its overhead and reports
allocated Wasm memory, not live heap/RSS. There are no provider calls, paid cycles,
production timing thresholds or default CI scale workload. This does not qualify
provider receipts or consumer integration.
