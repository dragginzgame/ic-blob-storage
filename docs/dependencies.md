<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Dependency setup

## Current dependency owners

The root [Cargo catalog](../Cargo.toml) owns all direct dependency requirements;
all thirteen members inherit package versions and dependencies. The root
[lockfile](../Cargo.lock) owns this checkout's selected graph. Member manifests
select features and targets. Registry consumers resolve their own compatible
lockfiles; local qualification does not cover every future resolution.

The root catalog and lockfile own version selections; the capability table below
is not a second version catalog. The [current handoff](status/current.md) records
source-bound qualification and its limits. Testkit shares Platform's decoder-limit
identity without an old-family alias; retired Platform families remain excluded
from the workspace and deployment.

| Owner | Responsibility |
| --- | --- |
| Blob core | Tenant authority, references, quotas, records, provider accounting, retention, deletion and recovery workflows |
| Blob client | Explicit replicated role clients; no service storage, endpoints or lifecycle |
| Blob Contracts | Runtime-free DTOs, method catalog, Blob codec projections, content identities and verification |
| IC Memory | Allocation ledger, grants and exclusive runtime leases; the host supplies backing and one allocation pool |
| IC Storage | Governed stable collections and codec traits; the stable-structures dependency stays upstream; Memory/Storage share the IC Persistence release line |
| IC Platform | Owned entrypoint/runtime adapters, Principal, external Candid contracts/decoding, management DTOs and native Agent/parser/certificate identities |
| IC Platform browser package | Published `@tokostudios/ic-platform` owns SDK Core selection, canonical Principal admission, checked identity restoration and explicit Agent construction; Blob retains trust, intents and effect policy |
| IC Wire | Bounded method/direction framing for owned service contracts and separate passive document encoding |
| IC Metrics | Standalone report numeric types/transport and heap telemetry plus private probe arithmetic; no core storage owner or Canic dependency |
| IC Testkit | PocketIC client/server selection and managed lifetime; bounded decoding shares Platform types directly |
| IC Host artifacts/fs/process/tools | Native artifacts, descriptor-rooted reads, durable publication, processes, framing and interface inspection |
| IC Host HTTP | Bounded native probe response accumulation; request policy, error projection and evidence custody stay with Blob |
| IC Build | CLI source fingerprints and optional archive verification; absent from public libraries and production Wasm |
| PocketIC | Testkit-selected local IC execution through Platform; no separate Blob dependency or server catalog |

[IC Tooling](https://github.com/dragginzgame/ic-tooling) now owns Host, Build and
Testkit; [IC Persistence](https://github.com/dragginzgame/ic-persistence) owns
Memory and Storage. Package names and ownership boundaries remain unchanged.
Blob consumes published packages, without sibling paths or compatibility aliases.

### Replicated call execution

Published Platform **0.7.3**, selected in the current lockfile, supplies the raw
`cdk::call::Call`, `Response`, `CallFailed` and `WaitPolicy` API used by every
production replicated transport. There is no Blob executor or generic dispatch
engine to replace. The remaining adapters have distinct domain responsibilities:

| Consumer | Platform selection | Responsibility retained in Blob |
| --- | --- | --- |
| [Role clients](../crates/ic-blob-storage-client/src/transport/mod.rs) | Borrowed raw bytes, explicit bounded timeout, no attached cycles | Method-bound Wire encoding and projection of original reject codes versus local non-submission; callers retain intent and recovery |
| [Cashier funding](../crates/ic-blob-storage/src/ops/caffeine/funding/transport/mod.rs) | Owned prepared arguments, unbounded wait, complete attachment and current call cost | Exact persisted attempt binding, liquidity policy, matching-callback refund capture before decoding or another await, and provider reply interpretation |
| [Provider queries](../crates/ic-blob-storage/src/ops/caffeine/query/transport/replicated/mod.rs) | Borrowed raw arguments and explicit bounded timeout | Service/Cashier/method authority, replicated execution and response-byte limits |
| [Installation recovery](../crates/ic-blob-storage/src/ops/service/recovery/mod.rs) | One bounded management call with owned external Candid arguments | History decoder limits, installation/actor binding, independent continuity evidence and same-execution proof consumption |

All four dispatch once, without automatic retry or fallback. The Platform call
cost includes attached cycles, so funding observes cost before adding its separately
accounted attachment. Only an actual unbounded callback supplies a matching refund;
construction, cancellation, enqueue failure and ambient callback state do not.
An invalid reply or unknown bounded-wait outcome never proves that no effect occurred.
Application reply limits apply after Platform's initial response buffering.

[Blob #108](https://github.com/dragginzgame/ic-blob-storage/issues/108) tracks consumer
qualification of the [Platform #75](https://github.com/dragginzgame/ic-platform/issues/75)
ownership contract. Platform 0.7.2 also publishes the optional `ic-call` package
for owned dynamic method names. Existing raw callers remain supported; Blob's
prepared funding method is static and its other calls borrow names in scope.
No extra custody package or local call layer is needed for those bindings.
Retain protocol adapters and qualify installed timeout/uncertainty behavior
separately from native rejection classification. The installed tenant-client
case holds a reply after the real mutation commits, advances beyond the client's
bounded wait and observes original `SysUnknown` code 6. Saved intent blocks another
dispatch before and after the late reply; an independent inspection sees the
committed enrollment. The private probe control requires its installed operator
and adds no production endpoint. `test-native-host` selects the tenant-client
cases for Linux and both macOS lanes; local execution does not establish hosted
qualification of the pending source.
The raw runtime is codec-free, but the client package's current resolved graph is
not Candid-free: Contracts selects Platform's `candid` feature for external provider
GC/onboarding codecs, certificate DTOs and account integer types. That is distinct
from dispatch ownership; removing this graph edge would require a separately
reviewed Contracts feature/public-API cut, not another transport implementation.

The released 0.51 boundary uses published Rust Platform 0.7.0 throughout Blob's
workspace catalog. Its removal of Candid JavaScript/TypeScript compiler exports
requires no local replacement: owned clients already use Wire generation.
Platform owns Blob's public Principal identity, so consumers must rebuild on
the same Platform line. Required external Candid and stored/Wire bytes retain
their existing contracts.

Released Blob 0.52 requires published Tooling 0.69.2 and Memory/Storage
0.51.12, retaining Platform 0.7.0. Tooling's native process-lifecycle, cache
admission and managed-server changes require no Blob API or feature migration;
the standalone normal/build Wasm dependency tree is unchanged by this adoption.
Their dependency minimums keep the older Platform 0.6 package out of the resolved
graph without a Cargo patch or type-conversion shim. Default and all-feature
metadata resolve one Platform identity; the standalone uses one CDK executor
2.0.0. Stable-structures retains its upstream identity through Storage/Memory,
and native Testkit transport retains the common external Principal type. The
dependency/deployment checks qualify the actual graph separately from the
direct declaration boundary.

Pending Blob 0.54.0 keeps a Tooling/Host/Build/Testkit minimum of 0.69.9; the
current lock selects published 0.69.10, including the Testkit server CLI and Build
release editor.
Testkit repairs cached timer/hook snapshots under
[Tooling #263](https://github.com/dragginzgame/ic-tooling/issues/263); Blob uses
direct snapshot fencing tests rather than the baseline pool or capture-set
helpers, so no custom cache recipe needs migration here. This does not qualify
Canic's separate recipes or change Blob's backup/restore guarantees.

The browser client declares `@tokostudios/ic-platform` as a regular dependency;
first-party runtime, test and build sources use its explicit ESM subpaths. SDK
Core remains transitive through Platform and the external Caffeine package.
The maintained workspace overrides Caffeine's SDK selection to 6.1.1, matching
Platform 0.1.1. Browser prerequisites enforce that graph and reject direct SDK
declarations/imports and aliases through Shared's opt-in SDK boundary checker.
Blob additionally rejects separate installed Agent identities. External
Caffeine Candid remains on Platform's `/candid` boundary; application bytes stay
with Wire. Rust and browser Platform packages have independent versions.

The standalone owns endpoint registration, caller capture, lifecycle and memory
composition. The core exports none of these implicitly. Consumer frameworks own
their wrappers and composition tests; see [Canic integration](#canic-integration).
The [service contract](service-contract.md) owns supported service invariants.

Contracts owns opt-in `provider-json` boundary codecs, including
[`provider::upload`](../crates/ic-blob-storage-contracts/src/provider/upload/mod.rs).
Use `ic_blob_storage_contracts::provider::upload`; Core does not own HTTP JSON
parsing or declare `serde_json` directly.
The Contracts crate has no default features; native callers of these codecs
must enable `provider-json`. CLI and PocketIC already select it explicitly.
The CLI retains its required Serde/JSON dependencies and Host test helpers,
but no longer declares Testkit. PocketIC retains Testkit and owns its server
selection; removing the CLI declaration does not retire simulator setup.

Collection constructors take exclusive `RuntimeMemory<Backing>` handles from the
host runtime. Store/workflow generic parameters name `Backing`; passive memory
bundles carry handles without granting access. Production hosts compose all
fifteen Blob requests before a single bootstrap. Cross-release installations
require [safe retirement](retiring-installations.md), not automatic migration.

Native fixture custody uses Memory's published `testing::fixture_runtime` with
retained backing and explicit leases. Raw bytes remain available only to native
corruption/restore fixtures. Memory fixture support is selected by core unit tests
and isolated probes; the default standalone build must exclude those selections.
The core's non-default `test-support` feature exposes independent heap reference
owners and their read-policy adapters for probes and parity tests. Default
production consumers use the durable installed owner. The authority probe and
reference integration suites select the feature explicitly; shared domain types,
capacity checks and pure lifecycle transitions are available without it.

Do not infer production feature admission from a workspace-wide unified build.

## Open native Agent advisory

[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) is the canonical
RUSTSEC-2026-0387 remediation owner. [Blob #102](https://github.com/dragginzgame/ic-blob-storage/issues/102)
is closed as superseded tracking, not a fix or exemption. The dated consumer
evidence below remains historical; no fresh advisory scan is claimed by snapshot adoption.
The 2026-10-24 working lock at released source `dcbb881`
(SHA-256 `da403d6db61b72a1f635bff19e7c9f3d1dfaf49629dac9d5f0fd6cfaf5dda8e2`)
selects Platform 0.6.11 and Agent 0.49.3, whose elliptic-curve 0.13.8 paths
select crypto-bigint 0.5.5. RustSec revision
`1645308cb4766561b0b274d8c93f7033a766fc96` requires crypto-bigint >=0.7.6.
This is the incoming working lock, not a claim about the released lock.

The Linux native CLI's normal/build graph includes the affected package.
Its [identity loader](../crates/ic-blob-storage-cli/src/native/mod.rs) supports
secp256k1 PEM signing as well as Ed25519; this is not a verification-only consumer.
The advisory concerns potentially secret-dependent arithmetic; the graph and
signing path identify a review obligation, not measured exploitability on a
particular compiler or CPU. Selecting an Ed25519 identity does not remove the
affected dependency or establish a package-wide exemption.

The standalone's normal/build `wasm32-unknown-unknown` graph excludes crypto-bigint
and Agent. The existing `make contracts-boundary-check` rejects deployed Agent
and other native owners. This target-specific graph evidence is distinct from a
linked-Wasm scan and does not qualify downstream host feature selections.

The older elliptic-curve requirement cannot accept the
patched crypto-bigint line through a lock-only update. Retain the supported
identity contract until a compatible supplier correction is available; do not
force a crypto override, implement local arithmetic or suppress the advisory.
After adoption, rerun the current-feed audit, native identity/Agent tests, MSRV
and deployment boundary checks. No exemption or security fix is claimed here.

## Native tooling boundaries

Host owns descriptor-rooted inventory and journal reads, retained creation,
validated publication and bounded subprocess framing. Blob retains input bounds,
request/reply correlation, JSON contracts, uncertainty records and error mapping.
Held descriptors do not freeze contents: callers exclude concurrent writers and
verify complete body digests. Interrupted probe evidence deliberately uses retained
creation rather than an atomic writer that discards partial bytes. These are
necessary consumer policies, not independent filesystem implementations.

Native probes use published `ic-host-http::read_body_into` with caller-owned
storage, preserving accepted bytes when the outer timeout cancels the reader.
Returned failures transfer their retained prefix back into Blob's evidence
buffer. Blob reads the body before classifying unsuccessful status, keeps its
existing byte/timeout limits and labels, and records fallible allocation refusal
as `body_allocation`. Reqwest remains explicit for client configuration and the
separate streaming verifier/file-sink download; buffering would break that
contract. HTTP has empty defaults and is selected only by the native CLI target.
Neither HTTP nor Reqwest enters the CLI or standalone Wasm graph.

The optional archive verifier exchanges Host process limits and typed errors
with Build, requiring the same Host package identity. Build and direct Host share the coordinated Tooling release line.
The [earlier mismatch](https://github.com/dragginzgame/ic-build/issues/9#issuecomment-6269225053)
retains its original reproduction. Keep the actual output limits and exit/signal
propagation; no copied process types or version bridge is used. Archive-verifier qualification is reported separately in the current handoff.
The normal Host process dependency supplies the default CLI's bounded framing;
`package-verification` activates Build, not Host process. HTTP fixture support
remains development-only. Build's source inventory is descriptive provenance,
not executable attestation or retry authority. Dependency owners retain their
upstream identities; Blob must not pass typed values across incompatible
package versions. Installed tool selection is separate from the application lock:
run `make install-testkit` then `make testkit-check` after changing Testkit.

Test dependencies select Platform's `pocket-ic` feature, import `ic_platform::pocket_ic` directly,
and call `ic_testkit::fake_principal`. Testkit still owns server startup,
lifecycle extensions and transport error handling. The simulator identity,
principal bytes and local decoder limits retain their original identities.
Host Tools' decoder limits belong to Platform; Blob uses that canonical type.
The optional package verifier and its Build dependency stay on the same Host
process identity. Archive qualification passes explicit default-feature selection to
Build; packaging, extracted metadata and tests use the same graph policy.
After dependency changes, explicitly prepare the locked Testkit CLI with
`make install-testkit` and the locked release editor with
`make install-release-editor`; application lockfile changes do not install tools.

The [service wire guide](service-wire.md) defines the current 0.43.0 contract.
All 27 owned methods and standalone lifecycle inputs use published IC Wire,
with explicit method/direction identity and no old-packet fallback.
Tenant frames use new 0.43 identities; all methods use Wire's bounded native
Serde codec. Passive offline values use a separate document identity, and new
application artifacts use `.wire`. Exact saved-input and signed-frame binding
remains with the CLI/workflow owner.

Native command grammar uses Clap with standard help, usage and error context;
color and suggestion features are not selected. Command handlers retain domain
validation. Blob and probe argument errors remain sanitized so identity/path inputs
are not echoed; generated help documents syntax without dispatching an operation.
Clap is excluded from canister targets.

Blob has no direct Ciborium dependency or local CBOR implementation. The adopted
Memory codec removes the former Ciborium tree; its ledger codec and byte
contract stay Memory-owned. Native test `serde_cbor` remains
for IC transport envelopes and certificates. Provider/management Candid and
stored records retain their external/persisted contracts. Platform owns the
supported parser/decoder/runtime APIs. The fixed provider DID is parsed through
Platform in the deployment-surface test; it is repository input, not an untrusted
service inventory. Host interprets the bounded Wasm function exports through
`ic-interface` without its optional Candid parser. Direct Candid/CDK declarations
remain forbidden by the boundary policy.

Platform owns non-cryptographic runtime adapters, using the original
executor 2.0.0 and IC0 1.2.0 identities. Callable entrypoints select codecs
explicitly. Blob's method emitters bind pre-copy admission to the declared
method, not inspect-message-only APIs. Provider Candid replies remain explicit;
no-argument management calls supply their original Candid argument bytes.
Private probe RPCs and lifecycle inputs use
`blob-test-protocol::fixture_wire`. Its method/direction framing uses the released
Wire native codec; method identities are FNV-1a-64 over
`blob.private.fixture.rpc:{method}`, bound statically through Platform's
const-generic codec paths. No update or reply reads an inspect-message-only IC
method-name API. These IDs are not authority. Installation and restoration
use separate `fixture_init` / `fixture_post_upgrade` identities. Positional tuples
are exact, including explicit unit for no arguments; old Candid, empty inputs and
extra results refuse. The global frame ceiling is 4 MiB, allocation 64 MiB,
262,144 nodes and depth 32; admission-probe inputs retain narrower 4/16 KiB byte
ceilings. Decode profiling measures the complete Wire decode, without fictitious
Candid header/type-table phases.
Only emulated external provider contracts retain the tuple decoder, 10,000
skipping quota, empty-input handling and reply arity through
`blob-test-protocol::provider_candid`. Native fixture diagnostic tools use Wire;
provider account/gateway reads and management scheduling still exercise actual
Candid packets. Private chunk replies use Wire bulk `ByteBuf`; host append
requests borrow `BytesRef` where no retained ownership is needed. Saved fixture
leaf records use `ByteBuf` too. Wire's canonical re-export removes the private
protocol's separate `serde_bytes` declaration.
Provider scheduling aliases intentionally retain the real provider packet where
the production decoder is the consumer: `fixture_gateways` returns a Candid
gateway list, `fixture_balance` exchanges Cashier account packets, and
`fixture_gateway_query` accepts the Cashier query's unit arguments. The controlled
`receive` endpoint takes Wire input but emits source-backed Candid funding replies.
These are explicit substitutes, never automatic format detection or fallback.

Published Wire supplies the native Serde codec; generic decode
callers explicitly choose `Deserialize<'de>` or `DeserializeOwned`. Blob's generic
service/document decoders now admit input-backed values. Borrowed tenant principal
views use `BytesRef<29>`; the native descriptor consumer uses `TextRef` over its
retained authenticated frame. Owned async requests/results, durable records and
exact retry buffers remain owned. There is no added dependency or alternate codec.
The [protocol](service-wire.md) owns the current limits and canonical bytes;
[measurements](evidence/borrowed-wire0390.md) separate actual upload/descriptor
operations from chunk codec controls. Provider bodies remain outside Blob RPC.

Platform's application `Principal` is owned and has no Candid or stable-key
trait. Application DTOs and runtime calls use it. Required provider/management
Candid DTOs, Agent/PocketIC arguments, persisted records and stable keys retain
`principal::ExternalPrincipal`, the original upstream identity, with explicit
conversions at those boundaries. Principal representations and key ordering stay
unchanged. The separate 0.50 cut changes Blob's stored record codec to bounded
Wire; required external Candid decoder budgets remain at their existing owners.
Runtime calls supply separately encoded bytes, including explicit Candid unit
arguments; reply decoders and transport/codec failure mappings remain at their
owners. Native fixture policy and historical-image helpers use Platform's
`DecodeLimits` and `DecodeError` directly, preserving their five reply budgets
without conversion, copied decoding or alternate transport.

Core and replicated client select `runtime`, retaining their existing CDK API
paths and shared executor identity without requesting the entrypoint generator.
Canister adapters select `cdk`. Explicit `WaitPolicy::Bounded` preserves all
three callers' original timeout values, raw arguments, cycles and retry policy.
Standalone service declarations select `candid-service` only on native targets;
Contracts selects it only for native tests. Deployed entrypoints retain explicit
Wire/provider codec hooks and no longer carry the retired no-op `hidden` option.
Upstream Candid printer dependencies can still be enabled transitively; these
feature choices do not claim a Candid-free deployment.

The native standalone `export_candid` example still exports the same provider
contract; Host's callable export inspection requires no Candid extraction pointer.
The resolved graph selects neither Platform 0.1 nor full CDK. No bounded Host
inventory implementation is copied into Blob. `make contracts-boundary-check`
rejects retired Platform 0.1/0.2/0.3/0.4 and full CDK across workspace features and targets,
including normal/build/dev edges. Its separate production normal/build Wasm
check additionally rejects Platform 0.1/0.2/0.3/0.4, Host, Build, Testkit, PocketIC,
Agent and the parser. Candid remains through required provider/management
contracts; Blob-owned persisted models use Wire after the 0.50 storage cut.
This is not a Candid-free deployment claim.

Platform's PocketIC module supplies `CanisterInstallMode` and
`common::rest::RawMessageId` for skipped-hook upgrades and pending-call recovery.
Its management DTOs retain PocketIC's own
identity, separate from Platform's independent management DTO version. Testkit
continues to own simulator selection and server lifecycle.

### Delayed provider fixtures

Testkit's opt-in `held-reply-fixtures` owns generic immutable reply slots
and manual PocketIC release. It is suitable for consumer callback races without
provider lifecycle operations. It does not replace Blob's durable
[gateway source](../canisters/test/gateway_source/src/model/mod.rs): the
[recovery cases](../tests/pocketic/tests/recovery/authority/sync/mod.rs) inspect
captured source journals, reject pending upgrades, restore old source bytes and
verify bounded exhaustion. Testkit's held controller explicitly excludes fixture
upgrade/reinstall/restore with held calls and keeps no durable source journal.

The [transport cases](../tests/pocketic/tests/storage_gateways/transport/mod.rs)
already cover cancellation, revocation, newer pending attempts and completed
replacements before an original reply arrives. Retain their shared source owner;
moving only these cases would add a second fixture without removing the required
recovery implementation. Use Testkit for new generic cases when that avoids a
new local scheduler; keep application authority and accounting assertions here.

Native fixtures and example staging/publication use published `ic-host-fs`
with empty defaults. Neither `capability` nor `temporary-file` is needed
by Blob's actual callers; `test-support` selects neither. Retained scratch files
remain distinct from the optional scoped `TemporaryFile`. Testkit's Host
selection no longer enables either backend. PocketIC still owns transitive
`tempfile`; Blob declares none directly.
Fixture directories clean up after success and retain failed-test
evidence during panic. Retained report directories remain caller-owned. Private
invocation files use Host's mode-0600 scratch writer within their owned directory.
Verified example outputs use Host's create-only publication of the checked file,
including typed failures after publication; reconcile those before retrying.
Snapshot bodies use retained-file publication, while the complete snapshot/report
is still not a crash-durable multi-file transaction.

All 26 private consumer-probe controls use bounded Wire value documents, with
a 4 KiB ingress ceiling and complete canonical decoding. Their command/status
values remain test-owned, outside the public service catalogue. Rust callers
and opaque browser packets use that encoder; JavaScript defines no fixture
schemas. Actual service operations use their method-bound Wire clients.
Private storage controls now use Wire argument/reply tuples under their existing
4 KiB ingress bound. They reuse the same document codec and native control caller;
provider certificate/GC callbacks retain Candid. Probe lifecycle arguments use
the separate Wire lifecycle identities described above.
Application-only DTOs no longer implement Candid traits. The private protocol
retains only the two Candid Cashier request shapes used at provider boundaries.
Authority, funding, gateway-source and consumer journals now serialize their
actual models with Wire through `fixture_storage::FixtureJournal` and Memory's
published `testing::FixtureCell`. The consumer no longer keeps copied persistence
DTOs, a JSON projection, a `CONS` envelope or a direct Core dependency.

These private journals use a storage-only `BLOBJNL1` identity and include their
allocation key in the frame. Their distinct keys end in `.wire.v1`; earlier
Candid journal keys are not reused. Creation requires empty backing; restoration
never falls back to empty state. Byte ceilings stay 64 KiB for authority/funding,
1,114,112 bytes for gateway-source and 16 KiB for consumer. Each has finite Wire
allocation (eight times its byte ceiling; sixteen for gateway-source's full leaf),
node (262,144) and depth (32) budgets.
These are Wire resource units, not equivalents of the retired Candid work quota.
Old private fixture images require reinstall; same-release restoration, fences,
uncertain effects and exact retries remain tested. Pinned historical consumer
upgrades now assert refusal and preserved stable bytes, not format compatibility.
Their old Candid initializer remains explicit and they require retained artifacts.
That private fixture cut changes no production record codec or public service
format; the separate [0.50 storage cut](service-contract.md#current-persisted-boundaries)
owns production record changes. Public application DTO trait
removal is the 0.38 hard cut tracked by
[#84](https://github.com/dragginzgame/ic-blob-storage/issues/84).
The Contracts golden vectors protect field/variant order and every owned method.

Retained probe records use Host's `write_retained_at_with`, preserving exclusive
creation, named partial evidence and separate creation/write/file-sync/directory-
sync failures. Ordinary atomic output publication is a different contract. The
simple local HTTP fixtures use Host's absolute-deadline `accept_tcp_until` and
blocking accepted streams. The multi-request fault proxy keeps its distinct
cancellation/backpressure policy.

## Canic integration

Canic owns its adapter and dedicated/embedded integration tests. Blob owns the
service contract, method catalogue and memory request factory. Consume published
core and Contracts from the same release line in the host workspace and each
independent consumer workspace; this guide does not authorize sibling edits.

For consumer adoption of this source:

1. Select compatible Blob core/Contracts packages and the host's matching Memory
   package identity. Store/workflow type parameters now name backing memory; constructors
   take exclusive runtime leases. Freeze the application's actual lock and every
   independent fixture lock; sibling paths are not registry-delivery evidence.
2. Include Blob's fifteen requests in the host's single declaration snapshot
   and grant the `blob.` namespace before bootstrap. Validate installation for the
   allocated Principal before opening stores. Mounting initializes nothing.
3. Preserve the host's authenticated initializer and one synchronous lifecycle.
   Translate its admitted configuration to `ServiceInstallationInput`; standalone
   initialization uses a method-bound Wire frame, not a template for the framework's outer envelope.
   A Candid outer initializer still requires zero skipping quota and exact argument
   count, including nested application configuration.
   Retain exact target/operation bytes for lost-reply reconciliation.
4. Mount both request and response codecs from the current method catalog:
   all owned methods use Wire, and provider certificate/GC callbacks use Candid.
   The catalog adds the `provider` query/update emitter families; update the host
   emitter and delegate to shared `ServiceWorkflow` handlers. Use the canonical
   `provider::gc` decoder bounds. Check actual Wasm names/modes and the provider-only
   Candid subset. A Candid service file does not describe the complete Blob API.
5. Run dedicated/embedded PocketIC initialization, target/caller refusal, rollback,
   lost-result, metrics and same-image recovery cases in Canic. Apply the
   [retirement contract](retiring-installations.md) before a cross-release reinstall.
   Same-release recovery never authorizes upgrading a retained installation.

### Adopt the 0.54 memory boundary

Pending Blob 0.54.0 selects published **Memory/Storage 0.54.0**. Blob publicly
re-exports Memory types, so embedding hosts must align their direct dependencies
with this package identity and rebuild. The published Memory/Storage implementation
sources match 0.53.1; stored formats and allocation custody are unchanged. The
coordinated Persistence release also makes `ic-metrics-stable` use Metrics 0.17,
removing the earlier type-identity obstacle for Canic's optional stable provider
policy. Blob does not gain a stable metrics store dependency.

Released Blob 0.53.1 already adopted Persistence's lifetime correction, releasing backing
claims before freeing their identity allocations during final teardown,
construction refusal and unwinding. It addresses the stale-claim lifetime defect
reported through [Persistence #137](https://github.com/dragginzgame/ic-persistence/issues/137),
without changing public signatures, stored formats or checkpoint validation.
Native macOS archive acceptance remains distinct from Linux qualification.
Blob publicly re-exports Memory, so every embedding host must use that same package identity.
Standard `VectorMemory` and canister stable backing already implement the new
`MemoryBacking` contract. Custom wrappers delegate their underlying identity;
aliases must retain the same identity and its allocation for every runtime/handle
lifetime. Never derive an identity from persisted bytes or mint one per raw alias.

Use one host runtime and release all component collections and detached handles
before replacing it. Memory rejects a second owner while any lease remains live.
The private storage probe's nested instrumentation retains an exclusive outer
grant in an Rc and shares that grant's identity across its aliases; it does not
create independent identities for raw stable-memory wrappers.

Memory validates its bounded manager metadata image at construction, initial
bootstrap, lease-open and growth checkpoints. Ordinary size/read/write operations
check poisoning and bounds without rescanning manager metadata. Full checkpoints
still poison detected stale owners without repair; raw edits during a live lease
need not be detected before its next read or write. Hosts must retain exclusive
custody between checkpoints. This narrows 0.52's per-access detection guarantee
and removes its repeated scans, without an unchecked mode or an endpoint speed claim.
It is not isolation from arbitrary raw IO, dishonest identities or separately linked
Memory copies.
MGR, ledger, cell, Blob record and Wire bytes are unchanged by these dependency
updates. Blob's cross-release retirement/reinstall rule still applies; there is no
automatic migration or alias-guard bypass. Requalify the actual Canic composition
and its selected limits independently of standalone/local-substitute evidence.

### Adopt the 0.52 hard cut

| Consumer boundary | Required change |
| --- | --- |
| Installation/configuration | Remove `ServiceReadInput` and `ServiceConfigurationInput.reads` from authored inputs and test constructors. Preserve the remaining explicit limits and provider bindings. |
| Status and CLI projections | Remove read-session status, budgets and fence assertions. Current CLI output remains schema 1; upload, funding and gateway fences remain. |
| Memory composition | Use `ops::service::installation::requests` and `ops::service::stores::grants::open`, rather than a copied key roster. The installation has fifteen requests, including its configuration and pending-deletion index. |
| Wire callers and generated clients | Regenerate from the selected Contracts catalogue. Configuration, local-status and installation frames changed identity with their DTOs; method names/modes and the required external provider Candid are retained. Do not reuse old encoded installation inputs. |
| Gateway decisions | Revocation still removes membership and invalidates pending syncs. An absent member with no pending sync no longer advances a retired read-generation counter; it may leave stable bytes unchanged. |
| Stored state | The manifest/index and singleton-cell layouts are a cross-release hard cut. Retire the old installation with its original tools, preserving uncertain effects and obligations, before reinstalling. No in-place conversion or fallback reader exists. |

Keep descriptor-based downloads and client-owned bounded body verification;
retiring on-canister chunk sessions does not retire those paths. Requalify both
Canic compositions using newly generated artifacts: initialization and exports,
caller/scope refusals, upload/registration/reference/descriptor operations,
gateway sync races, funding, public metrics and same-release restore/recovery.
The host retains its one synchronous lifecycle and its own release profile.
Blob's standalone size measurements do not predict a composed Canic artifact.

Use the [0.52 implementation/evidence record](evidence/simplification0520.md)
for the exact cut and measured limits. Previous-release test results establish
their recorded graph, not acceptance of a regenerated 0.52 adapter.

The [2026-10-16 receipt](evidence/canic-issues0310.md) retains its historical
source, stale fixture-lock failures and proposed caller repairs. It does not
establish current integration. The latest read-only source observations and
inspection limits are recorded in the
[architectural sediment audit](reports/audits/2026/10/18/architectural-sediment/01/report.md).
No sibling source or lockfile is changed by Blob adoption.

Managed installation does not complete application acceptance. The application
must authenticate two users, issue exact tenant-approved permissions, operate the
[trusted registration](trusted-registration.md), qualify deployed provider reads and
commit its media map only after exact completion/live references. See
[multi-user authority](multi-user-upload-authority.md),
[confirmed-reference publication](operator-guide.md#produce-a-complete-confirmed-reference-map)
and [funding qualification](funding-consumer-qualification.md). Provider payer,
credit, GC, deletion and billing facts cannot be replaced with local substitutes.

## Setup and checks

The browser prerequisite/build adapter selects `clients/browser`, `tests/browser`
and `scripts` for Shared's read-only SDK boundary check. It rejects direct Core
and legacy DFINITY declarations and literal imports; Blob retains locked/installed
identity and provider compatibility checks. Computed imports and arbitrary loaders
are not proved by a literal-source guard. The maintained bundles and actual
browser acceptance remain separate evidence.

The Platform boundary target enables Shared Tooling's reviewed external-IC package
inventory alongside Blob's stricter local rules. It checks aliases, optional,
build/dev and target-specific declarations. Deployment exclusions still use the
separate resolved-graph gate; a declaration check does not prove Wasm contents.
The common inventory is a known-package list, not automatic discovery of all IC crates.
`make tools-report` collects every offline tool-check result without installing tools;
release admission continues to use fail-fast `tools-check`.

`make install-tools` provisions the complete shared host/IC/Cargo set followed by
Blob's lock-selected Testkit CLI/server. `make tools-check` admits those selections
offline. For a changed Cargo-tool policy run `make install-rust-tools`; for a
changed Testkit selection run `make install-testkit`. Setup retains old generations
and failed evidence. Use the same shared-store setting for setup, checks and
failure collection; see [local setup](local-setup.md#shared-executable-storage).

Scoped builds/tests use `--offline --locked`. After dependency changes or cache
removal, run `make deps` explicitly before them; it fetches only the selected
Rust source and does not install executable tools. An explicit offline setting
remains authoritative. Missing dependencies or binaries fail rather than being
installed, replaced or skipped by tests.

PocketIC callers obtain the admitted absolute server path through `make testkit-check`.
Explicit `POCKET_IC_BIN` overrides remain caller-owned byte admission; no fallback,
implicit download or independent server-version equality gate exists here. Testkit
owns client/server compatibility and lifetime. The common five-tool IC bundle is
pinned separately by [ci/ic-tools.tsv](../ci/ic-tools.tsv).

## Release and formatting host checks

The [tooling workflow](../.github/workflows/tooling.yml) qualifies setup/admission,
formatting/hooks, exact-ID tooling archive upload/readback, coordinated package
archives, native CLI/PocketIC and original/optimized Wasms on Linux, Apple Silicon
and Intel macOS. Rust 1.91 has its separate minimum-version lane. Declaring those
jobs is not a passing result; require matching native execution on the actual source.
Cancelled or earlier-source results do not qualify new inputs.

`make evidence-check` and `make probe-check` verify retained records offline;
they make no new provider requests. Failed tooling and package/Wasm evidence stays
at the printed paths and is uploaded by the hosted collectors. Successful fixture
cleanup removes only its own disposable inputs. See
[development governance](governance/development.md) for command authority and the
[release guide](releasing.md) for maintainer validation, delivery and publication.

## Replicated clients and service storage

The `ic-blob-storage-client` package owns the existing role-specific replicated
calls and their common one-call mechanics. Its normal graph includes Contracts
and Platform execution, but excludes Core, Memory, Storage and stable structures;
`make contracts-boundary-check` enforces that distinction across targets.
Contracts and the native CLI retain their separate runtime-free checks.

Use `ic_blob_storage_client::{admission,download,funding,manifests,references,status,tenants}`
for remote calls. The old Core client paths are removed in the released 0.35.0
public cut. Core owns installed service state and workflows; consumers retain
intent, retry decisions, application publication and uncertain-outcome recovery.
The client package installs no endpoints, lifecycle hooks, stores or timers.
Publishing proceeds Contracts, Client, Core. Only maintainer release preparation advances their
coherent workspace version; registry publication does not qualify consumer
runtime composition or provider behavior.
