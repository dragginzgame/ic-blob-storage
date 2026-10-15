# Reply, historical claim and fixture transport convergence

Date: 2026-10-15. Released base: `e65834ce6e61b38954c6a411edd7417cf0090e5a`
(0.28.0). The pending batch is 0.29.0 because the maintainer's incoming public
Memory selection is 0.42. Package versions remain 0.28.0.

## Source and dependency custody

`.tmp/review-validation/convergence0281/before.tar.gz` retains the task's incoming
working source. Host 0.20/Memory 0.42 edits arrived during review; their settled
manifests/lock are retained separately as `incoming-Cargo.toml` and
`incoming-Cargo.lock`. Metrics 0.11.1, Testkit 0.46.0 and Build 0.6.1 remain selected.
The incoming package versions, checksums and source identities are preserved.

Build 0.6.1 exposes Host process 0.19 types. The opt-in verifier now inherits the
explicit root `ic-build-host-process` alias, including typed failure-status
handling. Its lock edge changes from process 0.20 to the already selected 0.19;
no package version is reselected. Other direct Host consumers remain on 0.20.
Testkit/Build retain their own Host 0.19 dependencies. Normal library/CLI builds
do not enable the verifier.

## Ownership and preserved behavior

Contracts owns bounded capacity/discovery reply decoding and structural binding
checks. CLI and fixture callers retain service authentication, distinct refusal
mapping, expected file lengths, observation retention and planning. Wide byte
headroom, independent operation identities, absent/retired roots and restore
fences remain observations, never allocation or retry authority.

The signed dispatch writer owns its historical reader. The flat JSON and CBOR
formats stay unchanged. Recovery retains operation/tenant/transport policy and
checks exact original packets. The common reader rejects changed hashes, expiry
correspondence and dispatch flags; expired historical packets remain inspectable.
The SDK inspection checks envelope correspondence, not cryptographic signature
verification. No inspected packet is returned for dispatch, and existing/partial
claims continue to forbid resubmission.

Fixture query/update attachment has one local owner. It catches SDK transport
panics while preserving query rejection versus update transport failure. Callers
retain exact target/caller/method and operation-specific decoding. Attachment
owns neither the existing instance nor a gateway; no fallback or application
retry is added. This is fixture infrastructure, not a Canic adapter.

The three cleanups do not change wire DTOs, durable records or installation
policy. The separate Memory 0.42 public API cut requires matching host types;
Canic owns fresh adapter/root/private-fixture qualification. Cross-release
retirement/reinstallation and same-release recovery remain required.

The final input check caught a later incoming lock refresh selecting Host
0.20.1. It is preserved as `incoming-host0201-Cargo.lock`; `host0201-inputs.json`
binds that source. Earlier command logs retain their Host 0.20.0 scope.

## Focused evidence

Raw command logs are retained in `.tmp/review-validation/convergence0281/`.
Contracts upload tests pass (18), as do fixture operator unit tests (23), core
unit tests (348) and both public memory-composition tests. Fresh standalone and
six-probe Wasms pass seven installed preparation/recovery cases, two standalone
preflight cases, two inventory cases and all 49 operator integration cases.
Strict affected-package all-target/all-feature Clippy and actual Rust 1.91
native/Wasm checks pass. The initial native HTTP sandbox refusal, test compilation
correction and dependency-identity/lint failures remain alongside their retries.

The final CLI run passes 119 command and five probe tests. Rebuilt native CLI
artifacts pass the seven preparation/recovery and two preflight cases again;
all seven installed publication-session cases pass, including control loss,
changed/partial sources and original-claim recovery. There are 67 distinct
installed cases across the selected groups; repeated runs retain separate logs.

`make package` passes real same-version archive, exact failure-status and final
source/manifest/lock-drift regressions. Fresh coordinated archives pass graph,
all-target/Rustdoc and final integrity qualification in
`target/package/blob-verification.lYAsve`; the regression fixture is retained at
`target/package-fixture.d7uhKp`. These candidate archives retain package version
0.28.0 and do not establish a 0.29.0 publication. `final-inputs.json` binds the
final maintained Rust/Cargo input selection; `lock-edge-review.json` records the
only edge difference from the settled incoming lock. Rustdoc warnings are errors.

The final Host 0.20.1 selection passes affected-package all-target/all-feature
Clippy, actual Rust 1.91 native checks, 119 CLI/five probe tests, 18 Contracts
reply tests and 23 fixture operator unit tests. Fresh artifacts pass seven
installed preparation/recovery, two inventory and six refresh cases. The earlier
67-case installed run remains explicitly scoped to Host 0.20.0. The published
four Host packages have byte-identical `src/**/*.rs` sources and normalized
manifests differing only in package version; hashes and reviewed manifest diffs
are retained in `host0201-source-review.json`. This comparison is source evidence,
not a claim that every earlier test was executed again on 0.20.1.

Fresh exact-graph archive qualification and regressions also pass for 0.20.1:
`target/package/blob-verification.jjRjbr` and `target/package-fixture.YeEA8F`.
The missing-source offline attempt and explicit locked fetch remain retained.
`host0201-inputs.json` supersedes the earlier input checkpoint for the final tree;
final drift, artifact hashes, formatting, dependency declarations and local
reference checks are retained with the command logs.

Full CI, native macOS, delivered Canic integration and deployed-provider
qualification remain separate. No sibling edits, Git publication, release,
registry publication or paid provider effects ran.
