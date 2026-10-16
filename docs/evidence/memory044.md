# Explicit Memory 0.44 runtime adoption

Source base: clean `ffe2b9a7f112de6ffb5313b7cb7a66d0f0dff060` (commit title
`0.29.1`, package metadata still 0.29.0; finalized release remains 0.29.0).
The existing graph selects Memory 0.44.0, Build 0.10.0, direct Host 0.22.1,
Metrics 0.11.5 and Testkit/Candid 0.48.0. Manifest and lock hashes are preserved.

The original check failed on Memory's removed global runtime facade. Blob now
retains only `grants::open(|key| runtime.open_memory(key))`; the host owns that
runtime and bootstrap. The obsolete singleton helper/test are removed, with
unbootstrapped capability and unchanged-memory checks retained in explicit-runtime
coverage. No new runtime, fallback, declaration registration or initialization
path is introduced. Existing missing-grant and namespace refusals remain covered.

The published Memory 0.44 README identifies `ICMDIRCT` snapshots containing direct
ledger CBOR, incompatible with earlier images. That source review is not an old
installation activation test. Pending notes target 0.30.0 because public types,
the removed helper and allocation-ledger format cross consumer boundaries.
Apply [installation retirement](../retiring-installations.md) before a fresh
cross-release reinstall; retained objects, uncertain effects and billing survive
logical release. Blob's application DTOs/record schemas are not changed here.

Live-graph library/all-target/all-feature compilation and strict lint pass, as do
all seven canister consumers. Four grant, two composition and 40 funding tests
pass. Actual Rust 1.91 library/native-test and standalone-Wasm checks pass, along
with warning-denied Rustdoc. Fresh standalone and six probe Wasms compile; the
standalone dependency graph excludes Memory's fixture support feature.
Four installed lifecycle cases pass with the fresh standalone Wasm, covering
restart/repeated same-release upgrades, restore fences, independent continuity
and preserved physical/billing obligations.
Six installed operator cases also pass on fresh authority/funding/gateway probes,
preserving pending callbacks, accepted-versus-credited totals and restored fences.
These are local PocketIC/substitute results, not deployed-provider evidence.

Before-images, original failure, exact input/artifact hashes and raw logs are
retained under `.tmp/review-validation/memory044/`. Full CI, native macOS,
actual package archives, Canic and deployed-provider acceptance were not run.
No package version, release, publication or sibling effects ran.
