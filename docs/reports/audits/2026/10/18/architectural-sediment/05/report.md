# Architectural sediment — post-Platform sanity pass

2026-10-18. **PASS WITH FINDINGS** within this source-led ownership review.
HEAD is released 0.33.1, `67b20989ebdb555e9fac6022685be51ba866fb4a`.
Incoming Platform 0.1.11, Shared test-root retirement and Rust 1.99.1 edits are
preserved; before-images distinguish this task from their accumulated diff.
Package versions, lock selection and reviewed snapshot are unchanged by this pass.

Method: `audits/flow-convergence-and-duplication.md`, authorized cleanup under
`audits/module-cleanup.md`, and the IC application addendum, at Shared 0.24.0
revision `cab12cd3dfa4fdb34846b25397f0d72881bb9ae0`. Local authorities are
`AGENTS.md` and `docs/governance/development.md`. Scope covers manifest/import
boundaries, core/Contracts/standalone entrypoints, CLI and browser preparation,
probe persistence and simulated operator commands, generators, CI/distribution
adapters, historical cuts and current guidance. This is not a complete correctness
or provider/deployment qualification audit. No sibling file or Git ref is edited.

## Verified ownership and consumers

- Core owns model invariants, policy, durable owners and workflow orchestration.
  Standalone's explicit `api` calls the shared `ServiceWorkflow<Host>`; the host
  retains caller authentication, endpoint registration, lifecycle and memory custody.
  Linking core mounts nothing. Contracts owns passive DTOs, method catalog,
  application codecs and content verification. The generated catalog is the
  endpoint/interface source of truth, not competing dispatch implementation.
- The native `blob-storage` command owns offline preparation, signed operations
  and retained evidence. `caffeine-probe` owns bounded explicit observation;
  `blob-package-verify` delegates package mechanics to Build. Browser certificate,
  transfer, publication and worker entrypoints retain application handoff and
  durable intents while Caffeine's SDK owns provider upload mechanics.
- Published Platform 0.1.11 owns supported Candid/CDK/native platform boundaries;
  Memory 0.48.2 owns allocation/backing custody, Storage 0.5.3 collections/codecs,
  Host 0.27.4 filesystem/process/artifact mechanics, Build 0.17.3 source/package
  evidence, and Testkit 0.54.0 local IC transport/server lifetime. Wire 0.4.1 owns
  tenant framing; the other owned CBOR methods remain their supported protocol.
  [The dependency guide](../../../../../../../dependencies.md) names these relationships.

Sibling sources are read-only supplementary evidence, not published API promises.
Reviewed heads include Canic `5583496bfa95069809249e8e5d16c77f1ffd744f`, Host
`bcc37e7afc1e831e7e8b41e2e11664f3ed873bf3`, Memory
`7594a26d09beb2e1524a0217aa357ae603aa7d0b`, Storage
`249da24992da229464b809d44ffed7f5ab8d4bae` and Platform
`ffc36b6363a1c87a4f1f4e88ee14b0ddf55db167`. Several checkouts are dirty;
full identities/status for these and Build/Testkit/Wire/Toko are retained in the
raw packet. Canic still has host-specific dispatch in its ordinary checkout;
that is not evidence that the existing draft adoption has landed or qualified.
Toko's older dependency catalog does not prove current installed Blob integration.

## Findings and completed cleanup

1. **LOW — executable guidance retained the old ingress contract.** The current
   `blob-storage --help` still called the JSON configuration proposal and saved
   CBOR inputs Candid. `clients/browser/README.md` likewise required nonexistent
   `installation.candid` and described saved permission bytes as Candid, although
   native preparation and the launcher consume CBOR. It also assigned manifest
   preparation to a removed core module instead of Contracts' maintained
   `provider::preparation`. History `aa0f133` establishes the 0.32 codec cut; current
   installation producer, exact CBOR readers and Contracts method catalog verify
   actual behavior. Correct the help formats and browser owner/filename; name the
   JSON installation test input accordingly. CLI flags, accepted bytes, errors,
   persisted evidence, private fixture controls and external provider Candid remain unchanged. The producer
   remains canonical; no compatibility reader or duplicate conversion is added.
2. **LOW — an obsolete path assertion survived model relocation.** Core's root
   Rustdoc had a compile-fail import of `model::service::upload::UploadAdmissions`,
   a path removed during the 0.28 probe isolation (`42ec500`). It did not exercise
   the actual feature-selected `test_support::admission` owner. Delete that
   historical-name assertion. Keep documentation of probe-only scope and the
   actual private store-construction, installed-owner field and evidence-taking
   certificate/exposure boundaries. The four remaining boundary doctests pass.

No additional production implementation deletion is justified by this pass.
The cleanup changes guidance and removes one obsolete doctest, not service state
machines, application protocols or supported public APIs.

## Necessary separation and unresolved work

Retained-file adapters preserve failed/partial evidence and exact operation
claims; ordinary atomic output has different failure semantics. Probe journals
have separate bounded schemas and retained-state obligations. The allocation
observer/fault wrapper protects attribution unavailable through generic backing
instrumentation; retain it pending a qualified owner API, as tracked in
[Memory #101](https://github.com/dragginzgame/ic-memory/issues/101).
Local PocketIC operator tools simulate callers and private probe methods; they
cannot replace authenticated production CLI commands. Frozen Candid vectors,
pre-cut images and historical reports remain independent evidence, not readers
or currently supported installation recipes. Public Memory re-export preserves
active host type identity and is not an expired compatibility alias.

Keep the supported CBOR contract and current application DTO traits until the
coordinated cut in [#84](https://github.com/dragginzgame/ic-blob-storage/issues/84).
Finish Canic's adoption in its owner via existing
[draft #603](https://github.com/dragginzgame/canic/pull/603); do not import framework
dependencies or duplicate qualification here. The known transitive deployment
admission gap remains described in current status; this library audit does not
create a deployment subsystem. No new ownership decision blocks these cleanups.

## Verification and limits

Compiled before/after CLI help confirms the corrected format guidance. Three
offline installation tests pass actual JSON proposal preservation, CBOR output,
binding/resource refusals and malformed/ambiguous inputs. All four remaining core
boundary doctests pass. Focused formatting, dependency/runtime-free boundaries,
snapshot/link and unchanged-input checks accompany the raw packet at
`.tmp/review-validation/sediment0332-05/`.

No failure was encountered in these focused checks. No runtime implementation,
manifest or dependency selection changes, so no new Wasm/installed/browser suite
is claimed; prior Platform receipts retain their exact graphs and results.
Native macOS, full CI, live provider/IC and installed Canic acceptance remain
outside this pass. No release, publication, deployment or paid probe ran.

## Task metrics

Rust physical lines: +20 / -24 / -4.
Type declarations: created 0; deleted 0. Type lines: +0 / -0 / 0.
Documentation physical lines: +134 / -8 / +126.
Task-only before-image comparison excludes incoming work, generated output and
snapshots. Rustdoc overlaps Rust/documentation; type lines are a Rust subset.
