# Current status

Date: 2026-10-13

## Released source and pending batch

Blob 0.22.15 is released at `e11780eb0ca13a4cfebe927d26a12b07c7772d3d`, validated
parent `0d720d1745c1c9e87fe2a9cb922ba21c1dec5293`. Remote main/tag match;
all twelve versions and the finalized receipt are 0.22.15.
Its Metrics 0.8 repair and Host 0.15 interface helper adoption are delivered;
[#62](https://github.com/dragginzgame/ic-blob-storage/issues/62) is closed against
matching delivered source, original focused proof and successful released Linux /
Rust 1.88 lanes. [Original evidence](../evidence/metrics080-host01502214.md) keeps
its own graph. Linux, Rust 1.88 and Apple Silicon pass in
[38280107658](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38280107658);
Intel passes tooling/archive steps but is cancelled during native qualification.
#56–#59 remain open for their exact native/archive/tooling scopes.
The preceding 0.22.13 run is now cancelled; its successful lanes remain historical.

Released 0.22.15 selects Testkit 0.35.0, unified Host 0.15.1,
Memory 0.35.19 and Metrics 0.8.0. Blob has no callers of Testkit's
removed generic build APIs/Host tools re-export and adds no IC Build dependency.

Pending compatible 0.22.16 applies the requested repository audit: current guides
replace accumulated obsolete setup/release instructions; the service contract
uses the current Memory 0.35 identity. Frozen publication batches own their exact
installation decoder, and store tests use the maintained grant assembler.
No public API, output/schema, effect policy or package metadata changes.
[The audit](../reports/audits/2026/10/13/repository-cleanup/01/report.md) records
scope, traces, intentional retention and qualification limits. The maintainer's
0.22.15 release ran before cleanup; its proof does not qualify these new edits.
All 113 CLI tests, 12 focused store/grant cases and scoped denied-warning Clippy
pass for the cleanup. The initial sandbox loopback refusal is retained separately
from the successful permitted local-HTTP run. Full new CI/native macOS and
downstream/provider qualification remain outside these focused results.

CLI Run retains the claimed directory descriptor; byte/JSON records converge on
Host's descriptor-relative create-new writer. Named ownership, no replacement,
private permissions and single-component names are enforced. Body transfers retain
failed body.part evidence and their verified EOF/sync/publication contract.
Ancestor custody and exclusion of concurrent directory writers remain caller-owned.

The reviewed Shared snapshot is now 0.9.10
`7e55adecf6f0b196c050f104b37f8c69b6bf434e`, matching upstream main at selection.
Its 112 payloads preserve the previous 110 and add the cache-key fixture companion
and the explicitly selected change-reporting rule. Existing product targets, twelve-member layout and pins
remain. Explicit common Cargo/Testkit setup completes for the changed build policy,
then offline admission passes; older generations and initial refusals remain.
Hosted tooling discovery precedes the existing exact pinned download/checksum path,
without repeated uploads or name-based selection.

The corrected canonical Cargo fixture runs directly from Blob's root on modern
Bash and genuine Bash 3.2, using its prepared compiler context and commit-free
history. The temporary wrapper is removed and Make calls the canonical fixture.
[Latest adoption evidence](../evidence/shared091002215.md) retains starting dirty
inputs and focused results; prior failures remain in the earlier cleanup packet.
Shared's cache workflow and producer-only artifact fixture remain producer policy.

## Focused qualification

[The cleanup evidence](../evidence/cleanup02215.md) retains current identities,
original refusals and exact limitations. Raw inputs/logs/artifacts are outside
Cargo clean at `.tmp/review-validation/cleanup02215.dFOUKG/`.
All 113 CLI tests, scoped denied-warning Clippy, Rust 1.88 native checks, fresh
probe/standalone Wasm and CLI builds pass. Actual local PocketIC restoration/reset,
independent credit/replay windows and rebuilt CLI installation-carrier cases pass.
The latter includes Candid/Wasm parity and wrong-service stable-byte preservation.
Affected common shell fixtures pass on modern Bash and genuine Bash 3.2.57 on
Linux; workflow lint, static exact-ID wiring and local tooling evidence checks pass.
No full Blob CI/release gate, optimized PocketIC, native macOS, provider/deployed
service qualification, publication, Git delivery or paid effect ran here.

## Canic boundary

Canic's wider Auth/Backup/Blob batch remains with its own owner under
[#517](https://github.com/dragginzgame/canic/issues/517) and
[#444](https://github.com/dragginzgame/canic/issues/444). The prior scoped #524
quota-zero fix remains. Its two native regressions now pass; prepared managed
cases test installed unknown-field/extra-argument rejection and rollback in both
compositions, using exact retained protected envelopes. Old locks/attempts remain.

Both installed cases now pass against frozen Canic/Testkit working-source inputs
in `/tmp/canic-524-isolated.v5md68w9/`, with their own target and explicit published
Testkit 0.35 CLI server owner. They retain the required fence after rejected
reinstall, preserve stable bytes/configuration/app state, and use actual operator
same-instance recovery. Scoped Canic/testing Clippy and final native cases pass.
Corrected source/required fixture locks are applied back only to unchanged, idle
original files. #524 remains open for family delivery. The standard source-path
runner refusal, interrupted check, first unfenced-view assertion and earlier
Timers 0.21 caller failure remain with their distinct recorded inputs. This is
local frozen composition proof, not the entire evolving Canic graph or provider.

Application-parent initialization remains Canic #493; guarded funding composition
remains #494, dependent on authenticated provider credit/payer facts in Blob #34.
Actual application two-user/verifier acceptance remains Blob #31/#33; media-map
transaction #4; serving #5; provider deletion/GC/billing #32/#6. No downstream
wrapper/fixture, fake provider evidence or funding bypass belongs in Blob.

[Prior handoff](review-before-02215.md) retains earlier Canic composition hashes,
probe evidence, Shared history and exact release identities. Historical guarantees
stay bound to their original sources and independent locks.
