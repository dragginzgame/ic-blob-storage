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

Preflight admits the pending version before preparing tools and fetching both
locked Cargo graphs. Validation runs the metadata projection fixture, tooling,
`make ci` (strict Clippy before compiled tests) and all eight installed suites.
Chromium qualification remains separately opt-in. Metadata preparation uses the
pinned IC Build editor and Toko transaction to update the root manifest, both
locks and changelog together. Private package versions and external selections
stay unchanged. Final checks reopen both locked graphs. The shared runner binds
validated source and the exact release index; final archive verification follows.

At the exact release commit, run `make publish-check`, inspect the four prepared
archives, then check their provenance before `make publish`:

```sh
bash vendor/toko-tooling/scripts/ci/check-cargo-package-provenance.sh \
  target/publication/provenance-NEW-ATTEMPT target/package/tmp-crate \
  https://github.com/dragginzgame/ic-blob-storage "$(git rev-parse HEAD)" \
  --package "ic-blob-storage@VERSION" crates/ic-blob-storage \
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
