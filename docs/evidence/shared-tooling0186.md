# Shared Tooling continuation — pending 0.18.6

Base is released Blob 0.18.5, `ab37a018e2050b3b06963938ceb78ebb385bb41c`,
with validated source `5c8f08b515e270da4bc587a890a44e6afa3efcec`.
The maintainer's incoming lock changes Memory 0.31.6 to 0.31.8, private Metrics
0.2.14 to 0.2.15 and Testkit 0.25.1 to 0.25.2. Host remains 0.8.2, PocketIC
client/server remain 16.1.0. These edits are preserved; package versions and
the release receipt remain 0.18.5.

## Committed shared adoption

A clean isolated clone exports 97 files from Shared Tooling
`4e274a2219c0b0cc3af68ec65658b373253518fb` (0.1.30). The owner checkout has
unrelated dirty version work; it is excluded. Add the canonical release-source
helper, npm declaration fixture and IC matrix to the previous 94-file selection.
All IC matrix rows are identical to Blob's already qualified 16.1 selection;
only comments change. The sole matrix returns to canonical snapshot ownership.
The existing installer fingerprints the full file, requiring explicit new bundle
preparation. The original bundle and downloads remain retained, with no cleanup.
[Shared #79](https://github.com/dragginzgame/shared-tooling/issues/79) tracks
comment-only catalog reuse at its owner; this adoption does not patch that behavior.

The release adapter uses the shared read-only status checker for clean source and
the exact allowed metadata set. Strict HEAD/receipt/source/parent checks remain
local. The allowance is derived from the existing release-file inventory, so no
second catalog is introduced. Git observation failures cannot become successful
admission. The simulation fixture supplies NUL-safe status with the real argument
contract and explicitly checks path diagnostics, pre-validation refusal and failed
prefix/status reads without metadata or publication effects.

The declaration gate passes the existing private browser root and Node/npm
selections to the shared npm checker. No npm install/resolution or browser traffic
is implied. The independent browser executable/engine checks remain because that
caller has no Cargo prerequisite and the shared gate inspects the complete repo.
No recurring task, sibling dashboard or Cargo-install assessment is enabled.

## Attempts and qualification

Raw inputs, commands and failed attempts remain in `.tmp/continuation0186-01/`.
The first offline dependency preparation refuses the missing Memory 0.31.8 archive;
explicit `make deps` then fetches that exact locked archive. No dependency version
is selected by preparation. The first attempted consumer invocation of the owner
source fixture fails at its owner-specific `ci/frontend/package-lock.json` path,
retaining `/tmp/release-source-test.K35GNb`. That fixture is not a reusable consumer
selection and is removed from the delivered snapshot. Running it against the
clean isolated Shared checkout passes; Blob's own adapter fixture supplies
consumer acceptance. These are separate evidence classes, not a hidden rerun.

Shared integrity, browser/npm declarations, actual prepared Node 24.21.0/npm 12.2.0,
the canonical npm fixture and the owner source/preservation fixture pass. Explicit
Linux IC preparation and offline receipt verification preserve exact PocketIC
16.1 identity. The incoming graph passes actual Cargo/rustc 1.88 native and Wasm
checks. `make test-native-host` passes 241 cases: 114 CLI/probe, 103 contracts,
21 examples, two actual PocketIC installation/Candid cases and one Metrics
restoration case. The complete substituted release-adapter and shared runner
fixtures pass. An actual dirty-source refusal names the affected paths while
preserving the release files and Git index. All 901 final build/tooling input
hashes remain unchanged across qualification; compiled package identity is
0.18.5. `qualification.json` retains the graph, results and raw log hashes in
the evidence directory. No full local CI/release gate is invoked.

## Issue disposition

Released [0.18.5 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37808256060)
passes its explicit minimum compiler job and Linux native job. Both native macOS
jobs remain queued at the final observation (`final-released-ci.json`).
[#35](https://github.com/dragginzgame/ic-blob-storage/issues/35)
is closed for its delivered gate; #29/#36 retain their separate native acceptance.
This hosted result does not qualify the incoming dirty graph/snapshot.

[#20](https://github.com/dragginzgame/ic-blob-storage/issues/20) is closed using
[Canic's exact owner evidence](https://github.com/dragginzgame/canic/issues/444#issuecomment-6064452882):
three independent Blob runtime/contracts 0.18.4 workspaces, coherent Memory
0.31.7 identity, eight artifact-bound Wasm graphs, 32-method strict Candid parity
and both final managed installation/refusal/lost-result/restoration/recovery cases.
The prepared adapter remains unpublished. Canic #444 retains actual Toko and
the separately owned multi-user/GC/verifier/parent/funding acceptance.

#31 now has a [concrete grant proposal](../multi-user-upload-authority.md), reusing
the existing authenticated tenant's exact retained permission instead of a new
uploader registry. Grant-owner selection remains pending; implementation would
change certificate trust/configuration and require a minor hard cut. This patch
changes no such public or durable contract. Provider and application acceptance
for #4/#5/#6/#32/#33/#34 remains with its existing evidence owners.

No agent commit, push, release/publication, sibling edit, workflow dispatch or
paid/live provider effect occurs. No existing named Rust function, method or type
is removed. Local `ensure_clean` and `allowed_changes` retain their names while
delegating status enumeration to the canonical owner.
