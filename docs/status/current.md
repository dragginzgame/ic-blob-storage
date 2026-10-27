# Current status — 2026-10-27

Released baseline: `2d130afc`, Blob 0.49.3. The accepted storage cut is committed
at `efb0de20`, with **0.50.0** notes and package versions still 0.49.3. The minor
is required by changed stored formats and removal of public Candid traits. The
handoff retained in that Git commit records the complete storage-cut qualification
and artifact-size measurements.

## Published dependency refresh

Adopted published Memory/Storage 0.51.6, Platform/macros 0.6.20 and the consumed
Host/Build/Testkit 0.68.10 family. Root minimums match the selected graph; features,
Rust 1.91 and normal deployment exclusions remain unchanged. No sibling source,
path dependency, compatibility alias or new optional Platform capability is used.
Persistence 0.51.7, Tooling 0.68.11 and Rust Platform 0.6.21 sibling work was not
published at selection and is not substituted into the graph.

Browser Platform now selects published `@tokostudios/ic-platform` 0.1.1 in both
manifests and the private browser lock. Its public tarball integrity matches
registry metadata. SDK Core remains one canonical 6.1.1 identity shared by
Platform and Caffeine 1.1.2 under the existing reviewed override and patch.
New signing/expiry exports require no changes to Blob's existing callers.
Wire remains 0.7.18, Metrics 0.16.3 and Shared Tooling 0.26.21.

Versions/checksums were verified against Cargo's official sparse index; critical
cached archives also match those checksums. The crates.io HTTP API returned 403,
so it was not used as publication proof. Published source/API review found only
compatible additions and internal cleanup on Blob's selected paths. No caller,
endpoint, resource-policy or stored-format change is needed for this refresh.
The incoming staged Cargo.lock already selected these Rust versions and is
preserved byte-for-byte, including its original index state.

## Accepted storage cut and retained installations

All 34 previously Candid-derived model types use Serde with Wire. The existing
`model::record_codec` owner supplies 17 independently stored encodings through
`ic_storage::adapter::Storable`, each with an explicit storage-only identity.
Nested models add no envelopes. Installation format is
`ic-blob-storage/installation:wire-records`. There is no Candid reader, migration,
alternate writer or automatic reset. Host allocation keys/ledger, key ordering
and collection bounds remain unchanged; keys do not establish payload compatibility.
Record limits remain finite, including large-leaf/header and full-width coverage.

Old Candid stores refuse the new image. Apply the
[retirement runbook](../retiring-installations.md) with original tools, retaining
objects, uncertain effects, balances, access and billing evidence. Reinstall is
not deletion or settlement; never clear a ledger or activate stale state to
bypass refusal. Public `ReadSessionLimits` loses Candid traits. External
provider/management Candid, owned endpoint bytes, saved command documents,
canonical content identities and exact operation/retry policy remain unchanged.

The replacement `standalone_record_cut` regression uses a freshly retained
0.49.3 Wasm/hash and proves populated Candid-state refusal without losing stable
bytes, the old heap or registered physical/billing obligations. It passes on the
new dependency graph; other runs require their own explicit pinned input.
Canic owns adopting the supplying release and qualifying dedicated/embedded
services. No sibling is changed or consumer acceptance claimed here.

## Latest qualification

Evidence, metadata, selected inputs, archive checksums, setup receipts and logs:
`/tmp/blob-upstream-20261027/`. Locked fetching and existing Testkit/Build setup
and offline admission pass; Testkit still selects PocketIC server 16.1.0.
Default and all-feature metadata, dependency pins, Platform boundaries, normal
Wasm deployment exclusions and browser dependency admission pass.

Strict affected Clippy passes for Core, Contracts, Client, CLI, standalone,
private protocol and PocketIC callers. Current-graph tests pass: 68 model cases,
7 allocation/lifecycle integration cases, 118 native CLI cases and 25 host-support
cases. Three CLI socket cases first hit sandbox permission errors; the exact
failed scopes pass with process permissions, retaining the initial logs.
Fresh default standalone Wasm and CLI builds pass; Rust 1.91 checks Core and
standalone for Wasm and the CLI's native targets/features. Installed scopes pass:
lifecycle (4), certificate/authority
(3), populated old-format refusal (1), and actual Chromium/PocketIC browser
journeys (4: upload/download/release, corrupt read, lost reply/withdrawal, two users).

Browser checks pass: dependency refusal fixtures (15), control/response tests (2),
publication/frozen-input and worker suites, real Chromium signer/bootstrap (17),
source-hash-checked Caffeine bundling and the single-SDK graph. The sandboxed npm
version observation and one initial missing report-directory invocation remain
retained as failed attempts; corrected checks pass without weakening admission.
All provider responses are local substitutes. No paid/live provider effect occurs.

The earlier full storage-cut suites and size comparison retain their original
Memory/Storage 0.51.5, Platform 0.6.19, Tooling 0.68.9 and browser 0.1.0 scope.
They are not relabelled as measurements or full acceptance of this newer graph.
Full CI, Binaryen variants, native macOS and live-provider qualification were not
rerun. Unselected historical/resource profiles retain their own input requirements.

The [native Agent advisory](https://github.com/dragginzgame/ic-blob-storage/issues/102)
remains unresolved. External Candid and historical input tests remain intentional;
this is not a Candid-free graph. No assistant staging, commit, push, release,
publication, deployment or sibling edit occurred; the user's staged lock is intact.
