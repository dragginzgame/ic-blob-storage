# Wire 0.26 and Toko 0.45.21 adoption

**Verdict — PASS.** All seven CI stages and all eight installed suites passed.

The review covers the four product crates, the qualification workspace and their
dependency declarations on Linux x86_64 with Rust 1.100.0. The baseline was
`890d53a` on main. The working tree already carried the Platform lockfile
refresh and the CLI and test feature cleanup. The review used the repository's
AGENTS.md at that commit and the [revision-bound Toko hygiene method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/code-hygiene.md).

| Finding | Disposition |
| --- | --- |
| Medium · Five canisters hid their entrypoints or error hook from native checks. | The locked registry Runtime 0.23.22 provides the required raw calls, system API and outbound call signature. Raised the minimum in both workspaces, moved four dependency tables and removed five Wasm guards and their five dead-code exceptions. Platform owns the export macros. |
| Low · Endpoint headers described caller roles as session scopes and omitted reply limits. | Reviewed all 31 lifecycle hooks and methods against their attributes, guards, handlers and codecs. Headers now identify the actual admission guard, handler permissions and budgets. The certificate response allows 1,024 bytes for a 4,096-byte request. |
| Low · Rust layout and the installed lint policy had diverged. | Both workspaces admit Toko's entity-doc spacing. Updated the public configuration and maintenance records in the Wire protocol and separated preceding statements from returned values across the two source trees. |
| Low · Consumer documentation still named older suppliers. | Updated the README and recorded the Wire 0.26 Rust type boundary in the pending minor changelog. Both graphs resolve a single Wire version. Blob uses its bounded frame and payload codecs. |

The published Wire sources differ in the optional schema module and its exports.
The encoder, decoder and frame implementation files are identical between
0.25.2 and 0.26.0. Neither Blob graph selects the optional schema features.
The standalone upgrade fixture uses the previous release's exact Cargo.lock
and a Wasm rebuilt from `v0.106.3` (SHA-256
`35865e966854ac8c676813baa9364dba6fe312514355861110bbaac470cccda4`).

The source inventory covered 81 Rust files. It found no module path overrides,
file/directory module collisions, Blob-owned Wasm target predicates or tracked
symlinks. Qualification's large management and callback fixtures drive installed
CLI and canister behavior. They remain in the private test workspace. The pass
did not change custody, storage layout or provider calls.

The aggregate gate covered strict Clippy, Rustdoc, formatting, dependency and
snapshot admission, documentation links and 25 Node browser-client tests. The
eight installed suites passed 21 local simulator tests. These include the
populated standalone upgrade from 0.106.3, reply-encoding rollback, delayed
authority and signed-principal isolation. The five rebuilt release Wasms export
the same 31 IC entrypoints recorded for 0.106.3. The opt-in Chromium test and
two historical incompatible-layout fixtures were outside this run.

Local evidence is in `target/hygiene-04521/`. The aggregate gate retained its
proof at `target/validation/blob.kTCW2a/proof.json`.
