# Current status

Date: 2026-09-25

The maintainer authorized creating and setting up
`/home/adam/projects/ic-blob-storage`, then requested sibling-style AGENTS and
patch/minor release tooling. Work is confined to this repository. Canic and
other siblings were inspected read-only.

## Completed setup

- Rust 2024 workspace and dependency-free ic-blob-storage library scaffold,
  pinned to Rust 1.98.1 with isolated build output.
- MIT license, expanded normative AGENTS, development/release governance,
  changelog and B1 service-contract checklist.
- Patch/minor/major and exact-version preparation, corresponding one-shot
  maintainer releases, individual staging/commit/push commands and separate
  registry publication commands. Effect-free release-plan is available.
- Release preparation validates clean committed input, updates Cargo/lockfile
  and changelog, and generates a source-bound record with release-file hashes.
  Failed preparation restores its inputs. One-shot success ends with cargo clean.

Git is initialized on main, with no commits, tags or remote. The package
remains version 0.1.0 and publish=false. No actual version transaction, staging,
commit, tag, push or publication ran. Commit-producing targets are human-only;
agents must never execute them.

## Validation

Bootstrap formatting and strict package Clippy passed previously. The release
tooling now passes Bash syntax, ShellCheck, Perl syntax and focused regression
tests for version selection, first-release handling, changelog finalization,
dirty/tagged-source rejection, failed validation, concurrent source changes,
failed lockfile/metadata updates, rollback, staging and receipt tampering.
Substituted Git/Cargo/validation commands also prove one-shot ordering, exact
atomic push selection, success cleanup and failed-push continuation. These
tests create no real commits or network effects.

Make help, release previews and dry-run command graphs resolve. Real offline
Cargo packaging and package compilation pass; Cargo reports the expected
missing repository/homepage/documentation metadata while those publication
locations remain unassigned. Full CI/release validation was not run.
All evidence establishes scaffold/tooling behavior only.

## Remaining gates and next work

Canic's exact 0.110 closeout still requires human acceptance before extraction
implementation. This setup neither accepts that verdict nor starts B2.

After that gate, finish B1 with named service/consumer/operator owners, the
concrete application, final package/publication plan, removal and obligation
inventories, and actual-provider evidence. Freeze the
[service contract](../service-contract.md) covering deployment, authority,
identity, accounting, restore and retirement. Canic removal remains B3 work
after qualified service publication.

Before enabling registry publication, assign ownership/remote/package metadata
and extend release-verify with the implemented service's actual PocketIC and
recovery qualification. Read [the release guide](../releasing.md) before using
the installed maintainer commands. No service API, provider effects or
canister endpoints are implemented.
