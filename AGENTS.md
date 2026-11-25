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
  checkpoint. The maintainer authorized crates.io delivery on 2026-11-16.
  The changelog and release records own delivered versions.
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

Use registry Storage 0.80 named allocations and governed stable cells in both
workspaces. Hosts distinguish fresh installation from reopening before constructing
Storage and use ordinary upgrades within the same layout. Follow the
[storage contract](docs/design/stable-storage.md) for incompatible predecessors.
The 0.104.0 OP heap and the consumed 0.74.1 probe require separate disposition.
Preserve original provider obligations and custody.

Prepare with `make install-tools`; `make ci` is the aggregate gate for tooling,
formatting, strict Clippy, local behavior, Rustdoc and documentation links.
Release validation uses the qualified change mapping in
[the release procedure](docs/operations/releases.md), with full fallback for shared
or unknown inputs. After changing that mapping or its callers, run
`make test-release-validation test-release-metadata`; these consumer wiring checks
are not unconditional release regressions. Publication still performs fresh
archive/source/registry admission. Existing local `test-*` targets retain setup.
`make dependency-pins-check` runs the installed Toko declaration checker across
both Cargo workspaces and action pins, including workspace inheritance and tracked
lockfiles. It also runs in `make ci`; runtime and npm checks retain their own owners.
Use the [release procedure](docs/operations/releases.md), current handoff and
[product design](docs/design/ic-blob-storage-design.md) to select affected checks.
The Makefile owns exact `test-*` command dispatch: local/reference/Caffeine checks,
installed standalone/capacity/endpoints, enrollment, callbacks, signed operations,
application composition, management and opt-in browser qualification. Do not
interpret native or synthetic-peer passes as live-provider support or production
deployment evidence.

Prepare the simulator explicitly with `make install-test-server` (also selected
by `make install-tools`). `BLOB_PREVIOUS_WASM` selects an exact predecessor for
upgrade qualification; preserve its artifact identity and retained state. Browser
checks require an explicit prepared `BROWSER` executable and install no browser.
Node is required for the client composition checks. Keep charges, capacity and
raw instructions distinct, and preserve original request/proof/custody evidence.
Current commands, fixture coverage and limitations belong with their owning
implementation and handoff, not duplicated in this overlay.

Use `make deps-list` for package inventory, `make update`/`make deps-update` for
lock updates and `toko-cloc` for requested source counts. Tooling changes use
`make toko-tooling-update` after coherent changes to both workspace selections.

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
