# Bounded registry metadata adoption for pending 0.22.2

Reviewed 2026-10-10 on released Blob 0.22.1,
`b6f4d7e83a1b301afe3619ccf3fadfc3503931c6` (validated source
`52661e437ee0968eca4dbd83640588b3d3c62d93`). The owning issue is
[#45](https://github.com/dragginzgame/ic-blob-storage/issues/45).
Package versions and the finalized receipt remain unchanged. Released Cargo
already selects Memory 0.35.0 and Metrics 0.5.0; the preceding handoff describes
an older qualified graph, not those released inputs. Preserve the incoming
direct Host artifacts/fs 0.12.1 lock update. Testkit 0.31.0 and its private Host
0.11 graph remain selected. Delivery checks must capture these actual inputs.

All selected archives match their lock checksums and packaged Rust/original
manifest files match both immutable owner source and the extracted cache:
Memory 0.35.0 at `13c664158e652bb7b42a26da3cac072aa1467141` (77 files), Metrics
0.5.0 at `01549632c0c3fa6e1ff315ce4de803ddfae904ad` (eight files), and direct
Host artifacts/fs 0.12.1 at `e5ecfa06c14d144cfeb85ea89d65906b1bf81636`
(22/23 files). Consumed Rust source/tests are unchanged from Memory 0.34.1,
Metrics 0.4.0 and Host 0.12.0. This is source evidence, separate from new graph
execution. `dependency-source.json` retains archive digests and file inventories.

## Source and ownership

Canonical export from a clean detached source selects the metadata helper and
its focused fixture from already-reviewed Shared Tooling 0.3.0,
`88a73139a0f083344c41a6f6f4b5c3a8aca7dc1d`. The snapshot now selects 98 files;
no mutable sibling source or exporter integration suite is imported.
Shared's helper has prior three-host acceptance at
[0.2.11](https://github.com/dragginzgame/shared-tooling/actions/runs/38034912323).

Both actual metadata callers delegate bounded transport and exact metadata
validation to that owner. Only a successful observation of absence permits
Cargo to attempt publication; transport errors, other HTTP statuses and invalid
metadata refuse. Blob rejects yanked rows and still authenticates the static
archive checksum, Git release source, clean VCS record and package path before
skipping an upload. Dry-run dependency availability and contracts-before-core
ordering remain consumer-owned; Cargo owns upload and index waiting.

Metadata requires jq and curl 8.4 or newer. Each observation uses a new private
directory under the printed publication attempt, retaining request URL, curl
version, response bytes, HTTP status, transport exit and diagnostics. Shared
supplies its identifying API User-Agent, with HTTPS-only redirects and implicit
curl configuration disabled. Archive transport retains Blob's identifying
version/repository User-Agent. No credentials, local publication journal, retry
loop or additional polling are introduced.

## Focused qualification and live observation

Raw readbacks and logs remain at `target/review-validation/issues0222/`.
Shared's registry fixture, prepared-tool admission, Bash syntax and ShellCheck
pass, including the Shared registry fixture under genuine Linux-built Bash 3.2.
That is not native macOS evidence. The complete isolated Blob release-adapter
suite passes, covering missing
and matching versions, malformed/oversized/multiple JSON, incomplete rows,
foreign package/version, nonboolean/yanked rows, archive checksum/source/dirty
VCS/path mismatches, HTTP/transport failures and insufficient curl. A failed
transport printing 404 still refuses upload; its status, exit and partial
response remain retained. A malformed paired-dependency read stops the core
dry-run after the contracts dry-run. Existing publication-order, lost-reply,
retry and archive-authenticated skip cases remain passing. Git, Cargo and
registry effects in this suite are substitutes; they establish adapter
behavior, not live upload or provider behavior.

Two separate bounded read-only observations use the actual Shared helper for
released contracts/core 0.22.1. Both return exact present, un-yanked metadata
(HTTP 200, transport exit 0). Their original responses and diagnostics remain
in the `live-ic-blob-storage-contracts-0221` and
`live-ic-blob-storage-0221` directories. These are live registry metadata facts,
not archive/source authentication, publication effects or service qualification.
No failed or inconclusive live request is omitted or rerun.

## Delivery acceptance

The release gate's actual routing, Shared runner and complete consumer adapter
checks pass separately. Their unchanged results are reused while the remaining
configured `make ci` targets run on the captured graph; no older runtime result
is relabelled. The complete delivery gate passes: 643 native cases, the full
PocketIC target including 73 standalone and 123 storage cases, Wasm checks and
fresh extracted-package tests. Explicit opt-in browser/resource/older-release
cases remain ignored. Full Rust 1.88 native/Wasm workspace checks also pass.
All 14,696 captured compiler inputs and 1,707 captured project files remain
unchanged during validation; the final documentation attestation is checked
separately. Logs, inputs and actual target selection remain in the packet.
Committed consumer native macOS acceptance
for this dirty source remains open in #45. No sibling edit, agent commit/push,
release/publication or deployed/paid provider effect is performed.
