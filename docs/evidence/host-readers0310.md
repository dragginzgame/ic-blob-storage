# Published Host readers and package licenses — pending 0.31.0

This batch starts from released Blob 0.30.0
`e9f71822ae361377686846fe7a4bf7c0e9a6822f` plus the existing dirty 0.31 work.
Exact before-images, API/PR readbacks, archive payloads, logs and artifact hashes
remain under `.tmp/review-validation/hardcuts0310/`. Registry selections and
Cargo.lock are unchanged: direct Host 0.23.1, Build 0.11, Memory 0.44,
Metrics 0.11.7 and Testkit/Candid 0.49. Own versions remain 0.30.0.

## Reader ownership

[Host PR #114](https://github.com/dragginzgame/ic-host-tooling/pull/114) is merged;
the required functions also exist in tagged Host 0.23.0 and published 0.23.1.
This activates the previously isolated consumer work from
[Blob #69](https://github.com/dragginzgame/ic-blob-storage/issues/69), adapted to
the current shared installation candidate rather than restoring old byte copies.

Inventory metadata and body selection retain one admitted directory descriptor.
No canonical descendant path is returned for a later body reopen. Host owns
component traversal, no-follow opening and regular-file admission; Blob retains
relative-name limits, stream/content budgets, exact digests, refused-input codes,
output claims and original-operation recovery. Explicit single-file paths keep
their separate established following behavior. Historical preparation journals
use the same Host reader under their existing origin and binding checks.
Held descriptors neither freeze contents nor prove current pathname containment;
source-writer exclusion and complete original-content verification remain required.
There is no new command mode, provider call or retry authority.

## Licensing and qualification

[PR #73](https://github.com/dragginzgame/ic-blob-storage/pull/73)'s source changes
are incorporated locally with notes under 0.31, preserving historical entries.
Both public package licenses match the root MIT text; all twelve members inherit
the workspace license. Fresh actual Cargo archives contain those exact license
bytes. Archive creation used `--no-verify` solely to inspect package contents;
this is not coordinated graph/build/archive or publication qualification.

127 CLI/probe/subprocess cases pass, including two new consumer boundary cases:
held-root metadata/body selection survives namespace replacement, and bounded
journal reads reject symlinks/traversal while preserving original bytes. Existing
preflight, partial output, digest, occupied-run and no-redispatch cases remain.
Strict all-target/all-feature CLI lint and actual Rust 1.91 compilation pass.
Seven installed publication-session cases pass with the newly built CLI and the
hash-matched standalone Wasm from the earlier qualification; no fresh Wasm build
is claimed. PocketIC 16.1 archive, decompression, executable and version admission
is repeated before execution. Source/manifest formatting, dependency declarations,
documentation links and diff checks pass.

The working batch has no hosted result. #69 remains open for delivery, current
archive qualification and required native CI. PR #73 still needs its owning
branch/main integration; this local incorporation does not modify or merge it.
PR #78 separately owns direct Host HTTP capture; Linux, Apple Silicon and MSRV
pass while Intel macOS is still running at the review checkpoint. Host framing
and retained-write producer PRs remain open; their #74/#76 adoptions are not
activated. Public decoder limits already provide the batch's 0.31 hard cut; this
work adds no unrelated public API or retained-format cut. No sibling edits, full
CI, native macOS execution, Git/release effects or deployed provider probe ran.
