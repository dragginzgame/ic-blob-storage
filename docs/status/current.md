# Current status

Date: 2026-09-27

## Active work — 0.2 service phase

The maintainer confirmed **0.1.19 is live** and authorized starting 0.2.
Cargo and the release receipt are 0.1.19. Local main, origin/main and v0.1.19
resolve to `12077974ed353fd09f84a425012ed41c2006a8fd`, released from
`82ced1bd41e6d782806176055f29001c84dc8af7`. Registry publication was not queried.
The 0.2.0 changelog is an undated development draft; no version mutation ran.

**Release recommendation:** this batch is a coherent 0.2.0 library cut: breaking
direct-upload admission, explicit enrollment/budgets and canonical metadata.
Focused checks pass; the read-only minor release plan selects 0.2.0. The maintainer
can commit the complete batch and run `make release-minor` from clean main. Its
full validation gate remains required before tagging/pushing. This recommendation
does not claim the production service or the full 0.2 plan is complete.

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
Candid work, skipped values and type headers before manifest conversion. A 10 MiB
declaration through admission, preparation, exact retry and local exposure costs
4,107,938 measured instructions and 1,766 request bytes, with allocated Wasm memory
steady at 1,245,184 bytes. Counters exclude diagnostics/reply encoding, persistence
and provider transport. See [resource evidence](../evidence/core-primitives.md#local-admission-resource-measurements).
Stop/start preserves its owner; unsupported upgrades reject atomically even when
the outgoing hook is skipped. There is no operational restore path.

Current targeted validation passes 29 native service tests, eight independent
Caffeine hashing tests (including the canonical media vectors) and 11 admission
PocketIC cases, with the release admission Wasm build, strict affected all-target
Clippy and warning-free library rustdoc. The preceding hard-cut validation passed
11 catalog-admission, 14 upload-read/tenant-obligation, 46 integrity/recovery and
one upload-ownership PocketIC cases; those unchanged paths were not rerun here.
Full CI was not run; package versions remain unchanged.
The independent integrity/recovery fixtures retain their own raw digest binding;
no mandatory byte-verification workflow remains in service admission.

The preceding upload-path evaluation rechecked official main, npm latest/integrity
and Toko development unchanged. This implementation made no provider/account call
or source refresh. Stop/start, same-release durable upgrade and older-backup
reconciliation remain separate requirements. The IC version counter does not
prove inventory completeness or identify only restores. A surviving full obligation
source remains required; no new server guarantee was established.
See [provider review](../provider-review.md#02-admission-follow-up--2026-09-27).

**Next after the library cut:** close the remaining M1 decisions: surviving recovery authority, provider
certificate replay/namespace/completion guarantees and the operational resource
envelope (remaining work includes total memory/instructions and read sessions).
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
