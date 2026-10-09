# Library adoption for pending 0.21.0

Released base is `71ddea9c0f4f007eba1b9f13500ff8b0f4ec1de6`, with validated
source `a58c62f7a0c0e589f3054098aaff1651e93b664e`. All twelve package versions
and the finalized receipt remain 0.20.0. Incoming Cargo files are preserved
byte-for-byte; no new resolution or unrelated dependency rewrite is performed.
Public Memory 0.33 changes the re-export's Rust type identity, making the complete
pending batch 0.21.0. Consumers must align Memory and rebuild; cross-release
retirement/reinstall requirements remain even though runtime schemas are unchanged.

## Selected source

Registry archive checksums match Cargo.lock. Every packaged Rust file and original
manifest in the seven selected libraries matches its recorded owner commit:

| Selection | Released owner revision | Relevant change |
| --- | --- | --- |
| Host artifacts/fs/process/tools 0.9.2 | `c5decaefd17809829bfa969966729d672f609c49` | Runtime crate sources unchanged from 0.9.1 |
| Memory 0.33.0 | `1cbecf601fd24afc64d1fcfccb29609537be36be` | Runtime sources unchanged from 0.32.0; public package identity changes |
| Private Metrics 0.3.1 | `2383dc0d684800b1610e3eb727449b0a87d562bb` | Arithmetic sources unchanged from 0.3.0 |
| Testkit 0.27.0 | `f2d9fc6f197bcb9e669a7abaa1135f4cdffdd9ee` | Structured startup failure/output/cleanup evidence |

The incoming tokio-util 0.7.20 selection and all unrelated lock bindings remain.
Shared stays at the reviewed 93-file 0.2.2 snapshot,
`ee48bb37c98c771e77b92fd891f0757d8c1c8b99`; dirty sibling changes are excluded.
No sibling file is edited.

Testkit's `PocketIcStartupError` retains its original `PocketIcStartupFailure`,
bounded output for the server owned by the failing operation and separate typed
command/server cleanup errors. Existing harness `expect` calls print the owner's
structured Debug diagnostics. There is no local matching of superseded enum
variants, compatibility alias or new error wrapper. The separate managed server
and instance ownership/drop order remain intact. This is source/API review;
consumer failure injection and native macOS acceptance are not claimed.

The bounded fixture reader and durable resource writer already delegate to Host.
Its new patch adds no runtime helper. Keep the buffered browser reader, probe
partial evidence/phase diagnostics, verified download publication, nonblocking
followed-symlink admission and recovery path confinement: their contracts differ
from the generic Host helpers. No function, method or type is removed.

## Focused local qualification

Explicit `make install-testkit` prepares the lock-selected 0.27 CLI and
authenticates the existing PocketIC 16.1.0 server. Offline consumer checks and the
real Testkit check with `PATH=/nonexistent` pass. Prior 0.26 CLI bytes, the old
six-tool bundle's eleven captured files and the selected server bytes remain
unchanged. Routing fixtures pass Bash 5 and Linux-built Bash 3.2.57, including
explicit binary overrides, missing/ambiguous locks and failure propagation.

- `make test-native-host`: 242 passing cases, including 115 CLI/probe, 103
  contracts, 21 examples, two real standalone installation/carrier cases and one
  Metrics restoration-read case.
- Six additional existing file/process cases pass: four bounded inventory inputs,
  exact resource artifact bytes/refused replacement, and prefetched subprocess
  output/stderr/nonzero status retention. They create no IC instance or HTTP probe.
- Strict CLI/harness Clippy across targets/features and actual Rust 1.88 native
  CLI/harness and standalone/probe Wasm compilation pass.

Raw entry Cargo files/diff, cache preparation, owner source comparison, tool
readback, logs, compiler-input manifest and eleven exact executed/fixture artifacts
are retained under `.tmp/libraries0210-01/`. All 17,750 captured inputs remain
unchanged during qualification; earlier graph results and failures are not relabelled.

The [intent](caffeine-probes/local/2026-10-09-libraries0210-01/intent.json) predates
the local substitute qualification; the
[summary](caffeine-probes/local/2026-10-09-libraries0210-01/summary.json) binds
its results, graph and artifact hashes. Four explicit local probe GETs, zero
deployed provider requests or paid effects. Successful temporary probe bodies and
request files follow existing test cleanup and were not separately archived;
source assertions, logs and executed bytes survive. Owned local scopes close and
no matching owned executable remains at readback. No external cleanup obligation.

## Acceptance limits

Released 0.20.0 [consumer CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37924115927)
and selected Testkit 0.27 owner CI are queued at readback. They do not establish
new native macOS acceptance. [#39](https://github.com/dragginzgame/ic-blob-storage/issues/39)
stays open for committed consumer setup/check and actual product startup on all
three native hosts. Dirty pending 0.21.0 has no remote CI result or full release
gate. Application/provider qualification remains with #31/#33, GC with #32 and
real funding evidence with #34. No commit, push, release or publication occurs.
