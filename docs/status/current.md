# Current status — 2026-10-24

## Released baseline and pending cleanup

Released baseline: `4a15d0c` (0.44.2). Pending compatible **0.44.3** changes no
package version, endpoint identities, Wire frames, stored schema or stable keys.
The incoming lock selects Wire/derive 0.6.13 and is preserved byte-for-byte against
`/tmp/blob-cleanup-before/Cargo.lock`. Platform 0.6.14, Memory/Storage 0.50.16,
Metrics 0.15.8 and Tooling 0.67.0 remain selected.

Shared Tooling remains `4c0f198e56d4c6d27b12a5c2d515e0e01070151e` (committed
VERSION 0.26.10; its title/changelog call it 0.26.11). No upstream adoption or
sibling checkout edit is part of this cleanup.

## Ten audit findings: disposition

1. Shared funding access: `ServiceWorkflow::<Host>::ACCESS` now implements
   `FundingJournalAccess`, observing platform continuity on every synchronous
   borrow. Hosts still own caller context, evidence acquisition and financial policy.
2. Public API narrowing: deferred until consumer migration. Canic currently uses
   the public model/policy/journal types and `stores_mut` through its funding adapter.
   Do not remove these in a patch or replace them with another forwarding hierarchy.
3. Provider JSON: explicit `provider-json` feature, enabled by default for existing
   tooling callers. Minimal workspace consumers disable defaults; CLI and browser
   integration tests opt in. Native schema generation enables JSON independently.
   The standalone normal/build deployment graph contains no serde_json; a feature
   guard and negative fixture checks reject provider JSON activation in deployment.
4. Schema metadata: native `projections()` supplies typed catalog metadata, limits
   and actual codec schemas. Serialized `methods()` and TypeScript output preserve
   their existing layout. Canic can adopt the typed boundary after release.
5. Grants: named keys map directly to owner fields, with no positional handle list.
   All existing keys, ordering and exclusive host runtime ownership are retained.
6. Installation bounds: candidate encoding checks the record owner's shared ceiling
   before invoking the trapping stored-record contract. Durable corruption still traps.
7. Capacity errors: explicit store/admission classifications retain current public
   failures; new top-level categories now require mapping review.
8. Funding conversions: retained deliberately. Stored records and public DTOs have
   different contracts; a generic mapper would couple them or hide the explicit
   conversion rather than reduce ownership. Existing round-trip coverage remains.
9. Registration terminology: clarify Rustdoc; keep public enum/variant identities
   and stored names. Registered content does not establish provider availability.
10. Restoration sizing: the existing one-upload installed resource regression passes.
    Production combined-occupancy qualification needs Canic's selected installation
    ceilings, which the maintainer does not yet know. Existing occupied-history tooling is
    explicitly fixture-sized, not production acceptance. No deferred activation or weakened
    recovery validation was introduced.

## Validation and evidence

Focused strict Clippy passes for Core, Contracts and CLI, all targets/features.
Native tests pass: installation 12 (including funding access/version-gap coverage),
grants 4, capacity 32, schema 3, provider 10 and the frozen Wire-vector test.
Dependency declaration/deployment checks and negative feature fixtures pass.
Rust 1.91 checks pass for Core/Contracts all native targets/features and the
minimal Core Wasm target. Formatting, documentation links and shell syntax are
checked separately; initial manifest-order failure is retained in the logs.
Fresh release storage-probe Wasm passes the installed restoration/read-attribution
case with two same-release upgrades. This is Linux PocketIC fixture evidence,
not Canic composition, deployed-provider or production-capacity qualification.

Logs: `/tmp/blob-cleanup-*.log`; the successful installed run is
`/tmp/blob-cleanup-restoration-qualified.log`. Earlier attempts are retained:
loopback socket admission failed in the sandbox, then the explicit fixture path
was missing. Both failed before the intended canister regression executed.
Intent is `/tmp/blob-cleanup-restoration-intent.txt`.
Wasm SHA-256: `a8fd4190f8aa473aad97526edc9ce16c3d342ccb1a2da9de9205f208c7e953f4`.
Lock SHA-256: `6ddef5c603a1bcecf4d6c249e49dfb9025ae264576ed0c71af8bb8d926449b75`.
Earlier released adoption evidence remains in `/tmp/blob-upstream-*.log` and
`/tmp/blob-tooling-*.log`; it is not relabelled as this cleanup's qualification.

## Consumer delivery and outstanding acceptance

Canic remains read-only. Its last reviewed three locks select Blob/contracts
0.44.1, Platform 0.6.14, Memory 0.50.16 and Wire 0.6.12. Canic must replace its
private funding accessor and consume typed projections in its own checkout after
adoption. Existing public APIs remain available during that transition.
[Funding guidance](../funding-consumer-qualification.md) documents the accessor.
Canic #613 owns frontend/schema delivery, #622 mounted acceptance and #494
independent credit qualification; these are not additional Blob funding engines.

Blob #4/#5/#6/#31/#34 still require application/provider evidence. Follow the
[acceptance checklist](../acceptance-plan.md#next-application-acceptance-run).
The earlier browser preflight failure remains `/tmp/blob-acceptance-preflight.log`:
selected npm returned no version instead of required 12.2.0. No fresh browser
qualification or tool installation occurred in this batch.
[Blob #102](https://github.com/dragginzgame/ic-blob-storage/issues/102) and
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) retain native
Agent crypto advisory work, separate from normal deployment.

No staging, commit, push, release, publication, paid effect or sibling edit.
Full release CI, full PocketIC suites, optimizer, native macOS, current-feed
security audit and live-provider acceptance are not claimed.
