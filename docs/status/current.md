# Current status

Date: 2026-09-26

## Released baseline

The maintainer reports 0.1.11 pushed. Local release/tag is `b1d5595`, from source
`3b5ab645e579e8193c951c7b47b649cd921abe65`; Cargo and the receipt are 0.1.11.
`make release-tag-check` passed after release. Registry publication was not
independently queried. Release/publication preserve artifacts; cleanup requires
an explicit request. See [release guidance](../releasing.md).

The library provides content/provider identities, streaming Caffeine hashing,
root-bound manifests, chunk verification/coverage and bounded missing-chunk pages.
It also provides billing/configuration validation, bounded provider reply codecs,
pure funding/readiness policy and scoped gateway synchronization with stale-reply
rejection. None establishes provider completion or durable recovery.

The transient catalog owns confirmed-object lifecycles, immutable root claims,
exact reference receipts and bounded deletion pages. It separates logical,
physical and liability capacity; zero-byte/settled objects retain lifetime slots.
Tenant/gateway policy checks full bindings and current authority on reads.

PocketIC fixtures exercise actual caller/controller isolation, reference replay,
gateway revocation and inter-canister sync races. Fixed content vectors run in
Wasm with an ordered-append instruction budget. Fixtures substitute provider facts,
have no persistence and do not qualify production endpoints or restore safety.
The unpublished host harness owns testkit; the core package excludes PocketIC.

Released source inventories remain historical: 0.1.5 matches `6bd0d45` (Cargo
0.1.4), 0.1.6 matches `d3ca8c4` (Cargo 0.1.5), 0.1.7 matches `600315e` (Cargo
0.1.6), 0.1.8 matches `20ec33d` (Cargo 0.1.7), and 0.1.9 matches `832b364`
(Cargo 0.1.8). The 0.1.10 chunk-verification inventory was verified against Git
source `aaa5057` (Cargo 0.1.9). Do not rotate these for later source/version changes.

The 0.1.11 upload-admission inventory also matches released source `3b5ab64`
(Cargo 0.1.10), verified against Git after release. It remains historical.
That release adds transient upload reservations, exact retries, uncertainty
retention and transfer into confirmed accounting; bounded tenant pages and
single-scan gateway batches include pending state. PocketIC covers actual
caller/controller isolation, cancellation replay and gateway revocation.

## Current follow-up — 0.1.12

The maintainer requested continuation, Canic's cloc helper, and alignment through
ic-memory with Canic and IcyDB. Cargo stays 0.1.11; completed changes are grouped
in the undated 0.1.12 changelog draft.

Replaced the unused direct stable-structures dependency with published ic-memory
0.14.3 and re-exported it. Stable collections come through
`ic_blob_storage::ic_memory::ic_stable_structures`. A combined graph with
canic-core 0.110.42 and icydb-core 0.261.11 resolves one ic-memory 0.14.3 and one
stable-structures 0.7.2. The core does not depend on either framework. New lockfile
entries are ic-memory and its missing transitives; existing versions are retained.

Native composition tests verify no implicit allocation declarations/bootstrap,
shared host handle types, isolated cells and preserved host bucket configuration.
The host owns bootstrap/policy/grants; no blob schemas, keys, IDs, stable stores,
raw manager or recovery workflow are added. See
[memory composition](../dependencies.md#memory-composition-with-canic-and-icydb).

Copied `scripts/dev/cloc.sh` from Canic and changed its canic-name filter to
manifest-bearing directories under crates/. `make cloc` runs it; cloc/jq are
optional developer tools, while shell syntax/lint joins shell-check. File-based
LOC classification retains inline tests in runtime LOC and reports their function
count separately. Canister fixtures/host tests outside crates/ are excluded.

Native suite, memory composition tests, workspace Clippy, Wasm, rustdoc, PocketIC
regressions, formatting and package verification pass. The cloc helper passes
Bash/ShellCheck and runs from the repo and /tmp. Evidence is in
[core primitives](../evidence/core-primitives.md#memory-dependency-alignment-after-0111).
No full CI, version mutation, commit, publication or sibling edit ran.

Continued 0.1.12 with bounded tenant pages for unsettled confirmed objects. These
include Live, DeletionPending and ProviderDeleted, including zero-byte objects;
Settled entries keep history slots but leave results. Each result exposes its
full binding and separate logical/physical/liability bytes. A private root index
holds one entry per confirmed object, bounds tenant-only scans and also serves
tenant usage. Pending upload reservations remain in the existing upload views.
Upload tenant usage now ranges over that tenant's existing ordered operation
keys rather than filtering global history. Global totals still include all
tenants; no cached counters or second upload index were added. A targeted mixed
phase test includes minimum/maximum IDs, multiple namespaces and neighboring
tenants. Admission/read tests, Clippy, Wasm, rustdoc and PocketIC regressions pass.

Native tests cover independent scan/result limits, sparse settled history,
scope rejection, replay/rejection/index consistency, upload confirmation transfer,
intervening settlement and newly inserted lower roots. PocketIC verifies actual
caller isolation and observes release, physical deletion and billing cessation
separately using explicit operator-supplied substitute facts. Native, Clippy,
Wasm, rustdoc and PocketIC regressions pass. See
[obligation evidence](../evidence/core-primitives.md#tenant-obligation-views-after-0111).
These reads are transient; no provider transport, stable schema or recovery
authority is introduced.

The next integration work remains durable intent/reservation ownership and
recovery fences, provider mapping/protection of pending roots, exact provider
reconciliation, shared handlers and both adapters. Memory allocation governance
does not establish transaction atomicity or resolve uncertain paid effects.
Resolve the gates below before provider transports or persisted workflows.

## Provider evidence and next work

The public-source follow-up rechecked Caffeine GitHub main at
`e5cacdfe5ce55e939edb02980fca800c0c13f421`, backend Storage/Mixin hashes and npm
latest 1.1.2/integrity; all match retained evidence. The current export and storage
cost guidance still supplies no exact upload/deletion/final-charge receipt
contract. This follow-up did not re-query Mops or deployed Cashier. See
[provider review](../provider-review.md#upload-admission-follow-up--2026-09-26).

Toko's retained source `6519b72d2a420564dabaf700fc55f7b8603d9fd3` supplies
`https://blob.caffeine.ai` and Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`.
[Deployment observations](../evidence/caffeine-deployment-observation.json)
identify a reachable candidate, not an owned account or deployment authorization.
Caffeine remains selected but unqualified. Its independent-export guidance says
managed file storage needs replacement; a separately supported arrangement is
not established. The earlier provider-contact/private-source question remains
unanswered; no message was sent externally.

Resolve before provider transports or persisted workflows:

1. Supported independent onboarding and exact account/project/bucket ownership,
   including exclusive namespace/callback authority.
2. Authoritative completion lookup tied to the original operation, including
   lost replies, incomplete objects and numeric evidence-retention bounds.
3. Exact top-up accepted/refunded amounts after a lost reply; balances and typed
   errors do not settle a particular payment.
4. Object-specific deletion/final-billing evidence, durable root/intent history
   and surviving authority across the same-release restore boundary.

The [service contract](../service-contract.md) also needs a concrete consumer,
accountable owners and production resource bounds. Local exceptions remain in
force; this batch does not waive provider or persistence gates. Refresh provider
pins before boundary implementation. Paid qualification needs explicit authority
and bounded resources. No end-to-end service capability is qualified.

## Ownership

The maintainer confirmed Canic's 0.110 acceptance for work here without changing
Canic's handoff. Agents must not commit, change versions or infer deployment/
publication authority from continuation. Siblings remain read-only. All Canic
blob capabilities must work here before removal there, with installation
retirement handled separately; see [parity](../canic-parity.md) and
[acceptance](../acceptance-plan.md). Library publication is separately enabled
and does not establish service qualification.
