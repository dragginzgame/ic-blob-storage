# Cursor, reply and replicated client cleanup

Date: 2026-10-13. Blob base: `7ad76317fb1ed2695c16ede4aabee655ded84187`.
Task baseline: existing dirty 0.23.3 source, retained in
`/tmp/blob-fourth-cleanup-before`. The earlier cleanup and Shared adoption work
is preserved. Shared 0.11.0 remains at
`a15ff323449f4b6190f647aec0de43cbcfb314ba`; incoming registry selections are Host
0.17.6, Memory 0.36.3, Metrics 0.9.1 and Testkit 0.37.0. The lockfile already
selected this graph at task admission; no dependency selection is changed here.
Admitted Cargo.lock SHA256:
`38ca6cda954b9c30e0dd3b34962cb644ce5eb69f432bf976a52555f9ca46254f`.

## Ownership and compatibility

Funding and upload history cursors use native `parsing::positive`, removing two
private decimal parsers. Saved JSON shapes, canonical positive u128 syntax,
2048-byte file bounds, full-width identifiers and distinct cursor scope checks
remain with their original owners.

The contracts crate's private `decoding::limits` constructs Candid configuration
for eleven reply boundaries. Each boundary still selects its own work, skipping,
type and header budgets, bounds encoded bytes before decoding, authenticates
through its adapter and checks original request bindings afterward. Configuration
envelopes and persisted-record decoders remain independent. No decoder is changed
to accept extra arguments or different unknown-field behavior.

The service's private `client` module admits principal/timeout syntax, builds one
bounded-wait call and classifies platform call failures. Seven clients retain
their existing public types, configured identities, role-specific predicates,
execution checks and validation order. Dispatch attaches no cycles and adds no
retry, fallback, journal, endpoint or lifecycle owner. Domain reply errors remain
distinct; raw rejection codes and not-enqueued outcomes preserve their meanings.
Three private error classification functions converge at the shared owner.

The compatible pending release remains 0.23.3. No generated Candid, persisted
schema, public API, CLI output or consumer migration changes are required.

## Qualification

Logs and admitted input hashes are retained in
`.tmp/review-validation/fourth-cleanup0233/`. Source and contract review follows
the adopted flow-convergence audit and authorized module-cleanup procedure.
Checks use Rust 1.99.0 on Linux and the unchanged locked dependency graph.
All 103 contracts and 346 service-core unit tests pass. All 113 unique native
CLI cases pass: the initial sandbox run passed 110 and refused three local
socket binds; those cases pass outside the sandbox. The first log remains
retained. Strict all-target/all-feature Clippy passes for the three affected
packages. Rustdoc, formatting, documentation links and runtime-free contract
graph checks also pass. Source comparison confirms all eleven original decoder
budget tuples are preserved.

The consumer and storage probes are rebuilt from the current source for Wasm;
their hashes are retained. Eleven installed client cases and eighteen consumer
transaction cases pass with the explicitly admitted Testkit server. These cover
all seven clients, exact identities and role distinctions, rejected query
dispatch, small/unusable replies, callback traps, cancellation, restoration
fences and original-intent recovery without redispatch. The local fixtures do
not qualify deployed provider behavior. Manifest, lock and snapshot hashes
remain identical to task admission.

Full CI/release validation, native macOS and deployed provider/Canic acceptance
remain outside this focused local qualification.
