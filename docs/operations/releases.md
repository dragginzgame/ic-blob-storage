# Product releases

The maintainer authorized crates.io publication on 2026-11-16, beginning with
`0.101.0`. The library, standalone canister, reference application and CLI share
one version; the separate qualification workspace stays unpublished. This does
not freeze API/persistence, migrate legacy installations or qualify Caffeine.

Commit the intended development batch, including its numbered pending changelog.
From a clean checkout on `main`, use `make release-patch`, `make release-minor` or
`make release-major`. A breaking pre-1.0 command/API change selects minor. Toko's
shared runner owns source admission, intent, metadata staging, commit, annotated
tag and atomic push to `origin/main`. No sibling release is implied. Recover an
interrupted saved attempt with `make release-resume VERSION=X.Y.Z`, after inspecting
its reported evidence; never restart uncertain Git or registry effects blindly.

Preflight admits the pending version, prepares common tools/the metadata editor
and fetches both locked Cargo graphs. `release-verify` invokes Blob's
[selector](../../scripts/release/select-validation.pl) and Toko's installed plan
runner/evidence verifier. [The stage roster](../../ci/release-validation.json)
also supplies repository CI's native stages; [Make leaves](../../make/validation.mk)
carry package, feature and target scope to Cargo. Setup for npm and the simulator
runs only when selected. There is no cross-run result cache.
These are local release checks, not a requirement to wait for hosted CI; report
hosted status separately. Pending hosted results alone do not delay publication.
Chromium qualification remains separately opt-in. Metadata preparation uses the
pinned IC Build editor and Toko transaction to update the root manifest, both
locks and changelog together. Private package versions and external selections
stay unchanged. Final checks reopen both locked graphs. The shared runner binds
validated source and the exact release index; final archive verification follows.

The previous version's annotated ancestor tag is the trusted release baseline;
the release runner owns that version and clean source. The original fully tested
0.103.3 tag establishes the initial baseline. Subsequent maintainer release tags
carry the same full-or-selected validation obligation inductively. Missing,
lightweight or non-ancestor tags select the full product roster. Only Linux x86_64
with ordinary Cargo configuration has a narrow mapping; other hosts/overrides
broaden to the full product gate. Hosted macOS acceptance remains asynchronous.

| Changed input | Selected checks and guarantee |
| --- | --- |
| Every release | Snapshot/declaration/tooling binding, formatting and documentation-link admission |
| CLI implementation | CLI lint/tests/Rustdoc; installed application, callbacks and management preserve signing, custody, transfer and maintenance behavior |
| Browser source/tests/npm graph | Locked npm setup and Node tests; installed callback/catalog transfer and verified publication |
| Standalone host implementation | Host native/Wasm checks; standalone, capacity, endpoints, signed calls, application, enrollment and management journeys |
| Reference application implementation | App native/Wasm checks; installed application and callback/catalog journeys |
| One installed test owner | Its suite and required real artifacts/test lint; shared helpers and fixtures broaden to full |
| Library/protocol, manifests, locks, compiler, shared helper, CI or unknown input | Full native roster and all eight installed suites |
| Release/Make/tooling wiring | Full product roster plus the consumer's two-lock metadata fixture |
| Prose-only documentation | Common admission; executable data under docs is not classified as prose |

Unchanged metadata regression tests are omitted on product-only changes; real
metadata rendering/admission still runs before commit/tag. CI retains all native
lint, local behavior and Rustdoc obligations once, including all library tests;
it does not acquire the optional installed/Chromium roster. Installed preparation
builds the selected artifacts together, then suite leaves consume exact artifact
paths. Full qualification lint replaces narrower repeated test lint. CLI native
lint is not repeated to produce the executable artifact. Wasm lint checks a
different target and remains required. Existing focused `test-*` commands retain
their standalone lint/build setup.

Each run creates a unique `target/validation/blob.*/` containing canonical
baseline/context/plan, runner logs/checkpoints and verified `proof.json`. Every
stage is executed or explicitly excluded with a reason. Source/input drift and
required failure stop later work; failed attempts never provide reusable success.
The standard release runner alone owns saved phase recovery: a gate failure gets
a fresh plan/gate; a later failed phase uses `release-resume` and its exact saved
source/intent. Never rewrite earlier evidence or use cache hits as a pass.
Archive/source/tag/registry/provenance admission and uncertain-upload recovery
remain fresh through the unchanged publication runner and steps below.
All five product crates remain one coordinated version group: each release still
checks/publishes all five archives explicitly. A narrower validation plan does not
silently change that package selection or qualify a different release grouping.

When selection/recipes/runner wiring changes, run `make test-release-validation`
and `make test-release-metadata`. The former runs the actual selector, shared
runner and Make leaves in disposable Git scratch with recorded Cargo/simulator
substitutes, exercising scope, artifacts, failures, drift and recovery admission.
Generic parser/schema regressions stay with Toko. This comparison is deliberately
not an unconditional release or CI prerequisite. Its timings describe orchestration
with simulated product work; real product/host qualification and release latency
must be measured separately. [The handoff](../status/current.md) records the storage layout and installed
qualification for the current worktree.

At the exact release commit, run `make publish-check`, inspect the five prepared
archives, then check their provenance before `make publish`:

```sh
bash vendor/toko-tooling/scripts/ci/check-cargo-package-provenance.sh \
  target/publication/provenance-NEW-ATTEMPT target/package/tmp-crate \
  https://github.com/dragginzgame/ic-blob-storage "$(git rev-parse HEAD)" \
  --package "ic-blob-storage@VERSION" crates/ic-blob-storage \
  --package "ic-blob-storage-wire@VERSION" crates/ic-blob-storage-wire \
  --package "ic-blob-storage-reference-app@VERSION" crates/ic-blob-storage-reference-app \
  --package "ic-blob-storage-canister@VERSION" crates/ic-blob-storage-canister \
  --package "ic-blob-storage-cli@VERSION" crates/ic-blob-storage-cli
```

Replace `VERSION` and use a fresh evidence directory. Cargo 1.100’s coordinated
publish dry-run places these archives in `target/package/tmp-crate/`. Publication requires clean
source at its annotated release tag. Cargo verifies the coordinated archives and
orders uploads; Toko retains selection, dispatch intent and exit status under
`target/publication/`. It does not rerun the installed suites. Keep failed evidence;
a partial upload requires registry observation before selecting any remaining
`PACKAGE` subset. An interrupted command is not evidence that no upload occurred.
Independently confirm every version and archive checksum on crates.io afterward.

`make blob` installs the local CLI; users can instead run
`cargo install ic-blob-storage-cli --locked`. The packaged `tooling.sha256` fingerprints the repository’s managed snapshot;
`make tooling-binding-check` verifies that binding in CI, CLI builds and publication.
After an authenticated tooling update, refresh it with
`bash vendor/toko-tooling/scripts/ci/verify-file-checksum.sh --print sha256 .toko-tooling.snapshot > crates/ic-blob-storage-cli/tooling.sha256`.
This preserves exact checkout admission without copying the managed payload. The licence
is included by Cargo. No custody directories, signer keys or qualification Wasm
belong in package archives.

After delivery, review the pushed diff for architecture and maintenance sediment.
Keep delivery results in the current handoff; this procedure is not a second ledger.
The metadata adapter reuses the established IC Entropy two-lock pattern. The
historical Blob checkout was unavailable at this adoption; no legacy release
runner, recovery records or provider state were copied.
