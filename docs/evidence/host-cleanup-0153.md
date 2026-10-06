# Host adoption cleanup review — 2026-10-06

**Verdict: PASS within the cleanup scope.** Review uses
[module surface hardening](../../audits/module-surface-hardening.md) and its
[authorized cleanup procedure](../../audits/module-cleanup.md), from the declared
Shared Tooling 0.1.10 snapshot at `21f3ec3dd97f2968c9f0b08924451bb2f71770d1`.
The local overlay is [AGENTS.md](../../AGENTS.md), with command authority in
[development governance](../governance/development.md).

Entry source is `279b863`; the uncommitted host-owner adoption is preserved.
This review covers all workspace direct dependency declarations, maintained
scripts/Make/CI callers, superseded host imports and remaining native filesystem,
hash and decoding helpers. Public library, generated endpoint and persisted
service contracts retain external or recovery authority; absence of a local
caller is not grounds for removing them in a compatible patch. This is source
inspection and focused Linux native/Wasm execution, not a whole-service safety,
performance or deployed provider audit. Exact input, graph, command/log and
artifact hashes are in [the bound record](host-cleanup-0153.json).

| Candidate | Evidence and disposition |
| --- | --- |
| Standalone `serde` direct dependency | LOW, high-confidence stale declaration. No local derives or serde imports; decoding uses Candid's re-exported trait. Remove its manifest/lock edge. Native Clippy, Rust 1.88 Wasm compilation and exported Candid parity pass afterward. |
| Consumer fixture `serde` direct dependency | Retain with owner. Candid's re-exported `Deserialize` derive emits `extern crate serde`; it is a scanner false positive. Import `serde::Deserialize` explicitly in model/ops modules, with identical derives and records. Six recovery/model tests and native/Wasm checks pass; no ignore configuration is added. |
| Formatter fixture dependency pair | LOW, high-confidence stale caller. The original script refuses with `cannot prepare ordering-only manifest` after the facade is removed. Select adjacent `ic-cdk`/`ic-host-artifacts` declarations instead; actual hook sorting, index/lock preservation and mutating-formatter rollback checks pass. |
| `native::open_regular` | Retain live input admission: nonblocking open before descriptor inspection avoids FIFO blocking and preserves intentional source-link following. The shared 0.3 reader does not provide this complete opening contract. |
| `artifacts::Run::{create, open_body, publish_body}` | Retain live private run claim and streamed partial-body publication. Download/upload callers need private retained body.part, EOF/provider-root verification and no-replacement body.bin publication; generic byte publication does not own these transitions. |
| `probe::record::{save, json, read, hash}` | Retain live probe capture/integrity boundaries: exact create/write/directory-sync failure stages, interruption inventory, bounded no-follow reads and original raw hashes. Shared staging cleanup changes the record inventory after interruption. |
| Native provider-root/Candid/domain formatting helpers | Retain product authority and bounded trust checks. They verify protocol metadata and exact records rather than raw host artifact identity or ICP CLI envelopes. |
| `scripts/ci/dependency-pins.jq` | Retain live dynamic caller. The script inventory's sole missing filename reference is resolved by `include "dependency-pins"` with `-L "$SCRIPT_DIR"` in the shared checker. All 32 tracked scripts/helpers have maintained code/CI callers. |
| Remaining `ic-host-tools` 0.1.14 | Retain harness transitive ownership through ic-testkit. No direct tools/process facade or obsolete imports remain in executable source. Historical evidence and reviewed shared documents are not rewritten. |

No function, method or type is removed. The standalone dependency edge is the
only additional deletion; all package versions remain as selected at entry.
There are no unused-dependency scanner suppressions or hidden compatibility paths
introduced by this cleanup. The published host-owner record remains immutable
and qualifies its original source/lock, not this later lock edge removal.

Focused checks pass cargo-machete 0.9.2 across the workspace, locked offline
metadata, strict Clippy for standalone/consumer (all targets/features), Rust 1.88
Wasm compilation for both libraries, six consumer tests, one exported/deployed
Candid declaration parity test, the actual hook fixture, Rust/manifest formatting,
ShellCheck/syntax, dependency declarations, documentation links and the unchanged
56-file snapshot. The initial hook refusal and its complete private directory are
retained separately from success in `.tmp/host-cleanup-0153-01`.

No new full gate, PocketIC lifecycle run, native macOS execution, publication,
release/version bump, commit, push, sibling mutation, issue write or provider
request occurs. The parity test calls the compiled native interface and official
parser; it does not install a canister. Matching native macOS CI remains required
for the pending committed source.
