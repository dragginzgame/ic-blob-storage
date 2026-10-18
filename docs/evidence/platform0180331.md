# Published IC Platform 0.1.8 adoption

2026-10-18; HEAD `55d70cb0351093b5cc25fcc7b9c27a04e610a1f8` (released 0.33.0).
Existing dirty Platform/Shared/ownership work is preserved. Workspace versions
remain 0.33.0 and compatible notes remain pending 0.33.1. Starting content,
registry archives/index, comparison, logs and final delta are retained under
`.tmp/review-validation/platform0180331/`.

The newest non-yanked version in the canonical Cargo sparse index at observation
is **0.1.8**. Its published archive SHA-256 matches the index:
`4a3adb6d9785133587b854b66b37dfdd8f7945555bdc59b6d0be26995187484d`.
Root minimum and lock move from 0.1.7 to 0.1.8; no path patch or unpublished
sibling source is selected. All other resolved package versions remain unchanged.

## API assessment and boundaries

Published-source comparison against 0.1.7 changes only root module documentation
and additional management snapshot/status re-exports. Feature/dependency
selections are identical. All Blob-used runtime, history, Candid, Agent, parser,
certification and executor identities/implementations remain unchanged. History
still uses exact management 0.11.0 through Platform. New snapshot/status DTOs
serve [the existing Backup issue](https://github.com/dragginzgame/ic-platform/issues/13);
Blob needs no new caller or feature. Testkit continues owning local simulator
snapshot operations. No snapshot execution, recovery permission, memory/storage
migration or local implementation is introduced to consume otherwise unused APIs.

The previous [0.1.7 migration](platform0170331.md) already removed direct Agent
and native PocketIC Candid. Direct Candid remains only in seven current canisters
for their typed/default CDK macros. Core and seven canisters retain direct CDK;
native Testkit helper limits retain their distinct identity. Existing
[Platform #1](https://github.com/dragginzgame/ic-platform/issues/1) and
[#2](https://github.com/dragginzgame/ic-platform/issues/2) cover those unsupported
boundaries. All Platform issues were searched before reporting: no newly exposed
API/behavior/documentation gap requires another issue or repetitive outcome comment.
No existing issue is closed or consumer acceptance inferred from publication.

Candid 0.10.38, CDK/macros 0.20.3, one executor 2.0.0 and management 0.11.0 remain
transitive in default standalone; Agent/Candid remain transitive in native tools.
Public contracts, request bytes, decoder limits/errors, typed response matching,
callbacks/traps, intent journals, lifecycle and restore fences are unchanged.
Shared's canonical direct policy still covers aliases, normal/build/dev/target
and inactive optional declarations. Automatic transitive deployment exclusions
and deployed-digest binding remain missing consumer admission; these declaration
checks and a local graph observation do not supply it.

## Focused validation

- Native core/Contracts/standalone/CLI targets and features compile offline/locked.
- Three native continuity-policy cases pass. Two installed PocketIC cases pass
  on fresh standalone Wasm: management change during the history await and old
  snapshot rollback refusing current-instance recovery without clearing fences.
- Actual Rust 1.91 native core/standalone/CLI targets/features and default
  core/Contracts/standalone Wasm checks pass. Strict affected-owner Clippy passes.
- Fresh default standalone release Wasm builds. Canonical formatting, direct
  dependency boundary, runtime-free graphs, dependency pins, snapshot and local
  documentation links pass. No failed build/test is hidden by this qualification.
- Graph inspection keeps direct Agent/management/substrate forbidden and direct
  Candid confined to seven canisters. Default standalone normal/build/features
  exclude native parser/Agent and host/test packages. Source/lock/artifact hashes
  are retained locally; they do not attest to deployed bytes.

Earlier codec/Agent/dynamic-fixture results retain their original 0.1.7 source
identity. Their implementation files and dependency features are unchanged in
0.1.8; they are comparison evidence, not new test executions. No full CI, native
macOS, browser suite, package publication, live IC/provider or installed Canic
qualification is claimed. No sibling source edits, Git actions, release,
deployment, paid effect or issue mutation occurred.

## Task metrics

Rust physical lines: +0 / -0 / 0.
Type declarations: created 0; deleted 0. Type physical lines: +0 / -0 / 0.
Documentation physical lines: +96 / -14 / +82.

Task-only comparison uses retained starting dirty contents. Snapshots, registry
sources, lockfiles and raw evidence are excluded. No authored Rust changes occur.
