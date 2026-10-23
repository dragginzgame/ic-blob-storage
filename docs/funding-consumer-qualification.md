# Consumer funding qualification

This is a source review and acceptance recipe, not consumer execution or deployed
provider evidence. The [local handoff](status/current.md) owns implementation;
[funding credit](funding-credit.md) owns the current host API. Framework adapters
and integration tests remain with their consumer repositories.

## Current operational contract — 0.42

The [funding contract](funding-credit.md) treats a matching successful Cashier
callback as provider acknowledgment, not independent credit proof. Guarded
subsequent payments can proceed within the installed budget without waiting for
a new receipt API or a Caffeine response. The exact accepted attachment remains
spent; a missing/malformed reply or provider error with accepted cycles blocks.
A lost reply never authorizes resending the original payment.

The [bounded source review](evidence/caffeine-probes/local/2026-10-22-gc-credit-contract-02/summary.md)
distinguishes the public helper, which discards the response, from Cashier's
structured interface, which Blob already decodes. Independent credit confirmation
remains available for unacknowledged accepted payments when the host can establish
sufficient evidence. It is no longer mandatory for an acknowledged happy path.

[Caffeine #17 — Cashier credit and payer linkage](https://github.com/caffeinelabs/skills/issues/17)
asks about stronger credit guarantees, payer-link authorization and audit recovery.
Configured payer identity alone does not establish a provider relationship.
Existing account-link preparation/inspection interfaces remain the integration
boundary; hosts must establish the selected relationship and authority. No
undocumented account authorization or automatic external-payer setup is invented.

## Current Canic adoption — 2026-10-22

Read-only inspection of Canic's working tree based on
`cf2d3d1fba84a00a76cdbc541ae7e68fe36e9bb5` finds published Blob/contracts 0.42.0
selected in the root and both independent consumer locks. The new provider
emitters use Blob's bounded GC decoders and shared workflow. Dedicated and
embedded managed PocketIC logs under `/tmp/canic-blob042/` report passing
mounted callback, acknowledgment and restore cases; these were inspected, not
rerun here. The adoption is uncommitted consumer work, not default-branch proof.

Canic now implements opt-in guarded prepare, dispatch, credit and renewal.
Canic #494 owns accepted-payment and recovery qualification; its current refunded
and concurrent-call cases do not establish positive acceptance. Blob already supplies
the acknowledgment path; another credit API or local funding engine is not
needed. Payer linkage and real-provider acceptance remain separate.

## Historical Canic boundary — 2026-10-12

Released Blob 0.22.12 retains the host-internal credit and bounded renewal APIs
introduced in 0.15.0. They do not acquire or authenticate provider evidence.
The [authenticated published Canic adapter review](evidence/canic0229.md) finds
only funding history, exact outcome and preparation assessment mounted in 0.111.0.
There is no adapter dispatch, credit-confirmation or budget-renewal endpoint.
This is an integration gap, not a missing public "mark credited" API in Blob.
The newer [normal Canic composition review](evidence/shared083-canic02212.md)
qualifies published Blob 0.22.11 endpoint/placement/recovery integration; its
uncommitted 0.112.0 adapter still exposes only those passive funding operations.
That framework completion does not supply provider evidence or guarded funding.

[Canic #494](https://github.com/dragginzgame/canic/issues/494) owns guarded host
composition and its two-payment/recovery qualification.
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34) owns the
acknowledgment implementation and remaining provider/account qualification. A scheduler or pre-upload path
must reconcile the same durable original operation; it cannot bypass an
unresolved journal or repeat a payment from a missing reply. Keep
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
   Update the wrapper, method-bound Wire request/reply codecs, required external
   provider Candid and every affected independent lockfile. A Candid service
   export alone does not describe Blob's application endpoints.
   Freeze and verify exact clean source/artifacts for qualification. Retire
   retained old installations with their original tools and obligations before
   reinstalling; do not reset or migrate their ledgers.
2. Authenticate the selected Cashier/account/service relationship and complete
   relevant activity/liquidity holds. Use the shared guarded dispatch, preserving
   the original intent across its await. Persist its matching structured reply
   and callback refund before yielding again; no borrow crosses the call.
3. For an acknowledged success, inspect the exact retained result after lost
   output and proceed only with a distinct authorized intent. Do not call the
   independent credit API for an acknowledged operation. If the callback/reply is
   uncertain, stop further funding. Independent evidence can reconcile accepted
   unacknowledged transport; a balance change cannot manufacture the missing facts.
4. Declare finite initial allowance, reserve, cumulative `renewal_ceiling` and
   lifetime intent capacity. A host-authorized increase requires the latest
   acknowledged/credited intent and no unresolved history, before preparing the
   next intent. Use `workflow::funding::renewal::increase` once per original intent;
   real liquidity and all other dispatch gates remain independent.
5. Exercise two distinct acknowledged payments through the actual host dispatch.
   Verify spent/refund/acknowledgment totals, partial and complete refunds, typed
   provider errors, malformed/lost responses, exact inspection without resend,
   independent receipt conflict/reuse, ceiling and capacity exhaustion, IC write
   rollback, same-image restore and current-instance activation. Keep provider
   acknowledgment distinct from independently verified credit and spendability.

Canic #494 owns guarded composition and its executions. Local Cashier substitutes
qualify the local protocol and journals; they cannot prove deployed account-link
rules, charging or production credit behavior. No sibling changes, consumer
builds, deployment or paid effects occur as part of this repository's hard cut.
Lifetime acceptance remains with [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6).
