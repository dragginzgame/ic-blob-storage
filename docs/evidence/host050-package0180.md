# Cold package verification and Host 0.5.0

Date: 2026-10-08. Additional evidence for
[#27](https://github.com/dragginzgame/ic-blob-storage/issues/27), separate from
[the extraction record](contracts-0180.md) and
[the sanity pass](contracts-0180-sanity.md). Earlier inputs, failures and summaries
remain frozen; this record does not overwrite their evidence.

## Actual identity

Investigation started at `8dd87013068c0560b6baae01d0a332050d20ab56`, with incoming
Cargo edits selecting direct native Host artifacts/fs 0.5.0 and Memory 0.31.3.
Testkit 0.21.3 retains its declared transitive Host 0.4.6. Compiled/package identity
was **0.17.2**, pending the contracts hard cut at **0.18.0**. Entry manifests, lock,
receipt and both archives are retained in `.tmp/package-host-0180-03`.

During investigation the maintainer committed the package repair and completed
0.18.0: release `704b8ebf6bea85a715e465e32e34758b601852ec`, validated source
`a43aee3b2593574a326b9b54651ad4946e3dec24`, retained completed plan
`.git/release-state/0.18.0.plan`. That maintainer release is distinct from the
agent's scoped checks; old compiled artifacts are not relabelled 0.18.0.

## Package failure and executable regression

The reported 9 library / 10 test compile errors reproduce in
`package-before-fix.log`. All missing checked-input/configuration getters and
funding/gateway helpers exist in the extracted contracts source. Cargo archives
normalize source timestamps. Reusing the root target across extracted crates
with identical package/version identity admitted older contracts artifacts.
It can also return a false green without compiling the updated payload.

`verify-library-packages.sh` now gives every retained extracted verification its
own Cargo target. A real Cargo fixture verifies two successive same-version
contracts/core archives, warms the former shared target, then executes the second
binary. The private old-verifier control fails with stale value **1**; the repaired
verifier passes with updated value **2**. The fixture runs before actual package
verification. No Cargo clean, compiler-method workaround or public API addition.

The fresh payload run passes **517** library/unit/integration/example cases and
**two** doctests with exact frozen external selections. Its workspace is
`target/package/blob-verification.1JpIgF`, with its own `target/`; the retained
`verified-payload-inputs.json` hashes 473 payload/manifest/metadata files.
The root lock remains selected. `package-regression-before-fix{,-final}.log`
preserve both controls; `package-regression-fixed{,-final}.log` preserve successes.
The original failure workspace and each tiny fixture remain at their printed paths.
Earlier warm-package success logs cannot establish exact updated-payload
compilation; this correction is additional evidence, not a rewrite.

## Host owner and current consumer checks

All four Host 0.5.0 registry versions are non-yanked and match released owner
`db637fac8b7a9ef62301e1d9009ffeb5ffcd0be7`. Cached official artifacts/fs archives
match registry checksums, VCS source and all 19 / 18 Rust source files respectively.
Primary registry rows and latest-version reads are retained individually. The
[exact Host CI run](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37744999108)
passes Linux, Intel/ARM macOS and MSRV; raw source-bound jobs remain retained.

Source review covers numeric gzip levels, expanded Wasm facts/resource profiles,
process-group cleanup and closed-writer filesystem validation. Blob's bounded
reader, raw digest and durable filesystem calls are unchanged by these additions.
No gzip/Wasm admission migration or extra direct process/tools dependency is
needed here. Testkit's Host 0.4 types remain with Testkit; forcing minor-line
unification would invent a different dependency contract. No sibling file edits
or new Host defect are introduced.

Under the prior [intent](caffeine-probes/local/2026-10-08-host050-0180-03/intent.json),
`make test-native-host` passes **114** CLI/probe, **103** contracts, **21** examples,
**two** installation/carrier and **one** Metrics restoration case. Actual standalone
and storage Wasm are rebuilt; roles and servers are maintained local substitutes.
Affected strict Clippy and Rust **1.88** native/Wasm checks pass. Standalone Candid
readback is byte-exact against the retained released-0.17.2 baseline and maintained
service.did, SHA-256 `1770ce8dac80bec5f167572990866e5312dffd36f773310b98cb08b891c3c2cd`.
Raw logs/inputs are hash-bound by the
[summary](caffeine-probes/local/2026-10-08-host050-0180-03/summary.json).

Root native/Wasm bytes were not hash-frozen before the maintainer's release
validation; later root binaries must not be attributed to these checks. The exact
extracted payloads and saved Candid readback remain retained. These checks do not
qualify newly rebuilt 0.18.0 deployment bytes, Blob native macOS or the deployed
provider. Live provider requests and deployed paid cycles are zero. Owned local
servers/PocketIC scopes drop; no external cleanup is pending. No instruction,
build-time or Wasm-size gain is measured. Shared Tooling remains reviewed 0.1.20.
