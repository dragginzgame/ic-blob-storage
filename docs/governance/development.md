# Development and release governance

This document owns command, validation, version and publication policy.
[The release guide](../releasing.md) describes the executable workflow.

## Authority and repository boundaries

Ordinary implementation and continuation authorize scoped local work, not
version changes, Git publication, deployment or paid provider effects.
An explicit maintainer instruction authorizes the exact named action; no
second confirmation or magic phrase is required when the target is clear.
Agents must never create or amend commits, directly or through helpers.
Consequently commit-producing one-shot release targets are human-operated.

Sibling repositories remain read-only unless explicitly named and authorized.
Preserve unrelated dirty work. Never alter source beneath an active validation
run or compete for the same build directory. This repository's Makefile owns
its local target directory; do not redirect builds into Canic.

## Development and evidence

Use targeted checks while implementing. Full CI or release validation requires
an explicit request or an explicitly authorized version/release target.
Primitive Make targets do only their named operation. The complete current
gate is make ci (also make validate and make release-verify).

Keep implementation, relevant success/rejection/recovery evidence and cleanup
in one coherent batch. Tests assert typed failures or observable behavior;
do not add placeholder tests or exact aggregate-count assertions.
Canister lifecycle/inter-canister tests use PocketIC. Native tests and provider
substitutes do not prove deployed provider guarantees.

Release guards may enforce versions, hashes, schemas, exact identifiers,
executable behavior and required files. Never enforce explanatory prose,
heading inventories or manually rotated development markers. The version
transaction generates the release record after validation succeeds.

## Versions and changelog

Pre-1.0 breaking public API or semantic changes use a minor version even
though removed contracts are hard-cut. Compatible fixes may use patch.
Do not version each implementation slice or bump Cargo during routine coding.

Maintain CHANGELOG.md for meaningful behavior and maintained tooling changes.
Group work in Unreleased until a target is named, then use one undated
numbered section immediately below the empty Unreleased section. The release
helper dates that section, or promotes populated Unreleased notes,
automatically. Historical release notes remain immutable, including imported
undated entries at or below the current package version.

Repository-only work normally joins the next coherent release. An explicit
maintainer version/release request may choose a repository-only release.
Never claim that changing the version establishes service or provider safety.

## Publication and recovery

Release preparation requires committed clean input and full current validation.
It updates only Cargo.toml, Cargo.lock, CHANGELOG.md and docs/release.json.
Failed preparation restores these files; success leaves them for review.
The receipt binds exact release-file hashes to the validated source commit.
Release staging/commit reject unrelated changes. Push requires a clean main
branch, annotated current-version tag at HEAD and the validated source as its
direct parent. Push exactly main and that tag atomically without force.

A successful one-shot release finishes with cargo clean. Registry publication
is separate and remains blocked by publish=false at bootstrap.
The gate currently proves library buildability and release-tool behavior only.
Before service publication is enabled, B1 ownership and acceptance must close
and the gate must include the actual service's PocketIC/recovery evidence.
Do not invent a passing substitute for evidence that has not been implemented.
