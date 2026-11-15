# Caffeine integration contract review

Source review, mainnet reads, bounded funding/linking and a single-use Rust probe,
updated 2026-11-15.
This is the provider evidence companion to the
[rewrite design](ic-blob-storage-design.md#d0-first-gate-supported-independent-caffeine-access),
not a selected production protocol or a complete provider qualification.

**Finding:** DFINITY publishes an independent Rust integration and account-linking
recipe. Anonymous mainnet reads now confirm the advertised Cashier ABI, gateway
principal and price list. This supersedes the earlier search's lack of a public
Rust enrollment recipe. A fresh payer deposit/credit now succeeds with retained
request custody. The installed Rust probe now has a retained owner-origin refusal;
a temporary payer link allocated 1B and was removed. An absent-object HTTP read
returned 404. Positive upload admission, provider enforcement, safe recovery and
final closure remain unqualified. Proceed through independent qualification;
provider correspondence is not a prerequisite. Keep API/persistence experimental
and do not describe the service as resumable or settled yet.

## Evidence and its limits

Legacy sources below are pinned to
`25bed1352f3d2b9659f0d2583f50fa9af859ea84`. Provider sources are pinned to
`caffeinelabs/skills` at `59b9b29a4fad3828677c8b80ead0e06fdb40987e`, whose checked-in
frontend package identifies itself as `@caffeineai/object-storage` 1.1.2. This is
a source selection for review, not a registry-freshness claim or dependency adoption.

| Evidence | What it establishes | Limit |
| --- | --- | --- |
| [DFINITY Rust example][dfinity-example] and [implementation][dfinity-storage] | Published independent deployment, account funding/linking and exact Rust callback shapes | Example source, not fresh enrollment or proof of gateway enforcement; unsafe sample policies are excluded below |
| [November 15 mainnet observation](caffeine-observation.json) | Funding, installation, temporary link/allocation/removal, retained owner-call refusal, reported gateway budget and one missing-tree HTTP response | Neither reported budget nor a negative read proves upload admission, callback compatibility or charge cutoff; query observations are not consensus-certified contracts |
| [Legacy standalone trial summary][trial] | Reports two distinct authenticated uploaders, one original certificate call and two gateway PUTs each, and independently root/byte-verified 1 KiB downloads | Historical 0.73.0 behavior; private journals/artifacts were not revalidated; no representative multi-chunk or application-catalog acceptance |
| [Legacy provider setup summary][funding] | Reports a separate payer, deposit/sweep, payer-to-owner relationship, owner allocation and spendable gateway credit | No general enrollment guarantee, namespace exclusivity, itemized price or charge cutoff |
| [Legacy deletion follow-up][gc-readback] | November 5 summary reports the released object still DeletionPending, zero recorded acknowledgment attempts and an HTTP HEAD 200 | Historical observation only; telemetry is incomplete, HTTP may be cached, and neither origin deletion nor billing cessation was established |
| [Provider SDK][sdk] and [Motoko boundary][mixin] | Expose certificate method, transfer shapes and callback signatures | Client/helper source does not establish the gateway's server-side validation or failure semantics |
| [Current GitHub integration guide][hosting] | Says independently deployed exported apps must replace managed storage | Does not establish support for this lower-level arrangement; successful historical traffic does not settle support |
| [Caffeine #16 — Clarify object-storage GC callback contract for non-Motoko consumers][gc-issue] | Existing question covers key mapping, callback failure, deletion, replay and billing | Still open with zero comments when checked; no provider answer to adopt |
| [Rabbithole storage description][rabbit-storage] and [account integration][rabbit-account] | Independent consumer describes Caffeine-backed personal canisters; its public source funds the canister's own Cashier account | Motoko consumer implementation, not a Caffeine support commitment, server contract or live observation by this review |

Source, public guidance, retained legacy summaries and public Cashier reads were
used. The legacy trial's live references and unresolved charges stay with its owner.
The funding continuation below read the authorized `canic-mainnet` balances and
used its configured signer to convert ICP to cycles, fund the fresh payer and
create and install one owner probe, then qualify a temporary payer link. No key
was exported, legacy journal resumed, certificate issued or object uploaded.

### Independent qualification — 2026-11-15

The maintainer directs independent research without Caffeine outreach and permits
the local `canic-mainnet` wallet to pay cycles when needed. The immediate route is
public protocol evidence, then bounded fresh-account qualification, rather than
waiting for an answer to issue #16. Preserve possible effects before paid calls;
neither permission to fund nor a finite deposit establishes finite liability.

DFINITY's example is pinned to `ef29e8a6e8063c6fe654cac53a3497cab585fefa`.
Its integration checklist deploys a Rust backend, funds a payment account, obtains
gateway principals and links the backend with a daily spending limit. This is a
concrete **separate payer/owner bootstrap candidate**. The legacy self-funded
canister path remains a comparison, not a second implementation mode. The README's
10T top-up and 5T daily limit are examples, not this project's selected budget.
Its download of a moving `icfs/latest` executable was not followed or adopted.

The actual Rust source confirms:

- The host returns `{ method = "upload"; blob_hash }` from a root-only update.
  Usable authority comes from the IC-certified reply, not this plain record.
- Liveness accepts `vec blob` and returns `vec bool`; the deletion list returns
  **`vec text`**; confirmation accepts `vec blob`. This strengthens the text-list
  candidate found in Toko, but does not prove which profile the live scrubber uses.
- Gateway refresh calls `storage_gateway_principal_list_v1` with bounded wait.
  A client query declaration does not prevent an inter-canister update invocation.
- Certificate creation has no caller authorization and clears pending deletion;
  confirmation removes stored rows even if not pending. Liveness returns false
  for missing roots and is unauthenticated, despite the README's broader claim
  that lifecycle methods check the gateway. None of these policies is adopted.
  The sample also does not demonstrate preservation of its heap-only Cashier
  configuration across upgrade. Retain Blob's existing ownership/lifecycle rules.

Mainnet reads used `icp 1.6.0`, anonymous identity, `https://icp-api.io` and the
built-in mainnet root, targeting `72ch2-fiaaa-aaaar-qbsvq-cai`. The receipt records
exact methods, hashes and results. The extracted `candid:service` matches the
[legacy deployed interface][legacy-did] except trailing whitespace. Balance and
both gateway-list methods are advertised as queries; top-up takes an optional
record and returns a typed result. Both public query outputs match the hashes in
the [September observation][legacy-observation]. This is a new observation of
unchanged results, not a new qualification of that historical run.

The DID also exposes `storage_canister_api_version_get_v1` and a setter. Their
existence does not tell us which version maps to text/binary deletion lists;
resolve this during fresh-account callback qualification, not with fallback codecs.
The price response alone supplies neither the storage billing interval nor a
charge cutoff. DFINITY's README claims 30-day prepaid storage and eventual deletion
after zero balance; the consumer billing guide describes different behavior.
Record these as account-specific claims to investigate, not interchangeable terms
or a proven spending cap. No prices are compiled into Blob.

**Next operation:** preserve the completed probe and existing owner credit recorded
below. Investigate actual gateway admission and the callback/authority contract;
do not repeat the refused zero-attachment top-up or add another payer link. The
next implementation gate is the local provider-host slice below. The first object probe follows only after
the host enforces grant ownership, irreversible Closing and retained exposure.
Missing lifetime/closure evidence keeps exposure unresolved; observed deletion
must never unlock Settled. General guarantees cannot be inferred from one probe.

### Funding preflight and authorized mint — 2026-11-15

The authorized local identity is `canic-mainnet`, principal
`o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
Its default Cycles Ledger account is a concrete funding source; no wallet canister
is needed for this route. Anonymous Cashier balance lookup returned
`Err(AccountNotFound)`. Its deposit-subaccount query returned
`1d1837c40b25f7971f603981da2ca474f8264b001cb94eca51a4fc289d020000`.
Those were preflight observations, not certified proof of fresh state. They were
rechecked before the bounded funding run below; this account is now funded and
must not be treated as a new or absent account on a later attempt.

The live Cycles Ledger `um5iw-rqaaa-aaaaq-qaaba-cai` advertises `icrc1_transfer`
and `create_canister`; its queried fee is **100,000,000 cycles**. DFINITY's
[creation-cost table][cycle-costs] lists **500,000,000,000 cycles** on a 13-node
subnet, before operating funds. The initial 415,079,115,186-cycle balance was
insufficient. The maintainer then explicitly requested conversion of at least 50T
from newly supplied ICP. That conversion completed: **50,000,000,009,024 net cycles**
credited, giving **50,415,079,124,210 cycles** at the post-operation read.
That conversion funded the user's ledger account; it paid no cycles to Caffeine.
The subsequent, separately retained deposit is described below.

The conversion cost 30.60543552 ICP plus a 0.0001 ICP ledger fee. Original ICP
transfer block **40446894** and Cycles Ledger mint block **17206042** are retained
with exact signed transfer/notification requests, arguments, quote and outcomes
under private Git-ignored `.operations/mint-50t-20261115/`. This is completed
financial custody, not disposable `target/` evidence or a product subsystem.
Do not repeat the mint. Both balance reads reconcile with the acknowledged results.

The [CLI 1.6.0 mint implementation][cli-mint] performs a transfer with no explicit
creation timestamp and hides the transfer block until the subsequent notification
completes. We used its existing `canister call --sign-only` / `message send`
commands for the same protocol instead, retaining a timestamped transfer and then
notifying only its acknowledged block. No new signing, payment or recovery engine
was added. The configured signer was used without exporting its key.

The **enrollment budget** is 1T for one owner
creation (including the subnet creation fee), 10B for one Cashier deposit, and
100M for each ledger create/transfer fee: **1.0102T maximum source debit** under
the observed fee. Any Cashier sweep fee must come from the 10B deposit; credit is
measured, not assumed. Refresh fees before dispatch; changed fees require updating
this plan. Remaining owner cycles cover installation/execution, with no automatic
top-up. This allocation is not a bound on later provider storage liability.
The payer-only step spent 10.1B; the later empty-owner creation spent the remaining
1.0001T. The original source-debit budget is fully consumed. Funding was performed
first to isolate that useful witness from unresolved link semantics.

| Step | Exact operation | Required result / refusal |
| --- | --- | --- |
| 1 — complete | Ledger `create_canister`, amount 1T, explicit sole controller and selected 13-node application subnet, fixed `created_at_time`, default source subaccount | Acknowledged block 17206696, owner `mjjg7-wqaaa-aaaai-ayghq-cai`; reuse this owner and preserve its request custody |
| 2 — complete | Ledger `icrc1_transfer`, 10B, explicit 100M fee, returned payer subaccount, fixed memo/time | Acknowledged block 17206169; never repeat this deposit or use Cashier's default ledger account/canister top-up |
| 3 — complete | Payer-signed `cycles_ledger_deposit_notify_v1(record { account = opt payer })` | Acknowledged 9.9B credit, reported ledger block 17206177. This is a mutation, not a status lookup; missing later observations do not authorize repeating it |
| 4 — observed, interpretation corrected | Inspect overdraft and target settings | Zero overdraft and a null target were reported. Null selects the provider default; it does **not** disable refill. No settings mutation was made |
| 5 — completed experiment below | `payment_account_canister_add_v1` with actual owner, explicit payer, 1B daily allowance and ten-minute absolute expiry | Link allocated 1B immediately; exact terms and removal were read back. Expiry/daily-limit enforcement remains under qualification |

The static artifact findings and selected experiment below now supply the
relationship terms; the installed owner already exists. Reviewed legacy [funding evidence][funding-review]
supports this deposit/notify distinction but supplies no retry permission. The
DFINITY [setup script][dfinity-setup] calls `add_gateway_principal`, absent from
its Rust backend, and guesses a principal with text matching. Use the backend's
actual refresh method and typed replies; do not execute that script wholesale.

### Fresh payer funding witness — 2026-11-15

One ledger transfer and one Cashier notification completed under the 10.1B source
debit cap. The selected payer had no reported Cashier account, its deposit address
held zero, and the refreshed interface/destination/fee matched the preflight.
Afterward, an independent balance query named the expected payer and reported
**9,900,000,000 cycles**, entirely in `cycles_ledger`, with debt target `Ledger`.
The deposit address read zero. Source balance fell by exactly 10.1B to
**50,404,979,124,210 cycles**: 100M transfer fee plus the 10B deposit. The deposit
less credited amount was another 100M; this is an observed difference, not a
general fee schedule or itemized bill.

Payer-authenticated queries reported `overdraft_limit = 0`, `target_balance = null`
and an empty relationship list with no continuation token. No settings update,
owner creation, relationship, certificate or upload was performed. Exact signed
requests, preflight, acknowledged results and readback are retained privately in
`.operations/caffeine-enrollment-01007/`, outside disposable build output. Preserve
the funded account and its custody; neither a refund nor closure is inferred.
The observed happy path supplies no permission to retry an uncertain notification.

The [legacy short-expiry trial][legacy-expiry] was inconclusive: it read zero gateway
credit both before and after the candidate expiry. Returned nanosecond-shaped
timestamps alone do not qualify units or enforcement. The [later legacy setup][funding]
separately reported owner allocation and usable gateway credit. These distinguish
**payer credit, payment relationship, owner allocation and gateway admission**.
Next qualify the recorded fresh owner's binding/credit path using that evidence;
do not require another deposit simply because gateway credit is zero. Any expiry
experiment must be labelled as such and bounded independently of assumed expiry.

### Fresh empty owner witness — 2026-11-15

One timestamped Cycles Ledger `create_canister` call returned owner
`mjjg7-wqaaa-aaaai-ayghq-cai` and block **17206696**. The request explicitly selected
`brlsh-zidhj-3yy3e-6vqbz-7xnih-xeq2l-as5oc-g32c4-i5pdn-2wwof-oae` and the existing
`canic-mainnet` principal as sole controller. Before dispatch, the CMC default
subnet list included that subnet and the [IC dashboard API][subnet-directory]
reported an application subnet with 13 nodes. These are preflight observations;
the later status replies do not independently report routing subnet identity.

Source balance fell from **50,404,979,124,210** to **49,404,879,124,210 cycles**,
exactly the planned **1,000,100,000,000-cycle debit**. Authenticated management
status reported **499,998,056,000 cycles**, zero reserved cycles and no module;
public state-tree inspection agreed on the sole controller and absent module.
The initial owner balance is an observation after creation/status execution,
not an itemized creation bill. Status also reported an idle burn of
864,319,605 cycles/day; the empty canister's balance is not static.

Original signed request, exclusive dispatch claim, preflight, result and readback
remain in private Git-ignored `.operations/caffeine-owner-01008/`. Reuse this
owner; do not repeat creation, mint or payer funding. The full original 1.0102T
source-debit budget has been consumed. Its remaining canister cycles and the
previously credited 9.9B Cashier balance are separate resources, not permission
for another top-up or proof of bounded storage liability.

This witness establishes an independently controlled **empty IC canister**, not
a Rust/Caffeine integration. No Wasm, payment relationship, certificate or object
was introduced. The narrow owner-origin probe below is now locally qualified;
the subsequent retained installation witness is below. Link expiry/limit behavior, owner allocation and gateway credit still
need evidence. D0 remains open.

[subnet-directory]: https://ic-api.internetcomputer.org/api/v3/subnets

### Single-use enrollment probe — local qualification for 0.100.8

The unpublished [qualification package](../../qualification/ic-blob-storage-enrollment-probe/Cargo.toml)
owns exactly one immutable owner-origin `account_top_up_v1` attempt. It uses the
existing Platform transport/Candid codec, Storage cell and Wire byte bounds.
The product library, host and CLI gain no funding API or provider effects.

| Boundary | Selected rule |
| --- | --- |
| Installation | Explicit operator and Cashier; positive target balance at most the pilot's 9.9B credit. Operator must be a controller. Account is always the installed canister's own principal. Target balance is not an enforced debit/charge cap |
| Request | Retain exact Candid bytes at initialization: present request, present explicit account, present target balance. Method is fixed; attached cycles are always zero; bounded wait is 30 seconds |
| Dispatch | Configured operator must still be a controller. Persist the attempt before the external await. All subsequent invocations return the same retained attempt; no request ID, reset, retry, timer or reconfiguration path |
| Callback | Record transport result before rechecking operator authority. Raw replies up to 8 KiB are retained without interpreting them as provider success; rejection/oversize leaves effects unresolved |
| Inspection | Nonmutating replicated `enrollment_inspect` update; same authority check, no dispatch. `enrollment_attempt` is the only effect-capable endpoint. Both take unit Wire requests; lifecycle inputs use `enrollment_init` and `enrollment_upgrade` frames |
| Upgrade | Stop and drain runtime calls; pre-upgrade refuses an active callback. Reopen the bounded versioned cell and verify owner, configuration and original bytes. A retained attempt without a result never becomes ready again |

`make test-enrollment` passes one native maximum-record test and three installed
cases. They cover exact retained account/target arguments, caller denial and bad
frames without mutation, concurrent replay, upgrade refusal during a held call,
revocation before result delivery, lost ingress results, completed/rejected
outcomes through upgrade, malformed/maximum/oversized replies, unavailable peers
and refusal to reopen foreign/missing state. A same-owner pre-callback stable
image injected after draining also reopens as attempted without resubmission;
that synthetic interruption is not a snapshot-continuity witness. The maximum record uses Wire's
existing bounded byte type, avoiding per-element traversal of an opaque reply.
Testkit owns synthetic raw/held replies; they do not validate Caffeine behavior
or prove timeout/charge semantics. The separate mainnet installation witness below
does not exercise the provider call. Unrestricted controller reinstall/snapshot replacement is
outside the supported stop/drain/upgrade procedure; no continuity claim follows.

Targeted legacy review at `25bed1352f3d2b9659f0d2583f50fa9af859ea84` examined
[request encoding][legacy-top-up-request], its [schema/principal regressions][legacy-top-up-tests],
[attempt admission][legacy-top-up-attempt] and [installed transport cases][legacy-top-up-transport].
Retained lessons are explicit account/target binding, intent before exposure,
independent transport/provider outcomes and no retry after uncertainty. The new
probe independently implements those rules for one zero-attachment operation;
the legacy multi-operation journal, positive offers, refund accounting and
substitute-service graph have no consumer here and are not imported.

The [offline example](../../qualification/ic-blob-storage-enrollment-probe/examples/enrollment/main.rs)
uses the actual Wire codec and probe types to prepare initialization, inspection,
attempt and upgrade frames, or decode a retained inspection/attempt response.
It has no signer, network or identity-file access. Three native cases qualify frame
direction/method checks, CLI bounds and classification of the actual provider refusal;
`make test-enrollment` includes them. Wire decoding leaves provider bytes intact.
An explicit `decode-top-up` command classifies the selected Cashier Candid result
separately; an `Ok` tag does not interpret its payload or establish credit.
The [changelog quickstart](../../CHANGELOG.md#01008---unreleased)
shows preparation only; generated attempt bytes are not permission to submit.

### Mainnet probe installation witness — 2026-11-15

The existing owner `mjjg7-wqaaa-aaaai-ayghq-cai` now runs the locally qualified
329,459-byte Rust probe. One acknowledged `install_code` request used **install**
mode after verifying an empty module and the sole `canic-mainnet` controller.
Management status returned module SHA-256
`4fc165878bd4d4ab6334d93ec56114df64309ca79cb9088e2c44a7bafee4e0fe`, matching the
exact retained build artifact, and **498,369,997,713 cycles**. Installation used
the existing owner balance; no additional ledger debit or provider deposit occurred.
The observed balance difference also includes execution/time and is not an itemized fee.

An operator-signed replicated `enrollment_inspect` returned that same owner,
the existing operator, Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`, target balance
**1,000,000,000**, exact retained request bytes, and **no attempt or outcome**.
This witnesses independent Rust installation and durable configuration, not
Caffeine enrollment, owner credit or gateway admission. The target is a request,
not a provider-enforced maximum charge. No payer link or provider call was sent.

Exact source/lock/snapshot hashes, module, intent, signed requests, exclusive
dispatch records and readbacks remain in private Git-ignored
`.operations/caffeine-install-01008/`; public identities and hashes are in the
[compact observation](caffeine-observation.json). Do not reinstall or recreate
this owner. The signer key stayed in the existing named identity. The
[CLI 1.6.0 signing path](https://github.com/dfinity/icp-cli/blob/v1.6.0/crates/icp-cli/src/commands/canister/call.rs)
defaults management-call routing to `aaaaa-aa`; before dispatch, the retained
copy's explicit effective destination was corrected to the signed owner argument
using its [signed-message routing format](https://github.com/dfinity/icp-cli/blob/v1.6.0/crates/icp-app/src/signed_message.rs).
The signed request remained unchanged; the routed copy was dry-run validated.

Further source search found only the public example's no-expiry linking command
and a moving `icfs/latest` download. Subsequent static inspection below recovered
embedded contract documentation; the executable was not adopted. The legacy
short-expiry witness above remains inconclusive. Installation alone does not
establish provider enrollment or limit enforcement.

### Published CLI contract findings and bounded link experiment — 2026-11-15

Static inspection of the [published Linux CLI artifact][icfs-artifact] found
embedded Candid type documentation for `PaymentAccountCanisterAddRequest`,
`PaymentRelationship` and `AccountSettings`. The retained 21,194,256-byte artifact
has SHA-256 `0385e9211130af0d0dd4b341ef9947d973754cd5db00bf751007ce9f7e90ea6e`.
It was read as data, never executed or installed. Its URL is moving: the hash and
exact statement byte ranges in `.operations/caffeine-link-01008/artifact-review.json`
identify this evidence, not any later download. These are published type/CLI
claims, not audited deployed Cashier code or live enforcement witnesses.

| Field | Artifact's documented meaning | Qualification consequence |
| --- | --- | --- |
| Relationship expiry | Absolute Unix nanoseconds; inactive at or after that instant | Select the unit explicitly; visibility of a historical row does not show spending permission |
| Daily limit | Positive maximum transfer from payer to paid account, resetting at UTC midnight | It limits transfers according to documentation, not all storage charges or liability |
| Null account target | Uses the provider default; CLI identifies that default as 1T | Correct the earlier disabled-refill interpretation; avoid relying on null as an off switch |
| Removed relationship | Retains last payer/removal time for a reassignment cooldown | Removal is not object deletion, refunded credit or charge settlement |

Selected bounded experiment: link only the existing payer and installed owner,
with a **1B** daily allowance and expiry **ten minutes after preparation**, without
crossing UTC midnight. Inspect exact terms, call the probe's existing single-use
**1B target** with **zero attached cycles**, retain transport/provider observations,
then remove that exact relationship. No extension, replacement, retry, extra
funding, certificate or object is part of the experiment. Refusal or uncertainty
does not authorize repeating a mutation. Cleanup follows a matching relationship
read; an unknown add outcome first requires reconciliation, not a second add.

The existing 9.9B payer credit is the resource at risk; no additional source-ledger
debit is planned. The daily allowance and expiry are provider claims being tested,
not independently proved liability caps. The probe has no upload/certificate path,
timer or second dispatch. Preserve all results and any allocation even after
relationship removal; no closure follows. This experiment can witness enrollment
and accounting, but does not by itself prove expiry or daily-limit enforcement.

[icfs-artifact]: https://caffeinelabs.github.io/object-storage/artifacts/icfs/latest/icfs-linux-x86_64

### Gateway preflight and next local host gate — 2026-11-15

One HTTPS GET used the SDK's direct-read URL shape, this checkout's owner, an
explicit qualification project selector and a deterministic never-issued root.
It returned **HTTP 404**, a 92-byte missing-tree response, verified TLS and no
redirect. The request allowed at most 64 KiB/15 seconds, with no retries,
credentials, certificate or object write. Cashier separately reported no stored
callback API version for this owner. Exact request/results remain privately in
`.operations/caffeine-gateway-01008/`; [public evidence](caffeine-observation.json)
omits the unused session cookie. No request was repeated.

This witnesses route reachability and a negative lookup only. It does not show
which account checks ran, positive admission, project isolation, deletion or
charge cutoff. The advertised `X-Dry-Run` header remains unqualified and unused;
legacy's [transport review][legacy-transport] reached the same limitation.

Re-review of the pinned Rust example, Motoko mixin and legacy GC contract leaves
two incompatible deletion-list candidates: `vec text` and `vec blob`. Both use
the `_immutableObjectStorage*` names; a V2 marker alone cannot select the return
type. No live callback was observed here. Do not add automatic type fallback or
advertise a callback version to manufacture compatibility. The Rust example also
returns false for unknown keys and removes records on confirmation without first
requiring Closing; those policies must not be carried into this library.

The next local qualification must compose the **existing durable Service** with
one explicit candidate provider adapter, preserving these cases:

| Case | Required local result |
| --- | --- |
| Unknown/malformed key; reserved, retired or live root | Unknown keys are protected; malformed input refuses before mutation; no absent-record deletion permission |
| Two uploads share one root | One retention owner; releasing one reference preserves another live use |
| Final release and certificate ordering | Closing is irreversible; registration/exposure commits before certified reply; no later issuance or resurrection |
| Deletion-list profile | One selected Candid shape, with real nonempty fixtures; an empty response is not compatibility evidence |
| Confirmation from wrong sender or for live/unknown root | Refuse before mutation; controllers/operators cannot impersonate the configured gateway |
| Authenticated matching Closing confirmation | Retain an acknowledgment only; never erase history, free exposure or claim settlement |
| Upgrade with prior probe receipt | Keep the consumed attempt and original raw refusal; no reinstall, reset or repeated provider operation |

The current enrollment probe is not this host: it has no Service, certificate
endpoint or retention callbacks. Its Storage directory was created with **one
allocation and one extent**, occupied by `enrollment.control.v1`. Storage 0.74.1
intersects new runtime limits with persisted directory capacities; raising a
configuration number does not create another slot. Select and qualify the
composition/lifecycle path locally before reusing this owner for a service.
Do not overwrite the probe record, add a second root-memory custodian or rebuild
Blob's reference/accounting model inside the probe to evade that constraint.
No new canister, migration, callback or certificate-producing deployment occurred
in this preflight.

[legacy-transport]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-10-02-gateway-stream-01/summary.json

### Link, allocation and owner-call witness — 2026-11-15

The selected experiment completed with one link, one probe attempt and one removal.
Before linking, the fresh owner had no relationship and the payer held 9.9B cycles.
The acknowledged link returned the exact payer/owner, 1B daily allowance and expiry
`1794745153395000000` (12:19:13.395 UTC). It **immediately allocated 1B**, before
the probe call: the owner balance was 1B and reported period spend was already 1B.
Payer audit sequence **1416836**, event `RelationshipAdd`, attributes that transfer
to linking. The period start matched midnight UTC; this is not a rollover test.

The installed Rust owner then made its one exact `account_top_up_v1` call, requesting
a 1B target with zero attached cycles. Transport returned 105 bytes; the provider
result was **`Err(TopUpWithoutCycles)`**. The existing allocation must not be
credited to this refused call. A separate replicated probe inspection returned the
same durable attempt and raw reply. The retained bytes are now an offline decoder
fixture covering refusal, truncation, trailing bytes, wrong shape and size limits.
The probe remains consumed; no repeat, reset or positive-cycle fallback is authorized
by that failure. Legacy's explicit positive attachment remains a distinct protocol
path, not a reason to import its funding engine or change this experiment.

Cashier reported **166,666,666** available credit for the actual listed storage
gateway, with all usage counters zero. This is a Cashier budget observation, not
an HTTP gateway acceptance test or proof of the credit-allocation formula.
The explicit removal returned `Ok` and relationship readback became absent.
Balances afterward remained **8.9B payer / 1B owner**, with the same reported
gateway budget. Removal neither refunded that allocation nor established closure.

No additional source funding, certificate, object, expiry extension or repeated
provider attempt occurred. Preserve `.operations/caffeine-link-01008/`, including
signed requests, dispatch claims, static artifact, provider audit, raw reply and
cleanup readback. [The compact observation](caffeine-observation.json) records
hashes and exact identities. Next investigate gateway admission and the selected
retention/authority boundary using this existing owner/credit; do not treat the
zero-attachment top-up as an enrollment recipe. D0 remains open, while D2 now has
a real payer-to-owner allocation witness and a real owner-origin refusal witness.

[legacy-top-up-request]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/caffeine/funding/request/mod.rs
[legacy-top-up-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/caffeine/funding/request/tests/mod.rs
[legacy-top-up-attempt]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/workflow/funding/attempt/mod.rs
[legacy-top-up-transport]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/storage_funding/transport/mod.rs

### Independent-access follow-up for 0.100.2

Historical search result, superseded for the public Rust enrollment recipe by
[the November 15 independent qualification review](#independent-qualification--2026-11-15).

The November 14/15 search found unchanged Caffeine skills at the revision above,
package 1.1.2, issue #16 open with zero comments, and an export guide that requires
replacement of managed storage for independently deployed exports. That guide's
scope does not establish the terms of lower-level Cashier integration. No account
or provider calls were made in that earlier search.

Rabbithole source at `baa4d86314822711735cc9860220c109210b1ce9` supplies an
independent Motoko self-funded account candidate: [CashierAccount.mo][rabbit-account]
attaches cycles for its own canister principal, [the storage canister][rabbit-canister]
uses Caffeine's mixin, and [Cashier.mo][rabbit-cashier] declares consumer-side balance,
pricing and relationship types. Its automatic funding/delegation machinery was
not adopted. Source alone establishes neither enforcement nor charge closure.

The provider's [managed storage setup guidance][storage-guide] and Rabbithole
declare different top-up argument/result shapes. The later recovered historical
Rust ABI and mainnet metadata now give a concrete comparison; do not select types
by copying a convenient snippet or treating dropped response fields as evidence
of success. The earlier conclusion that no independent Rust recipe had been found
was a search limit, not evidence of impossibility or a continuing outreach gate.

### Canic legacy protocol evidence — 2026-11-15

This source-only investigation preceded the public Cashier reads above; its
no-call statements describe that investigation, not the entire current review.
The maintainer requested this read-only investigation. Canic legacy HEAD
`377fbceb3994c9d1f3033979ef4e4ce9e6b85801` retains the adapter, qualification and
demo acceptance notes, but its old protocol inventories were removed. Path history
locates their last pre-removal parent at
`d9d70322ade2ba6fbd17520acf93acad66dcf4aa`. The [Cashier inventory][canic-cashier]
and [gateway inventory][canic-gateway] cite Toko
`9ca150b396a2bde42f2b8977a04a7ca2c6172b56`; the original
[Rust storage implementation][toko-storage] and [account caller][toko-account]
were inspected directly, not inferred from the inventory's `Complete` label.
That label meant accepted consumer-side evidence for Canic 0.69/0.70, not a
provider implementation audit or qualification of this rewrite.

| Recovered fact | Consequence for this rewrite | Evidence limit |
| --- | --- | --- |
| Historical Cashier principal is `72ch2-fiaaa-aaaar-qbsvq-cai` | Concrete service candidate for later identity/ABI verification; do not install it as an implicit default | No call was made to verify its current code, controller or interface |
| Project canister funds its own principal's account | A separate payer relationship is not intrinsic to the historical self-funded path | The source comment says self-relationships are forbidden; this is a consumer assertion, not fresh provider enforcement evidence |
| `account_top_up_v1` takes **`opt record { target_balance : opt nat; account : opt principal }`**, with attached cycles | Preserve the outer option; the Rust caller sends `Some({ None, Some(self) })` | An exact historical ABI candidate, not proof that another provider revision accepts it |
| `account_balance_get_v1` takes `record { account : principal }`; balances contain signed Candid integers | Preserve typed errors and signed values at the external boundary; apply local range checks explicitly | A returned balance is not uniquely attributable payment evidence or a spending cap |
| `storage_gateway_principal_list_v1 : () -> (vec principal)` is called as an update | Gateway membership can be obtained from a configured service rather than guessed from client identities | Membership does not establish object deletion scope or what a confirmation proves |
| Certificate result is `{ method = "upload"; blob_hash }` after local registration | Confirms the Rust certified-reply path already identified in the SDK/Blob legacy review | No expiry, length enforcement or charge-closure rule is established |

There is an important historical ABI difference: the old Toko/Canic inventory's
`_immutableObjectStorageBlobsToDelete` returns **`vec text`**, while Canic legacy's
later [installed GC test][canic-gc-tests] decodes **`vec blob`** and exercises the
newer shared Blob profile. Both liveness and confirmation use binary roots in the
old inventory, but that alone does not make the list responses interchangeable.
Select and qualify one provider revision; do not introduce a fallback decoder or
copy the older `.did` wholesale. The later test's controlled gateway identity is
not evidence of a production callback principal.

Retain the later test's useful invariants: unknown roots remain protected,
confirmation of a live/nonpending root refuses, malformed mixed batches preserve
the whole pending set, and duplicate acknowledgment never removes outstanding
byte/liability accounting. The original Toko implementation instead allowed a
gateway confirmation to remove a stored live row and returned false for unknown
roots. Its hub status path could also trigger funding. Those policies conflict
with this rewrite and must not be copied. No funding timer, account ledger,
database layout, recovery record or deployment machinery is imported.

The [later funding case][canic-funding-tests] explicitly installs a Testkit reply
canister with bounded cycle acceptance. It proves wrapper/accounting behavior
under controlled replies, not live Cashier enrollment. Canic's
[demo acceptance record][canic-acceptance] still requires selected provider,
payer and gateway inputs and real upload/download/deletion/billing evidence.
[Legacy issue #494][canic-funding-issue] also distinguishes acknowledged payments
from independent credit; its later `not_planned` closure is not qualification.

**Disposition:** the historical Rust self-funded call path and ABI are now sourced;
we should not keep describing those as undiscovered or require a new receipt API
before ordinary acknowledged funding can work. The narrower next check is fresh
compatibility/enrollment for that explicit account model and selected service,
followed by the existing object-scope, proof-lifetime and closure gates. A service
principal is not a fresh customer account or an authorized spend limit. This
review selects no live target, performs no provider call and closes no D0 guarantee.

### Prepared provider clarification — not sent

Local clarification draft referencing [Caffeine issue #16][gc-issue]. The maintainer
declined posting it and subsequently authorized Caffeine calls when ready.
Provider-call approval does not authorize sending this message: retain these
questions as design inputs, not a queued outreach task. The approval removes the
earlier local-only action restriction; it supplies no missing provider guarantee.

> We are narrowing the `ic-blob-storage` rewrite to upload authorization and explicit
> application references, usable from an independently deployed Rust canister.
> Before implementing the provider boundary, could you point us to the supported
> contract/revision for this arrangement?
>
> 1. Is independent Rust access supported using
>    `_immutableObjectStorageCreateCertificate(text)` and the IC-certified
>    `{ method = "upload"; blob_hash }` reply? What is the supported Cashier
>    enrollment and owner/project/bucket binding procedure? We would provision
>    funding explicitly, without automatic refill or Motoko weak-reference tracking.
> 2. What limits the lifetime of that proof and of an accepted, unfinished tree?
>    Can the original recovered proof or delayed chunks recreate storage after
>    final release/deletion? We need the existing expiry/revocation rules, including
>    in-progress writes, rather than a new receipt service.
> 3. What exactly identifies the object whose retention our callbacks control,
>    and which length/tree/chunk bounds does the gateway enforce? What establishes
>    the end of storage and charge exposure? The callback questions above remain
>    relevant, particularly the meaning of deletion confirmation.
>
> The public SDK documents the client requests, and independent Motoko consumers
> provide useful integration examples, but neither establishes these server-side
> guarantees for a Rust host. Documentation or versioned protocol references are
> sufficient; we are not requesting deployment, funding or a provider experiment.

## Candidate protocol: what actually crosses the boundary

The client calls the Rust host's external Candid method
`_immutableObjectStorageCreateCertificate(text)` with the provider root. Its reply
is a record containing `method = "upload"` and `blob_hash`. The IC certifies that
ingress result; the client extracts the v4 response certificate and supplies it
to Caffeine. The plain record is not itself a signed certificate.
This path needs **no host-to-provider certificate acquisition call**. The legacy
[issuer][issuer] synchronously commits registration and possible exposure before
returning that record. Its [boundary tests][issuer-tests] exercise actual-uploader
checks, expiry/revocation, first-reference creation and repeat-issuance refusal.
The [release/issuance ordering rule](ic-blob-storage-design.md#object-transitions-and-closure)
therefore has no pending acquisition callback: cancellation first prevents issuance;
issuance first records exposure that survives late delivery of the certified result.
The local regression exercises both registration/cleanup orders, not IC delivery.

| Boundary | Observed shape | Consequence for the rewrite |
| --- | --- | --- |
| Host certificate request | Root only; no Blob upload ID | Root is a locator. Resolve exactly one eligible grant using the captured caller and immutable installation binding. Reject ambiguity; never select an arbitrary upload |
| Certified application reply | Operation and root; no explicit length, application expiry, reference ID or project/bucket | Local checks on those fields are not proof the provider enforces them |
| SDK tree PUT | `/v1/blob-tree/`; tree, bucket, declared byte count, owner canister, project, headers and certificate under `OwnerEgressSignature` | Establish which fields the gateway binds to authority and which are client assertions |
| SDK chunk PUT | `/v1/chunk/`; raw bytes with owner/root/chunk/index/bucket/project selectors; no certificate in this request | Qualify how prior tree authorization bounds later chunks and how long those chunks remain admissible |
| SDK direct read URL | Owner, project and root selectors | A URL shape is not proof of deletion scope, bucket isolation or incarnation identity |

Do not add our upload IDs or expiry fields to the provider reply and assume the
gateway enforces them. Do not infer uploader-bound usage from local acquisition
authorization: the inspected transfer sends the certificate, not a separate
uploader proof. Treat it as potentially bearer authority until the provider
contract establishes otherwise.

Eligibility must cover historical ambiguity as well as simultaneous grants: a
delayed root-only request must not consume a later grant for the same uploader/root.
The [application boundary](application-boundary.md#provider-facing-identity-gate)
selects refusal of another upload lifetime for every admitted root, including
cancelled/expired unexposed grants. The local model retains the original root claim;
an actual host must preserve that rule when mapping the provider request. This
local restriction does not qualify provider scope, proof recovery or closure.

The root commits to a chunk tree and metadata, not merely the raw file digest.
The SDK constructs 1 MiB chunks and includes a content-length metadata header;
its tree request also carries a separate byte count. These are useful candidate
checks, not evidence that malicious traffic cannot store more than admitted.

## Contract answers still required

| Contract | Available evidence | Required answer before selecting the profile |
| --- | --- | --- |
| Independent enrollment and account | Published Rust recipe, current ABI and this checkout's acknowledged fresh payer funding | Qualify one fresh owner binding, callback profile, owner/gateway credit and actual charge attribution; payer funding alone is not Rust-host enrollment |
| One provider-object owner | SDK uses owner/project/root plus bucket on writes; callbacks provide binary keys | What exact tuple identifies deletion scope? Are keys tree roots, chunks or both? Can another installation affect retention? Require exclusive scope or an independent retention handle; a local namespace number is not proof |
| Exact bounded effect | Metadata/tree commitments and client byte count | Does the gateway reject inconsistent length/header/tree/chunk sizes and extra or substituted chunks? Which limits also bound partial/duplicate uploads? Until qualified, counters are declared usage, not a proven physical bound |
| Authority recovery | Original request identity can be retained and its IC result queried | Will the gateway accept the recovered proof, within what window, and can the SDK resume with it without another certificate call or unsafe PUT replay? Absence of a result is not proof of no effect |
| Lifetime and final release | Host can refuse new issuance; no revocation/expiry field in its reply | What invalidates an issued certificate and an accepted unfinished tree? Can either recreate storage or charges after release/deletion? Include delayed chunks and recovered certificates, not just initial PUT |
| Deletion and settlement | Three callbacks exist; helper prunes local runtime bookkeeping | What is authenticated confirmation evidence of? When are origin bytes gone and all attributable charges closed, including late writes? |
| Charging and caps | Historical credit was spendable; retail guide describes upload/storage/download charging | What are this account's actual terms and enforceable limits? Balance exhaustion, relationship expiry, zero usage counters and local release do not prove cutoff |

The [retail billing guide][billing] says existing files continue to be served and
storage charges accrue after credits run out. It is a warning against assuming
prefunding bounds liability, not a specification of the trial's cycles account.

If closure cannot be established, final release remains **Closing**, retains its
outstanding exposure and consumes the relevant finite capacity. No timeout,
operator assertion or locally invented receipt may convert it to Settled.
If supported independent access is ruled out, return that product choice to the
maintainer; successful substitutes cannot resolve it.

### Field enforcement required before registration

The local manifest and read checks now make the remaining boundary concrete.
This matrix describes the reviewed SDK protocol, not a new wire schema. The
future host must authorize one immutable declaration before returning the
certified method/root reply. That reply alone does not explicitly bind every
field in the transfer request.

| Input | Blob's local responsibility | Provider guarantee still needed |
| --- | --- | --- |
| Caller, upload ID, first reference and grant deadline | Capture the actual caller; resolve one eligible grant; consume it and record possible exposure atomically before reply | Whether the resulting proof is bearer authority; validity after local expiry, revocation or final release |
| Root and ordered leaf hashes | Check bounded count and tree consistency with original metadata | Tree/root and chunk/hash/index validation, including rejection of extra leaves and oversized full/final chunks |
| Declared bytes | Check local byte limit and implied leaf count; compare the immutable declaration on retries | Reconcile `num_blob_bytes`, actual accepted chunk sizes and total accepted bytes; hashes alone do not authenticate length |
| Original metadata | Bound and normalize for the root; the future admission path must check any required Content-Length value against its declaration | Bind the accepted headers to the root and reject conflicting size assertions; the current preflight deliberately does not interpret HTTP headers |
| Owner, project and bucket | Use the immutable selected installation binding | Define which fields are bound to the proof and the actual deletion/charge scope; a client-supplied selector is not an isolated namespace |
| Callback binary key and authenticated sender | Resolve the qualified key to the existing owner; preserve unknown/mismatched obligations | Define key encoding, root-versus-chunk meaning, sender discovery and whether confirmation acknowledges permission, deletion or another event |

Host validation and provider enforcement are separate obligations. A successful
read against a trusted declaration establishes integrity of that read, not an
upper bound on what a malicious upload request could store or charge. Until the
provider bound is established, any byte accounting remains **declared usage**.

### Qualification sequence and stopping points

D0 selects evidence-backed rules and explicit limitations; D1 implements them
and checks the installed host with labelled substitutes. Bounded D2 experiments
against a fresh installation may refine those rules without closing unknown
guarantees. The maintainer has authorized Caffeine calls when ready and local
installed-host testing before D0 closes. The current local host keeps provider
effects disabled. D0 does not require a live test of an unimplemented host. A
successful D2 sample cannot supply missing general guarantees about old proofs,
namespace isolation or future charges. A protocol change found during D2 returns
to design review before the affected API/persistence model is accepted.

The next useful evidence is fresh enrollment under the published Rust recipe and
independent qualification of the matrix above. No provider response is required
to conduct that work. The October [legacy discovery][gc-discovery] and
[contract follow-up][gc-contract-followup] already looked for gateway semantics;
they found helper/runtime behavior, not a server contract. The November deletion
follow-up still did not establish closure. These are reasons to keep D0 open,
not evidence that no such arrangement can exist. Fresh public interface queries
are permitted; never resume legacy objects through the rewrite.

The public [troubleshooting guide][troubleshooting], read on 2026-11-14, describes
background removal after references disappear and client chunk retries. It does
not specify the Rust callback key mapping, certificate cutoff or in-progress
upload closure. Its suggested wait is not a protocol expiry or settlement rule.

Use the following cases to prepare D1/D2 qualification and refine D0's supported
limits. An unknown guarantee remains unknown; it must not become a local promise.
These are planned witnesses, not runnable tests or passing results. Start with
the existing deterministic 5 MiB + 17 byte fixture and a small file; those are
test inputs, not selected production size limits.

| Witness | Required observation |
| --- | --- |
| Valid control and namespace binding | Fresh supported Rust-host account/binding; two authenticated uploaders; direct SDK transfer and read checked against the original declaration. A second reference shares one owner; first release cannot permit deletion |
| Inconsistent size assertions | Separately vary tree byte count, root-bound Content-Length and full/final chunk lengths; include a root-consistent declaration whose leaf bytes have a different length. Observe the specified rejection boundary and disposition of partial effects, not just an HTTP error |
| Invalid tree or chunk | Substitute/reorder a leaf, append an extra leaf or use an incorrect index; retain the original control result and exact request identity. The gateway must enforce the selected object bound even when the SDK is bypassed |
| Binding substitution | Within separately owned test scopes, vary owner/project/bucket while holding the proof fixed. Outcomes must match the documented scope, without granting an independent deletion owner for shared storage |
| Lost authority reply | Recover the original IC result by request identity and show usable provider acceptance under the selected lifetime rule, or report blocked. Inspection of a local registration is insufficient; do not obtain a fresh certificate to make this case pass |
| Final release and escaped authority | Release while proof delivery or a chunk is delayed; then exercise the permitted expiry/revocation boundary. New local issuance stays refused; any late accepted effect stays recorded. No deletion acknowledgment alone releases exposure |
| Exact closure | Correlate authenticated messages to the exact object and show the selected authority cutoff, origin deletion and separate charge cutoff. Empty/aggregate callbacks, balance changes, HTTP 200/404 or elapsed time cannot substitute for that evidence |

Each eventual live plan must name its fresh account/scopes, exact objects and
request limits, funding/charge terms, permitted replay, observation deadline and
custodian for unresolved obligations. A deadline stops observation; it does not
settle an object. There is no live transfer runner or installed live Blob service.
The separate `qualification/` workspace exercises the real standalone host and
the single-use enrollment probe above; keep pure checks beside their owner.
The probe's one intent/result record exists to prevent duplicate enrollment
effects, not to introduce a parallel object ledger or a general funding framework.

## Recovery and SDK disposition

The reviewed [public declarations][sdk-api] expose `putFile` and `getDirectURL`.
They do not expose prepare-before-admission, existing-certificate resume, or
per-client transport/retry controls. The source automatically retries tree and
chunk writes and uses up to ten upload workers. `putFile` returns a root after
workers finish; it does not require a provider-signed completion receipt, and its
worker ignores the chunk result's `isComplete` value. A returned root or 100%
progress must not become Blob's availability flag.

The [legacy patch description][patch] documents preparation of a retained file
handle and per-client transport, retry and concurrency controls. Those supplied
real missing capabilities, not incidental convenience. Its one-use prepared
handle is ephemeral and is not, by itself, a resumable-upload solution. Do not
import this patch or duplicate the SDK's transfer implementation. Select a
supported external package path supplying the required capabilities, or obtain
documented idempotency and adapt the client requirements explicitly. A new SDK
selection requires current registry and compatibility review.

The [legacy certificate client][certificate-client] retains the signed ingress
envelope/request ID before dispatch and can query the original IC request status.
It verifies the certificate, original request and expected reply before recording
the outcome. That is a useful recovery approach to evaluate through Platform.
It does not prove that a later certificate is accepted by Caffeine or that a
transfer may be restarted. Fresh issuance is a distinct authority exposure.
Registration inspection alone never recovers usable proof.

Closing/revocation can stop the cooperating client from requesting or forwarding
new authority. They cannot retract a certificate already held by that client,
nor prevent it from recovering an available original IC result. Provider lifetime
semantics must carry that part of the guarantee. Recording a delayed authentic
result preserves exposure without reopening the object.

## Retention callbacks and selective legacy reuse

The [provider mixin][mixin] exposes a public positional liveness query, a
gateway-authorized deletion list, and a gateway-authorized confirmation update.
The [helper][storage] obtains gateway membership from the configured Cashier's
`storage_gateway_list_v1`; it also contains automatic refill behavior, which is
outside the rewrite's scope. Gateway membership is not equivalent to operator or
controller status.

The legacy [callback profile][gc] protects unknown roots, keeps reservations/live
references protected, bounds batches, validates a whole acknowledgment batch
before mutation and retains acknowledged lifetimes. Its acknowledgment is
deliberately separate from physical deletion and billing settlement. Preserve
these distinctions; qualify the conservative unknown-root response against the
real protocol rather than copying Motoko's weak-reference absence semantics.

| Legacy component reviewed | Reuse decision and reason |
| --- | --- |
| Issuer and its boundary regressions | Retain commit-before-reply, actual-uploader authorization, current-state recheck and no second issuance on uncertain exposure. Adapt to one provider-object owner rather than importing the old installed store graph |
| [Manifest model][manifest] and [manifest regressions][manifest-tests] | Adapt the bounded root/metadata consistency check into local preflight below. Keep actual-byte verification in the streaming owner. Its own test accepts a consistent tree with a false declared length, then rejects actual bytes: admission consistency is not byte-length authentication |
| Certificate transport | Retain original-request recovery and authenticated-result matching requirements. Reuse Platform capabilities and only the client state required by the selected protocol; do not import the whole browser/native journal system |
| Callback profile | Retain authentication, conservative unknown handling and acknowledgment/deletion/settlement distinctions. Requalify keys and closure before exposing deletion permission |
| SDK patch, funding and restoration machinery | Do not carry over. SDK capabilities need an external dependency solution; automatic funding is excluded; the rewrite selects a narrower management policy. Old installations keep their original recovery tools |

The initial review carried over no code. The subsequent local integrity checks below
selectively adapt first-party hashing and manifest checks; they adopt no legacy
service state or recovery records.

## Implemented local read verification

In development version 0.100.1, [RootVerifier](../../crates/ic-blob-storage/src/caffeine/mod.rs)
implements the reviewed SDK 1.1.2 tree computation for independently verifying a
received stream. The npm registry was checked on 2026-11-14 and still identifies
1.1.2 as latest; no new SDK preparation/resume capability was found in that API.
This selects a local integrity algorithm, not an account/object identity, upload
protocol guarantee or persistent representation. D0 remains open.

Adapted first-party sources at the legacy revision above are the
[streaming hasher][legacy-hasher], [metadata normalizer][legacy-metadata] and
[root verifier][legacy-verifier]. Retain the fixed-size chunk frontier, odd-tree
padding, original metadata, exact offsets and pre-mutation checks. Combine the
hasher/verifier into one owner. Omit raw-digest computation, leaf observers,
manifest builders, out-of-order/checkpoint modes and transport/file handling:
the current consumer need is one sequential comparison to an already trusted root.
Unlike the old whole-content SHA-256 computation, only individual bounded chunks
are hashed, so declared file length is bounded by the configured u64 limit.
No whole-file allocation follows from that declaration.

The [eight streaming tests](../../crates/ic-blob-storage/src/caffeine/tests.rs) adapt
legacy rejection/integrity scenarios and use selected frozen
[SDK algorithm vectors][legacy-vectors], including multi-chunk, odd-tree,
repeated-leaf and ECMAScript metadata cases. Expected roots are retained external
observations rather than generated by the implementation under test. The vector
record identifies the observed source hash and input recipe; its unusual Unicode
metadata cases test hashing, not HTTP validity. No live-provider observation or
fresh SDK execution is claimed by running these Rust tests.

`sha2` 0.11.1 is the direct dependency for integrity checks, inherited from the root with
default features disabled. The crates.io index identifies it as the latest stable
non-yanked release at selection time. Existing approved SHA-256 owns the provider
commitments; no cryptographic primitive is implemented here. This owner-local
slice needs no installed-host `qualification/` package. Its public API remains
experimental. Browser upload preparation and transfer still belong to the SDK.
The later local control model separately uses Platform Types for caller identities.

## Implemented local manifest preflight

The 0.100.1 checkpoint adds [check_manifest](../../crates/ic-blob-storage/src/caffeine/manifest.rs)
for bounded checking of a proposed declaration before any future authorization.
It adapts the length/count/metadata/root checks from the first-party
[manifest constructor][manifest] at the pinned legacy revision. The caller supplies
root, declared length, ordered leaf hashes and original metadata. Limits reject
excess length/count before hashing; the existing metadata and tree functions own
both streaming and manifest rules. The check borrows the leaves and returns only
success/failure, retaining no manifest, receipt or authority.

This is a local consistency preflight, not an SDK wire decoder or a final admission
decision. Decoders must bound inputs before allocating them. Unlike the legacy
manifest object, this slice needs no owned chunk vector, range API, per-leaf read
verification, builder, checkpoint, serialization or extra dependency. Those had
different consumers; the rewrite currently needs only preflight and sequential
read integrity. The experimental Wire host now calls this preflight through the
library service; the production provider/client boundary remains unimplemented.

Four additional owner-local tests adapt [legacy failure cases][manifest-tests]
and check frozen SDK leaf/root observations. They cover full/partial and uneven
trees, reordered/corrupted/replaced leaves, metadata substitution, budget/count
refusal and a consistent manifest with a false length. An `abc` leaf also matches
declarations of one, four or 1 MiB of content: its hash does not authenticate any
of those lengths. Checking actual bytes rejects the false declaration. Success
therefore cannot justify a conservative exposure bound or issuance by itself;
provider byte enforcement remains a D0 gate. No live provider claim follows.

## Next implementation boundary and acceptance

The maintainer-approved [experimental local model](ic-blob-storage-design.md#experimental-local-service-model)
rehearses the synchronous transition below with declared-byte accounting. Its
[installed profile](ic-blob-storage-design.md#experimental-installed-profile-in-01002)
now adds bounded persistence and a local host, without certificates or settlement.
Root matching remains provisional, not the provider deletion key. After the
contract selects identity and effect bounds, qualify the effect-capable transition:
application authorization/reservation → bound-uploader issuance → one object
owner plus known first reference and possible exposure → irreversible Closing.
For the observed protocol, registration happens **inside issuance before reply**,
not after acquiring a provider certificate. An additional use of the same object
joins its reference owner; another upload ID cannot independently gain deletion
rights or bypass uncertain authority. The root-only endpoint must have an explicit
unambiguous grant-resolution rule before it is mounted.

Use the existing reference component in that production owner. Do not add a
generic provider trait, fake certificate acquisition service or second state
implementation to make a substitute test pass. Keep the present API experimental
and defer provider identity/persistence types until the questions above close.

| Qualification boundary | Smallest useful witness |
| --- | --- |
| Local registration | Wrong caller, expired/revoked grant and conflicting declaration leave state unchanged; registration/reference/accounting/exposure commit together; overlapping upload identities cannot delete another live use |
| Installed standalone endpoint | Two signed callers exercise the actual method and verified IC reply; repeated issuance refuses; a lost reply is inspected by its original identity; real stable reopening preserves it |
| Client/SDK recovery | Recover an original usable proof without new issuance, or return explicitly blocked; interrupt tree/chunk replies and demonstrate only provider-qualified replay; reject corrupt, truncated and wrong-length reads against the original declaration |
| Provider contract | Fresh supported enrollment, immutable/exclusive scope, malicious declaration rejection, final release racing old authority/delayed chunks, and attributable deletion/charge evidence |

`make test-signed` now covers signed application/uploader calls to the local host,
authenticated responses, original-upload inspection after discarding a registration
reply and refusal of repeated registration. It exercises the existing experimental
transition only: it neither issues nor recovers provider authority. The installed
standalone row therefore remains partial, alongside the existing upgrade witness.

Local and installed tests may use controlled gateway responses, clearly labelled
as substitutes; those cannot establish the last row. A live trial needs separate
effect authorization and a concrete bounded plan after the supported arrangement
is known. No live trial or provider message is initiated by this review.

[trial]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-06-trial-v073-two-users-01/summary.json
[funding]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-06-trial-v073-provider-01/summary.json
[gc-readback]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-11-05-trial-v069-gc-readback-01/summary.json
[gc-discovery]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/local/2026-10-22-gc-contract-discovery-01/summary.md
[gc-contract-followup]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/local/2026-10-22-gc-credit-contract-02/summary.md
[dfinity-example]: https://github.com/dfinity/immutable-object-storage-example/blob/ef29e8a6e8063c6fe654cac53a3497cab585fefa/README.md
[dfinity-setup]: https://github.com/dfinity/immutable-object-storage-example/blob/ef29e8a6e8063c6fe654cac53a3497cab585fefa/scripts/setup.sh
[cycle-costs]: https://docs.internetcomputer.org/references/cycle-costs/
[cli-mint]: https://github.com/dfinity/icp-cli/blob/v1.6.0/crates/icp-app/src/operations/token/mint.rs
[funding-review]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-funding-review.json
[legacy-expiry]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-probes/deployed/2026-10-02-trial-link-02/summary.json
[dfinity-storage]: https://github.com/dfinity/immutable-object-storage-example/blob/ef29e8a6e8063c6fe654cac53a3497cab585fefa/rust-backend/src/storage.rs
[legacy-did]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-cashier.did
[legacy-observation]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/evidence/caffeine-deployment-observation.json
[sdk]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/frontend/src/StorageClient.ts
[sdk-api]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/frontend/dist/StorageClient.d.ts
[mixin]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/backend/src/Mixin.mo
[storage]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/packages/object-storage/backend/src/Storage.mo
[hosting]: https://help.caffeine.ai/hc/en-us/articles/46899843980692-GitHub-Integration-Overview
[billing]: https://help.caffeine.ai/hc/en-us/articles/49362898986644-File-Storage-Costs
[troubleshooting]: https://help.caffeine.ai/hc/en-us/articles/46899784111380-File-Storage-Troubleshooting
[gc-issue]: https://github.com/caffeinelabs/skills/issues/16
[storage-guide]: https://github.com/caffeinelabs/skills/blob/59b9b29a4fad3828677c8b80ead0e06fdb40987e/skills/extension-object-storage/SKILL.md
[rabbit-storage]: https://docs.rabbithole.app/how-it-works/storage/blob-storage
[rabbit-account]: https://github.com/rabbithole-app/v2/blob/baa4d86314822711735cc9860220c109210b1ce9/apps/backend/src/BlobStorage/CashierAccount.mo
[rabbit-canister]: https://github.com/rabbithole-app/v2/blob/baa4d86314822711735cc9860220c109210b1ce9/apps/backend/src/EncryptedStorageCanister.mo
[rabbit-cashier]: https://github.com/rabbithole-app/v2/blob/baa4d86314822711735cc9860220c109210b1ce9/apps/backend/src/Types/Cashier.mo
[issuer]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/certificate/mod.rs
[issuer-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage/src/ops/service/uploads/tests/certificate_boundary/mod.rs
[manifest]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/manifest/mod.rs
[manifest-tests]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/manifest/tests/mod.rs
[patch]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/patches/README.md
[certificate-client]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/clients/browser/certificate.js
[gc]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/docs/provider/gateway-gc-contract.md
[legacy-hasher]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/mod.rs
[legacy-metadata]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/metadata/mod.rs
[legacy-verifier]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/src/identity/caffeine/verification/mod.rs
[legacy-vectors]: https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/crates/ic-blob-storage-contracts/tests/fixtures/caffeine-hashing/vectors.json
[canic-cashier]: https://github.com/dragginzgame/canic-legacy/blob/d9d70322ade2ba6fbd17520acf93acad66dcf4aa/docs/contracts/BLOB_STORAGE_CASHIER_INVENTORY.md
[canic-gateway]: https://github.com/dragginzgame/canic-legacy/blob/d9d70322ade2ba6fbd17520acf93acad66dcf4aa/docs/contracts/BLOB_STORAGE_INVENTORY.md
[toko-storage]: https://github.com/dragginzgame/toko/blob/9ca150b396a2bde42f2b8977a04a7ca2c6172b56/fleets/toko/project/instance/src/ops/immutable_storage.rs
[toko-account]: https://github.com/dragginzgame/toko/blob/9ca150b396a2bde42f2b8977a04a7ca2c6172b56/fleets/toko/project/hub/src/ops/blob_storage.rs
[canic-gc-tests]: https://github.com/dragginzgame/canic-legacy/blob/377fbceb3994c9d1f3033979ef4e4ce9e6b85801/qualification/canic-tests/src/pic/blob_service/operator/provider/gc/mod.rs
[canic-funding-tests]: https://github.com/dragginzgame/canic-legacy/blob/377fbceb3994c9d1f3033979ef4e4ce9e6b85801/qualification/canic-tests/src/pic/blob_service/operator/funding/paid/mod.rs
[canic-acceptance]: https://github.com/dragginzgame/canic-legacy/blob/377fbceb3994c9d1f3033979ef4e4ce9e6b85801/docs/audits/working/blob-0411-adoption.md#current-acceptance--2026-11-11
[canic-funding-issue]: https://github.com/dragginzgame/canic-legacy/issues/494
