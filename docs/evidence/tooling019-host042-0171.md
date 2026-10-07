# Shared Tooling 0.1.19 and Host 0.4.2 — pending Blob 0.17.1

Reviewed 2026-10-07 on released Blob base
`790649b36ce1da95cb2d41f99facb3afe8280b4d`. The maintainer authorized further
tooling/Host work for pending compatible 0.17.1. Entry Cargo selections are
preserved: testkit 0.21.1, all four Host packages 0.4.2, Memory 0.31.1 and
private-probe Metrics 0.2.7. Package versions and release receipt remain 0.17.0.
The earlier [testkit record](testkit-021-0171.md) retains its different
0.21.0/Host 0.4.1 inputs; it is not rebound to this graph.

## Source and adoption

A clean private clone exports only committed Shared Tooling
`a06e4719e3839b8eefcfb88ec8923aa88eb63ccc` (0.1.19) through its canonical
refresh helper. The prior 73-file snapshot is independently verified first.
The final 77-file snapshot explicitly adds the IC pin parser required by the
refreshed installer, both PocketIC checkers and their fixture. Newer dirty
upstream governance/contribution rules are excluded. No vendored code is patched.
The approved role-based layout and all 11 workspace members remain unchanged.

The canonical installer now refuses redirected Rust directories, executables
and Cargo receipts before probes/Cargo, and rechecks the route after installation.
Shared release finalization preserves historical EOF bytes and refuses varied
dated headings. The shared logger owns unique combined failed-batch logs and
`latest-combined.log`; `latest.log` still contains only the last failed target.
The local logging integration compares both selected failed targets' raw bytes
in dispatch order, excludes the passing target and checks the last-target view.

`make pocketic-alignment-check` compares the locked client and reviewed server
pin with offline Cargo metadata. Native-host, PocketIC and standalone test
targets require it after explicit cache preparation. Both versions select 16.0.0.
The shared fixture covers missing/ambiguous/mismatched graphs, failed producers
and external binary checksum/version refusal. CI retains failed fixture and
metadata directories. The production Make check does not authenticate arbitrary
override binaries; managed bundle admission stays with the existing checker.
No disk-capacity threshold or cleanup operation is introduced.

All published Rust `src/` files match committed publishers: Host 0.4.2
`6501d0e9fa7ba0439ec7a4010ca7bf0205e1d712`, testkit 0.21.1
`a98d751fb6991c12a5d0cd8faaddabe1e879199d`. All five official cached archives
match non-yanked sparse-index checksums and declare Rust 1.88. Published source
verification, registry rows and checksums remain distinct evidence classes.
Native/Wasm core normal-edge trees exclude Host, Testkit and Metrics. The CLI
keeps only artifacts/fs; process/tools remain testkit-owned. No new Rust caller
or function/method/type removal is needed. Provider hashing remains in the
platform-neutral core; descriptor/nonblocking opening and create-new body
publication preserve their existing contracts.

## Focused local results

Raw evidence is retained under `.tmp/tooling019-host042-0171-01/`.

| Check | Result / raw log |
| --- | --- |
| `make tools-check dependency-pins-check shared-tooling-check` | Pass, `prepared-checks.log` |
| `make shared-tooling-tests` | Pass in the actual exported Make environment, `make-shared-fixtures.log` |
| `make release-check hooks-check` | Pass with isolated effects and rollback/index proof, `release-hooks.log` |
| `make pocketic-alignment-check release-commands-check` | Pass, `alignment-make.log` |
| `make test-native-host` | 110 CLI, 21 example, two installation/carrier and one Metrics restoration case pass, `native-host.log` |
| Strict all-target/all-feature CLI and harness Clippy | Pass, `native-clippy.log` |
| Rust 1.88 all-target/all-feature CLI and harness check | Pass, `msrv-native.log` |
| Changed shell helpers, workflow lint, formatting and local links | Pass, `shellcheck.log`, `final-maintenance.log` |

Selected Rust commands use `--offline --locked`. Initial offline cache preparation
refused missing testkit 0.21.1; the explicit locked online preparation succeeds
without reselection. Both attempts remain retained. One registry read used the
wrong package shard and returned HTTP 404; the corrected separate read succeeds.
These are preparation/inspection failures, not failed native qualification.

Linux Bash 5 and nested GNU Bash 3.2 pass the Rust install/check/retry/refusal,
consumer logging and common Make-command fixtures. Directory-alias/trailing-slash
TMPDIR cases pass under both shells; the PocketIC checker fixture also passes
with that context under Bash 3.2. These use controlled substitutes, not real tool
installation or external writes, and do not establish native macOS/system-awk
execution. Prepared real host/IC/Rust bundle verification passes offline.

The current-source standalone/probe Wasm build targets succeed; unchanged core
inputs reuse their valid artifacts. Actual PocketIC installation and restoration
tests run against those matching Wasms. `tested-artifacts.sha256` binds their
bytes, the prepared server, both CLI executables and every executed test binary.
`source-files.sha256` binds the final tracked consumer files and newly exported
helpers. `SHA256SUMS` binds raw attempts, selected graph, registry/source review
and this record. Entry manifest/lock/receipt byte comparisons and released
changelog-history comparison pass; earlier artifacts are preserved.

## Remote acceptance and remaining limits

Host 0.4.2's [exact-source run 37624014360](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37624014360)
passes Linux/MSRV and fails both native macOS jobs. Failed/full-job log requests
return empty files; annotations identify exit status 2 but no concrete assertion.
The older unnormalized tool-command fixture is still present in that source,
and [Host #6](https://github.com/dragginzgame/ic-host-tooling/issues/6) identifies
it as the prior macOS blocker. It is a plausible explanation, not a newly proven
cause for this run. Local consumer success does not supersede that remote gap.

Testkit 0.21.1 corrects the earlier reported startup/workspace/CI fixture defects.
Its publisher runs [37635583503](https://github.com/dragginzgame/ic-testkit/actions/runs/37635583503)
and [37635582990](https://github.com/dragginzgame/ic-testkit/actions/runs/37635582990)
are queued/in progress at inspection. No hosted run is returned for the adopted
local Shared Tooling 0.1.19 commit. Dirty Blob changes have no matching remote
result. [#25](https://github.com/dragginzgame/ic-blob-storage/issues/25) and
[#26](https://github.com/dragginzgame/ic-blob-storage/issues/26) remain open for
committed consumer/native delivery. Published Memory/downstream integration
acceptance remains separately owned under
[#20](https://github.com/dragginzgame/ic-blob-storage/issues/20).

No full CI/release gate, package-version mutation, commit, tag, push, publication,
deployment, provider request or sibling file edit occurs. Library publication
does not qualify the service or downstream instruction savings.
