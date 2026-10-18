# Canic shared-dispatch follow-up

2026-10-18. This implements the approved consumer follow-up to the earlier
[shared-dispatch implementation](../reports/audits/2026/10/18/architectural-sediment/03/report.md).
Blob remains released 0.33.0 with pending compatible 0.33.1 notes. No Blob
implementation, dependency selection, package version or Git ref changes here.

## Source and ownership

[Canic draft PR #603 — Adopt Blob shared dispatch and canonical endpoint codecs](https://github.com/dragginzgame/canic/pull/603)
is stacked on [PR #591](https://github.com/dragginzgame/canic/pull/591) at
`91d281b3c0a9f81f14c6305b1b26a29d9b7f72ff`. The contribution is isolated in
`/tmp/canic-blob-host0330`; initial implementation `b6f5b75ec` is refreshed by
`fe1d088c5`. The normal dirty Canic checkout and application consumers are untouched.

Blob's published `ServiceWorkflow<Host>` replaces Canic's local dispatcher,
provider access adapters and separate heap continuity checkpoint. Canic retains
endpoint admission, actual caller identity, lifecycle, memory runtime and metrics.
Raw async updates use the existing executor and shared async measurement owner.
Contracts owns the tenant Wire and other owned-method CBOR codecs. Provider
certificate Candid and the protected Canic outer Candid envelope remain separate.
Embedded application envelopes carry opaque Blob installation CBOR; their counter
uses Storage's cell over the existing host runtime lease.

Root, dedicated and embedded locks now resolve registry Blob/Contracts 0.33.0,
Memory 0.48.2 and Storage 0.5.3. The embedded catalog supplies its previously missing
Storage declaration and matches Memory 0.48. No unpublished source override supplies
those packages. Native installation refusal tests now follow the current codecs.

## Verification and limits

- Actual Canic macro package: all 48 library tests and strict all-target Clippy
  pass. Changed-source Rustfmt and diff checks pass.
- Endpoint expansions: native, Wasm and Rust 1.91 Wasm checks pass against actual
  Canic macros and published Blob/Contracts 0.33.0. Explicit substituted framework
  services make these shape checks, not runtime or installed acceptance. The
  reduced fixture emits an expected unregistered `canic_export_candid` warning;
  normal managed builds register that cfg.
- Two native installation byte-contract tests pass with the actual Canic decoder,
  adapter limits and published Contracts in an isolated fixture. The initial
  fixture-wide command also attempted Canic documentation examples unsupported by
  its reduced crate; the corrected library-only command passes both intended cases.
- Locked/offline metadata resolves both independent consumer graphs. Full adapter
  compilation now gets past registry resolution and fails in Canic Core on 15
  Memory caller errors: removed schema/allocation/re-export APIs and bootstrap
  policy members. This does not establish a successful full consumer build.

Raw logs and isolated fixtures are `/tmp/canic-blob-host0330-published-check.log`,
`/tmp/canic-blob-host0330-refresh-{macros-all,macro-clippy}.log`,
`/tmp/canic-blob-refresh-{codec,codec-unit,published-emitter-native,published-emitter-wasm,published-emitter-msrv}.log`,
`/tmp/canic-blob-refresh-{dedicated,embedded}-graph.json`,
`/tmp/canic-blob-emitter-check/` and `/tmp/canic-blob-codec0330-check/`.

Complete Canic's existing Memory caller adoption and coordinate the runtime owner
in [PR #595](https://github.com/dragginzgame/canic/pull/595), then run the actual
dedicated and embedded managed journeys, including malformed init refusal and
retained-data restoration. Keep #603 draft until that acceptance passes. No full
CI, native macOS, installed Canic qualification, merge, release, registry
publication, deployment or paid provider effects ran. The scoped contribution
branch and draft PR were published under the isolated contribution workflow.
