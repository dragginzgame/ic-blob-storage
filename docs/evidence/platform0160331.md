# Published IC Platform 0.1.6 adoption

2026-10-18. Repository-only follow-up at HEAD
`55d70cb0351093b5cc25fcc7b9c27a04e610a1f8`; package versions stay 0.33.0 and the
pending batch stays 0.33.1. Existing Platform/Shared and architectural cleanup
changes are preserved. Starting contents and task evidence are under
`.tmp/review-validation/platform0160331/`, including the incoming diff.

## Registry selection and availability

The canonical sparse index at `https://index.crates.io/ic/-p/ic-platform` selects
**0.1.6**, the newest listed non-yanked publication at observation. The downloaded
registry archive has SHA-256
`4c2a6b9c458b47f2913811998a9f6cc49394deaebb9a11e9d0a1bffb6b169cf2`, matching
its index record. The crates.io JSON API returned HTTP 403; the successful sparse
index/archive reads establish publication independently. Sibling 0.1.7 changelog
notes are unpublished working evidence and are not adopted.

The incoming lockfile already selected 0.1.6 before this task's source edits.
The root requirement moves from 0.1.5 to 0.1.6 so registry consumers cannot resolve
a version missing these exports. The targeted update selects no other package
versions. Lock changes move one existing 0.11.0 management dependency edge from
core to Platform; the upstream package identity stays exact.

## Migration and preserved behavior

Published `management-canister-types` now exports CanisterInfoArgs,
CanisterInfoResult, ChangeDetails and CodeDeploymentMode from exactly upstream
0.11.0. Core enables that Platform feature and
`crates/ic-blob-storage/src/ops/service/recovery/mod.rs` imports the four types
there. Both root/member direct declarations are removed; the existing canonical
boundary policy now forbids direct management declarations in all members.
No copied DTOs, conversions, call wrapper or compatibility re-export is added.

The actual CDK call, 30-second timeout, 20-change horizon, 64 KiB reply admission,
existing decoding/work/type quotas, scope and post-await version checks, change
classification, fences and retained obligations are unchanged. Wire/CBOR and
persisted record schemas, allocation/storage, endpoints and lifecycle ownership
are untouched. This is compatible at Blob's published boundary.

Registry module comparison confirms Candid, Agent, certification and executor
sources are byte-identical between 0.1.5 and 0.1.6. Earlier focused codec/signing
proofs in [the 0.1.5 receipt](platform0150331.md) remain scoped to those unchanged
implementations and their original graphs; they are not new 0.1.6 executions.
New ICRC and raw debug-print capabilities have no new Blob consumer and remain
unselected. Native Agent/parser features remain outside default standalone Wasm.

## Remaining dependencies and enforcement

- Direct CDK and macro-required Candid remain in core/canisters where necessary;
  [Platform #1](https://github.com/dragginzgame/ic-platform/issues/1) retains runtime
  qualification. Handwritten decoders and supported service generators already
  use Platform. Core/Contracts/protocol/CLI have no direct Candid declaration.
- Native dynamic Candid fixtures and Agent response enums retain their exact
  required identities under [#5](https://github.com/dragginzgame/ic-platform/issues/5)
  and [#9](https://github.com/dragginzgame/ic-platform/issues/9). Published 0.1.6
  still does not expose those surfaces.
- Testkit's own native limits identity still requires `ic-testkit-candid` under
  [#2](https://github.com/dragginzgame/ic-platform/issues/2).
- Management 0.11.0 remains transitive via Platform, with Candid 0.10.38;
  standalone retains CDK/macros 0.20.3 and executor 2.0.0. Removing the direct edge
  does not remove these packages or prove their absence from Wasm.
- Shared's direct checker covers aliases, all dependency kinds/targets and disabled
  optional declarations. Runtime-free Contracts/CLI graphs remain separate.
  Automatic transitive test/tool deployment exclusions and final deployed-digest
  binding remain missing consumer admission; this migration does not supply them.

The existing [management-history issue](https://github.com/dragginzgame/ic-platform/issues/12)
owns producer/consumer follow-up. The
[qualified adoption outcome](https://github.com/dragginzgame/ic-platform/issues/12#issuecomment-6248369696)
is posted without closing other consumers' qualification obligations.

## Validation and limits

- Three native continuity-policy tests pass.
- Four installed lifecycle/recovery cases and one snapshot-rollback case pass on
  fresh default standalone Wasm. Tests cover retained physical/billing obligations,
  repeated upgrades, expired history, changed execution during await and snapshot
  refusal without resetting the installation anchor or clearing unsafe fences.
- Offline locked all-target/all-feature native checks pass for core, standalone,
  Contracts and CLI. Strict affected-core/standalone Clippy passes.
- Actual Rust 1.91 native all-target/all-feature core/standalone checks and default
  Wasm checks pass. Fresh optimized Cargo standalone release compilation passes.
- Formatting, Shared canonical dependency boundaries, runtime-free normal graphs,
  pins, snapshot, documentation targets and whitespace checks pass.
- Resolved package/version identities match the incoming lock. The default standalone
  graph selects management DTOs without native Agent/parser, ICRC, raw IC or Platform
  executor features; host/test packages are absent from that inspected graph.

Fresh standalone SHA-256: `b77117a0fc6e72974b9a0420a5d34d4aa9129d7ab5143f6914e55093ae05d984`.
The source/lock/selected-artifact evidence is local build/install proof, not an
attestation of deployed bytes. The initial registry API 403 used a successful
independent canonical index/archive fallback; no selected build/test failed.

No full CI, native macOS, browser suite, published archive or deployed IC/provider/
Canic qualification is claimed. No sibling edits, package version changes, Git
publication, release, deployment or paid provider effect occurred. Registry and
GitHub operations are bounded reads and the relevant owner issue outcome.

## Task metrics

Rust physical lines: +1 / -1 / +0.
Type declarations: created 0; deleted 0. Type physical lines: +0 / -0 / 0.
Documentation physical lines: +153 / -29 / +124.

Task-only comparison uses the preserved starting dirty contents, including the
existing untracked policy. Snapshots, registry sources, lockfiles and raw artifacts
are excluded. Rustdoc overlaps documentation/Rust; type lines are a Rust subset.
