# Native cleanup and Shared 0.11.0 adoption

Date: 2026-10-13. Base Blob source: `7ad76317fb1ed2695c16ede4aabee655ded84187`.
This batch preserves the prior uncommitted Candid cleanup and Host 0.17.3 lock
selection. Incoming Memory 0.36.3 is preserved and fetched with `cargo fetch --locked`.
No package versions, index, release refs or sibling source files were changed.

## Owners and preserved contracts

- Host `Sha256Digest` parses lowercase inventory/browser hashes. Inventory
  decoding carries typed body and metadata digests through later comparisons;
  JSON still contains the original unprefixed lowercase format. The existing
  exact-length uppercase refusal test now uses 64 characters, isolating the case
  requirement from length rejection. Reopened files still undergo content checks.
- The local-body verifier constructor owns CLI frame/header ceilings for both
  file snapshots and provider reads. File EOF and HTTP status, encoding, deadline,
  request budget and retained outcomes remain with their respective adapters.
- Authenticated CLI fixtures own one artifact/proxy/TLS command constructor.
  Host owns child groups for ordinary output, interrupted observations and
  concurrent submissions. Interactive sessions retain their buffered reader;
  interruption and concurrency assertions are unchanged.

## Snapshot and consumer adapters

Remote main and the clean source clone select Shared **0.11.0** at
`a15ff323449f4b6190f647aec0de43cbcfb314ba`. Canonical preview/export records
115 payloads. Dirty newer sibling changes were excluded. Shared catalogs and
Make selection stay unchanged; existing installed tools pass offline admission.
No removed per-tool CI installer was a Blob entrypoint or caller.

The Node-only build adapter and Node/npm tool adapter now delegate to the shared
checker. Blob retains browser pins and explicit setup. Hosted evidence download
uses the exact upload ID, current repository/run and upload ZIP digest, followed
by the original inner archive checksum. Failed download attempts are uploaded
separately. The former discovery script is retired by the exporter.

Shared's download fixture includes assertions for its own `.github/workflows/ci.yml`;
it therefore runs in the clean source clone, not Blob's snapshot. The attempted
consumer execution and its retained failure remain recorded. Blob's workflow is
checked with actionlint; actual GitHub artifact transport remains hosted proof.

## Qualification

Logs: `.tmp/review-validation/next-three0233/`. The before-image for task-only
comparison is `/tmp/blob-next-three-before`; clean Shared source is
`/tmp/blob-shared0110`. No full Blob CI or release command was run.

Completed checks include 113 CLI tests, strict Clippy on both changed packages,
Shared consumer fixtures and installed-tool admission, Node/npm and Node-only
adapters, upstream download refusal/retry fixtures, snapshot integrity, ShellCheck
and actionlint. Fresh standalone/storage Wasm and CLI artifacts use the current
lockfile: seven installed session and three observation/submission cases pass.

Failed attempts are preserved: wrong CLI target selection, a test-only missing
import after moving verifier imports, initially uncached Memory 0.36.3, a helper
placed too broadly across integration targets, sandbox socket/npm restrictions,
and the owner-specific download fixture. Successful reruns do not overwrite them.
Native macOS, live provider behavior and downstream application acceptance are
outside this qualification. Host retained filesystem staging remains pending;
only the Node portion of the previous consumer patch is adopted here.
