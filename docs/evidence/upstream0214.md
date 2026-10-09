# Upstream continuation for pending 0.21.4

Released base is Blob `6045ad3ada915b771e0ba6d40ac880da43ea675e` (0.21.3),
validated source `f29448c08b4df1215491e6b6d0a00a94e57bfdfb`. All twelve
package versions and the finalized receipt remain 0.21.3. Preserve the incoming
lock-only selection of Host artifacts/fs/process/tools 0.9.7 and private
Metrics 0.3.4; only their versions/checksums differ from HEAD. Memory 0.33.3,
Testkit 0.27.2 and PocketIC 16.1.0 are unchanged. Pending 0.21.4 is compatible.

## Reviewed source and adoption

Canonical export from a clean isolated committed checkout adopts all 95 files
from Shared 0.2.8 `b2646cde9abbc8861857a4379c683a0c19eba43e`. Do not adopt
the sibling's uncommitted 0.3.0 work. The changed execution include selects its
adjacent behavioral probe independently of ambient tooling roots and uses the
actual Make executable instead of passing recursive arguments into the probe.
Unsafe modes remain rejected before recipes. Consumer fixture inputs remain
complete; no local snapshot patch or new helper owner is introduced.
[#44](https://github.com/dragginzgame/ic-blob-storage/issues/44),
[Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30).

Selected crate archives match Cargo.lock and unyanked sparse-index observations.
All 165 packaged Rust/original-manifest files match these committed owners:

| Owner | Release | Revision |
| --- | --- | --- |
| ic-host-tooling | 0.9.7 | `ca62e661918db2f4320743b9042a4993a5fff2aa` |
| ic-memory | 0.33.3 | `03e2c78e21318127fc82692ee77df763be54788b` |
| ic-metrics | 0.3.4 | `5a5f1dab1f7ee3e1e5624c9d889148c39abf45f2` |
| ic-testkit | 0.27.2 | `1a8f2ff570ea1c3bd58b215e28af52e5d99870e8` |

Host's 0.9.6 text hex parser consolidation is included in 0.9.7; JSON empty
responses, text empty rejection, typed errors and limits remain unchanged.
Blob has no direct caller of that decoder. The other Host crates' runtime
sources are unchanged from 0.9.5. Memory, Metrics and Testkit patches also leave
runtime sources and original crate manifests unchanged from their preceding
patches. No new public helper justifies additional Blob symbol removal.

## Focused local execution

Owner Make formatting, release routing and Git hook suites pass Bash 5 and
Linux-built Bash 3.2.57. Actual consumer release routing, hooks and formatting
pass both; `make release-check hooks-check fmt-check` passes Bash 5, including
the complete local adapter suite and its 60 direct/inherited mode refusals.
Release effects are isolated Git/Cargo substitutes. Actual consumer `help`
also admits `MAKE` arguments and both environment/command-line ambient roots
without dispatching an unselected sentinel helper. This does not prove macOS.

Nine existing native file/process cases pass: three bounded artifact/refusal
cases, four inventory cases, buffered subprocess response/stderr/status and
immutable retained resource artifacts. Eleven service-store cases also pass.
Rust 1.88 checks CLI/harness native targets/features and core/storage-probe Wasm
libraries/features. The storage probe is Metrics' actual consumer. No new
PocketIC canister execution or deployed-provider qualification is inferred.
The existing Testkit 0.27.2 offline CLI/server check succeeds without replacing
server bytes, old CLI slots or receipts.

Entry files/diff, registry/source/archive readbacks, exact test artifacts,
compiler inputs and raw commands/results remain in `.tmp/shared0280214-01/`.
All 14,646 captured compiler inputs remain unchanged; entry Cargo files and the
receipt remain intact. Initial offline metadata refused missing Host-tools
0.9.7; explicit `cargo fetch --locked` prepared that archive without resolution.
Initial guessed Host paths and GraphQL issue reads failed; corrected repository
identity and explicit JSON issue reads succeed. Earlier evidence keeps its
original graph. No removed Blob Rust/shell function, method or type.

## Hosted acceptance and limits

Released 0.21.3 [CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37956676496)
passes Linux/MSRV, including exact artifact readback, tooling and native product
qualification. Host 0.9.7's [CI](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37958032736)
also passes Linux/MSRV. Both repositories' macOS jobs remain queued at readback.
Dirty 0.21.4 has no hosted/full-gate result; #39/#40/#43/#44 retain their native
acceptance requirements. [#45](https://github.com/dragginzgame/ic-blob-storage/issues/45)
still awaits [Shared #94](https://github.com/dragginzgame/shared-tooling/issues/94)'s
proposed exact registry observation contract. No sibling file edit, full CI,
hook activation, agent commit, push, release, registry publication, deployment,
provider request or paid effect occurs.
