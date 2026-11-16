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
  Product versions are coordinated at the root; `0.100.0` was the first GitHub
  checkpoint. The maintainer authorized crates.io delivery on 2026-11-16;
  `0.101.0` is the selected first registry release of the rewrite.
  Existing crates.io versions are not reset by this rewrite.
- Work and release directly on `main`. Use version tags for release checkpoints;
  do not create version branches unless the maintainer explicitly requests one.
- In this repository, a maintainer request to "push" means a new GitHub version
  checkpoint: increment the current patch version unless an exact version is
  specified, finalize its changelog and push the coordinated product version and
  both lockfile updates. Preserve independent qualification package versions.
  Delivery-only handoff/review commits belong to that same checkpoint and do not
  trigger another version increment. Product releases now include crates.io
  publication; qualification remains private. Follow the [release procedure](docs/operations/releases.md).
- Production crates belong under `crates/`. Add qualification packages only for
  real installed behavior; do not create tooling-holder packages or placeholder
  tests. Use Rust 1.100.0 and edition 2024 with the inherited common lint policy.
- The maintainer approved a minimal owned browser transfer client in `clients/browser/`.
  Keep immutable preparation, existing-proof input, a caller-owned durable claim,
  serial PUTs and no automatic retries. Node 24+ runs `make test-browser`; there
  are no runtime dependencies in the byte-transfer entry point. Optional proof/custody
  uses published npm Platform and IndexedDB; prepare it with `make install-browser-tools`.
  It must stay independent of the Storage rewrite;
  work around that dependency through this client boundary, not live-state reset.
- The accepted [crate layout](docs/design/ic-blob-storage-design.md#selected-crate-layout)
  uses a thin standalone host and a Canic-owned adapter. The maintainer-approved
  qualification location is a separate `qualification/` Cargo workspace and
  lockfile, accepted on 2026-11-15. Its unpublished packages have independent
  explicit versions and sit directly under that directory. Both workspaces select
  the same installed Toko Tooling payload; only product crates inherit release versions.
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
  The maintainer also authorized the local `canic-mainnet` wallet for needed
  cycles and directs independent protocol research rather than waiting for
  provider answers. Select concrete bounded paid calls before spending; do not
  export keys or reuse legacy accounts/obligations.

## Checks

Prepare with `make install-tools`; `make ci` is the aggregate gate for tooling,
formatting, strict Clippy, local behavior, Rustdoc and documentation links.
`make dependency-pins-check` runs the installed Toko declaration checker across
both Cargo workspaces and action pins, including workspace inheritance and tracked
lockfiles. It also runs in `make ci`; runtime and npm checks retain their own owners.
Use `make test-references` for the in-memory reference tests and `make test-caffeine`
for streaming and manifest integrity tests. `make test-local` checks the composed
grant/reference/accounting model and bounded durable reopening. `make example-local`
runs the offline application walkthrough; `make test-example-local` checks its catalog
and read-completion boundaries. `make install-test-server` explicitly prepares the
local simulator (also included in `make install-tools`); `make test-standalone` tests
the actual production Wasm's callers and upgrades, independently of the native
aggregate. Set `BLOB_PREVIOUS_WASM` to install an explicitly selected predecessor
before upgrading to the current artifact. `make test-capacity` separately measures
the real host at full history, checking cleanup/reopen with platform execution-cycle
charges and allocated memory; these are not raw instruction counts or provider costs.
`make test-endpoints` checks raw frames
on the production host and encoding-failure rollback in a qualification-only probe.
These checks do not qualify Caffeine or a production management runner.
`make test-enrollment` checks the private single-use owner enrollment probe's
native capacity bound and installed authorization, reply custody and upgrades
against synthetic Cashier peers. It also checks incompatible upgrade refusal using
the actual standalone Wasm and preservation of the consumed receipt. It does not
deploy or contact Caffeine.
`make test-callbacks` checks the local candidate `vec blob` profile over the actual
Service, including gateway authorization, bounded decoding, acknowledgment custody
and installed upgrades. It also qualifies local certificate-reply ordering and
authenticated recovery of the original ingress request, including fresh CLI
processes reading durable request custody and preserving existing output. Node 24+
is required for the proof/CLI-claim/loopback-transfer/readback composition,
fresh-process replay refusal and the real reference-app catalog/cancellation cases.
Its separate probe is not
a supported live Caffeine host or a production resumable-upload client.
`make test-signed` checks signed application/uploader calls and authenticated replies
through the local gateway, including inspection after deliberately discarding a reply.
`make test-application` exercises the product reference application and Blob host:
exact CLI-prepared fresh-install inputs,
durable drafts/catalog, replicated inspection, cancellation during publication,
interrupted cleanup, bounded history and application upgrades. No provider transfer
is implemented; publication means the owning user's catalog decision.
`make test-management` tests native inspection, guarded maintenance and explicit
activation through signed requests and certificate verification against an owned
local HTTP gateway/root. It covers custody, interruption boundaries and reopened
admission, not the live IC or provider effects.
`make test-browser-runtime BROWSER=/absolute/path/to/chrome` qualifies the ES module
in isolated Chromium: authenticate retained local IC proof, exercise the bounded
browser custody adapter and local cross-origin reads/PUTs. Reload refuses replay
after success, a lost reply or in-flight cancellation. This does not qualify
provider acceptance, profile rollback or multi-device custody.
It installs no browser and stays separate from Node CI and provider qualification.
`make test-browser-application BROWSER=/absolute/path/to/chrome` joins that browser
flow to the installed reference catalog, checking publication after verification
and refusal after cancellation during readback, including stopped/drained
same-format upgrades of both canisters. Rust owns signed application calls;
this opt-in local check does not qualify a deployed UI or live provider.
Package inventory is `make deps-list`;
`make update` delegates to the shared `deps-update` target for Cargo lock updates.
LOC is `toko-cloc`. An exact tooling-pin change still requires the authenticated
`make toko-tooling-update` installation after updating both workspace pins and lockfiles.

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

## Post-push review focus

Apply the baseline's [post-push review](TOKO-TOOLING.md#post-push-review).
For Blob, check library/host/Canic/client/provider ownership, retained live custody
and any legacy carryover's current consumer need. Explain material growth using
comparable product/test/tooling evidence where useful; fewer lines alone prove
neither simplicity nor safety. Keep current handoff updates to changed next
actions, blockers and evidence links, not another copy of the delivery report.
