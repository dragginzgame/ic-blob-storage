<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Local preparation and PocketIC tools

These examples run from the repository root. Offline preparation reads local
files. Fixture commands attach to an already-running PocketIC instance with
explicit simulated callers; they do not provide production authentication.
See [dependency setup](dependencies.md) for the local server.

| Task | Tool |
| --- | --- |
| Build a file declaration or inventory | `prepare_upload` example |
| Save exactly the bytes that were hashed | `prepare_upload --snapshot` |
| Check downloaded bytes against a trusted claim | `verify_download` example |
| Observe inventory and service capacity | `blob-fixture-inventory` |
| Prepare and inspect exact reference intent | Native `reference-inputs`, `reference-receipt` |
| Inspect local state and blockers | `blob-fixture-status` |
| Preview or request a controlled balance read | `blob-fixture-refresh` |
| Preview or request gateway membership sync | `blob-fixture-sync` |
| Preview funding or recover an exact outcome | `blob-fixture-funding-preview`, `blob-fixture-funding-lookup` |

Fixture outcomes are local test evidence. Live provider behavior and operational
recovery remain separately qualified in the [acceptance plan](acceptance-plan.md).

## Install the native and browser tools

The supported source path uses one explicitly selected checkout for the native
CLI, browser modules and patched SDK build. Select a clean release tag, retain
its commit identity and keep that checkout with the installed tools. The CLI and
private browser package are not registry distributions; the separate library's
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

`--version` returns JSON with `tool: "blob-storage"` and the compiled library
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
  .tmp/browser/publication-host.js .tmp/browser/publication-worker.js \
  clients/browser/launcher.mjs clients/browser/native.mjs \
  clients/browser/session.mjs clients/browser/package.json \
  clients/browser/patches/caffeine-1.1.2.patch \
  > "$BLOB_TOOLS_ROOT/SHA256SUMS"
```

The existing bundler verifies original SDK hashes and peer versions, applies the
maintained patch to a build copy, and emits the host/worker bundles. Installed npm
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
cargo run --offline --locked -p ic-blob-storage --example prepare_upload -- \
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
restore fence. The unpublished
`blob-fixture-inventory` command connects a prepared report to these queries on an
already-running local admission probe:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests \
  --bin blob-fixture-inventory -- inspect \
  --server 127.0.0.1:PORT --instance INSTANCE --canister SERVICE \
  --caller TENANT --tenant TENANT --namespace NAMESPACE \
  --inventory prepared-inventory.json
```

Use the output of `prepare_upload --inventory`, or a snapshot's `inventory.json`.
The client requires explicit local targets and a simulated caller equal to the
tenant. It bounds input to 8 MiB, 4,096 assets, 2,048 distinct blobs and 8,192 total
distinct leaves, with at most 16 headers / 4 KiB framed metadata per object. It
checks manifest/root consistency, mappings and recomputed totals before any query;
it never opens source/body paths. There is one outstanding query at a time, at most
one capacity query and two queries per distinct root, with bounded reply decoding.

The JSON report marks a restored service as blocked even with spare quota,
including a fence observed by the later reference-capacity query. It
distinguishes content not visible to this tenant, unfinished
operations to recover, live blobs requiring new references, and retired roots.
It budgets one fresh reference per asset for existing live blobs. New-object
reference capacity remains unassessed. Not-visible content is not proof of global
absence or upload eligibility. Aggregate new-byte/object/leaf demand and concurrent
upload headroom are separate: a sequential batch need not fit every upload at once.
Observations are sequential and can become stale; no root, quota or reference is
reserved, and file bytes are not reverified. Exit 0 means a complete observation
with no observed blocker, 4 reports blockers, 2 rejects arguments, and 3 reports
input/query/reply failures without partial results. Production authentication,
provider transport and operation persistence remain outstanding.

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
cargo run --offline --locked -p ic-blob-storage --example prepare_upload -- \
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
cargo run --offline --locked -p ic-blob-storage --example verify_download -- \
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

## Inspect a running PocketIC fixture

The separate unpublished `blob-fixture-status` client attaches to an existing local PocketIC
instance. It requires a literal loopback address, instance ID, canister and
simulated caller; there are no inferred identities or targets. For example, with
these variables set from your running fixture harness:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-status -- \
  status --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_CANISTER" --caller "$FIXTURE_OPERATOR" \
  --kind authority --namespace 1
```

For the funding fixture use `--kind funding --peer "$FIXTURE_PEER"` instead of
the authority kind/namespace pair. `status` emits JSON and exits 0 on a valid read;
`check` emits the same report and exits 4 when reported blockers exist. Neither
assesses overall service qualification. Exit 2 means invalid arguments; exit 3
means transport, query, permission, decoding or binding failure. Unknown amounts
are `null`; full-width amounts and counters are decimal strings. The client calls
only the `operator_status` query, leaves the instance running and never falls back
to an update. It supplies no production authentication, discovery or provider access.

## Preview or refresh fixture balances

The separate `blob-fixture-refresh` tool previews or requests one controlled-source
balance read. Take the current revision and next lifetime attempt number from the
fixture status, and explicitly supply the configured source and account:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-refresh -- \
  dry-run --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_CANISTER" --caller "$FIXTURE_OPERATOR" \
  --kind authority --namespace 1 --source "$FIXTURE_SOURCE" \
  --account "$FIXTURE_ACCOUNT" --revision "$FIXTURE_REVISION" --sequence "$FIXTURE_NEXT_ATTEMPT"
```

Replace `dry-run` with `refresh` to request the local read. Preview grants no future
admission: the update checks the same bindings, revision, next sequence, capacity,
pending work and restore fence before persisting intent. Repeating a consumed
request cannot dispatch again. Both commands leave the existing instance running.
Refresh may advance simulator rounds; it attaches no provider cycles and never funds.

JSON separates `action` from `post_status`. Exit 0 means eligible preview or completed
refresh, 2 invalid arguments, 5 a typed operation failure, 6 unknown/uncertain outcome,
and 7 completed refresh with failed post-status. A typed failed observation may have
consumed its attempt; inspect status. No outcome triggers an automatic retry or a new
sequence. Failed/malformed update acknowledgements remain uncertain even if the
subsequent status read succeeds. Dry-run only queries `preview_balance_refresh`;
the existing status/check tool remains query-only. These are local test transports,
not production credentials or evidence of deployed Caffeine behavior.

## Synchronize fixture gateways

Gateway synchronization uses `--bin blob-fixture-sync -- dry-run` (or `sync`) with
the same explicit target flags, `--source`, `--revision` and `--sequence`, and no
`--account`. Read `sync_source`, `sync_revision` and `last_sync` from status; the
requested sequence is `last_sync + 1`. Sync revisions begin at 0. A null revision
means exhausted admission, not revision zero. Each revocation invalidates previous
previews, including when the member was already absent. Only a later explicit sync
may re-add it. Both action tools share the JSON/exit behavior described above.
The local reentrant fixture assigns its source canister the operator role; the CLI
must name that simulated caller explicitly. This is not a production role binding.

## Preview fixture funding

Funding admission has a passive preview:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-funding-preview -- \
  dry-run --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_SENDER" --caller "$FIXTURE_DRIVER" \
  --kind funding --peer "$FIXTURE_RECEIVER" --id 1 --amount 1000000 \
  --revision "$FIXTURE_BUDGET_REVISION"
```

Read `budget.revision` from funding status for `FIXTURE_BUDGET_REVISION` (initially
0). It changes on intent admission and completion, including full refunds. The
preview checks that revision, exact identity, protocol amount limit and full
amount without reserving or transferring cycles.
Exit 4 means a valid blocked preview; 2 means invalid arguments and 3 a read,
permission or binding failure. Exit 0 would mean unblocked observations, never
effect authority. The current fixture always reports missing funding prerequisites:
gross cycles are not authoritative spendability and transport acceptance is not
provider credit. Existing identities remain used even after full refunds. This
command cannot invoke the raw transfer experiment, override missing accounting
with flags or fall back to an update.

## Recover an exact fixture funding outcome

Use `blob-fixture-funding-lookup` to recover the retained result of an exact raw
experiment request after losing its ingress reply. For an original request with
ID 1, offered 1000000, accept 400000, reply InternalError and no callback trap:

```sh
cargo run --offline --locked -p ic-blob-storage-pocketic-tests --bin blob-fixture-funding-lookup -- \
  lookup --server "$FIXTURE_SERVER" --instance "$FIXTURE_INSTANCE" \
  --canister "$FIXTURE_SENDER" --caller "$FIXTURE_DRIVER" \
  --kind funding --peer "$FIXTURE_RECEIVER" --id 1 --amount 1000000 \
  --accept 400000 --reply InternalError --trap-callback false
```

Every original input is required. This command queries `lookup_funding` only.
Exit 0 means an exact retained transport observation; exit 4 means absent or
pending evidence. Exit 2 is invalid arguments and exit 3 a failed, denied or
conflicting lookup. None is a retry permit or provider credit. `Absent` means
missing from this journal, including an old restored backup; `Pending` also covers
callback rollback. Restored owners remain fenced even when an older journal omits
a paid operation. Lookups do not call the peer, reload stable memory, consume an
identity or change accounting. These are local PocketIC tools, not payment clients.

## Fixture funding accounting

The funding fixture requires `(peer, driver, FundingBudgetInput { allocated,
reserve, operating_reserve, other_liabilities })` at installation. Its positive
`reserve` limits transfer attachments;
status and preview label this `budget.scope = "local_attachment_budget"`. Available
allocation excludes accepted cycles and full unresolved attachments. Callback
refunds and proven unsent offers are reported separately. Incoming receipts or
added gross cycles never increase it. Separately, the transfer guard uses current
platform liquid cycles and call costs, preserving positive `operating_reserve`
slack and explicit `other_liabilities`. These holds cannot be reset or released by
a refund. Costs are sampled after intent persistence and before dispatch. A
`LiquidityBlocked` outcome consumes the identity, releases the unsent attachment
and reports no callback refund.
Callback trap controls apply only to actual callbacks; they cannot erase an unsent
refusal. Such refusals still consume the bounded journal's lifetime capacity.
The fixture uses the library's `model::billing::allocation::FundingAllocation`
for amount reconstruction. It requires complete sequential history and rejects
original reserve violations even if the eventual reply returned the full offer.

Preview `liquidity` figures are observations, possibly cached, and can change
without a budget revision. The update rechecks its own exact encoded call. These
local installed holds do not prove complete production liabilities, provider credit
or recovery; top-level spendability remains unknown. Restored owners stay fenced.

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
