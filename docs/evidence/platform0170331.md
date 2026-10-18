# Published IC Platform 0.1.7 adoption

2026-10-18. Follow-up over released HEAD
`55d70cb0351093b5cc25fcc7b9c27a04e610a1f8`, preserving the existing dirty Platform,
Shared and ownership-cleanup batch. Workspace versions remain 0.33.0; compatible
notes remain pending 0.33.1. Raw before-images, incoming diff, registry evidence,
commands, outputs and task delta: `.tmp/review-validation/platform0170331/`.

## Published selection

The canonical sparse index lists **0.1.7** as its newest non-yanked release at
observation. The static registry archive SHA-256 matches its index record:
`84225cc3f79736891da8751103a19dab8d74fd1f15e40d04e61a25ee3c974257`.
The root minimum and lock move from 0.1.6 to 0.1.7. No sibling checkout or path
patch supplies the dependency; unpublished 0.1.8 notes are not adopted.

## Migration and retained contracts

- CLI signed updates import `CallResponse` from `ic_platform::agent`.
  Headless certificate evidence imports `TransportCallResponse` there too.
  Root/CLI/native PocketIC Agent declarations are removed. The canonical policy
  forbids direct Agent dependencies in every member and dependency kind/target.
- Native refusal fixtures import `IDLValue`, `IDLField`, `TypeSerialize` and
  `ValueSerializer` from Platform's native `candid_parser` module. The existing
  native dev parser feature supplies the exact upstream identities. PocketIC's
  direct Candid declaration and exception are removed. Production Candid-only
  builds do not select the host parser/dynamic fixture module.
- Agent remains 0.49.2 with the same upstream response variants, signed requests,
  request IDs, certificate verification, caller-selected roots, byte/deadline
  limits and no-poll/no-retry decisions. Fixture dynamic values and nested/wide
  type serialization retain original encodings and rejection budgets. No local
  facade, conversion, serializer, transport or decoder-limit policy is introduced.
- Controlled application packets stay on Wire/CBOR; provider and persisted
  encodings stay with their existing owners. No memory/storage, public API,
  lifecycle, callback, executor or recovery-state migration occurs. The existing
  low-level default-configuration calls remain supported upstream identities;
  new default decoder exports do not require unrelated call-site churn.

## Remaining boundaries and issue disposition

Direct Candid remains only in standalone and six probe canisters for their
current typed/default CDK expansions. Core plus those seven canisters retain
CDK for actual call/system/entrypoint/lifecycle behavior. Platform documents a raw
zero-argument/manual-reply/hidden macro subset, but that does not replace these
maintained typed interfaces or the qualified runtime. The unsupported full runtime
is already [#1](https://github.com/dragginzgame/ic-platform/issues/1).
Published Testkit 0.53's helper limits/error identity still requires native
`ic-testkit-candid`, already [#2](https://github.com/dragginzgame/ic-platform/issues/2).
No conversion or replacement simulator transport is added to hide that dependency.

Existing [#5](https://github.com/dragginzgame/ic-platform/issues/5) and
[#9](https://github.com/dragginzgame/ic-platform/issues/9) own the two gaps resolved
by this release. All Platform issues were searched first. No newly uncovered API
or documentation gap justifies a duplicate issue. Consumer qualification outcomes
are reported on [#5](https://github.com/dragginzgame/ic-platform/issues/5#issuecomment-6248617509)
and [#9](https://github.com/dragginzgame/ic-platform/issues/9#issuecomment-6248617913);
their state and other consumers' acceptance remain with the upstream owner.

Candid 0.10.38, CDK/macros 0.20.3 and executor 2.0.0 remain in the default
standalone graph; Agent/Candid remain transitive in native transport. Direct
management types stay forbidden; exact management 0.11.0 is reached through
Platform. Direct declaration removal is not a claim of binary dependency absence.
Shared's canonical checker covers aliases, all dependency kinds/targets and
inactive optional declarations. Automatic transitive deployment exclusions and
source/lock/target/features-to-deployed-digest binding remain missing consumer
admission, as recorded in the dependency guide.

## Validation

- All native CLI/PocketIC targets/features compile; strict affected-owner Clippy
  passes. Five signed-update native unit tests pass against the final manifest.
- Five unique installed PocketIC cases pass: provider certificate dynamic refusal,
  admission work/type/header refusal, two headless certificate proof/recovery cases
  and the actual native account CLI journey. Final CLI compilation/provenance and
  its installed journey are repeated after manifest-order normalization.
- Fresh standalone, storage/admission probes and gateway-source release Wasms build.
  Actual Rust 1.91 native CLI/PocketIC targets/features and core/Contracts/standalone
  default Wasm checks pass.
- Shared direct-boundary, runtime-free graphs, dependency pins, snapshot integrity,
  canonical formatting, documentation targets and whitespace checks pass.
- Locked package identities change only for Platform 0.1.6 to 0.1.7. Published
  Platform dependency/feature declarations are unchanged. Default standalone
  normal/build/feature inspection excludes native parser/Agent and host/test
  packages. Registry archive, selected artifact hashes and source-input evidence
  are retained with the command logs.

The first targeted update refused a task-local manifest-edit typo joining two
dependency names; its log is retained. Restoring the two original declarations
allowed the targeted update/fetch and subsequent checks. Initial Rust formatting
refusal was corrected using the selected formatter. Cargo-sort 2.1.4 subsequently
moved the target dependency table one intervening item per write; placing it with
the other dependency tables produces a passing canonical check. CLI source evidence
is rebuilt against that final manifest. The first native CLI selector executed
zero tests and was refused; its corrected selector then lacked the gateway fixture
path. Both attempts remain with the successful fresh-fixture retry. No Platform
issue is inferred from these editing, selection or setup failures. Retired-image tests remain opt-in and require
their pinned artifacts; native compilation alone does not qualify those runs.

No full CI, native macOS, Chromium, package publication or deployed-provider/Canic
qualification is claimed. No sibling edits, commits, pushes, release, deployment
or paid effects ran. GitHub issue comments are the explicitly authorized feedback.

## Task metrics

Rust physical lines: +9 / -10 / -1.
Type declarations: created 0; deleted 0. Type physical lines: +0 / -0 / 0.
Documentation physical lines: +157 / -36 / +121.

Task-only delta is measured against retained starting dirty contents. Snapshot
copies, registry sources, lockfiles and raw evidence are excluded. Rustdoc
belongs to both Rust and docs; type lines are a Rust subset.
