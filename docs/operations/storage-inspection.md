# Storage inspection

The `blob` binary from `ic-blob-storage-cli` inspects a selected Blob installation,
reference application or Caffeine account. It does not enumerate an entire network
or discover all installations for a principal.
Run `make blob` from the repository root after `make install-tools`. It builds and
installs the development executable using Cargo's standard installer, normally
at `~/.cargo/bin/blob`, so the examples below use `blob` directly. Run `make blob`
again to update it from this checkout, including unreleased source changes.
Cargo refuses a conflicting executable owned by another package.

Rustup normally puts Cargo's bin directory on PATH. Cargo honors
`CARGO_INSTALL_ROOT`, `install.root` and `CARGO_HOME` overrides and warns if its
selected bin directory is missing from PATH. For the default location, add
`export PATH="$HOME/.cargo/bin:$PATH"` to your shell configuration if needed.
`make build-cli` remains build-only and produces the local artifact for qualification;
tests and CI do not install into your user bin directory.

## Find a command

Start with `blob --help`, then use a group's help for its operations:

| Command | Purpose |
| --- | --- |
| `blob overview` | Inspect one uploader in a Blob installation |
| `blob app catalog` / `draft` | Inspect the signing user's reference-application records |
| `blob app prepare-installation` | Prepare paired fresh-install inputs offline |
| `blob provider inspect` | Inspect one Caffeine account |
| `blob provider prepare-enrollment` | Prepare an offline enrollment proposal |
| `blob certificate --help` | Prepare, dispatch once, then recover the original certificate request |
| `blob transfer claim` | Authenticate proof and claim one transfer offline |
| `blob maintenance --help` | Advanced host inspection and upgrade operations |

Use `--help` after any operation for its required inputs. Since development
0.100.11, the former flat commands use these groups: for example,
`recover-certificate` becomes `certificate recover`, and `prepare-drain` becomes
`maintenance prepare-drain`. Update scripts when updating the CLI; old command
spellings are not aliases. Existing custody records and request identities remain
unchanged. Maintenance's prepare/dispatch/observe stages remain explicit.

The grouping reviewed the predecessor's argument routing at
[`25bed135`](https://github.com/dragginzgame/ic-blob-storage-legacy/tree/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-cli/src/native/arguments).
It retains explicit operation selection before execution. No legacy command
implementation or provider/recovery state was imported.

## One uploader in a Blob installation

```sh
blob overview \
  --url https://icp-api.io --canister "$BLOB_CANISTER" \
  --principal "$UPLOADER" --identity "$APPLICATION_PEM"
```

`--principal` selects the uploader bound to historical grants. The signer must be
the installation's configured **application authority**. Manager, controller and
uploader status alone do not grant this inventory permission. If that authority
is an application canister, the application must expose its own authorized
inspection path; a native CLI cannot sign as a canister. No such forwarding
endpoint is added to the reference application or Canic by this command.

The report contains distinct associated roots, matching upload IDs, declared bytes,
original upload ownership, local retention and all application reference IDs/states
for each selected root. It includes reserved, cancelled/expired and closing history.
References belong to the application; association with an uploader is not personal
ownership. Shared roots can appear in several uploaders' reports. Each report counts
a root once, but reports must not be summed across uploaders.

Reserved, live and unresolved bytes are separate declared counters, not quantities
to add as total physical storage. Final release removes live bytes and preserves
unresolved bytes. Local registration is not evidence of a stored file. Filenames,
catalog publication and verified provider availability are outside this inventory.

The library derives the view from its existing records; it writes no index or new
ledger. The standalone service returns its entire bounded history (at most 64 uploads
and 64 reference names) in one observation. There is no cursor or partial-success
mode. Older hosts without the appended Wire inspection variant reject the command;
the CLI does not guess an inventory. No durable format change or migration is needed.

## Your reference-application catalog and drafts

For a new local application, the [installation-input guide](../design/reference-application.md#prepare-fresh-installation-inputs)
describes offline preparation. Preparation performs no installation or inspection.

Use the reference application's canister ID and the owning user's key:

```sh
blob app catalog \
  --url https://icp-api.io --canister "$APPLICATION_CANISTER" \
  --identity "$USER_PEM"

blob app draft \
  --url https://icp-api.io --canister "$APPLICATION_CANISTER" \
  --identity "$USER_PEM" --id "$DRAFT_ID"
```

For an owned local replica, use its loopback `--url` and supply the independently
trusted `--root-key` DER file. The CLI never fetches a replacement trust root.
`catalog` lists only this user's Published drafts; `draft` inspects the original
nonzero ID, including Draft, Cancelled and Cleaned records. An empty catalog says
nothing about unpublished drafts, provider storage or other users. Knowledge of
another user's ID does not grant access, even if both users are allowed by the app.
Unknown and denied requests fail without printing a partial report.

The report shows the saved asset name, original root, declared length, first
reference ID and current application disposition. Asset names are escaped for
terminal display. `Cancelled` means cleanup remains pending; `Cleaned` means the
application finished reference cleanup or established that the exact grant expired
without admission. Neither status proves provider deletion or billing settlement.

These are signed queries with reply-signature verification, not consensus-certified
observations or current Blob liveness checks. There is no fallback to another
principal, host or endpoint, and no create, publication, cancellation or provider
effect. The CLI directly reuses the reference application's existing protocol
types; no new endpoint or catalog ledger is added. It cannot sign as an application
canister to provide `blob overview`'s application-wide inventory.

## One Caffeine account or owner

```sh
blob provider inspect \
  --url https://icp-api.io --cashier 72ch2-fiaaa-aaaar-qbsvq-cai \
  --principal "$ACCOUNT_PRINCIPAL" --identity "$AUTHORIZED_PEM"
```

This calls only `account_info_get_v1`. It checks the returned account principal and
required storage fields, and displays Caffeine's reported blob count/bytes, chunk
count/bytes, cycle balance and last-charge timestamp. These gauges are provider
reports, not an independently verified file inventory or final bill. The method
does not return individual roots, filenames or URLs. A payer may fund separate owner
accounts; this command does not aggregate their storage into the payer's totals.

Without `--identity`, the query is anonymous. On 2026-11-15, the new CLI's anonymous
query for our funded payer returned `NotAuthorized`; no storage totals were obtained.
Use an identity the provider permits for the account. The CLI accepts an existing
Ed25519 PKCS#8 PEM; it does not export keys from a named `icp` wallet. Do not export
the local `canic-mainnet` key merely to use this command. Named-wallet integration
is outside this first inspection slice.

Missing accounts, authorization refusal, wrong returned principals, malformed or
oversized replies and transport failures exit unsuccessfully. None becomes zero
storage. The adapter uses Platform's bounded external Candid codec; owned Blob
endpoints continue to use Wire.

Both query commands verify IC query signatures using the built-in mainnet root,
or an explicitly supplied `--root-key trusted-root.der`. Loopback HTTP requires
that key, obtained from your owned simulator; neither command fetches a new trust
root from its endpoint. Queries have a 30-second deadline. They are observations,
not consensus-certified state receipts or permission for a subsequent mutation.

## Prepare and dispatch an original certificate request

These experimental commands target the root-only issuer profile in the local
qualification host. The product standalone host does not export it. Authorize the
exact root through the application first; preparation itself makes no network call
and establishes no grant, enrollment or provider acceptance.

```sh
blob certificate prepare --url "$IC_API" --canister "$ORIGINAL_ISSUER" \
  --uploader "$UPLOADER" --identity "$UPLOADER_PEM" --root "$ORIGINAL_SHA256_ROOT" \
  --custody "$CERTIFICATE_CUSTODY"

blob certificate dispatch --url "$IC_API" --canister "$ORIGINAL_ISSUER" \
  --uploader "$UPLOADER" --identity "$UPLOADER_PEM" --root "$ORIGINAL_SHA256_ROOT" \
  --custody "$CERTIFICATE_CUSTODY" --checkout "$BLOB_CHECKOUT"
```

Supply `--root-key trusted-root.der` to both commands for the owned local simulator.
All operators/uploaders must use one trusted custody root. Below it, the trusted
network-key hash, issuer and canonical content root select one permanent claim;
changing uploader, URL or request filename does not create another lifetime.
Use a physical custody path without `..` or symlink components; Host FS rejects
those paths before parent creation. Existing parent permissions remain operator-owned.
Preparation signs a 120-second ingress and durably retains a bounded Wire intent
and exact CBOR packet before dispatch is possible. It prints the request path for
recovery. Any existing claim, including partial preparation, refuses replacement.
There is no reset, expiry renewal or automatic cleanup. Preserve stranded custody.

The native CLI uses Host FS/Artifacts 0.80.1. Request records and dispatch markers
are published through a held parent directory with mode `0600`, create-only
visibility and confirmed synchronization. A stopped publication may retain
`.NAME.pending` beside its destination; preserve it, including if no destination
is visible. Existing stages refuse reuse. A known occupied destination refuses
before staging; a concurrent destination is still protected by create-only commit.
Visible-but-unconfirmed publication reports failure and never permits dispatch.
The same policy applies to maintenance and recovered-proof files. Existing record
names and Wire formats are unchanged. Ordinary operator input paths are resolved
once before bounded regular-file reads; custody reads instead use their held
directory without following child symlinks. These handles do not fence hostile
writers: operators must exclude concurrent namespace/content replacement.

Dispatch admits the checkout's tooling, original bindings/packet and remaining
ingress lifetime, then durably creates a private `dispatch.claim` before invoking
the Agent. Any existing marker, even empty, bars another invocation across CLI
processes. A timeout, rejection or lost reply does not release it. Use original
request recovery regardless of the dispatch exit status; success alone does not
establish a successful issuer reply.

**One dispatch invocation is not one HTTP attempt.** The selected Agent 0.49.3
disables TCP retries here but internally retries HTTP 429/503 up to six times,
using the same signed packet/request ID. No application retry or fresh signature
is issued. This IC ingress behavior must not be transferred to provider tree/chunk
PUTs, whose replay contract remains unqualified. Custody assumes trusted, preserved
filesystem state: changing custody roots, deleting claims or restoring old copies
can defeat local arbitration and is unsupported. A retained signature is private
authority, not something this command can revoke.

## Recover an original upload certificate

```sh
blob certificate recover \
  --url "$IC_API" --canister "$ORIGINAL_ISSUER" --uploader "$UPLOADER" \
  --identity "$UPLOADER_PEM" --root "$ORIGINAL_SHA256_ROOT" \
  --request original-request.cbor --output recovered-certificate.cbor
```

This experimental command requires an already retained signed ingress envelope
and independently trusted original issuer, uploader and canonical root. It accepts
the root-only `_immutableObjectStorageCreateCertificate` profile qualified locally;
the product standalone host does not yet export that endpoint. The caller supplying
the packet remains responsible for retaining it **before** any original submission.
Recovery never dispatches that packet; the separate commands above own preparation
and guarded dispatch. Existing externally retained original packets remain usable.

The Ed25519 identity must match the original uploader. Only a signed IC read-state
request is sent. The CLI derives the original request ID, verifies the current
IC certificate (including delegation and freshness), and requires its reply to
name `upload` and the expected root. Expired ingress is still inspectable while
the network retains its outcome. Pending, rejected, unknown or pruned outcomes
refuse without creating a certificate; none authorizes a fresh request.

Mainnet uses Platform's built-in root key. For a local simulator, supply an
independently obtained `--root-key trusted-root.der`; loopback HTTP requires it.
The command never fetches trust from the endpoint. Input is limited to 8 KiB,
decoded replies to 1 KiB and encoded certificates to 128 KiB, within the existing
2 MiB HTTP response budget and 45-second command deadline.

The certificate is Platform-encoded CBOR preserving the authenticated tree and
signature, not necessarily byte-identical network CBOR. Host FS publishes it
durably with create-new semantics and mode `0600` under trusted parent custody;
Host FS may create missing parent directories.
Existing files, including partial evidence, are never overwritten. On any output
failure preserve both the original request and any output; a failure after file
publication does not prove the file is absent. No client journal or recovery schema
is needed for recovery. Keep potentially usable authority private even after local final release.

Success authenticates a **historical reply**, not current retention or permission
to upload now. Caffeine acceptance of this recovered proof and the lifetime of its
authority remain unqualified. Local final release does not revoke an old certificate.
Neither does the original ingress deadline: a later certificate may authenticate
the same earlier reply. Certificate time is observation time, not an upload-expiry
promise; see the [lifetime witness](../design/caffeine-contract-review.md#authority-lifetime-separate-clocks--development-01009).

## Claim one transfer offline

After the original CLI preparation, dispatch and proof recovery:

```sh
blob transfer claim --url "$IC_API" --canister "$ORIGINAL_ISSUER" \
  --uploader "$UPLOADER" --identity "$UPLOADER_PEM" --root "$ORIGINAL_SHA256_ROOT" \
  --custody "$CERTIFICATE_CUSTODY" --certificate recovered-certificate.cbor \
  --certificate-sha256 "$FROZEN_PLAN_CERTIFICATE_SHA256" \
  --gateway "$SELECTED_GATEWAY_ORIGIN" --project "$SELECTED_PROJECT" --bucket "$SELECTED_BUCKET"
```

For loopback, also pass `--root-key "$TRUSTED_ROOT_DER"`. The certificate digest is
the `sha256:...` value from the [client's frozen plan](../../clients/browser/README.md#application-boundary),
not an independently reselected proof. This command makes **no network call**.
It checks original intent/packet/dispatch identity, authenticates the certificate
with Platform's Agent (including delegation and its freshness policy), and requires
the original request's certified `upload` reply to name the selected root.
The application must separately admit the current permission and provider binding.

The existing network/issuer/root custody directory exclusively owns a `transfer/`
child. Host FS creates it once, then durably publishes bounded `intent.wire` and
the exact private `certificate.cbor` before success. The intent binds the original
request, gateway, project, bucket and proof digest; root/uploader/trust are retained
by the parent intent. Partial publication leaves the claim consumed. Preserve it
after any failure or cancellation; another gateway, proof or process cannot acquire
a second attempt through this custody. All consumers must use the same trusted
custody root. Deleting, replacing or rolling it back is outside this guarantee.

A native client's `claim(plan, signal)` callback returns true only after command
success. That admits one client attempt; it does not prove that a PUT occurred,
authenticate a gateway response, grant current application permission or establish
provider acceptance. This adds no transfer retry, reset, progress journal or
automatic certificate issuance. Browser-only callers need their own durable store.

## Offline enrollment proposal

```sh
blob provider prepare-enrollment \
  --url https://icp-api.io --cashier "$CASHIER" --ledger "$CYCLES_LEDGER" \
  --payer "$PAYER" --owner "$FRESH_EXISTING_OWNER" \
  --deposit-subaccount "$CASHIER_RETURNED_SUBACCOUNT_HEX" \
  --deposit-cycles 10000000000 --ledger-fee-cycles 100000000 \
  --max-debit-cycles 10100000000 --daily-limit "$PROPOSED_DAILY_LIMIT" \
  --expires-at-ns "$PROPOSED_EXPIRY_NS"
```

This prints a reviewable proposal to stdout without loading a signer, contacting
the network, signing a request or spending cycles. Values above illustrate the
deposit arithmetic, not permission to repeat the already completed funding witness.
The existing fresh owner, exact payer-specific deposit address, actual ledger fee,
daily-limit units and expiry must be independently checked. The proposal rejects
overflow or a deposit plus fee exceeding the selected debit bound.

The steps cover inspection, a possible deposit, acknowledgment/notification, a
proposed payer/owner relationship and owner/gateway inspection. They do not implement
those effects or establish enrollment readiness. Actual execution still requires
durable original-request custody under the [provider contract review](../design/caffeine-contract-review.md).
Owner creation, relationship effects, continuing charges and other fees are outside
the deposit bound. Generic wallet funding remains with the platform's operator tools.

## Qualification and legacy carryover

`make test-cli` checks retained certificate-request/reply bindings and independently encoded Cashier replies (including required
fields, wrong account, denied/missing account, malformed/oversized input), exact
integer preservation and offline debit bounds. `make test-local` checks uploader
filtering, application-only access, shared roots and historical release replay.
`make test-management` runs the actual CLI against an installed host with 64 upload
and reference names, 63 distinct roots, denied/empty views, untrusted-root refusal, no query writes and
closing history preserved through a drained upgrade. These are local witnesses;
they do not qualify Caffeine's accounting semantics or a live successful account read.
`make test-callbacks` also runs fresh CLI processes against the local issuer after
final release: recover from durable original-request bytes, refuse unavailable or
mismatched outcomes and an untrusted key, preserve partial/existing output, and
independently authenticate the saved certificate. With Node 24+, it also connects
the recovered proof to the actual `transfer claim` command and owned loopback PUTs:
corrupted proofs refuse, a complete read verifies against the original declaration,
and a fresh process cannot replay the transfer. Final release retains exposure.
This does not simulate power loss, browser deployment or provider acceptance.
`BLOB_PREVIOUS_CLI` can select a retained flat-command predecessor (through
0.100.10) only for certificate preparation in that fixture; dispatch, recovery and transfer claims still use
`BLOB_CLI`. Wire 0.24 was checked against earlier 0.22 custody, and Host 0.80.1
against the released Blob 0.100.10 CLI using Host 0.79.0. Neither introduces a
runtime fallback reader or changes live custody.

Targeted legacy review at
[`25bed135`](https://github.com/dragginzgame/ic-blob-storage-legacy/tree/25bed1352f3d2b9659f0d2583f50fa9af859ea84)
covered the replicated account adapter and its source/owner/payer rejection tests
(`crates/ic-blob-storage/src/ops/caffeine/query/transport/replicated/account/`),
service upload-history projection, and CLI upload-history regression tests
(`crates/ic-blob-storage-cli/src/native/upload_history/tests/mod.rs`). Retained
lessons are explicit source/principal binding, bounded decoding, full-width IDs,
local-state labels and refusal rather than empty success. This implementation uses
the rewrite's existing grant/reference owner. The predecessor's paginated tenant
inventory, cursors, funding engine and recovery custody are not imported: this
host has one application and a finite 64-record history, and this CLI has no new
provider-effect path. No legacy source code was copied.
