# Service contract — B1 draft

This is an unresolved contract checklist, not an implemented API or provider
suitability verdict. Freeze the decisions and evidence before B2 implementation.
The bootstrap package name does not decide the final package split.

## Acceptance target

Name one concrete application and accountable consumer owner. Its journey is:
an authorized tenant uploads a bounded object, resumes after interruption,
reads and verifies bytes, and releases the reference through confirmed provider
deletion and billing cessation. Exercise it against both standalone and
Canic-managed deployments with the same blob API and tenant rules.

Classify each behavior as existing behavior preserved, a safety correction
required for extraction, or a new capability deferred. B2 is bounded by this
journey and necessary corrections; optional ambitions do not gate extraction.

## Decisions and evidence required

| Contract | Owner role to assign | Decision and acceptance evidence |
| --- | --- | --- |
| Scope and publication | Service maintainer and consumer owner | Name application, maintainers, package split, registry/repository ownership and release plan; bind preserved behavior to source |
| Deployments and adapters | Service maintainer; Canic owners qualify generic integration | Both adapters live here, share handlers and tenant rules; core builds without Canic; retain Candid/artifact provenance and both journey results |
| Tenant authority | Service maintainer | Exact tenant/actor bindings and denial cases; a digest or Canic controller status grants no tenant authority |
| Identities and restore | Service maintainer | Recovery identity, non-reuse authority surviving older backups, stale-instance fencing and reconciliation before effects or admission based on stale accounting |
| Provider suitability | Service maintainer | For every paid/destructive operation: exact retry identity, authoritative completion evidence, retention horizon, typed uncertain outcome, bounded reconciliation, separate deletion and billing-cessation evidence |
| Accounting | Service maintainer | Deduplication choice, logical/physical quota basis, reservation/release timing, race-safe counters and ownership of costs until deletion and billing cessation |
| Existing obligations | Each affected installation operator; Canic owner inventories allocations | External objects, uploads, uncertain paid effects, balances and billing inventory; no-obligation evidence or completed owned decommission/disposition before reset |
| Canic removal and generic coverage | Canic runtime/facade, host/CLI and testing owners | Complete removal inventory and replacement evidence for surviving generic fixture coverage; changes occur only under separate Canic work |

Caffeine is a candidate, not a selected or qualified backend. Missing exact
retry binding, authoritative completion evidence, adequate evidence retention,
a safe uncertain-result disposition, or required deletion/billing-cessation
proof disqualifies the provider for the required contract. Provider evidence
must identify exact source/deployed interfaces and observed behavior.
PocketIC substitutes prove behavior under a model, not actual provider support.

Expired receipts or evidence never authorize repeating an uncertain paid
effect. Restoring sequence 40 after paid sequence 41 completed must not permit
reuse of 41 or erase its liability. Define fence entry and release, concurrent
instance exclusion and supported restore horizons. If sufficient evidence
cannot survive a restore, reject that path before effects and define a narrower
provable same-release backup/restore boundary.

Deduplication may be absent or tenant-local; shared cross-tenant deduplication
requires a concrete need and explicit privacy, charging and deletion ownership.
Releasing tenant quota never removes still-stored bytes from global capacity
or costs. Upload/release races must preserve references and counters; uncertain
effects retain conservative reservations or equivalent bounded liability.

Source allocation removal and installation retirement are separate. Before
reset, preserve the records needed to reconcile effects and settle obligations.
Unresolved paid effects stay fenced. Residual balances require controlled
return or explicitly reviewed terminal disposition; continuing liabilities
require preserved evidence, authority and funded ownership outside erased
state. No migration engine or old-state reader is implied.

## Exit evidence

B1 closes decisions, owner assignments, removal/obligation inventories and
actual-provider suitability evidence. It assigns exact service acceptance
cases to B2/B3 rather than claiming those tests already pass. Tests must cover
isolation, capacity exhaustion, corrupt bytes, interruption, lost responses,
older-backup restore, expired receipts, upload/release races and delayed
deletion/billing. Every guarantee needs a named owner and a test or evidence
reference; all such assignments remain open at bootstrap.
