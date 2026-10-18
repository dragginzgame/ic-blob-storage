# IC Platform adoption

Date: 2026-10-18. Starting Blob source: `55d70cb` (0.33.0), with the pending
Shared Tooling adoption and incoming dependency edits preserved. Package versions
remain 0.33.0; notes are pending 0.33.1. Only this repository was edited.

## Published selection and scope

Selected **ic-platform 0.1.4**, the latest non-yanked stable release observed in
crates.io metadata. Its registry archive checksum is
`174b06835875ba583f2ac542cf12edc3967b68fad717fd4ad9e85d9e8630b627`.
Archive verification and actual source/README review, not sibling availability,
establish the adopted API. Initial qualification used published 0.1.3; 0.1.4 was
published during the work and adds native parser APIs. Its Candid module is
byte-identical to 0.1.3. Earlier checks retain their original inputs and artifacts.

Platform has no default features. Principal is unconditional. Members select
`candid` for external types, encoding and finite bounded decoders. Native test-only
`candid-parser` supplies service equality and textual argument parsing; its module
and dependency are absent on Wasm. Platform's `executor` and `certification` are
not enabled by this adoption. The isolated 0.1.4 executor-only graph has no Candid
or full CDK and uses executor 2.0.0.

The audit covers the virtual root, all twelve member manifests and their normal,
build, dev, target, optional and feature declarations, public types, Rust macros,
examples, test support, scripts and browser callers. There are no other tracked
Rust workspaces or lockfiles. Browser JavaScript retains its SDK Candid interface
for external provider certificates; this Rust package supplies no JavaScript API.
IC Wire and the existing application CBOR surface remain unchanged.

## Changes and remaining owners

- Principal uses Platform's unconditional root export. DTO/record derives name
  the published contract and its `candid_path`, including generated record types;
  encoding uses its upstream functions. Field labels, arbitrary-precision types,
  serialized records and layout are unchanged. No memory/storage migration ran.
- Platform replaces the independent operator decoder's Testkit implementation.
  Byte/work/skipping/type/header ceilings and domain-error projections remain
  exact. Recovery history and storage-probe ingress use its finite decoder with
  the original limits and existing erased errors/traps. CLI historical vectors
  use explicit test-only limits rather than a direct upstream decoder.
- Remove the CLI's entire direct Candid dependency and the root/member parser
  declarations. PocketIC Candid and Testkit decoder declarations become native
  dev-only. No alias, compatibility re-export or replacement transport was added.
- Shared 0.24's canonical dependency-boundary checker is now selected through the
  exporter (137 snapshot payloads). `make platform-boundary-check` runs in CI and
  release validation against the guarded local exception policy.

Remaining direct Candid consumers are core, Contracts, standalone, six probes,
protocol support and native PocketIC tests. Core/Contracts retain the public
upstream DecoderConfig identity and optional upstream-default-header policy;
records/probe journals retain their existing low-level decoding/diagnostics.
The admission probe retains separately measured construction/value/finalization
stages. Native dynamic-value/refusal and existing decoder-oracle fixtures also
retain upstream decoding. No defaults, measurement stages or typed errors were
recreated locally to produce a cleaner manifest.

CDK remains direct in core, standalone and six probes: Platform has no entrypoint,
call/system API or service-export facade. Original endpoints, raw calls, cycles,
refunds, contexts, callback traps and synchronous restore remain with those APIs.
Testkit 0.53 still requires `ic_testkit_candid::DecodeLimits` for simulator calls;
that native dev dependency cannot adopt Platform's distinct type directly.

Blob-specific evidence was posted to existing Platform issues
[runtime/service export #1](https://github.com/dragginzgame/ic-platform/issues/1),
[Testkit identity #2](https://github.com/dragginzgame/ic-platform/issues/2) and
[staged/configuration policy #3](https://github.com/dragginzgame/ic-platform/issues/3).
[Parser #6](https://github.com/dragginzgame/ic-platform/issues/6) recorded the
0.1.3 gap; the published 0.1.4 feature is now adopted and qualified here.

## Enforcement and graphs

Shared's checker reads canonical package identities, so renames do not bypass
policy. It observes all direct declaration kinds, targets and disabled optional
features. Its exceptions permit packages, not individual uses, kinds or features.
Direct parser, executor, CDK macros/bindgen/timers and Candid derive packages are
forbidden; the necessary Candid/CDK/Testkit exceptions remain explicit.

Resolved standalone and native package-verification CLI normal/build graphs
exclude parser, Testkit/PocketIC, Metrics and Host tools. Standalone still reaches
Candid 0.10.38, CDK/macros 0.20.3 and one executor 2.0.0 identity. CLI reaches Candid
through Contracts/Platform and Agent. Host/test graphs retain their upstream
parser and Testkit dependencies. Neither direct cleanup nor graph inspection
proves those dependencies absent from all binary bytes or from all feature sets.

Automatic transitive deployment exclusions and source/lock/target/features to
final deployed-digest binding remain missing at the consuming deployment owner.
The new direct-policy gate does not replace that admission. Blob has no deployment
command. Normal/build graph inspection and local artifact hashes are scoped proof.

## Validation and retained evidence

Final selected-graph checks pass: 348 core and 118 Contracts units, 126 CLI/probe
cases, 25 independent operator cases, five billing vectors and 21 example tests.
Strict affected-library/CLI/test and canister/probe Clippy passes. Actual Rust
1.91 native all-target/features and Wasm checks pass. Fresh standalone/probe Wasms
and CLI are built against 0.1.4. Nine selected installed PocketIC cases pass:
interface/carrier, ingress refusals, recovery during an await, callback trap/refund,
receiver upgrade retention and the native account parser CLI journey. These use
local substitutes and the admitted Testkit 16.1 server, not a live provider.

Raw before-images, registry archives/metadata, issue payloads, source identities,
manifest inventories, graphs, input/artifact hashes and logs remain under
`.tmp/review-validation/platform0331/`; final runtime results are under `final/`.
Incoming Host 0.27.2/arc-swap 1.9.3 edits were preserved and final checks restarted;
the maintainer confirmed that graph. A later incoming Build 0.17.1 lock edit is
preserved separately; `late-*` logs qualify its affected CLI callers and retain
the new input checkpoint. Earlier installed results keep their original Build
0.17.0 lock/artifacts; unchanged standalone production code is not relabelled.
Build's 0.17.0/0.17.1 `src` trees match. CLI MSRV, strict lint, all 126 cases,
fresh build and the installed native parser/account journey pass on 0.17.1.
Earlier socket sandbox refusals, the
short-lived manifest-edit error, Clippy path-wrapping failures, incomplete-lock
attempts and zero-match parser selector remain retained beside successful retries.

Formatting, hooks, release routing, snapshot/pins, direct/runtime boundaries,
documentation links and whitespace checks pass. Native macOS, complete CI,
publication/archive qualification, downstream Canic, live provider and actual
installed-IC qualification were not run. No commit, push, release, publication,
deployment, sibling source edit or paid effect ran.
