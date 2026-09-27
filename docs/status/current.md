# Current status

Date: 2026-09-27

## Active work — 0.2 service phase

The maintainer confirmed **0.2.1 is pushed**. Cargo and the release receipt are
0.2.1. Local main, origin/main and v0.2.1 resolve to
`85967c82902c4730614ca5bd28ec81fea382f28a`, from validated source
`70039742055b2ae5b8d8cf3be092d8f2d8be5043`. The receipt records the
`release-verify` gate; registry publication was not independently queried.
Current work is in Unreleased; no version mutation or publication ran here.

Follow the [0.2 delivery plan](../roadmap.md). Its goal is a usable Caffeine-backed
service through shared durable handlers, both adapters and an operator client.
0.1 closes the foundation phase, not service qualification or Canic removal.

| Milestone | State | Remaining completion condition |
| --- | --- | --- |
| M1 — contract | In progress | Freeze admission/resource envelope, provider guarantees and operational recovery |
| M2 — durable standalone service | Not implemented | Persisted shared handlers and actual standalone lifecycle/journey evidence |
| M3 — Caffeine and operator integration | Not implemented | Qualified provider transport, completion/economics and production client |
| M4 — managed parity and acceptance | Not implemented | Same journey through Canic adapter, complete replacement evidence and handoff |

## What exists

- Core hashes/manifests/verification; transient reference, quota, reservation and
  separate physical/billing accounting; tenant/gateway predicates.
- Cashier request/reply codecs, funding arithmetic, bounded query inspection and
  passive diagnostics. These perform no production provider calls.
- PocketIC journeys with actual caller, reentrancy, transfer/refund, interruption
  and old-backup tests over controlled sources. Fixture journals restore into
  permanent inspection-only fences; they do not demonstrate operational recovery.
- Unpublished local operator commands. Production identity/targeting, shared
  persisted service handlers, standalone/Canic adapters and consumer client are
  still outstanding. See [core evidence](../evidence/core-primitives.md).

## Current 0.2 implementation and evidence

The maintainer directed Toko review to remote `development`; local Toko is absent
or stale. The [review record](../evidence/toko-0.2-review.json) pins HEAD
`6519b72d2a420564dabaf700fc55f7b8603d9fd3` and source hashes. Toko's media limit is
10 MiB per file; its main upload session stages 500 MiB in aggregate. Browser
upload precedes asset registration. The roadmap specifies project-to-uploader
admission and retained references/consumer outbox behavior across registration
failures; actual Toko integration remains outstanding.

The requested read-only [Toko Miner feedback review](../roadmap.md#toko-miner-feedback--2026-09-27)
adds a distinct proposed release-media consumer. M3 now explicitly includes a
resumable headless publisher; acceptance covers browser delivery/integrity and
overlapping releases. Its reported 702-file / 270.1 MiB checkout guides capacity
work, not a qualified publish list. Resolve live-object reuse, bounded retention,
deleted-content reintroduction and lifetime-history exhaustion before freezing
production contracts. Miner adoption/architecture remain unapproved; no sibling
edits or consumer tests ran. Its stale release-doc finding is addressed locally.

The maintainer selected direct browser-to-Caffeine upload. The shared admission
owner now accepts bounded manifests instead of file chunks; `UploadRequest` has
no raw-digest field. Its observations distinguish manifest binding, possible
exposure and independently confirmed completion. Project/uploader/service/operator
bindings, enrollment generations and exact retries remain enforced. Unconfirmed
exposure remains charged through expiry, revocation and failed asset registration.
Global/tenant lifetime manifest-leaf capacity is reserved before catalog mutation
and survives cancellation/settlement; one retained leaf array replaces streaming
verification state. These are transient models, not production persisted handlers.

Service metadata now requires canonical `Content-Length` equal to admitted bytes,
unique ASCII-token names ignoring case and values without controls, line separators
or surrounding whitespace. Raw budgets precede parsing; configuration must fit its
largest object's length header. Rejected metadata preserves manifests/accounting;
reordered valid headers remain exact retries. Consistent declarations still do not
prove actual stored length. Generic Caffeine hashing remains unchanged.

The unpublished admission probe uses the same owner with actual IC callers/time.
It emits no certificate or provider effect. Its 16 KiB command envelope bounds
Candid work, skipped values and type headers before manifest conversion. The earlier
accounting-build measurement of a 10 MiB declaration, preparation, retry and exposure was
4,116,047 measured instructions and 1,766 request bytes, with allocated Wasm memory
steady at 1,245,184 bytes. Counters exclude diagnostics/reply encoding, persistence
and provider transport. See [resource evidence](../evidence/core-primitives.md#confirmed-usage-and-indexed-exposure).
Stop/start preserves its owner; unsupported upgrades reject atomically even when
the outgoing hook is skipped. There is no operational restore path.

After 0.2.0, reservation and manifest-leaf accounting uses private totals updated
only by successful owner transitions. Root ownership now checks a bounded identity
index instead of scanning prior claims. Lifetime slots remain occupied through
cancellation/settlement; no restore or counter mutation API was added. Independent
native audits compare aggregate usage against individual operation/lifecycle states.
Confirmed-object usage now maintains all global/tenant fields through one mutation
path, including receipts recorded for failed lifecycle operations. A retained root
index resolves the exact service permission before all existing exposure checks;
failed admissions create no index entry. That accounting batch preserved public APIs.

The shared owner now adds `lookup_content`: an explicit tenant/namespace query
returns the original upload identity and current reservation or confirmed lifecycle.
It uses the retained index, discloses no foreign object and remains available to
suspended tenants. It allocates no reference, renews no permission and issues no
provider effect. Native overlapping-release cases prove exact receipt replay,
stale-read rejection, cleanup at receipt capacity and separate deletion/settlement;
settled root reallocation still rejects. Actual IC discovery checks cover caller
isolation, uncertainty, cancellation and stop/start. See the
[discovery evidence](../evidence/core-primitives.md#tenant-content-discovery-and-release-reuse).
The same tenant boundary now reports remaining reference identities, unreserved
receipts, reserved cleanup receipts and fresh distinct retains. Fresh retains need
two receipt slots, and report zero after deletion queues. Counts are observations,
not permission or reservations; exact mutations still enforce enrollment and state.

The new two-tenant workload fills 256 cancelled operations, retaining exact retries,
cleanup, isolation and stop/start behavior at capacity. Measured admission and
cancellation calls stay below the 5M test ceiling. Allocated Wasm memory grows
from 1,245,184 to 1,638,400 bytes,
64 KiB above the baseline's final allocation. Total call costs do not improve
uniformly at this size despite removing history scans. See the
[before/after record](../evidence/admission-history.json), including the latest
follow-up build. The native confirmed-accounting audit covers 64 objects across
three tenants, every usage field and wide/zero-byte accounting. An actual IC case
exposes the last permission at 256-operation capacity after rejected calls and
stop/start. This is not production capacity or full confirmed-catalog memory evidence.

The subsequent [multi-file workload](../evidence/core-primitives.md#multi-file-release-history)
now measures 704 confirmed objects / 288 MiB of synthetic media with 768 retained
leaves, 2,816 reference IDs and 4,928 receipts across four reference generations.
Allocated Wasm memory grows from 1,245,184 to 4,587,520 bytes. Cleanup works at
history capacity, and physical/billing byte charges end separately without freeing
lifetime slots. At the released baseline, roughly 98% of the last admission's
5.43M instructions occurred before workflow dispatch.
Provider facts are operator-only substitutes. Read sessions, larger per-object
histories and operational persistence remain unqualified.

The [post-0.2.1 decoder follow-up](../evidence/core-primitives.md#operation-specific-admission-inputs)
replaces the private probe's catch-all wire enum with operation-specific bounded
inputs, delegating to the same handlers. Separate header/value/dispatch counters
show the remaining pre-workflow cost. PocketIC 16's pinned IC implementation
charges each first memory-region access and write; the observed 80k jumps match
that granularity, not evidence of a core history scan. No allocator was replaced.
That 704-object final admission measured 3,985,302 instructions, peak retain
6,783,953 and peak release 6,622,482; allocated memory ends at 4,521,984 bytes.
Decoding still grows with retained state. Model APIs, provider gates and production
capacity remain unchanged; carry narrow input boundaries into M2 and investigate
allocation locality without treating phase counters as pure CPU costs.

The [reference-history follow-up](../evidence/core-primitives.md#reference-history-without-lifecycle-copies)
removes per-request lifecycle cloning. A private single-reference plan is checked
against cleanup receipt capacity before either change is published synchronously.
Public APIs, typed failure receipts and exact replay remain unchanged. A new IC
case retains 256 simultaneous references and 511 receipts, rejects work at capacity,
survives stop/start and releases every reference before separate deletion/settlement.
Whole-call peaks fall about 4–5% in that case; allocated memory stays unchanged.
The 704-object workload's peak retain falls to 5,104,779 and peak release to
4,867,114 instructions, with the same 4,521,984 final allocated bytes. Phase costs
are not uniformly lower, and the persistent/production envelope is still open.
An isolated Talc 5.1.1 experiment had mixed costs and used 128 KiB more final heap;
it was not selected. Its temporary dependency/allocator declaration were removed.
Default heap allocation and ic-memory stable-storage authority remain unchanged.

Latest targeted validation passes 68 lifecycle/service/catalog unit cases, 28
related integration cases, all 18 admission PocketIC cases, affected strict
all-target Clippy, warning-free library rustdoc, the admission release Wasm build,
formatting and diff checks.
The resource helper additionally emits `.tmp/reference-history.json`. Earlier
integrity/recovery results remain tied to their original revisions.
Full CI was not run; package versions remain 0.2.1.
The independent integrity/recovery fixtures retain their own raw digest binding;
no mandatory byte-verification workflow remains in service admission.

The preceding upload-path evaluation rechecked official main, npm latest/integrity
and Toko development unchanged. This implementation made no provider/account call
or source refresh. Stop/start, same-release durable upgrade and older-backup
reconciliation remain separate requirements. The IC version counter does not
prove inventory completeness or identify only restores. A surviving full obligation
source remains required; no new server guarantee was established.
See [provider review](../provider-review.md#02-admission-follow-up--2026-09-27).

Read-session follow-up: the private authority fixture now records one volatile,
operator-only call-context profile, including callback/decode/verification phases
and received/disclosed byte counts. Its single slot stays occupied across held
calls; rejected replies disclose no bytes. Callback traps publish no completed
profile, and restored owners deny these volatile diagnostics. Wrong-type and
truncated encoded replies join the existing fault coverage.

Candid's bulk blob decoder reduces the full 1 MiB read from 234.4–234.6M to
115.9–116.0M service instructions, including the fixture's journal saves.
Decoding falls from 122.1M to 3.63M; verification remains 81.1M. The matched
workload's allocated heap rises from 5,898,240 to 7,012,352 bytes but stays flat
over 12 full-chunk reads. The maintainer explicitly wants the default allocator;
it remains unchanged. Only the unpublished fixture adds a direct dependency on
already-locked serde_bytes. See [read evidence](../evidence/read-resources.json).
All 48 journey/recovery PocketIC cases, the focused report target, affected
strict all-target Clippy, release fixture Wasm and formatting pass. No full CI,
version change, provider call or production read protocol was introduced.

Download follow-up: `CaffeineRootVerifier` now binds the expected provider root,
exact length and original metadata before streamed verification, reusing the
existing fixed-memory hasher. It does not require an expected raw digest or leaf
list. Prefixes remain unverified; the local stdin example requires clean EOF and
rejects late read errors, trailing bytes, truncation and corruption. Its optional
output path now stages the exact checked bytes privately, syncs after verification
and publishes without replacing any existing destination. Seven local cases cover
EOF/errors, short/interrupted writes, cleanup, output races and symlinks. The
independent 10 MiB vector passes actual CLI verification and file output, including
no-clobber retry, corruption and budget rejection; a shell-piped body also passes.
Strict core all-target Clippy and the library Wasm check pass. The example alone
adds the already-locked tempfile development dependency. Caller-controlled paths
are required; interruption can leave residue or output without a success receipt.
No transport, authenticated descriptor or production service read endpoint was
added. The
[working download direction](../roadmap.md#consumer-download-verification)
keeps bulk hashing in the client; browser/CORS/CSP/MIME/cache qualification and
consumer adoption remain outstanding. Default allocation is unchanged.

Official Caffeine main and Toko development were refreshed read-only and remain
at the pinned revisions and file hashes. This does not refresh registry, deployed
provider or account evidence. Targeted validation passes 20 Caffeine model cases,
nine independent-vector integration cases, the example EOF/error case, strict
library all-target Clippy, Wasm compilation and warning-free rustdoc. The local
example also verifies the pinned `abc-text` body end to end. No full CI ran.

Descriptor follow-up: admission now retains the first validated header names,
values and order, under existing per-object metadata and lifetime object bounds.
`content_descriptor` borrows the original metadata, exact operation and current
lifecycle through the same tenant/service/namespace checks as discovery. Suspended
tenants can inspect; unprepared and foreign roots disclose no descriptor. Reordered
retries never replace metadata, and cancellation/deletion/settlement retain it.
The private bounded query delegates to that owner. Actual IC caller isolation,
oversized input rejection, stop/start and descriptor-to-client verification pass;
this is not production persistence, response certification or publication authority.

All 36 service model cases and 20 admission PocketIC cases pass (247 seconds),
along with affected strict all-target Clippy, release admission Wasm and warning-free
core rustdoc. The four-object maximal-metadata case retains 4 KiB framed headers
without changing its 1,245,184-byte allocated heap. The 704-object workload ends at
4,587,520 bytes (+64 KiB from the preceding reference batch); retain/release peaks
are 5,425,185/5,505,129 instructions, and final admission is 3,105,760. These are
sequential build observations, not pure CPU attribution or a production envelope.
See [descriptor evidence](../evidence/descriptor-resources.json). Default allocation
and versions remain unchanged. The earlier read report's Candid label was corrected
to the actual locked 0.10.37; its measurements/hashes were not changed.

Reference-qualified follow-up: `retained_content_descriptor` observes original
metadata only with confirmed completion and the exact live consumer reference,
checking service/tenant/namespace/object/incarnation in the same owner read.
Released references cannot borrow another consumer's liveness, and replaying an
old successful retain receipt cannot restore eligibility. Suspension retains read
access; cancellation and all post-release states remain available only through
the historical descriptor. There is no new state, allocation, receipt or effect
in the core read. The private adapter bounds input and delegates to this owner.
Thirty upload model cases, four discovery/descriptor PocketIC cases (including
stop/start and receipt replay), affected strict all-target Clippy and release
admission Wasm pass. Historical resource measurements above remain tied to their
recorded builds; they were not rerun or relabelled for this endpoint addition.
Copied observations can become stale: production publication still requires
trusted delivery and consumer publication/release coordination. No full CI ran.

Headless preparation: `CaffeineManifestBuilder` retains bounded ordered leaves
from the shared streaming hasher in one pass. Independent vectors and the 10 MiB
admission/descriptor PocketIC journey pass; see [preparation evidence](../evidence/core-primitives.md#client-manifest-preparation).
The local `prepare_upload` now also accepts `--inventory`: it preflights explicit
declarations and aggregate work, reads regular sources sequentially, preserves
asset mappings and groups distinct roots only after successful preparation.
Seven preparation cases and strict core all-target Clippy pass. A 704-entry CLI
check covers duplicate grouping, deterministic repeat and no partial report on
budget or late source failure; this uses tiny synthetic bodies, not a throughput
qualification. Core APIs, dependencies and allocator are unchanged by inventory
tooling. Optional `--snapshot PARENT_DIRECTORY` now saves the exact hashed buffers
and completed inventory in a fresh private directory, with one body per root.
Ten focused preparation cases cover preserved copies, duplicate mappings, repeat
isolation, failure cleanup and interrupted/short/failed writes. Saved local files
remain mutable and need reverification before later effects. Strict core all-target
Clippy and a 10 MiB CLI snapshot/root-verification round trip pass, including
deletion of the original source and failed-repeat cleanup. Files are synced but
this is not a crash-durable transaction or operation journal. Production capacity
inspection, publisher intent/resume, trusted delivery and provider guarantees
remain open. Historical Wasm resource measurements were not refreshed; no full CI ran.

Admission planning now has tenant-scoped `admission_capacity` over the shared
owner's maintained totals, propagated through a bounded private probe query.
It reports independent lifetime objects, concurrent uploads, manifest leaves and
byte headroom, plus enrollment and per-object metadata/size limits. Native cases
cover shared contention, cancellation, separate physical/billing cleanup, exhausted
history and wide byte arithmetic. Actual IC cases cover caller/scope/input bounds,
suspension, stop/start and retained manifest exhaustion after settlement. These
observations reserve nothing and do not establish root availability.
Targeted validation passes 35 upload model cases, three capacity PocketIC cases,
affected strict all-target Clippy, release admission Wasm and warning-free core
rustdoc. No full CI, provider calls, dependency/version or allocator changes ran.

The unpublished `blob-fixture-inventory` client now validates a prepared report
before querying capacity, content discovery and existing-reference headroom on an
explicit local probe. It preserves per-asset demand and distinguishes not-visible,
pending, live and retired roots, with no partial report after failures. Seventeen
operator unit cases and two actual executable/PocketIC cases pass, including
duplicate reference pressure, unchanged state, tenant isolation and stop/start.
Production delivery, new-object reference sizing and exact operation persistence
remain open. Existing locked serde/tempfile and the core library are reused only
by the unpublished host tools; no registry version or allocator changes.
Strict host-tool all-target Clippy, formatting and diff checks pass. No full CI
or historical resource refresh ran; provider effects remain substitutes.

**Next:** close the remaining M1 decisions: surviving recovery authority, provider
certificate replay/namespace/completion guarantees and the operational resource
envelope (remaining work includes total memory/instructions and read sessions).
Include repeated-release reuse/reintroduction and multi-file history sizing from
the Miner feedback; a single-file or cancelled-history fixture cannot qualify them.
The 704-object, 256-reference and single-slot read envelopes are now recorded.
Implement/qualify production descriptor publication, provider locator/serving policy
and the consumer adapter before choosing read concurrency; on-canister hashing
remains expensive. Account for production
storage/decoder locality and total message costs with the default allocator.
The admission probe still has no persistence/restore path, and no production
capacity was selected.
The [upload-path decision](../roadmap.md#upload-path-evaluation--2026-09-27)
is implemented locally. Before issuing real certificates, qualify pre-charge
size/tree enforcement, namespace/replay charging and independent completion/size
evidence. The current provider contract is not qualified by this hard cut.
Durable shared handlers,
consumer transaction/outbox implementation and cross-canister interruption evidence
must follow the applicable contract gates, then propagate through both adapters
and operator tooling. Do not substitute another transient fixture for M2 completion.

## Constraints that remain active

Caffeine is the sole provider target. The reviewed interface/package baseline and
unresolved server evidence are in [provider review](../provider-review.md).
Do not infer paid-effect retry safety from a root, upload progress or an audit row.
Completion/retry charging, lost-payment reconciliation, final billing cessation
and surviving recovery authority remain unresolved. A new installation is the
working proposal, not an approved funded account or namespace.

Keep direct tenant, uploader, service, operator and provider bindings separate.
Both adapters belong here and must delegate to shared handlers. Core builds without
Canic; libraries export no endpoints/lifecycle hooks. Production schema and
transport gates in [the service contract](../service-contract.md) remain active.

No controller/digest authority, cross-release compatibility shims or silent reset.
Cross-release transitions remain reinstall-only, with old obligations disposed of
before destructive retirement. Same-release recovery remains required. Whole
snapshot loads can bypass post_upgrade and are not qualified by fixture fences.

All Canic capabilities must work here before removal there; see
[parity](../canic-parity.md) and [acceptance](../acceptance-plan.md).
Canic source removal and each installation's retirement are separate obligations.
The maintainer's earlier acceptance for work here does not change Canic's handoff.
Sibling repositories are read-only; no commit, release, deployment or paid effect
is implied by continuation. Follow [governance](../governance/development.md).

Historical test results and source inventories remain evidence of their original
revisions; do not rotate them to a new checkout. The detailed pre-0.2 handoff is
retained in Git at v0.1.19; core evidence and the released changelog retain history.
Keep this status concise rather than accumulating that history again.
