# Standalone Candid and callable Wasm qualification

Date: 2026-10-12. Released base: Blob 0.22.12,
`601b09b1add57b9fb9a11293c03e704ffce9fec2`, validated parent
`8bae5f91f2d22a3480991829bf7cd33db2dd5c8b`. The release receipt's source/version
and committed release-file hashes verify. Pending compatible notes are 0.22.13;
package versions and finalized receipt remain 0.22.12.

## Integration boundary

The existing standalone check compares canonical Candid with generated native
types. Native registration equality alone cannot prove compiled dispatch.
Canic's prior imported-emitter failure and its now-qualified lexical emitter
are retained in the [earlier composition review](shared083-canic02212.md).

The standalone check now also derives method names and query/update/composite
query modes from canonical Candid and compares them with actual callable Wasm
function exports. Host artifacts owns structural Wasm inspection. IC CDK's
reserved internal-method namespace is excluded; lifecycle and non-method exports
are not application methods. No second endpoint inventory, binary parser,
framework dependency or lifecycle owner is added. Native type equality remains.

The existing native-host and PocketIC installation-test group select this check.
The optimizer qualification script now explicitly runs it for original, O3, Os
and Oz artifacts before its existing installation/restoration cases. This covers
standalone output; Canic still owns its actual mounted application artifacts.

## Focused execution

Current selected inputs are Memory 0.35.17, Metrics 0.7.3, Testkit 0.34.5 and all
four Host 0.14.9. Incoming Metrics/Testkit edits remain intact. The only new lock
change is a private-test dependency edge on already selected Host artifacts,
with its Wasm feature confined to native test support. Production library
dependencies, public APIs and schemas remain unchanged.

- Offline locked release standalone build and exact endpoint test pass.
- Admitted Binaryen original/O3/Os/Oz artifacts all pass exact endpoint equality.
- A labelled eight-byte empty core Wasm substitute passes structural parsing
  but fails endpoint equality after native Candid equality succeeds. Its failure
  is retained, not treated as a product failure or native installation proof.
- Warning-denied Clippy and Rust 1.88 native checking pass for the affected
  standalone test target; formatting, declarations, snapshot, shell/lint,
  documentation links and diff checks pass.

The initial locked build refused the new private dependency edge. An offline
Cargo check admitted that edge without changing any prior package selection;
the subsequent locked build/test passed. Both attempts remain retained. Root
manifest and finalized receipt match their pre-edit bytes, and the real index
remains unchanged.

Raw source/lock inputs, logs, admitted optimizer identity and four artifact
hashes remain at `.tmp/review-validation/canic-issue-pass-20261012/`, outside
Cargo clean. This is local compilation and static artifact qualification, not
new PocketIC execution, Canic runtime acceptance or deployed provider evidence.
No full CI/release gate, sibling edit, Git delivery, publication or paid effect
is performed.

## Issue disposition

[Blob #61](https://github.com/dragginzgame/ic-blob-storage/issues/61) remains closed
for its published owner and prior qualified normal adoption; this change adds
an upstream regression boundary. [Canic #493](https://github.com/dragginzgame/canic/issues/493)
and [#494](https://github.com/dragginzgame/canic/issues/494) remain parent-authority
and guarded-funding work at their owning adapter. Provider credit/account facts,
GC/billing, real verifier operation, browser and application media-map acceptance
remain their existing Blob/application issues. No missing new Blob runtime API
was established by this review.

Matching released [0.22.12 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/38249204646)
is queued at inspection; 0.22.11 and 0.22.6 are cancelled. The earlier 0.22.9
archive failure is already repaired in 0.22.10. [#56–#59](https://github.com/dragginzgame/ic-blob-storage/issues/59)
retain native/archive/tooling acceptance. Those sources are not qualified merely
by release, artifact presence or these focused checks.
