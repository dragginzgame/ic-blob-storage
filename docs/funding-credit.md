# Host-internal funding credit confirmation

The pending 0.15.0 library lets a trusted host record independent credit evidence
for an exact original top-up. Transport settlement alone still leaves a positive
accepted attachment `CreditRequired`. Confirmation clears that local obligation;
a subsequent distinct guarded preparation/dispatch can then pass this gate.
Every other admission requirement remains independent.

The integrating host owns provider authority, receipt verification and complete
account activity. Linking the library creates no endpoint or lifecycle hook.
Standalone exposes passive inspection only. Consumer frameworks must implement
and test acquisition in their own repositories; no automatic Toko funding timer
or deployed provider credit contract is qualified by this library change. The
[consumer qualification recipe](funding-consumer-qualification.md) records the
reviewed Toko/Canic boundary and the evidence still required.

## Establish and commit evidence

1. Retain the original intent and callback. Confirmation requires known positive
   transport acceptance, equal to the offer minus the original exact refund.
   Prepared/uncertain, proven-unsent and fully refunded attachments cannot be
   confirmed. Recover missing transport facts through their authenticated owner;
   never manufacture a callback to admit credit.
2. Independently authenticate the original provider evidence and uniquely
   attribute credit to the original Cashier, payment account and top-up. Check the
   local service, namespace, operation, offered amount and optional target against
   the retained intent. The provider request carries no local operation ID;
   a balance increase or successful top-up reply alone cannot prove attribution.
3. Preserve the original immutable receipt/evidence with the existing host owner.
   Compute SHA-256 over that unique provider evidence. Do not hash a locally
   renamed operation, mutable balance snapshot or caller-provided assertion.
   Credentials and unbounded evidence never enter the journal or passive reply.
4. In a synchronous IC update, call `workflow::funding::credit::confirm` on the
   existing `StableFundingJournal`, using authenticated `UploadContext` and the
   exact original `FundingIntent`. Its closure receives `FundingOutcomeView`
   only after operator, binding, fence and positive-acceptance checks. Return
   `None` if credit is not established, or a `FundingCreditConfirmation` with the
   exact intent, full `NonZeroU128` accepted amount and nonzero receipt digest.
   The host must not acquire these facts from ingress flags. Authentication and
   qualification happen at the host boundary, not inside the fingerprint type.

The equivalent single-step ops API is
`StableFundingJournal::record_credit(context, confirmation)`. Both APIs use the
same durable owner and transition. The workflow reports `NotEstablished`,
`Confirmed` or `AlreadyConfirmed`; the ops method returns true for a new commit
and false for exact replay. Stable traps must propagate to roll back the intent,
receipt index and accounting writes together. Native memory alone provides no transaction.

If receipt acquisition involves an await, release the journal borrow first.
Authenticate the resulting evidence and call the synchronous workflow afresh;
the journal rechecks current identity/state. Confirmation itself makes no provider
call and provides no dispatch permit across that await.

## Replay, budgets and restoration

The receipt covers the entire positive accepted attachment; partial confirmation
is unsupported. A different amount or fingerprint cannot overwrite it. Reusing
the same fingerprint for another operation in this journal refuses before writes.
The host must also prevent attribution/reuse across other accounts or installations:
a local digest is not a cryptographic proof or a global receipt registry.

After a lost confirmation reply, replay the exact already-established evidence;
never repeat the paid top-up. Exact replay changes neither counters nor transport
history. Fresh confirmations use a digest-to-original-operation index in the
existing accounting memory; exact replay remains unchanged. Same-contract reopen
checks every index binding and the exact number of receipt rows against original
intent records. Missing, misdirected or orphan rows refuse without repair.
No additional stable-memory grant or external journal is added.

`accepted` remains a lifetime spent total. `credit_confirmed` records the covered
portion and `uncredited` is their difference. Confirmation does not return cycles,
replenish available allocation, increase lifetime intent capacity or erase callback
refunds. A repeating host still needs a sufficient finite funding allocation,
operating reserve, complete liabilities and fresh provider/spendability/activity
qualification. Unknown transfers and other uncredited operations keep blocking.

Same-contract reopening reconstructs credit totals, validates receipt uniqueness
and enters restoration fences. Historical inspection remains available. Even an
identical confirmation replay refuses while fenced; credit evidence does not
prove safe identity allocation or authorize current-instance/stale-backup activation.

## Bounded host-authorized allocation increases

Every installation declares `funding.renewal_ceiling`, the immutable maximum
cumulative authorization. It must be at least `funding.allocated`; equal values
disable increases. Direct core builders use `FundingAllocation::new(...)
.with_renewal_ceiling(...)`. A grant supplies no platform cycles or provider credit.
The initial authorization stays bound to the installation; `allocated()` on the
current allocation view reports its cumulative total and `renewal_ceiling()` its
fixed maximum.

After credit confirmation and **before preparing the next intent**, the host may
call `workflow::funding::renewal::increase(journal, context, FundingBudgetRenewal {
intent, additional })` or the single-step `StableFundingJournal::renew_budget`.
The host/operator owns this financial decision. No public grant endpoint is
exported. The journal requires the exact latest original credited intent, no
uncredited or uncertain history, and a positive increase no greater than that
intent's accepted cycles. Each original intent permits one immutable grant.
Partial grants are allowed but cannot later be enlarged. The cumulative total
cannot exceed the installed ceiling, including at integer boundaries.

A new grant returns true; exact replay returns false even after later intents.
Conflicting grants, wrong callers/bindings, stale original intents and restore
fences refuse before writes. Persist the intent's immutable grant and totals
synchronously in one IC update; propagate traps for rollback. Lost grant output
requires only exact local replay/inspection, never another provider transfer.

A grant increases local `available` and cumulative authorization equally.
`accepted`, confirmed credit, callback refunds, original receipts, last operation
and lifetime intent count remain intact. Available + accepted + reserved/uncertain
always equals cumulative authorization. Reopen reconstructs each original offer,
transport, receipt and attached grant in order, so later grants cannot justify an
earlier offer. All dispatch gates still require fresh real liquidity, execution
reserve, qualified provider authority and complete account activity.

Lifetime intent slots are never renewed. At the cumulative ceiling or installed
history capacity, stop new funding and apply the [retirement contract](retiring-installations.md)
when replacing the installation. Bounded rolling authorization is not indefinite
operation, a history reset or a cross-release migration.

## Passive responses and hard cut

Operator status and preparation assessment expose mandatory decimal-string
`uncredited_accepted`, `cumulative_allocation` and `renewal_ceiling` in native JSON,
separate from lifetime `transport_accepted`.
Exact outcomes add `CreditConfirmed { accepted_cycles, receipt_digest }` in Candid
and `credit_confirmed` with `accepted` and `receipt_sha256` in native JSON.
`provider_credit: host_confirmed` means the authenticated service retained the
host's confirmation, not independent client verification of the provider.
Exact outcomes also carry mandatory `renewed_allocation` (decimal-string native
JSON); zero means no grant. Its amount requires confirmed credit and cannot exceed
acceptance. Reported balances remain separate; every outcome keeps
`retry_authorized: false`.

The current funding records require credit state and confirmed totals. Their
installation format is
`ic-blob-storage/installation:platform-anchor-funding-credit-index-renewal`.
Update hosts, DTOs, tools, codecs and fixtures together for 0.15.0. Prior layouts
are refused; there is no fallback reader or migration. Before reinstalling, apply
[installation retirement](retiring-installations.md) to provider objects, balances,
uncertain effects and continuing billing. The hard cut preserves the requirements
for same-contract interruption recovery and restoration.
