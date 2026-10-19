<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-blob-storage/ic-blob-storage-readme-header.svg" alt="IC Blob Storage — Uploads, verifies, and tracks large files" width="100%">
</p>

# Dependency setup

## Current dependency owners

The root [Cargo catalog](../Cargo.toml) owns all direct dependency requirements;
all thirteen members inherit package versions and dependencies. The root
[lockfile](../Cargo.lock) owns this checkout's selected graph. Member manifests
select features and targets. Registry consumers resolve their own compatible
lockfiles; local qualification does not cover every future resolution.

| Owner | Locked selection | Responsibility |
| --- | --- | --- |
| Blob core | Workspace version | Tenant authority, references, quotas, records, provider accounting, retention, deletion and recovery workflows |
| Blob client | Workspace version | Explicit replicated role clients; no service storage, endpoints or lifecycle |
| Blob Contracts | Workspace version | Runtime-free DTOs, method catalog, Blob codec projections, content identities and verification |
| IC Memory | 0.48.2 | Allocation ledger, grants and exclusive runtime leases; the host supplies backing and one allocation pool |
| IC Storage | 0.5.6 | Governed stable collections and codec traits; the stable-structures dependency stays upstream |
| IC Platform | 0.1.18 | Entrypoint/export macros, supported CDK calls/system APIs, Principal, external Candid encoding/decoding, management DTOs and native Agent/parser/certificate identities |
| IC Wire | 0.4.6 | Bounded method/direction framing for owned service contracts and separate passive document encoding |
| IC Metrics | 0.14.5 | Private probe measurement arithmetic; not a public Blob API |
| IC Testkit | 0.54.1 | PocketIC client/server selection and managed lifetime; bounded Candid decoding uses Platform |
| IC Host artifacts/fs/process/tools | 0.29.0 direct; 0.27.6 through Testkit | Native artifacts, descriptor-rooted reads, durable publication, processes, framing and interface inspection |
| IC Build | 0.21.0 | CLI source fingerprints and optional archive verification; absent from public libraries and production Wasm |
| PocketIC | 16.1.0 | Testkit-selected local IC execution; no separate Blob server catalog |

The standalone owns endpoint registration, caller capture, lifecycle and memory
composition. The core exports none of these implicitly. Consumer frameworks own
their wrappers and composition tests; see [Canic integration](#canic-integration).
The [service contract](service-contract.md) owns supported service invariants.

Contracts owns provider JSON boundary codecs, including
[`provider::upload`](../crates/ic-blob-storage-contracts/src/provider/upload/mod.rs).
For 0.34, replace `ic_blob_storage::ops::caffeine::upload` imports with
`ic_blob_storage_contracts::provider::upload`. The decoder, status and error
semantics are unchanged; Core no longer declares `serde_json` directly.
The CLI retains its required Serde/JSON dependencies and Host test helpers,
but no longer declares Testkit. PocketIC retains Testkit and owns its server
selection; removing the CLI declaration does not retire simulator setup.

Collection constructors take exclusive `RuntimeMemory<Backing>` handles from the
host runtime. Store/workflow generic parameters name `Backing`; passive memory
bundles carry handles without granting access. Production hosts compose all
seventeen Blob requests before a single bootstrap. Cross-release installations
require [safe retirement](retiring-installations.md), not automatic migration.

Native fixture custody uses Memory's published `testing::fixture_runtime` with
retained backing and explicit leases. Raw bytes remain available only to native
corruption/restore fixtures. Memory fixture support is selected by core unit tests
and isolated probes; the default standalone build must exclude those selections.
Do not infer production feature admission from a workspace-wide unified build.

Host owns descriptor-rooted inventory and journal reads, retained creation,
validated publication and bounded subprocess framing. Blob retains input bounds,
request/reply correlation, JSON contracts, uncertainty records and error mapping.
Held descriptors do not freeze contents: callers exclude concurrent writers and
verify complete body digests. Interrupted probe evidence deliberately uses retained
creation rather than an atomic writer that discards partial bytes. These are
necessary consumer policies, not independent filesystem implementations.

The optional archive verifier and HTTP fixtures use the same Host process package
identity. Build 0.21 and direct Host 0.29 must move together because the package
verifier exchanges Host process limits and typed errors with Build. Its normal
dependency also supplies the default CLI's bounded framing;
`package-verification` activates Build, not Host process. HTTP fixture support
remains development-only. Build's source inventory is descriptive provenance,
not executable attestation or retry authority. Testkit's transitive dependencies
retain their own identities; Blob must not pass typed values across incompatible
package versions. Installed tool selection is separate from the application lock:
run `make install-testkit` then `make testkit-check` after changing Testkit.

The [service wire guide](service-wire.md) defines the pending 0.36.0 contract.
All 31 owned methods and standalone lifecycle inputs use published IC Wire
0.4.6, with explicit method/direction identity and no old-packet fallback.
Finite tenant frames retain their bytes; other methods use Wire's bounded native
Serde codec. Passive offline values use a separate document identity, and new
application artifacts use `.wire`. Exact saved-input and signed-frame binding
remains with the CLI/workflow owner.

Blob has no direct Ciborium dependency or local CBOR implementation. Ciborium
remains transitively through Memory's persisted metadata; changing that encoding
belongs to Memory and is not part of this cut. Native test `serde_cbor` remains
for IC transport envelopes and certificates. Provider/management Candid and
stored records retain their external/persisted contracts. Platform owns the
supported parser/decoder/runtime APIs. Host owns bounded service inventories
and uses Platform for its parser; the test harness explicitly selects Host's
`candid` and `ic-interface` features. Direct Candid/CDK declarations remain
forbidden by the boundary policy.

Native fixtures and example staging/publication use published `ic-host-fs`
0.28.1. Fixture directories clean up after success and retain failed-test
evidence during panic. Retained report directories remain caller-owned. Private
invocation files use Host's mode-0600 scratch writer within their owned directory.
Verified example outputs use Host's create-only publication of the checked file,
including typed failures after publication; reconcile those before retrying.
Snapshot bodies use retained-file publication, while the complete snapshot/report
is still not a crash-durable multi-file transaction. No direct `tempfile`
declarations remain; Testkit and PocketIC still introduce it transitively.

All 26 private consumer-probe controls use bounded Wire value documents, with
a 4 KiB ingress ceiling and complete canonical decoding. Their command/status
values remain test-owned, outside the public service catalogue. Rust callers
and opaque browser packets use that encoder; JavaScript defines no fixture
schemas. Actual service operations use their method-bound Wire clients.
The private probe lifecycle and stored consumer record retain Candid. Public
DTO Candid removal remains the coordinated cut in
[#84](https://github.com/dragginzgame/ic-blob-storage/issues/84); incoming codec
adoption cannot erase actual persisted consumers such as `ManifestIntentRecord`.

For consumer adoption of this source:

1. Select compatible Blob core/Contracts packages and the host's same Memory 0.48
   identity. Store/workflow type parameters now name backing memory; constructors
   take exclusive runtime leases. Freeze the application's actual lock and every
   independent fixture lock; sibling paths are not registry-delivery evidence.
2. Include Blob's seventeen requests in the host's single declaration snapshot
   and grant the `blob.` namespace before bootstrap. Validate installation for the
   allocated Principal before opening stores. Mounting initializes nothing.
3. Preserve the host's authenticated initializer and one synchronous lifecycle.
   Translate its admitted configuration to `ServiceInstallationInput`; standalone
   initialization uses a method-bound Wire frame, not a template for the framework's outer envelope.
   A Candid outer initializer still requires zero skipping quota and exact argument
   count, including nested application configuration.
   Retain exact target/operation bytes for lost-reply reconciliation.
4. Mount both request and response codecs from the current method catalog:
   all owned methods use Wire, and the provider certificate uses Candid. Check actual Wasm names/modes and the provider-only
   Candid subset. A Candid service file does not describe the complete Blob API.
5. Run dedicated/embedded PocketIC initialization, target/caller refusal, rollback,
   lost-result, metrics and same-image recovery cases in Canic. Apply the
   [retirement contract](retiring-installations.md) before a cross-release reinstall.
   Same-release recovery never authorizes upgrading a retained installation.

The [2026-10-16 receipt](evidence/canic-issues0310.md) retains its historical
source, stale fixture-lock failures and proposed caller repairs. It does not
establish current integration. The latest read-only source observations and
inspection limits are recorded in the
[architectural sediment audit](reports/audits/2026/10/18/architectural-sediment/01/report.md).
No sibling source or lockfile is changed by Blob adoption.

Managed installation does not complete application acceptance. The application
must authenticate two users, issue exact tenant-approved permissions, operate the
[completion verifier](completion-verifier.md), qualify deployed provider reads and
commit its media map only after exact completion/live references. See
[multi-user authority](multi-user-upload-authority.md),
[confirmed-reference publication](operator-guide.md#produce-a-complete-confirmed-reference-map)
and [funding qualification](funding-consumer-qualification.md). Provider payer,
credit, GC, deletion and billing facts cannot be replaced with local substitutes.

## Setup and checks

`make install-tools` provisions the complete shared host/IC/Cargo set followed by
Blob's lock-selected Testkit CLI/server. `make tools-check` admits those selections
offline. For a changed Cargo-tool policy run `make install-rust-tools`; for a
changed Testkit selection run `make install-testkit`. Setup retains old generations
and failed evidence. Use the same shared-store setting for setup, checks and
failure collection; see [local setup](local-setup.md#shared-executable-storage).

Scoped builds/tests use `--offline --locked`. After dependency changes or cache
removal, run `make deps` explicitly before them; it fetches only the selected
Rust source and does not install executable tools. An explicit offline setting
remains authoritative. Missing dependencies or binaries fail rather than being
installed, replaced or skipped by tests.

PocketIC callers obtain the admitted absolute server path through `make testkit-check`.
Explicit `POCKET_IC_BIN` overrides remain caller-owned byte admission; no fallback,
implicit download or independent server-version equality gate exists here. Testkit
owns client/server compatibility and lifetime. The common five-tool IC bundle is
pinned separately by [ci/ic-tools.tsv](../ci/ic-tools.tsv).

## Release and formatting host checks

The [tooling workflow](../.github/workflows/tooling.yml) qualifies setup/admission,
formatting/hooks, exact-ID tooling archive upload/readback, coordinated package
archives, native CLI/PocketIC and original/optimized Wasms on Linux, Apple Silicon
and Intel macOS. Rust 1.91 has its separate minimum-version lane. Declaring those
jobs is not a passing result; require matching native execution on the actual source.
Cancelled or earlier-source results do not qualify new inputs.

`make evidence-check` and `make probe-check` verify retained records offline;
they make no new provider requests. Failed tooling and package/Wasm evidence stays
at the printed paths and is uploaded by the hosted collectors. Successful fixture
cleanup removes only its own disposable inputs. See
[development governance](governance/development.md) for command authority and the
[release guide](releasing.md) for maintainer validation, delivery and publication.

## Replicated clients and service storage

The `ic-blob-storage-client` package owns the existing role-specific replicated
calls and their common one-call mechanics. Its normal graph includes Contracts
and Platform execution, but excludes Core, Memory, Storage and stable structures;
`make contracts-boundary-check` enforces that distinction across targets.
Contracts and the native CLI retain their separate runtime-free checks.

Use `ic_blob_storage_client::{admission,download,funding,manifests,references,status,tenants}`
for remote calls. The old Core client paths are removed in the released 0.35.0
public cut. Core owns installed service state and workflows; consumers retain
intent, retry decisions, application publication and uncertain-outcome recovery.
The client package installs no endpoints, lifecycle hooks, stores or timers.
Publishing proceeds Contracts, Client, Core. All three 0.35.0 packages are now
available from crates.io. Only maintainer release preparation advances their
coherent workspace version; registry publication does not qualify consumer
runtime composition or provider behavior.
