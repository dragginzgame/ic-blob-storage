# Current status

Date: 2026-10-02

Release draft: [0.5.0](../../CHANGELOG.md#050), undated below empty Unreleased.
Package version and release receipt remain 0.4.16. Commits and release operations
belong to the maintainer; no version mutation occurs in this implementation batch.

## Active work — Rust compatibility and development toolchain

The maintainer requests a lower justified MSRV and development Rust 1.99.0.
The workspace minimum is now 1.88.0 for the library, standalone, CLI, examples
and local canister fixtures. The unpublished PocketIC harness alone declares
1.89.0 because its native journal uses standard-library file locking. Locked
dependency declarations and maintained `slice::as_chunks` use establish the
1.88 floor; actual native and Wasm checks verify the supported packages.

Development Rust, Clippy and rustfmt are independently pinned to 1.99.0. Its
strict all-target/all-feature workspace lint, formatting and Wasm check pass.
The new assertion lint is addressed with equivalent comparisons and one
result-use annotation; five recovery delays use the same 24-hour value through
the older seconds constructor. No lint suppression or dependency update is added.
The [dependency guide](../dependencies.md#rust-versions) records repeatable
minimum-toolchain checks. Intent, metadata, compiler identities, initial failures,
fixer limitations and final logs remain in `.tmp/msrv-toolchain-01`.
Full CI/release validation and new runtime/provider probes have not run.
Notes extend the existing 0.5.0 draft; earlier removal work is preserved.

## Active work — independent library and ic-memory 0.15

The maintainer explicitly removes Canic 100% from this repository, including its
adapter and tests. The Canic adapter crate, managed canister fixture, complete
managed PocketIC suite, dependencies, targets and framework-specific inventory/
parity documentation are deleted. Core and standalone remain, alongside native
and browser tooling and framework-free local IC fixtures. AGENTS now assigns
consumer wrappers and integration tests to the consumer repository. No sibling
repository has been edited. There is no opt-in Canic suite or framework fallback
left here; ordinary validation and releases require no Canic tool/publication.

The official Cargo index identifies ic-memory 0.15.0 as the latest non-yanked
release. The root pins =0.15.0 and resolves one runtime identity without any Canic
package. Direct growth callers use typed results; generic stable-memory wrappers
retain the upstream trait contract. The real refusal fixture accepts only typed
BackingRefused with unchanged extent before deliberately trapping after writes.
Shared installation, tenant rules, grants and all-owner restore fences remain.

Workspace native check and strict all-target/all-feature lint pass. Shared-store
and installation unit cases, standalone exported Candid and release Wasm builds
pass. Actual typed growth-refusal rollback and standalone stop/start/repeated fenced
restore checks pass. Intent, official index, graph, logs and prior editing failure remain in
`.tmp/memory-independent-release-01`; earlier sealed captures remain unchanged.
Framework-free Wasm check, formatting, warning-free core/standalone rustdoc,
package verification and maintained documentation/graph checks also pass.
Full CI/release validation has not run. Notes join the existing 0.5.0 minor draft
because the public memory API changes and the adapter API is removed.

## Maintained functionality and next work

The library owns durable tenant enrollment, upload permissions/manifests, references,
receipts, indexed accounting, read sessions, gateway state, funding intents and
uncertain provider effects. The standalone host delegates to shared handlers and
restores synchronously into inspection-only fences. Native tooling prepares exact
inputs/snapshots, authenticates service requests, verifies complete downloads and
records/retrieves verifier statements. Browser tooling composes the maintained
Caffeine SDK with one certificate/gateway journal and disabled retries.

A complete live Caffeine upload/download trial remains unqualified. All four
certificate host facts stay false: PrechargeLimits, ProviderNamespace,
ReplayCharging and Recovery. Client limits and successful substitutes cannot set
them. No trial canister/account, numerical financial terms or paid-effect authority
is selected. Review exact namespace, provider financial/expiry terms and supported
recovery/retirement responsibilities before effects. Offline installation-check
and account-link-inputs prepare proposals without deploying, linking or funding.
See the [trial plan](../operator-guide.md#isolated-uploaddownload-trial-plan),
[acceptance](../acceptance-plan.md), [contract](../service-contract.md) and
[probe ledger](../evidence/caffeine-probes/README.md).

## Consumer integration feedback

Canic now owns its own wrapper and integration tests against the public library;
that work has not been performed in its repository. Consumers must adopt one
ic-memory package identity, supply explicit caller/service and allocation grants,
use shared workflows, and preserve synchronous restoration and fences. A framework's
activation or controller status cannot grant tenant, verifier or provider authority.
This is consumer work, not a library release dependency. No upstream messages were
sent and no sibling edits are authorized by this removal.

Toko review uses remote development; the local checkout is absent/stale. The
[source review](../evidence/toko-0.2-review.json) and
[Miner findings](../roadmap.md#toko-miner-feedback--2026-09-27) retain consumer input,
not approved adoption or production sizing. Real application transactions,
consumer outbox/worker acceptance and operational restart remain open.

## Constraints and history

Pre-1.0 is a hard cut: current APIs/schema only, no compatibility/migration paths.
Cross-release transitions are reinstall-only after obligations are preserved or
discharged. Same-release recovery and retry remain required. Removing source cannot
erase external objects, uncertain effects, balances or continuing billing.
The allocator is unchanged. Do not deploy, pay, commit, publish or clean builds
without the appropriate explicit authority. Use targeted checks during development.

Earlier source-bound implementation and removed integration observations remain
in [core evidence](../evidence/core-primitives.md), immutable release history and
sealed captures. Their former adapters, commands and acceptance claims are historical,
not a maintained integration contract. All Caffeine probe artifacts and limitations
are retained; no new provider probe or effect occurs in this batch.
