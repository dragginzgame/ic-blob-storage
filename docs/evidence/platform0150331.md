# Platform 0.1.5 dependency routing

Date: 2026-10-18. Starting Blob source: `55d70cb` plus the existing pending
Shared/Platform changes, captured before this follow-up. Package versions remain
0.33.0 and notes remain pending 0.33.1. Sibling repositories were not edited.

## Published contract and migration

Registry metadata confirms non-yanked IC Platform 0.1.5; its authenticated archive
has SHA-256 `ad5262d2a30adfd89b55960da7c9faffb5a68b7cab4c219af5958f1ab2ce8fe4`.
The lock already selected it; this task raises the declared minimum from 0.1.4.
Published source, not the sibling draft, supplies the adopted capabilities.
No external version/source selections change; lock edits update local edges and
Platform's existing optional Agent/certification reachability. Incoming Host
0.27.2, Build 0.17.1, SHA2 0.11.1, Memory and Storage selections are preserved.

Platform's raw upstream DecoderConfig, IDLDeserialize and configured decoders now
own handwritten low-level Candid configuration and staged decoding. Work/skipping/type/header ceilings,
optional header defaults, full/compact diagnostics, construction/value/done/drop
measurement points and typed error identities remain unchanged. Previously
unconfigured fixture decoders pass an upstream default config, without choosing
new bounds. No wrapper, copied codec or compatibility alias is introduced.

Core's record-generating macro and standalone endpoint/service-export generator
now name Platform's published paths. DTOs, record bytes and layout do not change.
Core, Contracts and protocol support remove direct Candid; CLI already removed it
in the preceding adoption. The direct-policy allowance shrinks accordingly.

All supported native Agent, identity, signing and certificate types/functions
route through Platform. The native CLI selects `agent`; native test-only support
selects `agent`, `candid-parser` and `certification`. Consumer HTTP middleware,
root trust, certificate checks, byte limits, deadlines, no-resubmission behavior
and uncertainty journals remain local and unchanged. No native feature is selected
in default standalone Wasm. The existing CDK executor/callback model is untouched.

## Remaining direct dependencies and upstream feedback

- Candid: standalone and six probes need CDK-generated absolute paths; native
  PocketIC fixtures still need IDLValue/IDLField and TypeSerialize/ValueSerializer.
  Other reflection and parser operations migrate independently. Blob evidence is
  posted to [Platform #5](https://github.com/dragginzgame/ic-platform/issues/5).
- CDK: core and seven canisters still use its call/system/entrypoint/lifecycle
  contracts. Platform 0.1.5 deliberately has no full runtime facade
  ([#1](https://github.com/dragginzgame/ic-platform/issues/1)). Raw ic0 is not used
  to recreate that missing contract; it would not remove these dependencies.
- Agent: CLI's CallResponse::{Poll,Response} and the certified native fixture's
  TransportCallResponse remain absent from the published exports. Their concrete
  consumers and exact pending/immediate-reply semantics are recorded on
  [#9](https://github.com/dragginzgame/ic-platform/issues/9).
- Management DTOs: recovery uses CanisterInfoArgs, CanisterInfoResult, ChangeDetails
  and CodeDeploymentMode. Platform exposes HTTP outcall DTOs only; newly filed
  [#12](https://github.com/dragginzgame/ic-platform/issues/12) requests these exact
  upstream history identities with continuity/refusal acceptance checks.
- Testkit decoder: native dev calls need Testkit 0.53's own limits identity until
  its published migration ([#2](https://github.com/dragginzgame/ic-platform/issues/2)).

The canonical direct checker enforces package identities including aliases across
normal/build/dev/target/optional declarations. Exceptions are still package-level;
it does not whitelist individual import uses or enforce feature isolation.
Transitive graph inspection is separate from binary-content/deployment admission.
Candid/CDK and one executor still exist in the canister graph. Native Agent and
its Candid dependency still exist in the CLI graph. No reduction in artifact size
or instructions is claimed. Automatic transitive deployment exclusion and exact
source/lock/target/features-to-deployed-digest admission remain missing.

## Qualification

Raw before-images, issue payloads, authenticated-release context, declaration
inventory, graphs, source identities, checkpoints, artifacts and logs remain under
`.tmp/review-validation/platform0150331/`. The first scripted edit briefly matched
an already-qualified decoder name; those five source files were restored from the
captured before-images and the migration rerun with a qualified-path guard before
compiled qualification. No unrelated pending work was reverted.

Native/Wasm compilation, actual Rust 1.91 all-target/feature checks, strict library,
CLI/test and canister/probe Clippy pass. Units, frozen vectors, examples and
doctests pass; fresh default standalone, six probe Wasms and CLI build successfully.
Thirteen selected installed PocketIC cases pass with authenticated local server
and artifact hashes: provider interface/carrier, Wire/CBOR ingress refusals,
recovery during an await, callback trap/refund, receiver upgrade retention, native
Agent account observations, two certificate authentication/recovery journeys,
staged admission refusals and bounded content verification. These are local
substitute results; artifact/input checksums remain unchanged through the run.

Full CI, native macOS, actual installed IC, live provider and downstream Canic
qualification remain separate. No memory/storage migration, version bump, staging,
commit, push, release, publication, deployment or paid effect ran.
