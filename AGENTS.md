# AGENTS.md

Apply the installed [Toko Tooling baseline](TOKO-TOOLING.md) and its
revision-bound references. [.toko-tooling.snapshot](.toko-tooling.snapshot) owns
the selected payload; never hand-edit managed files or use a moving sibling
checkout as executable tooling.

Read [the current handoff](docs/status/current.md) first.

## Scope

- This is the independent clean-start rewrite. The sibling
  `../ic-blob-storage-legacy` is read-only historical evidence unless separately
  authorized. Do not copy its architecture, tests, recovery state or dependencies
  by default; establish a current consumer need first.
- Legacy provider obligations and path-bound recovery records remain with the
  legacy repository. Never resume or import them through this fresh checkout.
- There is an experimental local standalone host, but no frozen API, migration
  or production deployment runner yet.
  The rewrite uses the `0.100` release line, inherited from the root workspace;
  `0.100.0` is its first GitHub checkpoint. Keep publication disabled until a
  separately authorized release.
  Existing crates.io versions are not reset by this rewrite.
- In this repository, a maintainer request to "push" means a new GitHub version
  checkpoint: increment the current patch version unless an exact version is
  specified, finalize its changelog and push the coordinated workspace/lock update.
  Delivery-only handoff/review commits belong to that same checkpoint and do not
  trigger another version increment. Registry publication remains separately gated.
- Production crates belong under `crates/`. Add qualification packages only for
  real installed behavior; do not create tooling-holder packages or placeholder
  tests. Use Rust 1.100.0 and edition 2024 with the inherited common lint policy.
- The accepted [crate layout](docs/design/ic-blob-storage-design.md#selected-crate-layout)
  uses a thin standalone host and a Canic-owned adapter. The maintainer-approved
  qualification location is `qualification/crates/`, with packages belonging to
  the root workspace and lockfile; it does not establish a separate workspace.
- No initial commit, remote creation, push, publication or live-provider effect
  is implied by the local scaffold. Custom future deployment entrypoints must
  admit the tooling snapshot before effects.
- The maintainer explicitly approved the experimental local service model while
  D0 remains open, and subsequently authorized progressing to a locally installed
  standalone host and Caffeine calls when ready. Carry that authorization forward;
  do not request it again merely to cross the former no-deployment boundary.
  Provider calls still require a concrete qualified operation, durable custody of
  possible effects and an identified fresh target/account; authorization is not
  evidence of provider support, bounded charges or closure. Keep declared-byte
  accounting, no unsupported settlement claim and unfrozen API/persistence.
  The earlier instruction against contacting Caffeine concerned external messages;
  no issue comment, email or other outreach is authorized by provider-call approval.

## Checks

Prepare with `make install-tools`; `make ci` is the aggregate gate for tooling,
formatting, strict Clippy, local behavior, Rustdoc and documentation links.
Use `make test-references` for the in-memory reference tests and `make test-caffeine`
for streaming and manifest integrity tests. `make test-local` checks the composed
grant/reference/accounting model and bounded durable reopening. `make example-local`
runs the offline application walkthrough; `make test-example-local` checks its catalog
and read-completion boundaries. `make install-test-server` explicitly prepares the
local simulator (also included in `make install-tools`); `make test-standalone` tests
the actual production Wasm's callers and upgrades, independently of the native
aggregate. Set `BLOB_PREVIOUS_WASM` to install an explicitly selected predecessor
before upgrading to the current artifact. `make test-endpoints` checks raw frames
on the production host and encoding-failure rollback in a qualification-only probe.
These checks do not qualify Caffeine or a production management runner.
`make test-signed` checks signed application/uploader calls and authenticated replies
through the local gateway, including inspection after deliberately discarding a reply.
`make test-application` exercises the product reference application and Blob host:
durable drafts/catalog, replicated inspection, cancellation during publication,
interrupted cleanup, bounded history and application upgrades. No provider transfer
is implemented; publication means the owning user's catalog decision.
`make test-management` tests native inspection, guarded maintenance and explicit
activation through signed requests and certificate verification against an owned
local HTTP gateway/root. It covers custody, interruption boundaries and reopened
admission, not the live IC or provider effects.
Package inventory is `make deps-list`;
`make update` delegates to the shared `deps-update` target for Cargo lock updates.
LOC is `toko-cloc`. An exact tooling-pin change still requires the authenticated
`make toko-tooling-update` installation after updating Cargo.lock.

## Learn from the predecessor before subsystem work

- Before implementing or substantially changing a subsystem, inspect the relevant
  legacy Blob Storage implementation and regression tests. Consult its change
  history and investigations where they explain a fix, recovery rule or tradeoff.
  Identify the original problem, the established solution and which assumptions
  change in this rewrite. This is targeted research, not a full legacy audit for
  every routine edit.
- Reuse or adapt sound first-party solutions where they fit the chosen contract;
  independent reimplementation is not itself a benefit. Explain concrete reasons
  for replacing a solved mechanism. Repository size or age alone is not a reason,
  and reuse does not authorize importing the old architecture or feature set wholesale.
- Bring applicable failure and regression cases into this workspace's qualification
  and verify them here. Cover identity/authorization, malformed input, interrupted
  operations, retry and recovery where the selected subsystem owns those obligations.
  Historical passing tests do not establish correctness of a new implementation.
- Cite the inspected source or revision when explaining a concrete carryover.
  Distinguish adapted code from retained invariants, algorithms, API ergonomics and
  independently implemented lessons. Explain replacements, exclusions and deferrals,
  including changed consumer or recovery obligations. Keep the legacy research
  read-only under the scope rules above.

## Architecture check after every push

- After every successful push to this repository, review the pushed changes and
  their affected owners for architectural sediment before completing the delivery
  handoff. Use the revision-bound [complexity review](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/complexity-and-technical-debt.md)
  within that scope; this is not a full-repository audit or broad test gate.
- Check that Blob's library, standalone host, Canic adapter, client and provider
  boundary still have the responsibilities in the design. Look for duplicated
  policy/state, unused abstractions, speculative modes, obsolete compatibility
  paths and old mechanisms left alongside their replacements. Confirm that legacy
  carryovers have a current consumer reason and were requalified here.
- For code pushes, inspect product, test and tooling growth separately using
  existing LOC/dependency reports where useful. Explain material growth and trace
  the real callers; fewer lines alone do not establish a simpler or safer design.
  State when a comparable prior baseline is unavailable. For documentation-only
  pushes, check scope and architectural consistency without rerunning code suites.
- Report the pushed commit(s), reviewed scope, findings, justified retention and
  next actions concisely in the delivery summary and current handoff. Track any
  deferred actionable work in owning GitHub issues, not a new local debt ledger.
  Do not declare the architecture clean when required evidence is missing.
- This check does not authorize another push, unrelated repairs or a new audit
  framework. Preserve necessary authorization, persistence and recovery guarantees
  when proposing simplification. The aim is a codebase the maintainer can understand
  and change, without returning to an unmaintainable accumulation of mechanisms.
