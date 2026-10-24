# Current status — 2026-10-24

## Released baseline and completed cleanup

Released baseline: `9534f42` (0.44.3). The completed cleanup preserved
endpoint identities, Wire frames, stored schema and stable keys.
The prior cleanup preserved the incoming Wire/derive 0.6.13 selection against
`/tmp/blob-cleanup-before/Cargo.lock`. Platform 0.6.14, Memory/Storage 0.50.16,
Metrics 0.15.8 and Tooling 0.67.0 remain selected.

Shared Tooling remains `4c0f198e56d4c6d27b12a5c2d515e0e01070151e` (committed
VERSION 0.26.10; its title/changelog call it 0.26.11). These describe the released cleanup; see the adoption below for the pending
snapshot and graph. No sibling checkout was edited.

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

## Prior cleanup validation and evidence

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

Canic remains read-only. The current root lock selects Blob/contracts 0.44.3 and
Wire 0.6.13; its two independent consumer locks still select Blob 0.44.2 and Wire
0.6.12. Its funding/schema adapters still use `InstalledJournal` and `methods()`.
It can replace those with the released shared access and typed projections in its
own checkout. Keep existing public APIs until those callers actually migrate.
[Funding guidance](../funding-consumer-qualification.md) documents the accessor.
Canic #613 owns frontend/schema delivery, #622 mounted acceptance and #494
independent credit qualification; these are not additional Blob funding engines.

Blob #4/#5/#6/#31/#34 still require application/provider evidence. Follow the
[acceptance checklist](../acceptance-plan.md#next-application-acceptance-run).
The earlier npm preflight failure is superseded by the environment diagnosis and
fresh local qualification below; its original log remains retained.
[Blob #102](https://github.com/dragginzgame/ic-blob-storage/issues/102) and
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) retain native
Agent crypto advisory work, separate from normal deployment.

No staging, commit, push, release, publication, paid effect or sibling edit.
Full release CI, full PocketIC suites, optimizer, native macOS, current-feed
security audit and live-provider acceptance are not claimed.

## Pending 0.44.4 browser selection repair

`make test-browser-standalone` now selects `standalone_browser::`, rather than
an old name prefix which skipped the renamed lost-reply/withdrawal regression.
No Rust runtime, dependency, protocol or persisted contract changed.

Fresh checks on release 0.44.3 plus the selector change pass: Node/npm admission,
three browser-control tests, generated TypeScript for 27 methods/137 frozen Rust
frames, and all four standalone Chromium/PocketIC cases (24.86s). These cover
happy-path verified download/release, corruption refusal, lost reply/withdrawal,
and distinct uploaders with independent references and retained liabilities.

Logs `/tmp/blob-followup-wire-schema.log` and `/tmp/blob-followup-browser.log`;
intent `/tmp/blob-browser0444-intent.txt`; raw local evidence
`/tmp/blob-browser0444/`; artifact/source hashes `/tmp/blob-browser0444-identity.json`.
Node 24.21.0/npm 12.2.0 pass outside the sandbox without installation; the earlier
empty-version failure was environment-specific. Local loopback provider fixtures
are not production acceptance. No live provider/account or paid effect occurred.
Historical trial documents remain labelled historical; no attestation is restored.

## Pending 0.45.0: published Platform/Wire and Shared Tooling

Adopted published Platform/macros 0.6.15 and Wire/derive 0.7.2, resolving the
publication blocker in [Platform #55](https://github.com/dragginzgame/ic-platform/issues/55).
The incoming lock already selected Platform 0.6.15 with both Wire lines; the
updated direct minimum leaves only Wire 0.7.2. Tooling 0.68.0 from the prior
batch remains selected. No path patch, sibling edit or schema wrapper is used.
Shared Tooling 0.26.14 is exported from clean committed revision
`e14050bcc0c627f9c8a9064051699cac44fd7ab3`, preserving the selected entrypoints.
It adds busy-tool admission and manually committed candidate recovery fixes.

Wire's decoder no longer requires Serialize or reserializes decoded values.
Collections/custom visitors may normalize input; successful decoding is not an
exact-byte identity proof. Original retry/signature frames remain authoritative.
Malformed primitives now report Malformed; the removed Noncanonical variant and
public Wire dependency require pending 0.45.0 notes. Package versions remain
0.44.3 until maintainer release. Endpoint IDs, frozen encoder bytes, explicit
decimal validation, persisted schemas and retained retry bytes are unchanged.
No DTO map/set schema, dual reader or automatic data migration was introduced.

Focused checks pass: Contracts all features (127 unit tests plus integration and
doctests), strict Clippy for Contracts/CLI/PocketIC all targets/features, generated
TypeScript (27 methods, 137 frozen frames), Rust 1.91 native checks and standalone
Wasm compilation. Shared snapshot/dependency/deployment checks, negative feature
fixtures and the Shared Tooling consumer suite pass. Its initial sandbox run
refused a local download-fixture socket; the admitted local rerun passes.
Evidence is retained in `/tmp/blob-adoption-*.log`.
No macOS or live provider acceptance is claimed. Canic must adopt the aligned
published Blob contracts before switching its exposed Wire types.

Fresh standalone Chromium/PocketIC qualification passes all four cases (27.03s)
with the adopted graph; tools, formatting and documentation checks also pass.
The first installed attempt stopped at missing report-parent creation before
browser execution; its log remains `/tmp/blob-adoption-browser.log`. Successful
results are `/tmp/blob-adoption-browser-qualified.log` and the matching report
directory. These use local substitute providers, not production acceptance.
