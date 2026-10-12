# Shared 0.8.3 adoption and Canic completion review

Date: 2026-10-12. Blob base is released 0.22.11 at
`ebb560f492b6b53925faae8890316fc66bfd6def`, validated parent
`cca756f521fb0775a411d390f5191b1b88b9e1fb`. Pending compatible notes are 0.22.12;
package versions and finalized receipt remain 0.22.11. Preserve incoming lock edits.

## Shared Tooling adoption

Committed Shared source is 0.8.3
`8cca7e67b03ce6e701b6e7dcec2e9753eb98337e`, matching GitHub main at inspection.
Clean isolated canonical preview/refresh retains the existing 105 payloads and
adds the Cargo-tool freshness task, checker and substitute fixture, giving 108
files. Direct diagnostic and test consumers are recorded as entrypoints. No
payload is retired. Shared files keep their exact committed hashes/modes.

Changes include strict manifest-field parsing, atomic individual payload
replacement, complete minimal Make companions, retained build-identity mismatch
observations and refusal of redirected diagnostic files. Formatting admission
uses the selected Rust catalog; executable pins and build policy are unchanged.
The new online diagnostic reads receipts and bounded crates.io responses, never
executes installed tools or selects updates. The consumer CI adds only its
offline substitute fixture, not an online freshness gate.

Canonical actual refresh and verification pass. Selected-source distribution
tests pass, including malformed fields, companion closure and interrupted copy
retry. Consumer Make and Rust-tool fixtures pass on Bash 5.2 and genuine Bash
3.2.57 on Linux, including split catalogs, environment-change evidence and
redirected-observation refusal. The freshness fixture passes on both shells.
Actual online inspection reports observed Cargo selections current, retaining
original response evidence. Complete offline admission initially refuses the
selected Testkit 0.34.4 CLI; explicit setup succeeds with PocketIC 16.1.0 and
final complete admission passes. Earlier installations and failed evidence remain.

## Canic claim and original evidence

Published Canic is still 0.111.0 at
`25b73cd403b20b4e148c6e340f8c760a26d97f6e`. Its pending 0.112.0 source is dirty;
this review changes no Canic files or artifacts and runs no Canic command that
builds or deploys. Its normal adapter now consumes published Blob/contracts
0.22.11 and replaces repeated service declarations with the delivered owner:
113 endpoint lines instead of 376. Host authority, bounded decoding, caller
capture, memory and lifecycle ownership remain explicit.

Original Canic evidence under `.tmp/blob-published-20261012/` records passing
dedicated, embedded and production Root journeys (23.80s, 123s and 15.12s;
177.26s owning runner). Both 32-method Blob interfaces pass strict structural
Candid equality. Actual composition tests admit finalized artifact manifests
and compare callable Wasm exports with generated Candid. The earlier imported
emitter alias deferred registrations and failed; the normal adapter uses a
lexical emitter. Blob's public macro/README now document that concrete host
requirement and validation boundary.

Dedicated complete build:
`f7aca7dd2fc40c90fcdc1a61d54eaa9c5824268e74551150f273669565830db5`.
Embedded complete build:
`72de1096e637bbf67e544f901c0b0a4003c87c598397ff3abdcda2963bfab8ea`.
Both recorded fixture lock hashes match the actual current files; they select
Blob 0.22.11 with Auth 0.5.1, Memory 0.35.14, Metrics 0.7.1 and Timers 0.19.0.
The root manifest still matches its recorded hash. The root lock has since
changed from `c21bc626993d7845628c1e711d6fd05838ea0b4d22f464399217fa5cb3d39e41`
to `506948a79e8e753c859a04ed1a8cd94c340182fea7806fbe6ff52f72768cd960`, selecting
Memory 0.35.16. Earlier native Clippy results do not qualify that changed graph;
the separately locked runtime proofs retain their original identities.

The evidence supports completion of Canic's local framework endpoint/composition
slice. [Blob #61](https://github.com/dragginzgame/ic-blob-storage/issues/61) is closed.
Its larger uncommitted family delivery and application/provider acceptance stay
with [Canic #444](https://github.com/dragginzgame/canic/issues/444). Application
parent binding remains [#493](https://github.com/dragginzgame/canic/issues/493),
still controller-only in the inspected Root command; guarded funding remains
[#494](https://github.com/dragginzgame/canic/issues/494), with only passive
inspection in the adapter. Provider payer/credit/complete account activity is
[Blob #34](https://github.com/dragginzgame/ic-blob-storage/issues/34). Browser,
verifier, media-map, GC/deletion/billing and actual application proofs remain
their existing owners; local composition is not those deployed observations.

## Limits and retained owner

Raw new review/adoption inputs and attempts are outside Cargo's build tree at
`.tmp/review-validation/shared08302212/`. Canic requests, logs and artifacts stay
with Canic's existing evidence owner. Earlier Blob raw target paths are now absent;
tracked historical records remain, without fabricated replacement logs.
Incoming Blob locks are preserved and unqualified as a new product graph.
No full CI/release gate, fresh product build, native macOS, sibling edit, staging,
commit, version mutation, publication, deployment, provider request or cleanup
is performed. [Blob #59](https://github.com/dragginzgame/ic-blob-storage/issues/59)
retains native tooling/archive/transport acceptance. The current source/doc
changes leave public API and stored schemas unchanged.
