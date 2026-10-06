# Retire an installation before a hard cut

Cross-release transitions are reinstall-only. A reinstall erases the local owner;
it does not delete provider objects, return balances or stop billing. Apply this
runbook before resetting a canister or discarding its only authoritative records.
The [service contract](service-contract.md#current-persisted-boundaries) owns the
requirements; the [operator guide](operator-guide.md) owns command syntax.
Library publication is separate from permission to retire or deploy a service.

## Freeze the owner and preserve access

Assign an operator, reconciliation owner and budget. Stop application publication
and admission producers, including scheduled jobs. Record how this was enforced;
a stopped client is insufficient if another producer still has authority. Keep
inspection access and the old identities available. A restored owner stays fenced;
do not clear its fence to make retirement easier.

Freeze the old module, source/dependency lock, compiled release, installation input,
matching CLI/browser tools and their hashes. Record the actual service principal,
namespace, provider owner/project, tenant bindings, operator, uploader, verifier,
payer and cashier. Keep secrets under their existing access controls, separate
from public evidence. New ic-memory 0.27 tools cannot read an earlier ledger; use
the frozen old contract for inspection. A failed upgrade is not a migration.

Preserve backups and raw journals under an operator-controlled location that will
survive destruction of the installation. Hash an immutable manifest of these
artifacts and record who can retrieve them. Backup possession does not authorize
activating a stale instance or establish provider settlement.

## Reconcile an exact inventory

Use authenticated `status`, bounded `upload-history` and `funding-history` pages,
their exact follow-up request/outcome readers, and the application’s existing
reference/publication records where the frozen release supports them. Retain each
request, signed result and continuation cursor through the end of that local
range. Preserve failures and incomplete pages. Inspect through the original
tenant/operator authority; a digest or controller is not substitute authority.
Older contracts without these readers require their original retained records;
do not install a newer reader over them to obtain an inventory.

| Inventory | Required retained identities and evidence |
| --- | --- |
| Objects and declarations | Service/namespace/tenant, upload/object/root, original permission and manifest, provider owner/project/object mapping, exposed or confirmed state, physical bytes and conservative holds |
| References and application releases | Exact reference IDs and retain/release operation IDs, receipts and current status, every advertised release/rollback mapping and the owner of each remaining use |
| In-flight or uncertain effects | Original signed requests, certificates, expiry, dispatch journals, callback/transport outcomes and exact inspection replies; include funding, uploads, reads and provider calls |
| Economics | Payer/cashier/account bindings, balances and observation times, funding outcomes, reservations, stored-byte liabilities, request fees and continuing billing |
| Recovery authority | Module/installation identity, old tools, snapshots, fences, independent continuity history and who retains inspection/reconciliation access |

Local pagination does not enumerate an entire provider account. Reconcile these
records with independently retained provider object/payment evidence and the
consumer’s published asset inventory. Document scope, observation time and gaps
for each source. An aggregate counter, missing receipt, inaccessible URL, 404 or
zero usage report cannot prove that no obligations remain. If complete enumeration
or a surviving reconciliation owner cannot be established, stop the reset and
preserve the frozen installation.

## Discharge or preserve every obligation

Choose a disposition for each exact inventory row, with its owner, budget,
evidence and review date. Never infer disposition from another row’s outcome.

1. Withdraw application use only after its publication/rollback policy permits
   it. Release the exact reference through the old contract. Save the original
   operation and inspect its receipt after an uncertain reply; do not invent a new
   operation or repeat an uncertain provider effect. Last-reference release removes
   logical quota, while physical and billing liabilities remain.
2. Obtain separately authorized provider deletion evidence for the exact object
   and namespace. A local substitute or logical release does not qualify actual
   provider deletion. Preserve uncertain effects and conservative holds; expiry
   never permits repeating a potentially paid request.
3. Reconcile balances and final billing independently. Record controlled return
   or an explicitly reviewed terminal balance disposition. Keep continuing
   stored-byte/request-fee liability assigned to an owner with funding and access.
4. For obligations that cannot yet be discharged, preserve necessary records,
   authority and funded reconciliation ownership outside any state proposed for
   erasure. Retaining only a hash or anonymous summary is insufficient. If that
   handoff is incomplete, keep the old installation rather than resetting it.

Deletion and billing cessation remain provider qualification gaps. This runbook
does not supply a provider deletion command or authorize a paid effect.

## Review the reset decision separately

The maintainer reviews the immutable inventory and per-row disposition, including
unresolved effects, residual balances, continuing billing, retained authority and
cleanup ownership. Record the exact old canister/module and evidence location
before destructive action. Exhausted lifetime budgets or zero logical bytes do
not waive this review.

Install a new release only with independently reviewed service/namespace/provider
bindings, identities, budgets and funding. Do not implicitly copy old counters,
receipts or credentials, reuse an allocation identity to escape a fence, or claim
the new owner inherited settled state. Rebuild and verify the compiled release
using the [installation carrier](releasing.md#publication-and-deployment).
Retain the old records afterward until their actual obligations are closed.

The earlier isolated 0.6.0 and separate live 0.7.0 owners remain frozen with
exhausted lifetime capacity and provider/billing obligations; this local work
does not retire either one. Their exact evidence stays in the
[probe ledger](evidence/caffeine-probes/README.md).
