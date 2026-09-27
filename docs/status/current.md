# Current status

Date: 2026-09-27

## Active work — after 0.2.3

The maintainer confirmed **0.2.3 is pushed**. Cargo and the release receipt are
0.2.3. Local main, origin/main and v0.2.3 resolve to
`c85f6e989f65fe7fdf3a4971ee066a88f9b80079`, from validated source
`4d9bcdb938c1b12aaafe7554977166ab82a854c5`. The receipt records the
`release-verify` gate; registry publication was not independently queried.
The worktree was clean at the start of this batch. Completed work is drafted in
the undated 0.2.4 changelog section, with Unreleased empty;
no version mutation, commit, publication, deployment or provider effect ran here.

Follow the [0.2 delivery plan](../roadmap.md). Its goal remains a usable
Caffeine-backed service through shared durable handlers, both adapters and an
operator client. Library publication does not qualify the service or Canic removal.

| Milestone | State | Remaining completion condition |
| --- | --- | --- |
| M1 — contract | In progress | Freeze admission/resource envelope, provider guarantees and operational recovery |
| M2 — durable standalone service | In progress: durable upload/reference/settlement bookkeeping and IC evidence | Complete reads/provider journals, shared handlers and operational recovery/journey evidence |
| M3 — Caffeine and operator integration | Not implemented | Qualified provider transport, completion/economics and production client |
| M4 — managed parity and acceptance | Not implemented | Same journey through Canic adapter, complete replacement evidence and handoff |

## Current focus — durable local funding intents

`ops::service::funding::StableFundingJournal` uses two host-granted memories for
bounded exact intent history and maintained attachment accounting. It binds the
service/operator/Cashier/account/namespace and an explicit allocation, reserve and
lifetime limit. New externally supplied IDs increase; replay checks the complete
local identity and amount. Prepared/uncertain intents retain the full attachment
and block later reservations. The first attempt marker persists possible dispatch;
repeating it rejects. The intent retains the sole maintained top-up method and
exact optional target balance. Shared encoding always supplies an explicit account
and preserves the target option independently of the attachment. `mark_attempted`
returns canonical call arguments after the marker write; operator request inspection
also works while fenced and grants no dispatch authority. No provider call or payment
endpoint is enabled.

Trusted-host enqueue-failure or exact unbounded-refund observations update one
intent and its totals in a synchronous IC transaction. Identical outcome replay
changes nothing; conflicting evidence and return-total overflow preserve state.
Outcomes additionally check separately supplied actual service/original target
against the retained identity before mutation. The host must authenticate that
transport context; payload assertions cannot supply it.
Accepted amounts remain charged independently of provider credit. Accounting shares
the existing `FundingAllocation` model, which also reconstructs retained history.
Reopen rejects missing/inconsistent rows and changed scope/allocation, then fences
every mutation, including late outcomes. Its own ordered IDs cannot prove freshness.

The storage probe now has two additional host memories and labelled bookkeeping
controls; it sends no cycles. Fourteen storage PocketIC cases pass, including
intent/accounting rollback, every funding phase through upgrade and canonical
request preservation with changed-argument/source rejection.
Targeted validation passes 169 core billing/policy/catalog/service/lifecycle/Cashier cases,
nine existing funding-probe unit cases, affected strict Clippy, release storage
Wasm and warning-free rustdoc. No full CI or resource benchmark refresh ran here.
A concurrent Cargo edit updated ic-testkit from 0.10.0 to 0.10.1 and reformatted the
workspace list; it was preserved, and final checks used that state. The allocator
and package version remain unchanged by this work.

Anonymous Cashier metadata was refreshed on 2026-09-27 and matches the retained
interface hash. Independent didc request vectors cover absent and maximal target
balances. The wire method has no operation-ID field; local intent correlation does
not establish remote idempotency or provider credit.

This is a coherent maintainer release checkpoint; the 0.2.4 changelog is drafted and full
release validation still needs to run. Next, qualify actual Cashier transport and
authenticated outcomes together with account activity, spendability and execution-cost
gates. Other provider journals, read sessions, adapters and operational
restoration remain incomplete. Continue keeping uncertain obligations inspectable
without permitting repeated effects or releasing the restore fence.

## Released foundation

The detailed prior handoff is retained in Git at v0.2.3. Historical results belong
to their original builds; see [core evidence](../evidence/core-primitives.md) and
[the changelog](../../CHANGELOG.md). The maintained implementation includes:

- Streaming Caffeine hashing, bounded manifests/builders and root verification.
  Direct browser/client upload is selected; admission has no mandatory raw digest
  or on-canister file hashing. Provider enforcement remains unqualified.
- A transient shared upload owner with explicit tenant/uploader/service/operator
  and namespace bindings, enrollment generations, exact permissions/retries,
  manifest/metadata validation and retained original headers.
- Indexed content discovery, exact-live-reference descriptors, admission/reference
  headroom and maintained quota totals. Reservation, logical release, physical
  deletion and billing cessation remain separate. Cancelled/settled operations,
  roots, leaves, references and receipts consume lifetime capacity.
- An unpublished IC admission probe delegating to that owner through bounded
  operation-specific inputs. Actual caller/time, stop/start and history-capacity
  evidence exists. Both upgrade hooks reject; operational persistence is absent.
- Independent billing/read/recovery PocketIC fixtures over labelled substitutes.
  Their restored journals stay permanently inspection-only; they do not prove
  operational recovery or deployed Caffeine behavior.
- Local manifest/inventory preparation and saved-body snapshots, verified file
  output, and `blob-fixture-inventory` joining prepared reports to capacity and
  reuse queries. Production transport, authentication and publication remain open.

The default Rust allocator is required; ic-memory remains stable-memory authority.
The released reference path avoids full lifecycle copies. Historical envelopes
include 704 objects / 288 MiB synthetic media, 256 references, maximal metadata and
one read slot. The recorded descriptor build used 4,587,520 allocated Wasm bytes;
final admission was 3.11M instructions and peak retain/release 5.43M/5.51M. Bulk
Candid decoding reduced the fixture's 1 MiB read to about 116M instructions, with
hashing still about 81M. These artifact-bound observations are not production
limits. Do not rotate their hashes or relabel old measurements for a new build.

## Durable upload baseline — released in 0.2.3

The maintainer clarified that the filesystem journal is optional client tooling,
not a requirement to run a local version of the service. Continue on shared stable
canister storage; do not expand local journal tooling as the primary delivery path.

`ops::service::tenant::StableTenantEnrollments` is the first shared stable
component. It accepts a host-granted `ic-memory` memory and writes individual
bounded v1 enrollment records, using the same model transition as the heap owner.
Metadata binds service, operator, namespace and lifetime tenant limit. The host
still owns installation/release checks and all other configuration/state stores.
No endpoint, lifecycle hook, grant or memory ID is exported implicitly.

Fresh installation rejects any allocated memory. Reopening uses load-only access,
checks the binding and every enrollment in one bounded pass, and returns an
inspection-only owner with all mutations fenced. Missing or corrupt state cannot
be initialized away. Records retain suspension, activation generation and lifetime
capacity. Synchronous canister hosts must propagate stable-memory traps for IC
rollback; native tests do not supply that platform transaction guarantee.

`StableRootClaims` adds two host-granted stable maps for immutable root claims and
the reverse object index, sharing the heap model's claim rules. Metadata binds
service, namespace and lifetime root capacity; full-width object identities are
preserved. Exact replay needs no extra capacity. Reopen validates the index
bijection, rejects missing/conflicting/orphaned history without repair, and fences
mutation. A root claim alone does not check enrollment or reserve object bytes.

`ops::service::uploads::StableUploads` exclusively owns enrollment, root claims,
exact permissions, immutable manifests, confirmed lifecycles, individual references
and receipts, maintained global/tenant totals and a root/request index. Ten distinct host-granted
memories commit together in synchronous IC updates. This replaces the earlier
unreleased pending-only owner; no compatibility alias or old schema path remains.
Shared model decisions govern admission, references, cleanup receipt capacity,
physical deletion and settlement. Ordinary reference mutations touch individual
rows instead of loading or copying the complete reference/receipt history.

Unexposed cancellation releases bytes but retains operation/root/leaf history.
Exposed uncertainty remains charged after revocation. Exact completion transfers
the reservation to a confirmed object and first reference without dropping bytes.
Last release removes logical bytes, physical deletion removes physical bytes, and
billing cessation removes liability bytes. References and exact success/failure
receipts remain retained after settlement. Replay never reactivates references;
suspension blocks fresh retains while preserving receipt replay and release.

Indexed tenant discovery preserves independent request/object IDs. Original metadata
and reference-qualified descriptors come from one bounded manifest record; the
latter require confirmed completion and the exact live reference. Unknown/foreign
roots return the same absence. Tenant and operator history/cleanup scans bound
inspected rows and results independently. Cursors bind service, namespace, scope,
filter and last inspected tenant/request ID; empty filtered pages advance. Fresh
sweeps are required for changes behind a cursor. Continuing billing remains in
the outstanding view after physical deletion. Admission/reference headroom uses
maintained counters and shared heap-model arithmetic, preserving release receipt
capacity and lifetime operation/reference/leaf history. Operator-only root batches
use indexes, retaining order, duplicates, malformed positions and every local phase.
Scope/authority checks apply even to empty batches. These observations do not
implement gateway callbacks or grant provider deletion/retry authority. Suspended/restored reads remain
inspection-only and cannot release the mutation fence.

Bounded v1 records preserve full-width identities and first-accepted metadata.
The manifest codec enforces 64 KiB while variable-size tree pages avoid allocating
maximum-value nodes for small declarations. Installation rejects oversized
manifest envelopes before any allocation. Reopen validates upload configuration,
all store relationships, manifests, reference/receipt counts and recomputed totals without repair, then
fences every mutation. Host release identity and billing/provider configuration
checks remain separate. Reopening is inspection, not operational recovery.

The unpublished storage probe uses this single owner. Eleven PocketIC cases cover
admission, preparation, cancellation, completion, reference/receipt and settlement
write traps, then same-release upgrades of pending and every confirmed phase.
Stable and cached state roll back together; missing receipts after a trap do not
consume cleanup headroom. Stop/start preserves state; restored mutations remain
fenced. Changed-operator restoration leaves the prior instance unchanged. Faults
exist only in fixture memory wrappers. Completion/deletion/settlement facts are
operator-only labelled substitutes, not evidence of deployed Caffeine behavior.
The core's confirmation APIs require independent host authentication/correlation.
The expanded admission fault case includes the root/request index write. The two
new read cases check actual caller isolation, exact-reference descriptors through
upgrade, changed cursors, bounded empty pages and fresh cleanup sweeps.
Two planning cases additionally check admission/reference headroom through cleanup
and settlement, caller/scope isolation, raw root-batch bounds and order, uncertain
and cancelled roots, unchanged accounting and fenced inspection after upgrade.

The 0.2.3 implementation validation passed 122 targeted catalog/service/lifecycle cases,
including heap/stable accounting agreement, cleanup headroom, immutable historical
failures, every release phase, missing/orphaned rows, codec widths, near-bound
manifests, small-manifest allocation, root/request consistency, bounded reads,
heap/stable headroom agreement through settlement, shared global contention and
operator root observations across every phase.
The final storage layout is also checked by the eleven PocketIC cases, strict affected all-target Clippy,
release storage-probe Wasm and warning-free core rustdoc. External dependency
versions and the default allocator are unchanged. No full CI or resource
benchmark refresh ran; the small fixture is not production sizing evidence.

The heap owner still supplies verified-read APIs. Complete provider-call intents,
callback authority/correlation, read sessions, provider
economics and actual adapters remain incomplete. Do not imply all service
obligations survive yet. Operational restoration still needs a complete obligation
source and independently surviving authority; the current fence has no unfence API.

## Receipt inspection and local intent journal — released in 0.2.3

`UploadAdmissions::reference_receipt` authenticates and reads an exact reference
operation's original result without mutation. Mutation replay uses the same check
path. Unknown roots and changed identities remain errors; an absent receipt
allocates nothing. Suspension, full history and settlement preserve original
success or typed lifecycle failure. Historical retain success is not current
liveness. The private bounded query also checks the original upload's declared size.

`blob-fixture-reference` saves a bounded exact `ReferenceIntentRecord` in an
identity-keyed local journal, then queries only that receipt. The record binds the
asset label, service, tenant, namespace, upload/object/lifetime, root, bytes,
reference, operation and retain/release action. IDs are explicit full-width decimal
strings, never automatically allocated. The fixture fixes object ID to upload ID
and incarnation to one; both are recorded explicitly.

Saving holds an exclusive OS file lock, syncs a private file, installs it without
replacement and syncs the journal directory before acknowledgment. Exact retries
recover the same file; changed payloads conflict. At most 4,096 entries besides
the permanent `.writer.lock` are permitted, including interrupted residue. Full
journals still allow exact recovery; there is no automatic deletion or eviction.
The existing journal directory must be durable and caller-controlled. The lock
file must never be removed/replaced; process exit releases the OS lock.

This is tested Linux local persistence, not production service recovery. Files
remain mutable, storage errors can leave unacknowledged records, and copies or
rollbacks have no independent freshness authority. No saved file or receipt proves
ID freshness, restored-instance authority or safe provider retry. The tool sends
no mutations. Consumer transaction and outbox coordination remain outstanding.

Latest targeted validation passes all 26 host-tool unit cases (nine reference
cases), a subprocess termination/lost-output recovery case and the three updated
query/executable PocketIC cases. Strict host-tool all-target Clippy passes. The
preceding receipt step passed 37 upload model cases, release admission Wasm,
affected Clippy and warning-free core rustdoc for that preceding step.
Existing locked tempfile and sha2 are used by the unpublished executable; library
dependencies, allocator and package versions are unchanged. No full CI, hardware
power-loss test or resource benchmark refresh ran this batch.

## Next work and open gates

Before publisher dispatch, settle production intent storage, copy/restore fencing
and a surviving allocation authority. The local journal now serializes cooperating
writers and recovers exact writes, but cannot detect a stale copy. Preserve exact operations after
unknown outcomes; absence never creates retry authority. The inventory's one fresh
reference per asset is explicit, but new-object reference sizing is unassessed.
Production descriptor delivery, provider locator/serving policy and consumer
registration/release coordination still need implementation and evidence.

Close M1 provider guarantees before real certificates: pre-charge size/tree
limits, namespace/owner/project/bucket and replay charging, independent completion
and actual size, lost paid-response reconciliation, physical deletion and final
billing cessation. Recovery needs a surviving complete obligation source; an IC
version counter or counter in the same old backup is insufficient. No trial
account, namespace or paid budget is selected. See [provider review](../provider-review.md)
and [service contract](../service-contract.md).

Reuse requires a live reference. Final release queues deletion; retired roots
cannot be reallocated even after settlement. Deleted-content reintroduction is an
open M1 identity/provider decision. Never weaken callback safety or erase charges.

Toko review uses remote `development`; the local checkout is absent/stale. The
[pinned review](../evidence/toko-0.2-review.json) records 10 MiB/file and 500 MiB
staging inputs. [Miner feedback](../roadmap.md#toko-miner-feedback--2026-09-27) adds
proposed headless release media with 702 files / 270.1 MiB; this is not a qualified
publish list or approved adoption. Recheck upstream before depending on new
provider/consumer behavior. No sibling edits or consumer tests ran here.

## Constraints that remain active

Caffeine is the sole provider. Core builds without Canic; both adapters belong
here and use shared handlers. Linking a library exports no endpoints/lifecycle.
Production schema/transport gates remain open: another transient fixture does not
complete M2, and local file storage does not qualify the production outbox.

AGENTS.md explicitly requires 100% hard cuts before 1.0: remove superseded forms
and update consumers/tests/docs together, without compatibility or migration paths.
Pre-1.0 cross-release transitions remain reinstall-only; same-release interruption
recovery is required. Source removal and installation retirement are separate.
Never erase the only provider, balance, uncertain-effect or billing records. All
Canic capabilities must work here before removal there; see [parity](../canic-parity.md)
and [acceptance](../acceptance-plan.md). This work does not accept Canic's closeout.

No controller/digest authority, automatic funding, compatibility shims or silent
reset. Siblings are read-only. Commits remain maintainer-owned; release/publication
need explicit authority and cleanup is never implicit. Follow
[governance](../governance/development.md); use targeted implementation checks.
