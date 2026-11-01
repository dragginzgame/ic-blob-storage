# Current status — 2026-11-01

## Delivered baseline and pending cleanup

Tag `v0.63.0` identifies release **0.63.0** at
`6be5c1d2ab45d8383835f945183a872296d59383`; see
[the release receipt](../release.json). Registry publication and hosted checks
were not reread during this cleanup. The preceding dependency-adoption handoff
is preserved in `/tmp/blob-simplify064.1L5ZdK/previous-handoff.md`.

Pending notes target **0.64.0**: the memory composition, gateway reply vocabulary/
frame identities and saved session provenance change intentionally. Own Cargo
versions stay 0.63.0 for maintainer preparation. No staging, commit, push, release, registry publication,
deployment, sibling edit or paid provider effect was performed.

## Published dependency adoption

Adopt **Build/Host Foundation 0.73.5** and **Memory/Storage 0.60.7**. Official
crates.io index records and cached archive checksums establish publication;
sibling Unreleased notes do not. Align all seven root declaration minimums with
the incoming lockfile, preserving that lockfile byte-for-byte. Host Tools remains
at the latest published **0.76.0**. Own package versions remain unchanged.

Host Foundation and Memory/Storage have no Rust source changes from the prior
locked patches. Build adds qualification of existing archives; Blob retains its
supported archive-producing API and actual adapter regression. No memory layout,
stored bytes, decoder budget or recovery policy changes arise from these patches.
The lock-selected Build 0.73.5 release editor is explicitly installed and admitted.

## Completed complexity reductions

1. **Root/operation correlation:** the immutable forward root claim now includes
   the exact nonzero upload operation. Remove the separate root_requests map and
   grant while retaining reverse object uniqueness, exact replay and complete
   restore validation. There are thirteen store grants and fourteen installation
   memories including configuration. The root-correlation Wire record has a new
   identity with schema 1; no old reader, metadata-only reset or migration exists.
2. **Shared fixture ownership:** remove the generic Cargo metadata matrix from
   Blob's Make roster and snapshot selection through the canonical exporter at
   the same reviewed revision. Narrow local logging checks to Blob's actual
   nested stable/drift/deleted-input gates. Shared owns diagnostic/highlighting,
   parser and generic inheritance matrices. Snapshot 0.27.22 remains revision
   f4f431627290e18f0ccc1e5f8525d16b197cf7c3, with 138 files.
3. **Permission validation:** manifests and exposure use one exact retained
   permission lookup. Preserve uploader-only mutation, tenant/uploader historical
   inspection, binding/error precedence, complete permission equality and all
   freshness/activation checks at their effect owners.
4. **Descriptor reads:** carry the resolved permission's manifest state into
   descriptor construction, avoiding another permission lookup. Historical
   inspection and active live-reference serving retain their distinct gates.
5. **Session provenance:** save per-file setup/transfer records instead of two
   parallel arrays. Require both explicit optional fields, reject wrong counts,
   unknown/missing fields and the old format. Original source paths and signed
   claims still own retry decisions. Older runs retain their original tools.
6. **Capacity errors:** keep explicit public binding/denial classifications and
   one conservative Internal fallback instead of enumerating unrelated mutation
   errors. No authority or disclosure policy changes.
7. **Gateway refresh:** an explicit operator refresh supersedes an older pending
   read token. Late replies/cancellation cannot overwrite its successor; revocation
   and restore fences still win. Explicit cancellation remains available. Remove
   the obsolete Busy refusal and change only sync/cancellation Wire identities
   to 0x424c_4f42_0064_0012 / 0x424c_4f42_0064_0013. Four frozen vector files are
   reviewed for these header changes and the shifted ReplyTooLarge error tag;
   a regression rejects retired identities. Other endpoint bytes remain frozen.

Hosts must rebuild their memory declarations using Blob's request factory and
regenerate the two affected Wire callers. Follow
[retirement/reinstall](../retiring-installations.md) with the old installation's
original tools; removing allocation source cannot erase provider objects,
uncertain effects, balances or continuing billing.

## Archive candidate retained after coverage review

The eighth candidate remains: authority-probe archive recovery uses disposable
model transitions to inspect a permanently fenced record, never a second live
service owner. Unique cases cover partial hashes, wrong raw digests, trapped/
in-flight reads and billing after physical deletion. StableUploads does not own
those consumer states; replacing the validator would recreate a fixture model
and lose independent coverage. Keep these bounded non-default cases until their
scenarios retire. See `archive-disposition.txt` in the cleanup evidence below;
production standalone restoration remains separately tested.

## Validation and evidence

Source `95fa5cae` failed the consumer completion fixture because nested CI omits
console excerpts. The correction checks retained diagnostics, preserving drift/
deletion refusal. The parent-logged `shared-tooling-tests` stage passes (28s),
as do Bash syntax/ShellCheck. `/tmp/blob-nested-logging-fix.AwXAaL/` retains all
runs, including the initial loopback sandbox refusal. No full CI or release was
rerun; the correction remains uncommitted.

The cleanup's earlier evidence is isolated in `/tmp/blob-simplify064.1L5ZdK/`,
bound to its source/input/artifact hashes and exact commands. It predates this
dependency adoption. Initial execution reused
`/tmp/blob-cleanup-1101`, already referenced by historical 0.62 evidence, and
overwrote some temporary logs there. Do not use overwritten files to qualify the
old release. Only this run's explicit case roster was copied to the new directory;
tracked historical evidence was not changed. See `evidence-custody.txt`.

That earlier roster passes Core 297, Contracts 132, CLI 122 plus eight boundary
cases, frozen Wire and TypeScript vectors, memory composition, docs, scoped
Clippy/MSRV/Wasm, snapshot/logging checks and 36 distinct installed cases. Initial
fixture-slot and retired-Busy failures and corrected runs remain separate.
See its exact case roster and custody note; those results are not reattributed
to the newly selected dependencies.

Fresh adoption evidence is `/tmp/blob-deps-adopt.ZKYN6N/`: published versions,
checksums, source comparisons, preserved incoming inputs and exact commands.
Offline locked metadata, pins, direct/deployment/default-client boundaries and
their refusal fixtures pass. Scoped strict Clippy, Core 297, both memory-composition
cases, client ten plus one documentation case, and CLI 122 plus eight boundary
cases pass. Rust 1.91 native CLI all-target/all-feature and standalone Wasm checks,
native CLI and three release Wasm builds, scoped formatting and documentation
links pass. Build's actual archive adapter passes warm-cache/current-payload,
graph admission and final byte/source/manifest/lock drift refusals. Its disposable
fixture remains at `target/package-fixture.mbD3Z4`; no real release was executed.
Fresh local PocketIC qualification passes nine cases: backing-growth rollback,
gateway refresh supersession, five lifecycle/recovery cases, signed gateway CLI
decisions and native download refusal before provider GET. Exact selections and
artifact hashes are retained in `installed-results.json` and `artifacts.json`.
No full workspace/release gate, native macOS/Windows run, actual registry-consumer
installation of pending source or deployed-provider qualification is claimed.
No performance improvement or production Wasm saving is inferred from this cleanup.

## Next: Canic adoption and bounded provider acceptance

Last read-only Canic observation was main 0.114.0 (`e75378595`), with Blob 0.61
in its root and two independent consumer graphs. Canic was not reinspected or
built during this dependency adoption. Its owner must align the published Blob,
Platform and Memory identities across all three graphs, then qualify dedicated/
embedded mounting, generated clients, lifecycle and recovery. Do not infer
package publication or a deployed Blob service from a Git tag or local fixtures.

Use [the Canic owner handoff](https://github.com/dragginzgame/canic/issues/622#issuecomment-6546806175)
and [the acceptance plan](../acceptance-plan.md#before-the-acceptance-deployment).
The maintainer knows the provider is Caffeine; retain the original operator/
service selection rather than inferring a current deployment from trial principals.
A bounded provider-only trial may precede a finished frontend, but does not qualify
application login, serving or catalog publication. Deployment and paid effects
retain their separate authorization and budget boundaries.

The last GitHub readback found no open Blob PRs and four open acceptance issues:
[#5 — Consumer publication and serving](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#31 — Application multi-user uploads](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#6 — Reference lifecycle and deletion/billing](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#34 — Project-payer linkage and funding evidence](https://github.com/dragginzgame/ic-blob-storage/issues/34).
This pass did not refresh or close those issues. Dependency validation resolves
none of their deployed-provider acceptance obligations. The
[probe ledger](../evidence/caffeine-probes/README.md) retains intent, observations,
failures and cleanup; no new provider probe was run.
