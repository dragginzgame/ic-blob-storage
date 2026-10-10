# Consumer funding qualification

This is a source review and acceptance recipe, not consumer execution or deployed
provider evidence. The [local handoff](status/current.md) owns implementation;
[funding credit](funding-credit.md) owns the current host API. Framework adapters
and integration tests remain with their consumer repositories.

## Current Canic boundary — 2026-10-10

Released Blob 0.22.2 retains the host-internal credit and bounded renewal APIs
introduced in 0.15.0. They do not acquire or authenticate provider evidence.
The [current Canic review](evidence/canic0223.md) finds only funding history,
exact outcome and preparation assessment mounted in its newer local adapter.
There is no adapter dispatch, credit-confirmation or budget-renewal endpoint.
This is an integration gap, not a missing public "mark credited" API in Blob.

[Canic #494](https://github.com/dragginzgame/canic/issues/494) owns guarded host
composition and its two-payment/recovery qualification.
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34) owns the
provider receipt/account-activity prerequisite. A scheduler or pre-upload path
must reconcile the same durable original operation; it cannot bypass an
uncredited/uncertain journal or repeat a payment from a missing reply. Keep
confirmation and financial renewal host-authorized, with all current bindings
and fences rechecked after evidence-acquisition awaits.

The [Canic adoption steps](dependencies.md#canic-integration) cover package,
memory and lifecycle delivery separately. This review authorizes no provider
request, payment or application scheduling change.

## Historical observed boundary — 2026-10-06

Toko Miner HEAD `061cfb6e3702a7075ab3c118bfaf315d7d2b0053` delegates its blob
component to `canic_blob_service::canister!()`. Its lock selects registry blob
0.14.9. The reviewed Canic wrapper at HEAD
`e1a211a00f01568ccc99bedc494c62a7141444dd` declares exact blob 0.14.9 and ic-memory
0.25; it supplies passive funding history/outcome/preparation inspection.
There is no qualified guarded-provider-dispatch/receipt-acquisition path there.
The raw review retains hashes of actual working files, rather than treating a
dirty parent checkout's HEAD as the complete source identity.

Toko's configured threshold/amount top-ups use Canic's execution-cycle funding
and the management `deposit_cycles` path. Those grants fund a canister's real
cycle balance. They do not independently credit the storage-provider payment
account or clear the blob journal's provider-credit obligation.

The reviewed Cashier top-up request has no local operation-ID or namespace field.
A success response and reported balance provide neither a unique durable receipt
nor complete competing account activity. There is no verified receipt acquisition
contract available in this review. Do not manufacture one from a balance delta,
caller assertion or locally renamed operation.

## Consumer-owned acceptance

1. Select the current published blob contract and one matching ic-memory runtime.
   Update the wrapper, generated Candid and every affected independent lockfile.
   Freeze and verify exact clean source/artifacts for qualification. Retire
   retained old installations with their original tools and obligations before
   reinstalling; do not reset or migrate their ledgers.
2. Obtain the provider-owned authenticated receipt contract and complete account
   activity evidence. Preserve the original receipt, authority, exact Cashier,
   payment account, accepted attachment and unique top-up attribution. Hash that
   evidence once. A host closure returning a constant digest proves only a local
   substitute and cannot close this acceptance item.
3. Keep the original durable intent across dispatch and any evidence-acquisition
   await. Release borrows before awaiting. Authenticate/recheck current state,
   then synchronously use `workflow::funding::credit::confirm`. A lost confirmation
   reply requires exact receipt replay, without another paid top-up.
4. Declare finite initial allowance, reserve, cumulative `renewal_ceiling` and
   lifetime intent capacity. If ongoing funding uses grants, make the explicit
   host/operator decision after full credit reconciliation and before preparing
   the next intent. Use `workflow::funding::renewal::increase` once for that
   original intent; real liquidity and all dispatch gates remain independent.
5. Exercise two distinct guarded top-ups through the actual host acquisition path.
   Verify retained receipts and spent totals, lost confirmation/grant output,
   exact replay without another provider request, receipt conflict/reuse,
   uncredited/uncertain refusal, ceiling and capacity exhaustion, IC write rollback,
   synchronous same-image restore and independent current-instance activation.
   Retain original request/reply bytes, artifacts, graph and provider evidence.

The library's local PocketIC receipts/Cashier tests do not close item 2 or prove
these consumer-owned executions. No sibling changes, consumer builds, deployment
or paid provider effects were performed in that review. Current provider and
host-integration work remains with Blob #34 and Canic #494 above; lifetime
acceptance remains with [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6).
