# Current status — 2026-10-26

Source baseline: `64031753`, released 0.49.2. Package versions remain 0.49.2;
pending compatible notes target 0.49.3. The
[released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/64031753/docs/status/current.md)
retains upstream qualification; [history](history.md) retains older records.

## Private fixture Wire cut

The remaining private Candid RPCs, fixture lifecycle arguments, native diagnostic
requests/replies and owned inter-canister controls now use published Wire's native
codec through `blob-test-protocol::fixture_wire`. Frames bind method and direction;
installation/restoration have separate identities. Explicit unit replaces missing
inputs, positional results are exact, and old Candid packets have no fallback.
The [dependency guide](../dependencies.md) records identities and finite budgets.
Admission keeps its narrower input ceilings and measures total Wire decoding;
Candid header/value profiling and type-table-only tests are retired.

Previously migrated Wire document controls and their retained command documents
keep their existing representation. Public Blob service frames, endpoint names,
authentication and retry rules are unchanged. Private saved probe journals now
use bounded Wire as described below; production record codecs are unchanged.
Cross-release service transitions remain reinstall-only. No dual reader or
automatic storage migration is added.

The private protocol retains Candid derives only for two emulated provider request
shapes; saved-journal DTOs no longer require Candid. The duplicate journey
certificate is deleted; it uses Contracts' canonical provider response directly.
Emulated provider boundaries use explicitly named `provider_candid` hooks and
`provider_calls`, while private RPC uses Wire. Private chunk requests and replies
also use Wire. Bulk payloads use upstream `ByteBuf` / borrowed `BytesRef`, avoiding
per-byte node admission; saved fixture leaves now use bulk bytes too. Candid
gateway, Cashier, certificate/GC and management packets remain where they exercise external
contracts and the library's real bounded decoders. Provider substitutes do not
qualify Caffeine, independent credit, deletion timing or billing cessation.

## Private journal cleanup

Authority, funding, gateway-source and consumer probes persist their actual models
through `fixture_storage::FixtureJournal`, published Wire and Memory's canonical
`testing::FixtureCell`. Their allocation keys are distinct `.wire.v1` identities;
the storage frame binds its key under `BLOBJNL1`, separate from RPC/document frames.
Existing byte ceilings remain; Wire adds explicit allocation, node and depth
budgets rather than claiming equivalence to Candid work quotas. Source leaf
payloads use `ByteBuf` to avoid per-byte node admission for a full 1 MiB leaf.

The consumer's copied Candid persistence schemas, named JSON conversion bridge,
custom `CONS` envelope and redundant direct Core dependency are removed. It retains its
own models and the client/contract interfaces it actually consumes. Creation
requires empty backing, restoration refuses missing/corrupt/foreign data, and
same-release fences, persisted intents, uncertain callbacks and retries remain.
Earlier private journal images require reinstall. Historical opt-in consumer
upgrade tests now expect rejection with unchanged stable bytes and old heap,
not migration compatibility; their pinned Wasm inputs were not available here.
Production stable collections, record schemas and service contracts are unchanged.

## Qualification

Earlier RPC-cut evidence and retained failures:
`/tmp/blob-fixture-wire-20261026/`. These checks preceded the journal cut; the
journal-specific qualification below supersedes their saved-state scope.

Passing: all-target compilation and strict Clippy for the six probes, private
protocol and PocketIC package; protocol tests (11), host diagnostics (25), funding
request-size coverage (1), and fresh probe Wasm builds. Protocol coverage includes
malformed, method/direction, lifecycle identity, exact arity, bulk bytes and saved
journal refusal/reopen. Rust 1.91 checks the authority probe's Wasm library,
including the static Platform codec paths. A fresh CLI and standalone Wasm were
prepared for installed consumers.

Installed coverage across the scoped runs: admission (30), authority (2), content
(2), funding (19), gateway scheduling (3), journey (49), obligations (2), operator
(49), standalone (75), storage (122), uploads (2). Failed cases were corrected and
rerun, reusing passing evidence for unchanged paths. Final targeted reruns also
cover the chunked admission-upgrade refusal and current consumer installation/
restoration. Fifteen opt-in historical/resource cases remain ignored, including
old-image checks requiring pinned artifacts. Their original Candid initializer
is explicit and separate from current Wire initialization; there is no format
detection or fallback. Default deployment exclusions, dependency pins, Platform
boundaries, formatting, documentation links and diff checks pass.

Retained failed attempts first exposed an invalid IC method-name lookup during
updates, then per-byte node admission for large chunks and old chunked-upgrade
arguments. Hooks now use static const-generic identities; payloads use bulk byte
types and chunked lifecycle callers use Wire. Required Caffeine callbacks have
explicit Candid callers rather than the private Wire caller. Corrected scopes
pass; the earlier failures remain retained and are not relabelled as passes.

Current journal-cut evidence and retained failures:
`/tmp/blob-journal-wire-20261026/`. Passing: native tests for protocol and affected
probe models (36), strict all-target/all-feature Clippy for protocol, four journal
owners and their PocketIC callers, and fresh six-probe Wasm builds. Installed
scopes pass: journey recovery (33 unique cases, with the affected six source
cases rerun), funding (19), consumer transactions (19; two pinned old-image cases
ignored), and storage clients (12, covering manifests, cancellation, funding,
tenants and downloads). These cover maximum leaves, exhausted histories,
missing/corrupt journals, upgrade rollback, callback uncertainty, fences and
no-redispatch recovery. Native journal tests cover Candid/`CONS` refusal,
method/direction/key mismatch, truncation, trailing data, resource admission and
retained bytes after failed writes/creation.

Initial journal tests exposed Memory's required `.vN` key suffix and a too-small
synthetic test allocation policy. Installed maximum-leaf cases exposed
insufficient source-journal allocation headroom under Wire's conservative
accounting. Corrected keys and explicit policies pass; failed attempts remain
retained. Existing byte/object ceilings are unchanged. Dependency pins, Platform
boundaries, deployment exclusions, Rust/manifest formatting and diff checks pass.
Rust 1.91 cross-checks the consumer Wasm library with its new Memory cell path;
this is cross-compilation, not native macOS qualification. Removing the consumer's direct Core
edge does not remove Core transitively: private protocol conversion adapters still
consume real service model/policy types. JSON is absent from the consumer graph.

The root lock adds already-selected Wire/Memory edges to the private probes,
removes the consumer's Core/JSON edges, and removes the protocol's direct
`serde_bytes` edge in favor of Wire's canonical re-export. The now-unused workspace
`serde_bytes` declaration is removed; that package remains transitive where
required. Published dependencies remain Host/Build/Testkit 0.68.9,
Platform 0.6.19, Wire 0.7.18, Memory/Storage 0.51.5 and Metrics 0.16.3.
Shared Tooling remains 0.26.21. Prior source/tool evidence remains at
`/tmp/blob-upstream-20261026.CuhRRC/` with its original source scope.

## Ownership and remaining boundaries

Blob owns tenant authority, service state, provider accounting, retention,
deletion and recovery. Frameworks own their adapters and acceptance; no sibling
checkout is changed. Canic owns its remaining adoption and delivery. The earlier
read-only review found dedicated/embedded, generated-client, CLI and operator
recovery evidence; newer Canic Auth/source closeout remains Canic-owned.

The [native Agent advisory](https://github.com/dragginzgame/ic-blob-storage/issues/102)
remains unresolved. This cleanup changes no cryptographic guarantee. Full CI,
native macOS and live provider qualification are outside this focused pass.

No staging, commit, push, release, publication, deployment or sibling edit occurs
as part of this cleanup.
