# Current status — 2026-11-14

This is the independent clean-start Blob Storage rewrite. The old checkout was
renamed to `/home/adam/projects/ic-blob-storage-legacy`; GitHub repository ID
1387534636 is now private at `dragginzgame/ic-blob-storage-legacy`. Its history,
branches, tags, provider custody and durable journals were preserved. Legacy
recovery actions remain fenced by their original root identity; read its
`LEGACY.md` before any authorized recovery work.

The workspace contains one unpublished `ic-blob-storage` crate at 0.100.0 with a
first in-memory reference component and seven behavior tests. There are no product
dependencies, endpoints, durable service or provider integration.
Published Toko Tooling 0.43.0 is selected once in the root catalog and inherited
as a dev-dependency by the product crate. Source tools, license and contribution
policy must come from its authenticated published installer.

Bootstrap completed with the authenticated 0.43.0 snapshot (49 managed files),
canonical license/contribution policy, standard tools and the local formatting
hook. `make ci`, `make deps-list`, native Clippy/Rustdoc and Wasm library
compilation pass on Rust 1.100.0 / Linux x86_64. The LOC report runs. These checks
covered the empty library scaffold at bootstrap, not the later reference component
or service qualification. Current focused evidence is recorded below.

The maintainer authorized initial GitHub delivery to branch `0.100`, followed by
preparing the next development version. The destination is a fresh private
`dragginzgame/ic-blob-storage` repository: the old URL currently redirects to
repository ID 1387534636 (`ic-blob-storage-legacy`) and must not receive this push.
No legacy refs or object alternates are imported. Machine-local agent/credential
directories are ignored. Publication and deployment remain disabled. Bootstrap logs are in
`target/bootstrap-20261114/`. The first preview refused consumer-local scratch;
the external-scratch retry and installation passed.

Legacy `.git/legacy-relocation/` retains a verified history bundle, unchanged
main/frozen ref inventories, connectivity checks for all 25 frozen stores,
unchanged intent hashes and `migration.json`. Only Git path metadata was
rebound; the saved release intents still refuse the relocated root. Existing
missing historical checkouts were not recreated or retired. The legacy tools
were re-admitted at their new path. Its local `AGENTS.md`/README/`LEGACY.md`
notices remain uncommitted; historical product commits and published versions
are unchanged.

The maintainer accepted Caffeine-first product scope and ownership: one shared
library, standalone hosting, direct SDK transfers and Canic-owned composition.
[Design revision 2](../design/ic-blob-storage-design.md) proposes precise object
ownership, application-owned references, irreversible Closing, authority-lifetime
closure, accounting and replay rules. API and persistence remain unfrozen.
No legacy contract or storage format is adopted.

The maintainer accepted the [crate layout](../design/ic-blob-storage-design.md#selected-crate-layout):
library plus thin standalone host, Canic-owned adapter, and installed-service
tests under `qualification/crates/` in the same root workspace/lockfile. There
is no host to install yet, so no placeholder qualification package was created.
Reference source now uses `references/mod.rs` beside its existing tests, preserving
its public Rust path and behavior. Crate documentation assigns persistent state
and provider-object resolution to the library; hosts supply caller capture,
memory grants and lifecycle composition. No product dependency was added.
The layout change passes all seven reference tests, rustfmt, package Rustdoc
with warnings denied, and local documentation-link/whitespace checks. The moved
reference implementation is byte-identical; no new behavior tests were needed.

Standing maintainer requirements are recorded in [AGENTS.md](../../AGENTS.md):
evaluate relevant legacy code and regressions for selective reuse before subsystem
work, and check for architectural sediment after every successful push. Review
affected ownership, redundant mechanisms and growth; report findings and justified
retention in the delivery handoff. This instruction update performed no push.

Design review inspected the fresh scaffold, legacy GitHub documentation,
manifests and public library surfaces at `25bed1352f3d2b9659f0d2583f50fa9af859ea84`,
and Canic's integration ownership. The expected legacy sibling path is absent in
this environment; the relocation/custody details above are the retained bootstrap
record, not reverified recovery evidence. No legacy journals were accessed and
no sibling source was changed. Source links and review limits are in the design.

First gate: establish supported independent Rust-host access to Caffeine and
complete the provider contract table before API/persistence freeze. Public Caffeine
guidance does not establish that arrangement; old trials are historical leads.
Authority scope/recovery, exclusive deletion scope, root/length enforcement and
actual charge/cutoff terms remain unqualified.

The proposed lifecycle selects a trusted restricted management path, stopped/drained
upgrades and refusal of snapshot load/state replacement before dispatch. It does
not promise detection of an old snapshot forced in through a controller bypass.
Name and qualify the standalone entrypoint before D1 lifecycle tests; Canic owns
its corresponding composition. Full service acceptance remains documented rather
than qualified. Provider/IC sources were checked during the design revision.

Started the [local reference slice](../design/ic-blob-storage-design.md#first-implementation-local-object-references):
one owner, bounded live/released identities, replay without resurrection and
irreversible Closing. Release reuses existing records, so capacity exhaustion
cannot prevent cleanup. Legacy transition code/tests were inspected at the pinned
revision above; applicable invariants and scenarios were adapted, with differences
documented in the design. No legacy source or recovery state was imported.

The [Caffeine contract review](../design/caffeine-contract-review.md) now traces
the legacy standalone trial summaries, issuer/regressions, manifest length boundary,
certificate recovery and callback profile, alongside provider source pinned at
`59b9b29a4fad3828677c8b80ead0e06fdb40987e`. The observed certificate path is a
synchronous host update followed by client extraction of its IC certificate, not
a host-to-provider acquisition call. Public SDK preparation/resume/retry controls
do not meet the proposed client contract; the old patch is not adopted. Final
release cannot retract a held or independently recovered original certificate.
Current Caffeine guidance and the unanswered upstream GC issue were checked;
support, effect bounds, recovery acceptance and final closure remain unqualified.
The maintainer supplied no separate provider agreement/contact; use available
evidence. No private trial journals or provider objects were accessed.

Next: obtain the missing provider contract answers before binding objects,
selecting persistence or exposing a service. The review defines the next local
registration transition and installed/SDK/provider witnesses; none is claimed
implemented. Application/asset and management-entrypoint inputs remain open.
GitHub delivery is being prepared; perform the required architecture check after
the push and record its exact commit. No deployment or provider effect is authorized.
The maintainer selected `0.100.0` for the
rewrite, inherited from the root workspace. The crates.io sparse index checked
on 2026-11-14 lists `0.82.1` as the highest published stable version and contains
no `0.100.0` entry. Publication remains disabled; old releases and obligations
are unchanged. Recheck version availability during any future release.
Locked Cargo metadata and `make deps-list` confirm `0.100.0` with publication
disabled. The lockfile changed only the local package version; formatting,
documentation links and whitespace checks pass. No behavior changed.

Validation: tool installation and snapshot admission passed. `make test-references`
passed all seven tests. Package-scoped Clippy (`--lib --tests`, warnings denied),
Rustdoc (warnings denied) and rustfmt passed on Rust 1.100.0. Clippy initially
requested five `must_use` annotations; they were added and its rerun passed.
Local-link and whitespace checks cover this edit batch. No full CI or installed
canister/provider tests were run; none of this establishes D0 or D1 completion.

Tooling setup checked on 2026-11-14: the crates.io sparse index identifies 0.43.0
as the latest non-yanked stable Toko Tooling release, matching the installed
49-file snapshot and checksum. No pin, lock or managed-file change was needed.
`make update` now delegates to shared `deps-update`; its real online run succeeded
with zero package changes. `make deps-list`, `toko-cloc`, snapshot admission and
offline tool checks pass. LOC reports 81 product and 113 test Rust lines, with no
unclassified code. Help and README document all three commands and update selectors.
This tooling-only change leaves the reference-behavior evidence above applicable.

The provider-review batch changes documentation only; local Markdown links,
source consistency and whitespace were checked. Product code and dependencies
remain unchanged, so the focused reference evidence above still applies.
