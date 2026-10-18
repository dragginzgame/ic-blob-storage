# Architectural sediment audit

## Scope, identities and verdict

2026-10-18; source inspection followed by authorized local cleanup. Start:
clean `e938a1da7b4be94f21d6e6b24021a13c2855a6af` (subject `0.33.0`). Actual package
metadata remains 0.32.2; the pending ledger is 0.33.0. A commit subject is not a
release receipt. No package versions, lockfile, Git refs or release effects change.

Methods: `audits/flow-convergence-and-duplication.md`,
`audits/complexity-and-technical-debt.md` and authorized `audits/module-cleanup.md`,
under Shared 0.23.0 revision `0b351cd2ba2bcd8927b783a5fec89afc8013f708`.
Local overlay: root `AGENTS.md` and `docs/governance/development.md` at the starting
commit. Scope covers manifests/imports, library and host entrypoints, native and
installed fixtures, browser handoff, build/release adapters and current guides.
This is an ownership review, not exhaustive provider/security qualification or a
comparable runtime benchmark. Verdict: **PASS WITH FINDINGS**; justified local
repairs pass focused checks, and probe instrumentation still needs an upstream
capability before its extra manager can be removed.

Raw packet: `.tmp/review-validation/sediment0330/` retains source/input hashes,
commands, outputs, dependency metadata, read-only issue evidence and final diff.
Historical reports and frozen provider/trial artifacts retain their original owners.

## Ownership and entry-to-result trace

| Capability | Canonical owner and evidence | Entry and consumers |
| --- | --- | --- |
| Service semantics and records | Blob core `model`, `policy`, `ops`, `workflow`; core has no Canic dependency | Host endpoints capture caller/context, invoke workflow, then policy/ops and durable owners; installation/restoration uses `ServiceInstallation` |
| Passive contracts and content verification | `crates/ic-blob-storage-contracts/src/lib.rs`, `endpoints/mod.rs`, `identity`, `wire` | Clients and host emitters share one method catalog; linking Contracts mounts nothing |
| Endpoint/lifecycle composition | `canisters/standalone/src/api/mod.rs`, `ops/memory/mod.rs`; downstream frameworks own their wrappers | Explicit mounting and synchronous restoration; one host runtime and seventeen grants |
| Allocation and collections | Published Memory 0.48.1 and Storage 0.5.2; core manifests/imports and `tests/memory_composition.rs` | Host bootstrap issues exclusive `RuntimeMemory<Backing>`; Storage owns collection mechanics; Blob owns schema, quotas and accounting |
| Codec framing | Published Wire 0.4.0 plus Contracts' Ciborium boundary; actual standalone emitter and method catalog | Two tenant methods use Wire; other 29 owned methods/lifecycle use CBOR; provider certificate and external calls remain Candid |
| Native infrastructure | Host 0.27.1; CLI readers, journals, publication control and package verifier | Host supplies filesystem/process/artifact mechanics; Blob retains exact requests, bounds, typed mapping and interruption interpretation |
| Build provenance | Build 0.16.0 and CLI build script | Source fingerprint and optional archive verifier; not a public-library runtime dependency |
| Local IC qualification | Testkit/Candid 0.53.0 and PocketIC 16.1.0 | Managed server, native decoding and installed tests; Blob probes model provider substitutes and service-specific failure cases |
| Common tooling | Reviewed Shared snapshot, `make/consumer.mk`, snapshot manifest | Common install/format/hook/release engines; Blob selects product inputs, publication ordering and validation/evidence adapters |
| Application composition | Canic `integrations/blob-service`, dedicated and embedded consumers | Application owns wrapper/lifecycle and media-map transaction tests outside this repository |

Published registry sources, rather than mutable siblings, establish adopted API
availability. Sibling source inspection is supplementary. Reviewed identities:
Memory `f2abbdac950379677259ac9311ce5cb9984b7aa0`, Storage
`009412d62f72844016ff5825536a88c715fedab9`, Host
`c8cd434586c7cde97677938fbdcf8fb4bff4a54c`, Testkit
`55ac3974c7d3c24571a78a538f91738782f11bfd`, Wire
`ce56d5eba2e3b683073a7978547837ebb9974215`, Canic
`5583496bfa95069809249e8e5d16c77f1ffd744f`. Memory, Storage, Host and Canic have
uncommitted changes; observations of those files are dirty-source evidence, not
qualification of their committed or published versions. No sibling files change.
Canic's observed catalog selects Blob 0.32 and Memory 0.48, and its adapter still
uses runtime-handle owner annotations. That does not qualify pending Blob 0.33's
backing-type API. No Canic build or installed application test ran here.

## Findings and implementation

1. **LOW — fixture custody still masqueraded as backing.** The latest Storage
   adoption (`e938a1d`) introduced `test_support/memory/mod.rs` with an optional
   recursive runtime and a forwarding implementation of Memory. Those mechanics
   were solely needed to fit fixture owners into memory-bundle trait bounds.
   Memory's published fixture bootstrap already owns allocation admission.
   The local owner now holds VectorMemory plus its retained runtime directly;
   stores use VectorMemory as their backing parameter. Passive bundles no longer
   require their payload to implement Memory; actual store constructors still
   require exclusive runtime leases. This removes the fake backing interface and
   optional owner state, without moving Blob corruption/recovery assertions out
   of their service tests. All affected native service fixtures use this owner.
   Existing no-repair byte comparisons, drop-before-reopen, independent restored
   images and late-result fencing remain. This changes no persisted record or
   production access path and claims no performance improvement.
2. **LOW — superseded instructions competed with current owners.** The protocol
   guide, Contracts/standalone READMEs and wire Rustdoc still called Wire unpublished
   or described a not-yet-released 0.32 conversion. The dependency guide said Host
   root-relative reads were awaiting publication while current callers already
   used them; README/Canic instructions selected obsolete Blob/Memory families.
   History (`aa0f133` codec cut, later caller/adoption commits and current manifests)
   confirms these are stale instructions, not evidence that CBOR was retired.
   Replace them with the actual per-method surface, current locked owner table and
   framework-independent integration recipe. Remove adoption chronology from the
   current setup section; original receipts and Git history remain intact.
   Native tools, browser opaque packets, lifecycle and external Candid continue
   using their existing codecs. No unsupported conversion is performed.
3. **LOW — boundary proof could fail for the wrong reason.** The core root
   compile-fail example referenced RuntimeMemory without importing it after the
   storage cut. Import it so the example reaches the intended private component
   constructor restriction. All five documentation boundary tests still pass.
4. **LOW — probe observation cannot yet converge without losing attribution.**
   `canisters/test/storage_probe/src/ops/mod.rs` inserts per-allocation read/write
   fault wrappers under nested fixture runtimes because Storage requires exact
   RuntimeMemory handles. This is fixture-only; normal standalone has one manager.
   Published Memory and the inspected sibling provide no allocation observer hook.
   Generic backing instrumentation is a different boundary: it cannot identify a
   service allocation inside the manager. Related producer work is recorded in
   [Memory #101 — Canonical Testkit memory IO instrumentation](https://github.com/dragginzgame/ic-memory/issues/101);
   it is not proof that allocation-level hooks are available. Retain this probe
   and its disclosed measurement overhead. Recommend a separately scoped owner
   decision for opt-in allocation observation/fault injection in Memory, followed
   by Blob adoption and fresh rollback/resource profiles. No sibling API or local
   replacement framework is introduced by this audit.

Rehearsals: changing fixture backing now changes its concrete backing and runtime,
not a recursive owner/Memory implementation; changing codec framing still follows
the catalog and each current client boundary; changing provider or Host failure
mapping stays with Blob's relevant operation rather than a new universal wrapper.
Only fixture state space is reduced; production state machines are unchanged.

## Intentional retention

- CBOR, Candid, Serde and CDK have active owned/provider/persisted/platform callers.
  The mixed protocol is explicit and contains no codec-detection fallback.
- The public Memory re-export preserves a single compatible host type identity;
  it does not bootstrap a runtime. Probe-only facades still have maintained
  consumers and remain behind the non-default test-support boundary.
- Provider DTOs and byte/work/authority checks translate an external contract.
  Native client and installed-probe validation are independent trust boundaries.
  Similar decoding/error-handling shapes alone do not justify merging them.
- Probe journals, protocol DTOs and tests model Blob-specific failure/restore
  semantics. They are not obsolete Canic wrappers or copies of Testkit's server.
- Shared scripts are snapshot payloads. The manifest verifies canonical hashes
  and modes; generated service metadata and mounted declarations share the same
  catalog. The provider DID is generated from its endpoint declaration. No manual
  output-only change, second generator or common-tool installer is introduced.
- Probe evidence uses retained file creation so interrupted bytes survive. Atomic
  Host publication would change that evidence contract; the small write/sync
  adapter is therefore retained. Prepared private-journal reads and authenticated
  browser/native handoff also retain their consumer-specific admission policies.
- Frozen pre-cut canisters, trial results, failed probes and historical receipts
  are retained evidence. Removing allocation sources cannot retire provider
  objects, balances or continuing billing.

## Verification and limits

Core native tests pass: 348 unit plus 38 integration cases. All five compile-fail
Rustdoc cases pass after repairing the import. Strict core all-target/all-feature
Clippy passes. Actual Rust 1.91 core native targets/features and core/default
standalone Wasm checks pass. Contracts/native CLI normal dependency graphs remain
runtime-free. Cargo metadata confirms all twelve members have neither direct nor
renamed stable-structures dependencies and no downstream Canic dependency.
Formatting, dependency pins, snapshot integrity and documentation links are checked
with their scoped commands; final results and source diff remain in the raw packet.

No test expectation, fixture bytes, generated protocol, manifest selection or
production record codec is changed. No pre-existing behavioral failure surfaced
in executed checks. Full CI, package archives, native macOS, installed PocketIC,
Chromium, deployed provider and downstream Canic behavior are not requalified by
this fixture/documentation batch. Production endpoint semantics are unchanged;
no paid/network provider probe, release or publication ran.

## Task metrics

Compare the starting commit with task-owned working-tree files and this report.
Rust physical lines: +140 / -164 / -24.
Type declarations: created 0; deleted 0. Type physical lines: +15 / -14 / +1.
Documentation physical lines: +309 / -262 / +47.
Type lines are a subset of Rust; Rustdoc overlaps Rust/documentation. Generated
output, snapshots, lockfiles and raw evidence are excluded. The symbol inventory
is confined to the final maintainer handoff.
