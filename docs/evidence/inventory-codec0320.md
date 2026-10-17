# Inventory query codec repair

Date: 2026-10-17. The reported library run failed 12 inventory observation cases
with `InvalidReply`. Their reply fixtures still used Candid, but capacity and
discovery validation had moved to CBOR. Inspection also found Candid requests,
a Candid reference-capacity decoder and three matching Candid admission-probe
query exports. Fixing only the reply fixtures would leave the actual command broken.

The inventory command and its three query routes now encode/decode CBOR through
Contracts. The admission probe shares bounded CBOR ingress and reply adapters
for those routes. Other probe commands and lifecycle inputs retain their separate
Candid fixture contract. Capacity/discovery binding validators, exact reference
echo checks, passive query ordering, failure mapping and blocker calculations are
unchanged. Reference replies still refuse over 4,096 bytes before decoding, with
malformed/trailing input distinguished from excessive input. Existing coverage
now also asserts those reference-reply refusals.

The new probe adapters explicitly depend on Serde. Cargo.lock gains only that
private member edge to the already locked package; external package identities,
versions and checksums do not change. The starting graph already selected Wire
0.3.3; earlier Wire 0.3.2 and prepublication receipts retain their original scope.
Own versions remain 0.31.0 and pending notes remain 0.32.0. Incoming unrelated
work is preserved in the retained before-image.

All 25 PocketIC library cases pass, including the reported 12. Both affected
packages pass strict all-target/all-feature Clippy and actual Rust 1.91 checks.
A fresh admission Wasm passes six installed cases: three capacity cases, two
actual inventory executable cases and one content-discovery case. They cover
caller/scope refusal, byte admission, retained history, suspension, unchanged
accounting, absent/pending/live/retired content, reference demand, tenant isolation,
denied access and invalid-report refusal without partial output. Local fixture
substitutes establish neither deployed provider behavior nor Canic integration.

Raw commands, source snapshot, prepared input hashes, server admission, artifact
hashes and results remain under `.tmp/review-validation/inventory-codec0320/`.
The existing PocketIC 16.1.0 archive/binary/version is admitted before use; the
harness owns instance/server cleanup. Zero deployed requests or paid effects run.
The initial locked command refused the missing member dependency edge; explicit
offline metadata reconciliation added only that edge, followed by successful
locked checks. The original failure remains retained. Formatting, declaration,
documentation and diff checks complete the scope. No full CI, release, sibling
edit, Git index/ref change or publication action runs.
