# Storage inspection

The `blob` binary from `ic-blob-storage-cli` has two separate storage views. Neither
enumerates an entire network or discovers all installations for a principal.
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

## One Caffeine account or owner

```sh
blob provider-inspect \
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

## Offline enrollment proposal

```sh
blob prepare-enrollment \
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

`make test-cli` checks independently encoded Cashier replies (including required
fields, wrong account, denied/missing account, malformed/oversized input), exact
integer preservation and offline debit bounds. `make test-local` checks uploader
filtering, application-only access, shared roots and historical release replay.
`make test-management` runs the actual CLI against an installed host with 64 upload
and reference names, 63 distinct roots, denied/empty views, untrusted-root refusal, no query writes and
closing history preserved through a drained upgrade. These are local witnesses;
they do not qualify Caffeine's accounting semantics or a live successful account read.

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
