# Blob Storage code hygiene review

**Verdict — PASS.** Four low-severity findings were fixed and verified in the local development tree.

On 18 November 2026 the maintainer requested a repository-wide pass during the upgrade to Toko Tooling 0.45.8. The reviewed tree was local main at `23f7e84b0bcb082d6e510a96e8a1a84a327e7bc5` with the pending Wire 0.25.0 adoption and the changes described below. This is the first review under this method. The repository’s AGENTS.md and product design supply the ownership rules. [Toko code hygiene method at the selected revision](https://github.com/dragginzgame/toko-tooling/blob/626da7cd9626cc2e152594862a2d82ea82ef51da/audits/code-hygiene.md)

## Findings closed

| Owner | Finding and disposition |
| --- | --- |
| Browser transfer and custody | **Low.** Two implementations checked the gateway and provider selectors. Both accepted an empty query or fragment on a gateway because the URL getters returned an empty string. Both entry points now share one check. Regression tests exercise rejection before upload claims, custody creation and reads. The byte-transfer entry point still has zero npm dependencies. |
| Native CLI | **Low.** An input path could insert a line break in terminal diagnostics, and a resolution failure dropped the filesystem reason. The CLI escapes the path and retains the reason. A test exercises a failing filename containing a newline. Maintenance now uses the CLI’s existing hex formatter for its custody directory. |
| Rust hosts and qualification | **Low.** Enrollment’s durable record definition was interleaved with Wasm storage operations. The installed probe now keeps those operations in one gated module. Native tests exercise the same record and request encoding. Mounted methods and guards describe their roles, and test declarations sit beside their parent modules. |
| Installed CLI tests | **Low.** Several assertions accepted a lossy conversion of command stdout. They now check the complete byte stream for valid UTF-8 before inspecting text. Lossy rendering remains appropriate when printing stderr as failure context. |

## Scope and retained decisions

The review covered the four product crates, four private qualification packages, browser modules and tests, CI and release scripts, dependency selections and installed tooling. A scan of the 84 authored Rust files found no module path overrides, file/directory module collisions, unsafe blocks or item-level lint allowances. The source inventory contained no symlinks. Both workspaces inherit the same function-length lint policy. The long management and issuance scenarios exercise ordered recovery steps against one installation, so splitting them into independent tests would lose the state the later assertions inspect.

The service owns authorization, references and declared-byte accounting. Hosts capture the caller and reopen storage. Invalid Wire inputs are rejected at the host boundary. A trap during a synchronous mutation remains part of the rollback contract. The admission subprocess runs in the operator-selected checkout and returns an exit status. Browser upload still requires the caller’s durable claim before its first PUT.

## Verification

Local runs used Rust 1.100.0 on Linux x86_64, Node 24.21.0, Chromium and the prepared PocketIC server. The two Cargo lockfiles and the CLI snapshot digest agree with the 92-file Toko 0.45.8 payload. `cargo machete` reported no unused dependencies in either workspace.

- All seven stages of `make ci` passed, including 39 library tests, four example tests, 22 CLI tests and 25 browser-client tests. Source validation evidence is in `target/validation/blob.IB1G60/proof.json`.
- All eight installed suites passed. Chromium exercised successful transfer, cancellation, lost replies, reload with durable custody, and publication in the installed catalog. Populated hosts from 0.103.3 and 0.104.0 retained their state after refusing incompatible upgrades. A populated 0.105.0 host reopened under the candidate.
- The release selection, metadata, workflow and failure-evidence checks passed. Run logs and the categorized Rust line count are in `target/hygiene-0458/`.

The scope covers checked-in code and local fixtures. Live Caffeine behavior, hosted release delivery and Canic integration remain under their respective qualification procedures.
