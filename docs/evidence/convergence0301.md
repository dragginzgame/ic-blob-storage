# Upload projection, per-file recovery and journal consistency

Source base is released 0.30.0 `e9f71822ae361377686846fe7a4bf7c0e9a6822f`,
plus incoming Testkit/Candid 0.49 manifest/lock edits. Their exact before-images
and the task's source before-images are retained under
`.tmp/review-validation/convergence0301/before/`; package versions are unchanged.
The graph selects Memory 0.44.0, Metrics 0.11.5, Host 0.22.1 and Build 0.10.0.

Certificate and history/discovery replies share the nine-field upload identity
projection in the existing admission boundary. Caller-specific role checks and
independent expected-value fixtures remain. Session recovery holds one entry per
file, with optional setup/transfer/observation paths. The persisted three-array
schema, its format string and strict cardinality remain unchanged. Saved paths
are checked in the original column order before transposition; later claims keep
the original path-conflict checks. No recursive recovery pointer or retry owner
is added. Sources are reconstructed before callers can use the assembled state.

Contracts supplies journal consistency alongside the unchanged arithmetic-only
budget predicate. Assessment retains its passive-profile blocker checks; native
status retains scope/gateway checks and its error mapping. Native status now
rejects malformed capacity/count/last-identity, allowance and accepted-credit
reports. These checks do not establish liquidity, credit, readiness or authority.
Notes target compatible 0.30.1; no public wire or durable layout changes occur.

Focused Linux checks pass 8 session, 9 native-boundary, 11 Contracts funding,
5 certificate, 2 discovery and 1 admission cases. Regressions cover frozen saved
source shape, short/long columns, path substitution and contradictory journals.
The first native-boundary attempt passed seven cases but could not open sandbox
sockets; its two failures and the passing nine-case loopback rerun are retained.
Strict all-target/all-feature lint and actual Rust 1.91 native checks pass for
the three changed packages. Warning-denied Rustdoc and public-library Rust 1.91
Wasm checks pass. Fresh CLI and standalone Wasm artifacts pass 11 installed
session/certificate/history/status cases, including recovery without redispatch.

The explicit PocketIC 16.1 executable matches the decompressed archive whose
digest is pinned by published Testkit 0.49. This does not qualify the selected
CLI installation or complete tool gates. Logs, source/Cargo/artifact hashes and
before-images remain at the raw evidence root above. These are local/substitute
results; full CI, native macOS, actual archives, Canic and deployed-provider
acceptance were not run. No sibling, release, publication or Git effects ran.
