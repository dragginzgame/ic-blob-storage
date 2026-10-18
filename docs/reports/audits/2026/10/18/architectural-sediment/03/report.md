# Shared host dispatch implementation

2026-10-18. Implements the accepted cross-repository follow-up to the
[second audit](../02/report.md). Blob starts at `e938a1d` with that audit's dirty
changes preserved. Actual packages remain 0.32.2; notes remain pending 0.33.0.
Before-images, incoming diff and raw evidence are in
`.tmp/review-validation/host0330/`.

## Canonical owner and local result

`crates/ic-blob-storage/src/workflow/service/mod.rs` owns complete service dispatch.
Hosts implement `ServiceHost`: synchronous access to their sole installation,
actual platform version/time and explicit `ServiceExecutionLimits`. The existing
endpoint catalog binds to `ServiceWorkflow<Host>` without another generator or
implicit registration. Host globals, runtime custody, endpoint attributes and
lifecycle invocation stay with their hosts. Existing lower-level workflows remain
the independent policy/orchestration APIs; their semantics are not reimplemented.

Standalone's duplicate workflow module and separate provider access adapters are
removed. One shared implementation now owns borrowing projections, provider
transport selection and the post-await continuity observation. Execution limits
preserve each existing numeric value; the library supplies no inferred defaults.
The installation owns its volatile checkpoint. Fresh construction captures its
original installation version; restoration begins fenced with no observation,
and recording the first current version never clears a fence. Version observation
methods now accept only the actual current version. No persisted schema or byte
encoding changes, no lifecycle takeover and no new retry/recovery authority occur.
Contracts exposes the existing tenant request byte ceiling for host preallocation
admission; it remains 512 bytes.

## Canic contribution and prerequisites

Normal sibling checkout is untouched. The isolated checkout
`/tmp/canic-blob-host0330` contains commit `b6f5b75ec`, based on immutable PR #591
head `91d281b3c0a9f81f14c6305b1b26a29d9b7f72ff`. It deletes the duplicate dispatch
and checkpoint plumbing, binds host access to the shared facade and uses Contracts'
per-method tenant Wire/other-owned CBOR functions. The external provider certificate
remains Candid. The protected Canic outer envelope remains Canic-owned; Blob
installation bytes are CBOR, carried opaquely by embedded application envelopes.
The managed callers, malformed installation inputs and combined raw/Candid export
inspection follow those contracts. The embedded counter adopts governed Storage
under the selected Memory identity.

Async raw updates use the existing executor and `measure_endpoint_async` after
admission, rather than spawning unmeasured work from a synchronous wrapper. The
shared raw signature validator permits async only at the managed update boundary;
query and native component restrictions remain. A direct macro regression checks
guard-before-spawn ordering and selection of the async measurement owner.

This is a draft contribution: Blob 0.33 is not published, the aligned Canic Memory
adoption is not on the contribution base, and actual managed qualification cannot
yet run. Registry check stops at the unavailable Blob requirement. Three consumer
locks are deliberately retained rather than fabricated against unavailable
registry packages. Resolve them and qualify dedicated/embedded restoration after
the prerequisites land. Prepared PR description: `/tmp/canic-blob-host0330-pr.md`.
Automatic approval review rejected pushing the branch and creating the draft PR,
requiring explicit permission for that external publication. No remote branch or
PR was created by this task; local isolated commit creation was successful.

## Verification and limitations

- Eleven focused installation tests pass, including exact retained bytes and
  active-gap versus already-fenced restoration behavior.
- Ten installed standalone cases pass: three ingress/rollback, four lifecycle and
  recovery, two gateway cancellation/fencing, one account scope/state case.
- Fresh standalone and gateway-source Wasms build. Strict core/standalone Clippy
  and actual Rust 1.91 native/Wasm compilation pass.
- Actual Canic async macro regression and strict macro library/test Clippy pass.
- All Blob emitter expansions compile natively and for Wasm using actual Canic
  macros and actual Blob source with explicit framework/host substitutes. These
  are compile/shape evidence, not authorization, startup or installed Canic proof.
- The first gateway run failed because its source Wasm was absent. Its explicit
  fresh build and correctly configured retry pass; both attempts are retained.

No full CI, browser run, native macOS, deployed provider qualification, paid effect,
merge, release or publication ran. Root manifest, lock and Shared snapshot remain
unchanged. The independent Testkit API migration and Storage hard cut are preserved.
