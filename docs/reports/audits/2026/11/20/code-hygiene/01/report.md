# Wire 0.27 and Toko 0.45.25 adoption

**Verdict — PASS WITH FINDINGS.** Full typed schema comparison remains open.

Reviewed the four product crates, four qualification packages and their CI and
Cargo wiring on Rust 1.100.0, Linux x86_64. The starting point was `5a1bffe`
and its AGENTS.md, with pre-existing supplier refreshes in both lockfiles. The
Toko 0.45.25 registry archive has SHA-256
`8ee626aa8f6e773f28e9132fb5e9e63d66212ed4b0fe8de67f3468a46f5dc50e`.
The review used the [revision-bound hygiene method](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/code-hygiene.md).

| Finding | Disposition |
| --- | --- |
| Low · The standalone guard repeated Wire's frame-header length. | The validator's fixed-size argument now determines the stack buffer size. The guard still checks message length before copying the header. |
| Low · CI built both product Wasms without comparing their callable methods. | Linux PR CI uses IC Host Artifacts to enumerate methods from those exact builds and Toko to compare against the two 0.107.1 artifacts. It detects an intentional query/update mismatch. |
| Medium · Request and reply schemas can change while names and modes stay the same. | The published exporter could not describe the existing nonzero ID and size types in a compile probe. Typed mount descriptors and upstream projections are needed for full manifests. The endpoint suite checks a tagged client's literal upload inspection frame and its unknown-upload reply. See the [baseline scope](../../../../../../../../qualification/api/README.md). |
| Low · The public Rust codec type crosses a minor-version boundary. | Both workspaces select Wire 0.27.1. The pending minor changelog names the consumer migration. |

The pinned Toko lint auditor examined all ten manifests across both workspaces
and reported zero discrepancies. Source and manifest inspection found no
Blob-owned Wasm guards, Rust path overrides or tracked symlinks. The repository
Cargo recipes inherit the caller's network setting. The two public hosts and
the three private probes retain their existing owners. Local evidence is in
`target/hygiene-04525/`.

All seven aggregate CI stages passed, followed by the release-Wasm callable
comparison. All eight installed suites passed (21 tests), including the populated
standalone upgrade from 0.107.1. The endpoint fixture also passed separately
with the released frames. The 40 release-selection assertions, two-lock metadata
fixture, 31 diagnostics assertions and 69 workflow assertions passed. The opt-in
Chromium and two historical incompatible-layout fixtures were outside this run.
The aggregate proof is at `target/validation/blob.JBksCF/proof.json`.
