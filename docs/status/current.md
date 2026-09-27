# Current status

Date: 2026-09-27

## Active work — 0.2 service phase

The maintainer confirmed **0.2.0 is live**. Cargo and the release receipt are
0.2.0. Local main, origin/main and v0.2.0 resolve to
`f90d50cc58086ed2d45948eda8b5714cba341fbc`, from validated source
`6f0a12e534c0a0d3f4758056c422f423ac5141aa`. The receipt records the
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
lifetime slots. All measured updates stay below 10M instructions, but roughly 98%
of the last admission's 5.43M occurs before workflow dispatch. Investigate decoder
and allocation costs before production sizing; the specific cause is not proven.
Provider facts are operator-only substitutes. Read sessions, larger per-object
histories and operational persistence remain unqualified.

Latest targeted validation passes 15 lifecycle and 33 service unit cases, all 16
admission PocketIC cases, affected strict all-target Clippy, warning-free rustdoc,
the admission release Wasm build, formatting and diff checks. The resource helper
additionally emits `.tmp/release-history.json`. Earlier accounting, catalog and
integrity/recovery results remain in the linked core evidence at their original
revisions. Full CI was not run; package versions remain 0.2.0.
The independent integrity/recovery fixtures retain their own raw digest binding;
no mandatory byte-verification workflow remains in service admission.

The preceding upload-path evaluation rechecked official main, npm latest/integrity
and Toko development unchanged. This implementation made no provider/account call
or source refresh. Stop/start, same-release durable upgrade and older-backup
reconciliation remain separate requirements. The IC version counter does not
prove inventory completeness or identify only restores. A surviving full obligation
source remains required; no new server guarantee was established.
See [provider review](../provider-review.md#02-admission-follow-up--2026-09-27).

**Next:** close the remaining M1 decisions: surviving recovery authority, provider
certificate replay/namespace/completion guarantees and the operational resource
envelope (remaining work includes total memory/instructions and read sessions).
Include repeated-release reuse/reintroduction and multi-file history sizing from
the Miner feedback; a single-file or cancelled-history fixture cannot qualify them.
The 704-object local baseline is now recorded; isolate the growing pre-workflow
instruction cost and size larger reference histories/read sessions next.
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
