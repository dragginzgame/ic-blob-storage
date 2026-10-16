# Probe diagnostic and decoder-limit convergence

Source base is released 0.30.0 `e9f71822ae361377686846fe7a4bf7c0e9a6822f`, plus
prior pending upload/session/funding cleanups and incoming Cargo edits. The task
before-images, starting status and head are retained under
`.tmp/review-validation/probe-decoding0310/`. The starting root already selected
direct Host 0.23.1, Testkit/Candid 0.49, Memory 0.44.0, Metrics 0.11.5 and Build
0.10.0. The initial progress attribution of Host changes to active validation was
incorrect; saved before-images prove the selection preceded this batch.
The root manifest and every external lock selection remain byte/row-identical.
The only task-owned lock change adds the fixture protocol's local library edge.

## Ownership and contract

Authority and funding probes share passive blocker/warning conversion in the
private fixture protocol. Their owner snapshots and policy observations remain
independent. That unpublished package now depends on the service library; the
production library and Contracts do not depend on the fixture protocol.

Funding status holds each original attempt together with its checked transport
facts and reconciliation. Workflow selects the pure assessment function, and
activity diagnosis reads the resulting rows. Rendering consumes the same rows;
there are no parallel status vectors or positional joins. Contradictory/missing
transport evidence remains unknown; saved labels never override exact facts.
The diagnosis still observes all retained obligations and grants no credit,
freshness, spendability, retry or fence-release authority.

Provider and account decoders use one explicit byte/work/skipping/type budget.
Audit retains its independent CSV and reported-entry bounds. Callers keep every
numeric value, method/source/account/token check, byte-check ordering and typed
failure. There are no defaults, aliases, alternate readers or wire-format changes.
The [host upgrade guidance](../service-contract.md#explicit-provider-decoder-budgets)
names the public Rust replacements. This requires pending 0.31.0; package versions
remain 0.30.0. Existing retirement/reinstall obligations remain in force.
Five native fixture child callers select Host 0.23's explicit kill-and-wait policy,
matching Host 0.22's former default without changing process-group ownership.

## Focused Linux qualification

- 47 provider cases retain independent Candid fixtures, malformed/oversized replies,
  decoder/skipping/type quotas, binding-before-decoding and exact pending state.
- 11 durable gateway, one account store, three public boundary and 18 probe unit
  cases pass: 80 unit/boundary cases in total.
- Fresh authority/funding/gateway-source and standalone Wasms plus the ordinary CLI
  pass 27 installed PocketIC journeys and one native subprocess-output regression.
  These cover status JSON, refunds, callback rollback, old-backup fencing, lifetime
  history, passive account replies and original session recovery without redispatch.
- Seven directly affected packages pass all-target/all-feature checking and strict
  lint, plus actual Rust 1.91 checks. The public library passes a separate Rust
  1.91 Wasm check. Library/fixture Rustdoc passes with warnings denied.

The explicit PocketIC 16.1 executable was re-admitted against published Testkit
0.49's Linux archive digest and its decompressed payload before fixture building.
This does not qualify the lock-selected CLI installation or complete tool gates.
Commands, selected inputs, source/artifact hashes and logs remain at the raw root.

The first check found two mechanically changed import paths and one qualified
constructor; its failed log survives the repair. The next check found the five
pre-existing Host spawn-signature mismatches; that failed log also survives.
The final affected-package check and all focused tests pass after repair.

The optional package-verifier check separately fails with two Host process type
mismatches: Build 0.10 uses Host 0.22 while the incoming direct graph uses 0.23.
Its source and those dependency selections predate this batch. No alias, override,
producer edit or dependency reselection was introduced to conceal that boundary.
[#69 — Adopt Host custody and archive qualification](https://github.com/dragginzgame/ic-blob-storage/issues/69)
retains its ownership; packaging is not qualified on this graph.
Full CI, actual package archives, native macOS, Canic and deployed-provider
acceptance were not run. No sibling, Git, release, publication or paid effects ran.
