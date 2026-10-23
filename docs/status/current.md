# Current status

Date: 2026-10-23

## Compile-time method descriptor follow-up

Pending **0.43.4** also addresses [Blob #101](https://github.com/dragginzgame/ic-blob-storage/issues/101).
Runtime ID/limit lookups are now const functions. `MethodSpec::wire()` supplies a
passive `WireContract` with the ID and both complete allowances; provider Candid
refuses. Optional `describe = callback` in the existing mounting macro visits
all canonical declarations before endpoint emission, providing codec classification,
Rust identifier and a const descriptor expression. Hosts can define constants
without repeating the method roster or numeric policy. The schema exporter uses
the same descriptor owner; lifecycle lookup names remain outside the callable
catalog. Existing mounting callers and frame/storage/lifecycle behavior are unchanged.

Canic's raw declaration support is already implemented locally. After publication,
Canic owns descriptor adoption, dedicated/embedded final-artifact matching and
release binding; #101 is not closed by an unpublished Blob working tree. No sibling
files or framework-specific section formats are introduced. The existing CI batch
and incoming Cargo.lock changes are preserved. Evidence for this follow-up is
under `.tmp/review-validation/issue101/`.

Pass: strict workspace Clippy, contracts/all-feature tests including 31 static
mount descriptors and separate lifecycle constants, Rust 1.91 native all-targets
and normal standalone Wasm, dependency/feature boundaries, rebuilt standalone
Wasm export/provider classification, formatting and docs checks. Generated schema
JSON is byte-identical to its pre-change output; TypeScript passes all 137 frozen
Rust frames and refusal cases. All 29 owned/lifecycle IDs are unchanged. No full
release gate, fresh macOS or Canic installed qualification was run. Nothing was
staged, committed, pushed or published; the incoming lockfile is unchanged.

## Generated-client CI follow-up

Released HEAD is **0.43.3**, `45738f5`. Pending **0.43.4** adds an independent
Linux `Wire schema and generated clients` job to the existing PR/main workflow.
It selects Node from `tests/browser/.nvmrc` and npm from the browser manifest,
prepares the locked npm/Cargo dependencies, then runs contracts Clippy, the
all-feature Rust contract suite and `make test-wire-schema` offline. The job
retains generated schemas/clients and check logs for 30 days, including failures.
It uses existing owners and adds no dependencies or protocol changes. Incoming
Cargo.lock work remains preserved. Hosted execution awaits delivery; local
qualification is recorded under `.tmp/review-validation/wire-schema-ci/`.
Pass: actionlint, scoped strict Clippy, all-feature contracts tests, generated
TypeScript parity/rejections (27 methods, 137 frozen frames), documentation links
and whitespace checks. The incoming lockfile is byte-identical to its before-image.
The hosted setup/artifact transport has not run; no commit or push was performed.

The schema implementation below shipped in 0.43.3. Its earlier pending/publication
wording records the original batch; Canic consumer acceptance remains separate.

## Released contract and pending schema delivery

HEAD is released **0.43.2**, `fbf924d`. Pending **0.43.3** implements the supplier
API requested by [Blob #100](https://github.com/dragginzgame/ic-blob-storage/issues/100).
Contracts' opt-in native `wire-schema` feature derives metadata and enumerates all
27 owned callable methods from the existing endpoint declaration catalog. It
exports method IDs/modes, serialized request/reply schemas and complete native
byte/allocation/node/depth allowances. Provider Candid classification stays in
`SERVICE_METHODS`; lifecycle belongs to each host and is not a callable method.
Tenant metadata uses actual bounded projections; account Int/Nat fields use
validated underscore-grouped decimal strings. The small generated runtime adapter
supplies only those custom scalar validators to the unmodified Wire runtime.
No endpoint identity, serialization, persisted record or lifecycle is changed.

The selected published Wire/derive is **0.6.10**; Platform requires at least 0.6.9
for Principal metadata and resolves to **0.6.10**. Incoming lockfile updates to
Memory/Storage 0.50.13 and Metrics 0.15.7 are preserved. Host/Build/Testkit remains
0.66.0; Shared Tooling remains 0.26.8 at `0b07c2a`. No sibling source overrides,
new installations, commits, staging, releases or publication were performed.

Use the contracts README's host-schema API and `wire_schema` example. Canic still
mounts this catalog in `integrations/blob-service/src/endpoints/mod.rs` (read-only
source review); its #613 owns adoption, embedded/dedicated artifact qualification
and immutable release-sidecar binding after publication. Do not copy the schema
or infer it from DID. #100 stays open until supplying API publication and consumer
acceptance. No deployed provider claim follows from these local checks.

Current evidence is under `.tmp/review-validation/issue100/`: contracts/all-feature
unit, integration and golden tests; strict workspace Clippy; Rust 1.91 native
all-target/all-feature contracts and normal standalone Wasm checks; dependency
boundaries; documentation links; generated TypeScript qualification of 27 methods
and 137 frozen Rust frames including identity/direction/trailing/truncation/byte
limits and custom decimal refusals. The standalone optimized Wasm is rebuilt and its actual export inventory and
provider Candid classification test passes. The feature guard also detects an
explicitly enabled schema negative control.
Normal deployed feature admission explicitly rejects schema generation. Exported
Rust allocation budgets are not JavaScript heap guarantees. Native macOS and
Canic installed qualification were not rerun; full release CI remains required.
Initial Node/npm sandbox refusal and the corrected null-prototype assertion are
retained alongside passing checks. Historical evidence below is not a claim that
those full runtime suites were rerun for the schema addition.

## Service ownership and remaining integration

Blob owns the funding journal and acknowledgment policy, root/reference/GC owner,
quotas and liabilities. Matching successful Cashier replies permit another
bounded distinct payment; missing/malformed accepted outcomes remain unresolved.
GC acknowledgment stops repeat listing but retains physical and billing liability.
Unknown roots remain protected. Source rollback fences and original identities
remain authoritative; no balance delta or timeout authorizes an uncertain retry.

Canic owns its adapter, lifecycle and mounted tests. Last read-only review found
local guarded funding composition and refunded/concurrent-call coverage under
[Canic #494](https://github.com/dragginzgame/canic/issues/494). Accepted-payment,
partial-refund, positive-credit and renewal qualification is separate and uses
Tooling's cycle-accepting fixtures. The current Tooling release includes those
fixtures; Blob does not add a second consumer implementation or test suite.
[Canic #622](https://github.com/dragginzgame/canic/issues/622) reports local GC races
qualified and awaits delivery. Recheck current consumer status before claiming
completion; this adoption did not rerun Canic or modify any sibling checkout.

[Blob #32](https://github.com/dragginzgame/ic-blob-storage/issues/32) is closed after
callback delivery. Remaining application/provider work stays in #4/#5/#6/#31/#34:
media-map publication, serving, multi-user acceptance, payer linkage, deletion and
billing. Provider acknowledgments are not independent credit or deletion proof.
[Funding guidance](../funding-consumer-qualification.md) and
[GC contract](../gateway-gc-contract.md) retain the limits and external questions.

## Validation

Previous 0.65 adoption evidence is retained in
`.tmp/review-validation/host065-shared0267/`, including snapshot preview/refresh,
locked metadata, boundaries, Clippy and explicit tool setup/checks.
All pass, along with contracts/golden tests, Rust 1.91 native CLI all-feature
checks, formatting/hooks, 126 snapshot payloads and 1,556 documentation links.
Shared fixture tests pass on the localhost-permitted rerun.
The first shared-fixture attempt failed only when the sandbox refused its local
HTTP listener; retain that log and `/tmp/tool-downloads.ZRexp6` separately from
its localhost-permitted rerun. No lock removal or ignored failure was used.
Prior Wire migration evidence remains in `.tmp/review-validation/wire062/` and
prior funding/GC implementation evidence in `.tmp/review-validation/provider-hardcut042/`.
No full release gate, native macOS, live provider operation or deployment is
claimed by this adoption. No staging, commit, push or release was performed.
Focused native CLI upload-input tests exercise the adopted Host filesystem path;
results are retained in `cli-inputs.log` alongside the adoption checks.

Current 0.66 adoption logs are in `.tmp/review-validation/host066/`.
Locked metadata, declaration/runtime boundaries, strict workspace Clippy,
formatting and documentation checks pass. The harness changes only explicit
ownership transfer; its existing drop order remains authoritative.
Rust 1.91 standalone-test compilation, release standalone Wasm and the installed
tenant ingress regression pass on the new harness. The PocketIC test exercises
server startup, application refusal/success and normal harness teardown. Full CI,
native macOS and live-provider qualification were not rerun.

Latest Shared 0.26.8 / Wire 0.6.7 adoption evidence is retained in
`.tmp/review-validation/shared0268-wire067/`. Locked metadata, snapshot and
runtime/declaration boundaries, strict workspace Clippy, contracts/golden vectors,
342 core tests, Rust 1.91 standalone Wasm and shared fixture tests pass.
Explicit common Rust/Testkit/Build setup completed under the new build policy.
No new runtime code, protocol identities or fixtures were introduced. Full CI,
fresh installed IC execution and native macOS were not rerun for this slice.
