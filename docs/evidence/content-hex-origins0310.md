# Probe content, fixture hex and browser endpoint convergence

Source base is 0.30.0 `e9f71822ae361377686846fe7a4bf7c0e9a6822f`, plus the
prior dirty preparation and incoming dependency selections. Exact before-images,
starting status and source/command/artifact evidence are retained under
`.tmp/review-validation/content-hex-origins0310/`. The starting graph already
selects Build 0.11.0, Metrics 0.11.7, direct Host 0.23.1, Memory 0.44.0 and
Testkit/Candid 0.49.0. All incoming registry selections remain unchanged.
Only direct development declarations for the already locked hex decoder and
their four workspace lock edges are added; normal package dependencies are unchanged.

## Ownership and retained boundaries

The private fixture protocol owns passive original-content and lifecycle
projection for the heap admission and durable storage probes. Lookup authority,
error categories, owner borrows and borrowed/owned header handling remain at each
adapter. Root-batch and gateway callbacks use the same projection; uncertain,
cancelled, physically deleted and economically unsettled states stay distinct.

Test fixtures use the existing hex decoder instead of local nibble/pair loops.
All 64 retained hex/JSON inputs are unchanged, with hashes at the evidence root.
Fixture loaders retain file selection, edge whitespace handling and failure policy;
fixed-width JSON hash vectors now require their complete 32 bytes. No fixture is
regenerated from the implementation being tested. Production provider hash syntax
and the canister funding substitute's reply construction keep separate contracts;
this cleanup does not add a runtime dependency to either owner.

Browser validation shares only a predicate over an already-parsed URL. Certificate
and worker callers still validate their own boundary. Certificate URL-constructor
behavior, worker string/4096-character limits, caller refusal codes and HTTPS-only
gateway selection remain independent. Credential/path/query/fragment refusals and
loopback HTTP admission happen before journal writes or transport. The new cases
exercise both factories separately, not only the shared predicate.

## Focused Linux qualification

- 47 provider, one public balance-boundary, two onboarding, 11 upload-input,
  three installation and four probe cases pass: 68 unit/boundary cases. The
  admission probe currently has no unit cases; its installed cases provide proof.
- Fresh admission/storage Wasms pass 20 installed PocketIC cases covering original
  content identity, owned metadata, root batches, gateway callbacks, all cleanup
  phases, exact live references, suspension and fenced restoration.
- Rebuilt browser bundles pass 58 worker, 35 publication and 17 actual Chromium
  bootstrap cases. Their retained summaries report no IC/provider requests.
- Seven directly affected packages pass strict all-target/all-feature lint and
  actual Rust 1.91 checks. This includes the optional package verifier under
  incoming Build 0.11/Host 0.23; the former type mismatch is resolved on this graph.
  The public libraries pass separate Rust 1.91 Wasm checks. Fixture Rustdoc passes
  with warnings denied; source formatting, dependency and documentation checks pass.

The initial two sandbox browser-tool checks returned empty npm version output;
their failed logs are retained. The same prepared Node 24.21.0/npm 12.2.0 selections
pass outside that subprocess restriction before bundling/browser execution.
PocketIC 16.1's explicit existing executable was independently re-admitted against
published Testkit 0.49's archive hash and decompressed payload before fixture
building. This does not qualify lock-selected CLI installation or full tool gates.

Public API/storage behavior is unchanged by these cleanups. Notes remain pending
0.31.0 because the earlier public decoder-limit change requires that hard cut;
package versions are untouched. Full CI, actual archive qualification, native
macOS, Canic and deployed-provider acceptance were not run. Prior failed evidence
retains its source/graph scope. No sibling, Git, release, publication or paid effects ran.
