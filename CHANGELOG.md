# Changelog

## [0.22.4] - 2026-10-10

### Changed

- Adopt Shared Tooling 0.3.7 and delegate Testkit CLI selection directly to its
  consumer-lockfile installer, removing Blob's separate version projection.
  Keep explicit setup, offline admission and Testkit-owned server lifecycle.
  [#54](https://github.com/dragginzgame/ic-blob-storage/issues/54).
- Adopt private Testkit 0.33.0's scoped connection-reset classification and
  Metrics 0.5.4, plus all four Host 0.12.7 patches. The maintained Host writer
  rejects NUL publication paths before parent creation. Public Blob contracts
  and stored schemas remain unchanged.
- Keep only the newest CI run per workflow and branch or PR, cancelling older
  queued and running checks while retaining the existing host matrix and gates
  ([Shared #108](https://github.com/dragginzgame/shared-tooling/issues/108)).

### Fixed

- Reject failed mandatory assertions explicitly on Bash 3.2 in shared installers
  and Blob fixtures; reject incomplete consumer hook checks and retain failed
  evidence instead of reporting success.
  [#55](https://github.com/dragginzgame/ic-blob-storage/issues/55),
  [Shared #107](https://github.com/dragginzgame/shared-tooling/issues/107).
- Stop pre-commit formatting when a Git tree observation fails, preserving its
  status and the original working files and index through the canonical hook.
  [Shared #106](https://github.com/dragginzgame/shared-tooling/issues/106).

## [0.22.3] - 2026-10-10

### Changed

- Adopt Shared Tooling 0.3.5 and its reviewed Binaryen 133 pins. Prepare the
  updated toolset explicitly; previous bundles remain retained. Qualify original
  and O3/Os/Oz Blob installation, authority and restoration in the delivery gate
  and all three native CI lanes, retaining exact artifacts and failed attempts.
  [#53](https://github.com/dragginzgame/ic-blob-storage/issues/53).
- Preserve incoming Memory 0.35.3, private Testkit 0.32.2 and all four Host
  0.12.4 patches; ordinary Blob builds remain unoptimized.
- Refresh Canic integration guidance for the current Blob 0.22 / Memory 0.35
  host pool, seventeen permanent requests and dedicated/embedded lifecycle.

### Fixed

- Keep synthetic consumer logging tests out of the parent gate's log directories
  and GitHub summary, preserving its original evidence.
  [#52](https://github.com/dragginzgame/ic-blob-storage/issues/52).

## [0.22.2] - 2026-10-10

### Changed

- Adopt Shared Tooling 0.3.3: preflight common setup before installation,
  preserve Cargo jobserver descriptors and include advisory README reviews.
  [#51](https://github.com/dragginzgame/ic-blob-storage/issues/51).
- Use Shared Tooling's bounded crates.io metadata owner for publication and
  dry-run dependency readback. Require jq and curl 8.4 or newer, retain each
  observation's transport and parser evidence, and preserve yanked-version,
  archive/source and contracts-before-core refusal boundaries.
  [#45](https://github.com/dragginzgame/ic-blob-storage/issues/45).
- Preserve the incoming private Testkit 0.32/Host 0.12 graph; qualify the
  selected Testkit 0.32.1, Host 0.12.3, Memory 0.35.2 and Metrics 0.5.3 patches
  through the current consumer gate.

### Fixed

- Reject malformed validation nesting metadata and premature successful exits;
  retain incomplete runner and fixture evidence, including Blob-owned helpers.
  [#50](https://github.com/dragginzgame/ic-blob-storage/issues/50),
  [#51](https://github.com/dragginzgame/ic-blob-storage/issues/51).

## [0.22.1] - 2026-10-10

### Changed

- Adopt Shared Tooling 0.3.0's complete ordered setup: host, IC and Cargo tools,
  followed by the lock-selected Testkit. Run `make install-tools` explicitly;
  offline checks stop before builds when any selected tool is unavailable.
  Existing bundles, receipts and server artifacts remain retained.
  [Shared #98](https://github.com/dragginzgame/shared-tooling/issues/98).
- Select private Testkit 0.31.0, retaining 0.30's bounded Cargo diagnostics and
  complete tool/workspace probe admission. Adopt incoming direct Host 0.12
  with unchanged runtime sources. Keep existing PocketIC ownership and server
  selection.
  [#39](https://github.com/dragginzgame/ic-blob-storage/issues/39),
  [Testkit #49](https://github.com/dragginzgame/ic-testkit/issues/49),
  [Testkit #50](https://github.com/dragginzgame/ic-testkit/issues/50).

### Fixed

- Adopt Shared's CI-log inspection fix so missing failed-step logs remain an
  explicit evidence gap with retained readbacks.
  [Shared #97](https://github.com/dragginzgame/shared-tooling/issues/97).

## [0.22.0] - 2026-10-10

### Breaking

- Adopt public Memory 0.34's host-wide allocation pool and explicit namespace
  grants. Hosts compose permanent key requests, bootstrap once with the pool and
  open committed keys through the current API. Consumers must align Memory and
  rebuild; cross-release installations remain retirement/reinstall only. Blob's
  record layouts and same-release restore fences are unchanged.
  [#48](https://github.com/dragginzgame/ic-blob-storage/issues/48),
  [Memory #44](https://github.com/dragginzgame/ic-memory/issues/44).

### Fixed

- Adopt private Host 0.11 and Testkit 0.29 together, keeping one Host graph and
  the existing authenticated PocketIC server. Native publication, buffered
  subprocess output and restore qualification use the maintained owners.
  [#39](https://github.com/dragginzgame/ic-blob-storage/issues/39),
  [Host #46](https://github.com/dragginzgame/ic-host-tooling/issues/46).
- Adopt incoming Memory 0.34.1 and private Metrics 0.4.0 tooling updates;
  their packaged runtime sources and consumed APIs are unchanged.
- Adopt Shared Tooling 0.2.13: concise formatting with retained failures,
  independent unsafe-Make-mode admission, recorded snapshot identity and invalid
  directory-path refusal. Include the new formatter companion in isolated
  fixtures and CI failure evidence; prepare selected executable tools before
  release validation and check them before focused native/IC builds.
  Refuse LF/CR release directories without selecting a trimmed neighbor, and
  recheck source identity and cleanliness after selected-tool preparation.
  [#47](https://github.com/dragginzgame/ic-blob-storage/issues/47),
  [Shared #96](https://github.com/dragginzgame/shared-tooling/issues/96).
- Refresh Canic adoption guidance for published Blob 0.21/Memory 0.33, exact
  target-bound installation and synchronous restoration. Distinguish local
  managed acceptance from adapter delivery and the separate repeated-funding
  evidence requirement; correct stale Memory and funding release wording.
  [Canic #444](https://github.com/dragginzgame/canic/issues/444),
  [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34).

## [0.21.5] - 2026-10-09

### Changed

- Adopt Host 0.10.1's consolidated native record writer, preserving private
  create-only output, permanent run claims and terminal redacted file errors.
  [#46](https://github.com/dragginzgame/ic-blob-storage/issues/46).
  Parent-directory publication also finishes synchronization when another
  writer wins creation. [Host #43](https://github.com/dragginzgame/ic-host-tooling/issues/43).
- Select Testkit 0.28, removing the older transitive Host graph. Run
  `make install-testkit` to prepare the matching CLI; PocketIC 16.1.0 and
  prior installations remain retained.
  [Testkit #44](https://github.com/dragginzgame/ic-testkit/issues/44).
- Select private Metrics 0.3.5 with unchanged arithmetic and dependency edges.

## [0.21.4] - 2026-10-09

### Fixed

- Adopt Shared Tooling 0.2.8 so Make admission uses the selected snapshot even
  with an inherited tooling root or recursive `MAKE` arguments. Preserve
  rejection of modes that skip recipes or hide failures.
  [#44](https://github.com/dragginzgame/ic-blob-storage/issues/44),
  [Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30).

### Changed

- Select Host artifacts/fs/process/tools 0.9.7, retaining existing bounded
  file/process delegation and response formats.
- Select private ic-metrics 0.3.4 with unchanged arithmetic and dependency edges.

## [0.21.3] - 2026-10-09

### Fixed

- Adopt Shared Tooling 0.2.7 to reject Make modes that hide failures, bind
  release qualification to its disposable tooling snapshot, and preserve
  newline-ending checkout paths during hook qualification. Keep all twelve
  members covered and retain local delivery and formatter rollback checks.
  [#44](https://github.com/dragginzgame/ic-blob-storage/issues/44),
  [Shared #30](https://github.com/dragginzgame/shared-tooling/issues/30),
  [#7](https://github.com/dragginzgame/shared-tooling/issues/7),
  [#90](https://github.com/dragginzgame/shared-tooling/issues/90).

## [0.21.2] - 2026-10-09

### Fixed

- Read back CI evidence through the pinned action's authenticated REST path,
  scoped to the current repository/run and exact uploaded artifact ID. Preserve
  digest rejection and the separate archive checksum check after internal lookup
  returned no matches for a successfully uploaded artifact.
  [#43](https://github.com/dragginzgame/ic-blob-storage/issues/43),
  [Shared #93](https://github.com/dragginzgame/shared-tooling/issues/93).

### Changed

- Adopt Shared Tooling 0.2.6's release and Rust formatting Make includes,
  replacing repeated recipes while preserving direct-delivery admission,
  metadata exports and all twelve members' formatter coverage. Formatting
  resolves prepared Cargo through the exported recipe PATH.
  [#42](https://github.com/dragginzgame/ic-blob-storage/issues/42),
  [Shared #91](https://github.com/dragginzgame/shared-tooling/issues/91),
  [#92](https://github.com/dragginzgame/shared-tooling/issues/92).
- Select ic-memory 0.33.2 with unchanged runtime sources and dependency edges.
- Select Host artifacts/fs/process/tools 0.9.4 with unchanged runtime sources
  and dependency edges, preserving the existing file/process delegation.

## [0.21.1] - 2026-10-09

### Fixed

- Refresh Shared Tooling to committed 0.2.5. Formatting hooks preserve
  trailing-newline checkout and hook paths; installation refuses failed Git
  reads without replacing existing configuration.
  [Shared #89](https://github.com/dragginzgame/shared-tooling/issues/89).

### Changed

- Select ic-memory 0.33.1 and private ic-metrics 0.3.2. Their runtime sources
  and dependency edges are unchanged; service APIs and stored schemas are unchanged.
- Select Host artifacts/fs/process/tools 0.9.3 with unchanged runtime sources
  and dependency edges, retaining existing file/process delegation.
- Select Testkit 0.27.1 with unchanged runtime sources. Run
  `make install-testkit` to prepare the matching CLI; existing PocketIC server
  bytes and prior CLI installations remain retained.

## [0.21.0] - 2026-10-09

### Breaking

- Expose separate tenant logical, global physical and global billing-liability
  byte headroom in public capacity replies and native preflight reports.
  The existing usable minimum and admission rules remain. Preflight refuses
  inconsistent minima and identifies each exhausted byte dimension, replacing
  the generic `byte_capacity` blocker. Rebuild clients against the new Candid
  and Rust contract. [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6).
- Move the public Memory re-export to ic-memory 0.33. Consumers must align the
  Memory type identity and rebuild. Runtime schemas are unchanged; the existing
  cross-release retirement/reinstall contract still applies.

### Changed

- Refresh to committed Shared Tooling 0.2.4 source with explicit companion checks
  for reusable fixtures and guidance on diagnosing native CI queues.
- Remove the upstream exporter integration fixture from the consumer snapshot
  and tooling gate. Shared Tooling owns exporter/governance qualification;
  Blob retains snapshot verification and local adoption checks.
  [#40](https://github.com/dragginzgame/ic-blob-storage/issues/40).
- Adopt Testkit 0.27.0. Its structured startup errors retain the original cause,
  bounded server output and separate command/server cleanup failures. The
  existing harness preserves these diagnostics without adding a second owner.
  Run `make install-testkit` to prepare the lock-selected CLI before offline checks.
- Select Host artifacts/fs/process/tools 0.9.2 and private Metrics 0.3.1.
  Their runtime Rust sources are unchanged from the previous selections; retain
  the existing Host file/process delegation and Metrics arithmetic.

## [0.20.0] - 2026-10-09

### Breaking

- Transfer PocketIC setup and offline admission to the lock-selected Testkit CLI.
  The shared IC bundle contains five tools and no longer provides
  `.tools/ic/bin/pocket-ic`. Run `make install-tools` to prepare Testkit and the new
  bundle; `make tools-check` admits its server and PocketIC tests default to it.
  Explicit binary overrides retain caller-owned admission.
  Previous bundles, receipts and failures remain retained. Service APIs and stored
  data are unchanged; this hard cut changes developer tool setup.
  [Shared #76](https://github.com/dragginzgame/shared-tooling/issues/76).
- Move the public Memory re-export to ic-memory 0.32. Consumers must rebuild
  with the same Memory type identity. Runtime schemas are unchanged; the existing
  cross-release retirement/reinstall contract still applies.

### Fixed

- Refresh Shared Tooling to reviewed 0.2.2. Pre-commit formatting finds the
  checkout's prepared tools without a shell PATH export; Cargo tool setup rejects
  conflicting receipts and redirected installation paths while retaining failed
  builds and their original status.
  [Shared #85](https://github.com/dragginzgame/shared-tooling/issues/85),
  [#65](https://github.com/dragginzgame/shared-tooling/issues/65).
- Reject multi-document dependency-pinning exception catalogs that could bypass
  validation; valid single-array catalogs remain accepted.
  [Shared #86](https://github.com/dragginzgame/shared-tooling/issues/86).
- Install and verify the final selected IC tool even when its pin matrix has
  no final newline. [Shared #87](https://github.com/dragginzgame/shared-tooling/issues/87).

### Changed

- Delegate fixture inventory file reads and resource-profile artifact publication
  to Host. Preserve the 8 MiB input limit, invalid-request projection, exact report
  bytes and refusal to overwrite prior evidence; Host owns bounded allocation,
  regular-file admission and atomic durable publication.
- Adopt Host 0.9.1 and Testkit 0.26.0 on one Host type identity, including the
  owner's durable-publication simplification and prompt output-pipe release.
  Private Metrics moves to 0.3.0 with unchanged arithmetic APIs. No local
  compatibility aliases or server version policy are introduced.
  [Host #38](https://github.com/dragginzgame/ic-host-tooling/issues/38).

## [0.19.3] - 2026-10-09

### Removed

- Remove the unused service-core dependency from private test protocol DTOs,
  retaining their contracts dependency and the core dependencies of actual
  consumers. [#38](https://github.com/dragginzgame/ic-blob-storage/issues/38).
- Retire unused fleet reporters and their dedicated consumer tests. Local
  workspace `make cloc` and tool setup/check remain available; fleet inventories
  run from Shared Tooling.
  [#37](https://github.com/dragginzgame/ic-blob-storage/issues/37).

### Changed

- Refresh Shared Tooling to reviewed 0.1.35, including optional fleet reports,
  selected Cargo tool installation and explicit dependency-preparation guidance.
  [Shared Tooling #83](https://github.com/dragginzgame/shared-tooling/issues/83),
  [#65](https://github.com/dragginzgame/shared-tooling/issues/65),
  [#84](https://github.com/dragginzgame/shared-tooling/issues/84).
- Use Host 0.8.8's bounded streaming hash for retained probe bodies, avoiding a
  complete body buffer while preserving exact length/digest and refusal checks.
  [Host #33](https://github.com/dragginzgame/ic-host-tooling/issues/33).
- Align private Host/Testkit/Metrics selections at 0.8.8/0.25.4/0.2.18 and public
  Memory at patch 0.31.10, retaining the public 0.31 contract.

## [0.19.2] - 2026-10-09

### Fixed

- Preserve native CI qualification for each pushed commit while allowing newer
  PR revisions to supersede earlier review runs.
  [Shared Tooling #80](https://github.com/dragginzgame/shared-tooling/issues/80).
- Project SDK preparation results into the browser harness's declared control
  fields, avoiding rejection of the SDK's internal `maxChunkBytes` field before
  upload setup. [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31).

### Testing

- Exercise two independent Chromium signing identities against one standalone
  service, with distinct uploads, native verification/download and separate
  reference release. Keep real application login and deployed-provider acceptance
  separate from the local provider substitute.
  [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
  [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33).

### Changed

- Refresh Shared Tooling to reviewed 0.1.32, documenting the pending PocketIC
  provisioning handoff while retaining the functioning published setup/check
  path and current binaries.
  [Shared Tooling #76](https://github.com/dragginzgame/shared-tooling/issues/76).

## [0.19.1] - 2026-10-09

### Changed

- Refresh Shared Tooling to reviewed 0.1.31. Reuse verified IC tools when pin
  comments or row order change, preserving exact installation provenance and
  refusing changes to any host's selection.
  [Shared Tooling #79](https://github.com/dragginzgame/shared-tooling/issues/79).
- Qualify incoming private Metrics 0.2.16 and TOML parser patches on the existing
  Host 0.8.4/Testkit 0.25.3 graph, retaining the public Memory 0.31 contract.
- Document the current Caffeine GC callback wire contract and the provider
  evidence required before mounting deletion handlers. GC and missing-method
  retention/billing behavior remain unqualified.
  [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32).

## [0.19.0] - 2026-10-08

### Breaking

- Let enrolled projects authorize individual uploaders through exact per-upload
  permissions. Remove `trusted_uploader` from installation/configuration,
  `UploadIssuerAuthority`, its accessors and the `TrustedUploader` blocker.
  Native installation preparation no longer accepts `--trusted-uploader`, and
  standalone initialization rejects skipped fields and extra arguments.
  Regenerate consumers against the changed DTO/Candid contract; retire old
  installations before reinstalling.
  [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31).

### Changed

- Refresh Shared Tooling to reviewed 0.1.30 and return the unchanged PocketIC
  16.1 pin selection to its canonical snapshot owner.
  [Shared Tooling #76](https://github.com/dragginzgame/shared-tooling/issues/76).
- Apply shared npm declaration and lock agreement checks to the private browser
  build root, using its existing Node/npm selections.
  [Shared Tooling #77](https://github.com/dragginzgame/shared-tooling/issues/77).
- Qualify incoming Memory 0.31.8, private Metrics 0.2.15, Testkit 0.25.3 and Host 0.8.3
  patches, preserving the existing dependency lines.

### Fixed

- Use the canonical read-only release-source checker to identify staged,
  unstaged and untracked paths before validation or metadata preparation.
  Preserve the exact local metadata allowance and failed Git-read refusals.
  [Shared Tooling #74](https://github.com/dragginzgame/shared-tooling/issues/74).

## [0.18.5] - 2026-10-08

### Changed

- Align native tooling and the private test harness on Host 0.8.2 through
  Testkit 0.25.1, preserving one Host dependency line.
  [Testkit #32](https://github.com/dragginzgame/ic-testkit/issues/32).
- Refresh the reviewed Shared Tooling snapshot, preserving direct release
  delivery, protecting tracking refs against symbolic-ref races, and supporting
  safe repeated snapshot refreshes. Include the new installer-evidence companions
  so the selected fixtures remain runnable.
  [Shared Tooling #62](https://github.com/dragginzgame/shared-tooling/issues/62),
  [#64](https://github.com/dragginzgame/shared-tooling/issues/64),
  [#60](https://github.com/dragginzgame/shared-tooling/issues/60).
- Adopt Shared Tooling 0.1.28's installer path admission, simulation-only release
  fixtures and maintenance task catalog.
  [Shared Tooling #75](https://github.com/dragginzgame/shared-tooling/issues/75),
  [#70](https://github.com/dragginzgame/shared-tooling/issues/70).
- Document a private application-operated completion verifier using existing
  observation, one-shot submission and receipt recovery tools.
  [#33](https://github.com/dragginzgame/ic-blob-storage/issues/33).

### Fixed

- Select reviewed PocketIC 16.1.0 server assets and remove the independent
  client/server version-equality gate; Testkit/PocketIC retain compatibility
  ownership and the shared installer retains binary authentication.
  [#36](https://github.com/dragginzgame/ic-blob-storage/issues/36).
- Check the advertised Rust 1.88 minimum explicitly for native and Wasm paths
  in a dedicated CI lane.
  [#35](https://github.com/dragginzgame/ic-blob-storage/issues/35).
- Archive retained tooling failures before uploading so legal Unix filenames,
  permissions and symlinks survive diagnostic collection. Keep 30-day failure
  retention and qualify exact uploaded archive bytes on the native CI matrix.
  [#29](https://github.com/dragginzgame/ic-blob-storage/issues/29).

## [0.18.4] - 2026-10-08

### Fixed

- Identify registry readback requests so crates.io accepts publication checks.
  Preserve exact archive/source verification and refusal on inconclusive responses.
  [#30](https://github.com/dragginzgame/ic-blob-storage/issues/30).

### Changed

- Refresh the release handoff for 0.18.3 and record the upstream correction for
  inherited Make context in independently configured fixtures.
  [Shared Tooling #7](https://github.com/dragginzgame/shared-tooling/issues/7).

## [0.18.3] - 2026-10-08

## [0.18.2] - 2026-10-08

## [0.18.1] - 2026-10-08

### Fixed

- Isolate release fixtures from parent Make overrides so delivery-policy refusal
  tests also pass when invoked through the complete release validation gate.

- Share native/browser fixture process cleanup through Host's owned child via
  Testkit 0.22, and retain prefetched final browser responses when draining output.
  Preserve protocol framing, idle deadlines and caller-selected IO. The incoming
  lock now has one Host 0.5 line and private-probe Metrics 0.2.11.

- Adopt reviewed Shared Tooling 0.1.24 and committed archive path corrections.
  Recheck final release payloads and tags,
  authenticate completed recovery, and refresh matching tracking refs after
  confirmed delivery. Keep Blob's direct release policy explicit and refuse PR
  delivery before effects. [Shared Tooling #58](https://github.com/dragginzgame/shared-tooling/issues/58),
  [#62](https://github.com/dragginzgame/shared-tooling/issues/62).
- Correct sibling LOC reporting for temporary path aliases, `bin/` tools and
  checkouts awaiting their first commit. [Shared Tooling #57](https://github.com/dragginzgame/shared-tooling/issues/57),
  [#61](https://github.com/dragginzgame/shared-tooling/issues/61).

- Make `make publish` publish contracts before core without a mandatory package
  selector. Retrying checks exact registry archives against the release source
  before skipping completed uploads; inconclusive readback stops publication.
  Keep optional individual selection and make unpublished-contract dry-run limits
  explicit without weakening clean-source, receipt or tag checks.

## [0.18.0] - 2026-10-08

### Breaking

- Extract `ic-blob-storage-contracts` as the single runtime-free owner of DTOs,
  validated identities, method names, bounded request/reply codecs, content
  verification and immutable installation inputs. Remove the former service Rust
  paths without compatibility reexports. Service, CLI, examples and adapters now
  depend downward; Candid and hash identities stay unchanged. Hosts must rebuild
  with the new imports; service transitions remain retirement/reinstall only.
  See [#27](https://github.com/dragginzgame/ic-blob-storage/issues/27) and the
  [migration record](docs/evidence/contracts-0180.md).

### Changed

- Move offline preparation/verification examples and independent hash/account-link
  fixtures with their contract owner. Check normal contracts/CLI dependency graphs
  for service, CDK or Memory edges. Keep storage, mutation authority, accounting,
  durable journals, recovery and provider dispatch in the service. Remove its
  unused direct hashing dependency now owned by contracts.
- Assemble and verify both extracted library packages with frozen external
  selections. Explicit maintainer publication selects contracts before service
  through standard Cargo. Advance local catalog requirements with the workspace
  release transaction, preserving locked external selections and rollback.

## [0.17.2] - 2026-10-07

### Changed

- Select native Host 0.4.6 and stream CLI JSON records through its typed durable
  writer, avoiding a complete encoded JSON buffer. Preserve private permissions,
  create-only publication and existing output/error contracts. Require the
  compatible filesystem patch 0.4.5 or later, including its macOS compile fix.
- Qualify the selected native harness on Testkit 0.21.2, retaining exact
  publication recovery, lifecycle/reference checks and the existing service graph.

### Added

- Retain bounded MIME, cache and CORS response-header observations beside native
  verified downloads, preserving duplicate values and marking incomplete capture.
  Keep these observations separate from original hash metadata and browser
  qualification. See [#5](https://github.com/dragginzgame/ic-blob-storage/issues/5).

### Fixed

- Reject duplicate provider roots in fresh publication batches before creating
  output or sending requests, even when operation, object and reference IDs
  differ. Existing content requires its exact retain/recovery path. See
  [#4](https://github.com/dragginzgame/ic-blob-storage/issues/4) and
  [#6](https://github.com/dragginzgame/ic-blob-storage/issues/6).

## [0.17.1] - 2026-10-07

### Changed

- Upgrade the native PocketIC harness to ic-testkit 0.21.1 and unify its Host
  packages on 0.4.2, removing the older duplicate dependency graph. Preserve
  explicit server ownership and the public service and durable contracts. See
  [#26](https://github.com/dragginzgame/ic-blob-storage/issues/26).
- Check the locked PocketIC client against reviewed server pins before the
  main canister test targets, using the shared offline alignment helper.
- Refresh Shared Tooling to committed 0.1.19. Retain complete combined failure
  logs alongside per-target logs and verify the extended consumer logging path.

### Fixed

- Adopt shared Rust installer path protection, portable temporary-directory
  handling and byte-preserving release changelog finalization. See
  [#25](https://github.com/dragginzgame/ic-blob-storage/issues/25),
  [Shared Tooling #55](https://github.com/dragginzgame/shared-tooling/issues/55)
  and [Shared Tooling #56](https://github.com/dragginzgame/shared-tooling/issues/56).

## [0.17.0] - 2026-10-07

### Breaking

- Align the public memory re-export and host grant types with ic-memory 0.31.
  Hosts must select that Memory line and rebuild composed graphs; Memory 0.30
  types are no longer interchangeable with the service API. Runtime source and
  durable layouts are unchanged; cross-release installations still require safe
  retirement and reinstall. Carry the complete pending tooling batch into this
  minor release. See [#20](https://github.com/dragginzgame/ic-blob-storage/issues/20).

### Changed

- Refresh the reviewed Shared Tooling snapshot to 0.1.18 and use its common
  Make include for setup, offline tool verification and LOC reports. Prepare
  checksum-pinned ripgrep with PCRE2 and cloc alongside jq/yq; expose the
  read-only sibling tooling inventory through `make cloc-tooling`. Retain the
  approved role-based workspace layout and complete member coverage.
- Use the shared pinned Cargo-tool setup for cargo-sort, cargo-sort-derives and
  candid-extractor. Local setup and CI install and verify the same repository-local
  set, preserving failed installation build output.
  See [Shared Tooling #51](https://github.com/dragginzgame/shared-tooling/issues/51).
- Preserve Make's validation failure status and support retained validation logs,
  timing records and interruption. Support independent LOC
  workspace selection with `make cloc CLOC_MANIFEST=path/to/Cargo.toml`.
- Pin the opt-in browser fixture's Node/npm preparation tools and refuse
  mismatched tools or manifest/lock declarations before browser build effects.
- Refresh direct native Host artifacts/filesystem packages to 0.4.1. The
  testkit-owned harness retains its 0.3.3 Host graph; public service contracts,
  bounded reads and durable schemas are unchanged. Preserve the selected
  Metrics 0.2.7 dependency in the private storage probe.
- Qualify the private Metrics restoration-read window in the existing native
  host matrix using a fresh probe Wasm, preserving operator-only observation
  and fenced restoration. See
  [#19](https://github.com/dragginzgame/ic-blob-storage/issues/19).
- Run bounded JSON example tests in the existing native host CI matrix, so
  the shared stream-reader adoption receives consumer coverage on Linux and
  both macOS architectures. See
  [#21](https://github.com/dragginzgame/ic-blob-storage/issues/21).
- Remove duplicated release version/lost-push scenarios and narrow descendant
  recovery to one Blob adapter integration. Retain exact receipts, publication
  admission, failed-preparation restoration and real-index proof; the coverage
  map identifies the canonical owner of each retired assertion. See
  [#22](https://github.com/dragginzgame/ic-blob-storage/issues/22).

### Fixed

- Make the native HTTP backpressure fixture consume bounded complete service
  requests and isolate cancelled subnet lookups. Give the cold native CI matrix
  a 60-minute budget so Intel macOS can complete the expanded checks. See
  [#24](https://github.com/dragginzgame/ic-blob-storage/issues/24).
- Adopt canonical LOC fixture isolation for inherited Cargo targets and
  enclosing workspaces, removing the consumer workaround. Exclude aliased
  build output from runtime/test totals. Keep production target ownership and
  explicit custom-target checks intact. See
  [#23](https://github.com/dragginzgame/ic-blob-storage/issues/23).
- Reject Make options and assignments as validation targets before any gate
  runs, through the canonical validation runner. See
  [Shared Tooling #30](https://github.com/dragginzgame/shared-tooling/issues/30).

## [0.16.0] - 2026-10-07

### Breaking

- Align the public memory re-export and host grant types with ic-memory 0.30.
  Hosts must select the same Memory line and rebuild their composed graphs;
  0.28-owned Rust types are no longer interchangeable with the service API.
  Runtime source and durable layouts are unchanged by this dependency update.
  Cross-release installations still require safe retirement before reinstall.
  See [#20](https://github.com/dragginzgame/ic-blob-storage/issues/20).

### Added

- Use published ic-metrics arithmetic for bounded restoration-read instruction
  diagnostics in the local storage probe. Preserve existing report fields,
  operator access and reset windows; diagnostic totals saturate independently.
  See [#19](https://github.com/dragginzgame/ic-blob-storage/issues/19).

### Changed

- Use the shared bounded stream reader for example declaration, claim and
  inventory JSON. Preserve input limits, diagnostics and caller-owned file
  opening; the dependency remains native-only and outside production graphs.
  See [#21](https://github.com/dragginzgame/ic-blob-storage/issues/21).
- Refresh the reviewed Shared Tooling snapshot to 0.1.14, aligning local repair
  and relevant issue work with its shared authority while preserving commit,
  release and sibling file-edit boundaries. Declare the governance file list and
  exact-commit CI inspection helper. See
  [#15](https://github.com/dragginzgame/ic-blob-storage/issues/15).
- Retain canister and packaged test helpers in their role-based `canisters/`
  and `tests/` layout under the maintainer-approved exception. Formatting and
  release/source inventories cover every declared workspace member. See
  [#17](https://github.com/dragginzgame/ic-blob-storage/issues/17).
- Refresh the selected native ic-host-artifacts/ic-host-fs packages to 0.3.2;
  preserve native-only placement and the existing bounded-read/private-record
  contracts.

### Fixed

- Refuse inherited Make modes that skip execution or ignore failures before
  release, validation and pre-commit formatting. Preserve formatter rollback
  and exercise the consumer's refusal boundaries. See
  [#18](https://github.com/dragginzgame/ic-blob-storage/issues/18).
- Exclude Cargo's configured build directory from code and test-function reports,
  including build output nested inside a package. Exercise canonical snapshot
  distribution fixtures in consumer tooling checks.
- Stop highlighting successful Rust test names containing `error::` as validation
  failures; preserve real diagnostics, contextual output and raw failed logs.
  Exercise the shared runner's refusal/retention tests in tooling checks and CI.
- Preserve exact changelog version comparisons beyond floating-point precision
  through the shared finalizer and its existing release fixtures.
- Verify snapshot hashes independently of inspected checksum code. Recheck the
  release destination after validation and before push, and dispatch to the
  captured URL rather than resolving a mutable remote name again. See
  [#16](https://github.com/dragginzgame/ic-blob-storage/issues/16).

## [0.15.3] - 2026-10-06

### Changed

- Remove the unused standalone canister serde dependency; retain the consumer
  fixture's required derives through explicit serde imports. Update the formatter
  adoption fixture for the current host crate catalog.
- Adopt the published ic-host-artifacts/ic-host-fs 0.3 split for native CLI
  hashes and bounded file reads. Publish complete private run records through
  the shared durable create-new owner, preserving claimed directories,
  nonreplacement, error codes and interrupted body evidence. Remove the unused
  direct ic-host-tools facade and its process dependency from the CLI graph.
- Retire the unused standalone yq installer; the shared host-tool bundle remains
  the setup owner. Record the unchanged evidence-checksum fixture in the shared
  snapshot instead of maintaining it as local code.
- Refresh the reviewed Shared Tooling snapshot to 0.1.10 and use its offline
  formatter prerequisite check for formatting and release preflight. Require
  pinned cargo-sort and working rustfmt before validation or metadata changes;
  retain hook rollback and failed fixture evidence. See
  [#12](https://github.com/dragginzgame/ic-blob-storage/issues/12).
- Finalize release notes through the shared owner using the previous version,
  target and date saved in release intent. Preserve imported history, metadata
  rollback and exact prepared-state recovery. Note content no longer gates a
  release; ambiguous identities and failed transformations still refuse without
  writing. See [#13](https://github.com/dragginzgame/ic-blob-storage/issues/13).

## [0.15.2] - 2026-10-06

### Changed

- Refresh the reviewed Shared Tooling snapshot. Add offline local documentation
  link checks and shared release-command routing checks to validation and native
  host CI, while retaining the repository's release and recovery checks.
- Delegate local lockfile version rewriting and generic formatting-hook checks
  to their shared owners, preserving release recovery and the consumer's failed
  formatter rollback case. See [#10](https://github.com/dragginzgame/ic-blob-storage/issues/10).
- Check Cargo package/dependency inheritance through the shared declaration gate;
  include its metadata refusal fixtures in native host CI. See
  [#11](https://github.com/dragginzgame/ic-blob-storage/issues/11).
- Qualify ic-memory 0.28.2, native ic-host-tools 0.2.0 and the unpublished
  PocketIC harness's ic-testkit 0.19.2. Preserve the library API, durable format
  and Rust 1.88 minimum; host tooling remains excluded from production Wasm.

### Fixed

- Generate and verify file digests through one portable backend. Reject failed
  receipt traversal and filenames that the IC tool receipt format cannot
  represent before activating a tool set; retain failures and the previous set.
- Switch accepted local download-fixture sockets explicitly to blocking mode,
  avoiding inherited nonblocking writes on macOS. Require complete writes for
  successful verification while preserving early-close refusal cases.
- Apply the same explicit blocking mode and bounded IO to the native backpressure
  fixture, retaining its one-request query/update refusal checks on macOS.
- Reject failed Git and metadata reads during release admission even when they
  print matching values. Stop before validation, metadata preparation or registry
  publication and preserve the selected inputs.
- Upload failed shared Cargo metadata and lockfile fixtures from native host CI,
  preserving their input files and diagnostics for
  [#10](https://github.com/dragginzgame/ic-blob-storage/issues/10) and
  [#11](https://github.com/dragginzgame/ic-blob-storage/issues/11).

## [0.15.1] - 2026-10-06

### Changed

- Provision reviewed jq, yq and IC executables with `make install-tools`, and
  verify them offline with `make tools-check`. Use repository-local tools and
  one IC pin matrix on Linux and macOS; validation never installs them implicitly.
  See [#9](https://github.com/dragginzgame/ic-blob-storage/issues/9).
- Reuse ic-host-tools for bounded native CLI file reads and raw artifact hashes,
  preserving file-selection rules, error codes and retained evidence formats.
- Adopt Shared Tooling's exact annotated-tag checker and reject Cargo test
  selections that execute no passing tests. Add focused native CLI and PocketIC
  installation qualification to the host CI matrix.

### Fixed

- Check ShellCheck and the reviewed cargo-sort version during release preflight,
  with explicit setup guidance before validation or version preparation starts.
- Retain native CI test failure logs in the same directory searched by the
  artifact uploader on Linux and macOS.

## [0.15.0] - 2026-10-06

### Breaking

- Adopt ic-memory 0.27's current ownership ledger through the public memory
  re-export. Hosts must update removed history/timestamp APIs and fixtures;
  earlier durable ledgers are unsupported. Preserve or discharge installation
  obligations before reinstalling; do not clear the ledger to bypass recovery.
  See [the service contract](docs/service-contract.md#current-persisted-boundaries).
- Replace funding records with mandatory immutable credit receipts and confirmed
  totals, a bounded receipt index and immutable allocation grants. Install explicit
  `funding.renewal_ceiling`; regenerate DTOs for cumulative/ceiling status,
  `uncredited_accepted`, `CreditConfirmed` and `renewed_allocation`,
  use matching tools, and retire prior installations before reinstalling. The
  installation format explicitly refuses earlier layouts.

### Added

- Add [host-internal funding credit confirmation](docs/funding-credit.md) for
  exact original top-ups. A verified, uniquely attributed receipt clears that
  operation's uncredited blocker so a subsequent guarded top-up can proceed.
  Replay is idempotent; wrong amounts, conflicting receipts and receipt reuse
  refuse. Accepted cycles remain spent, and recovery fences remain enforced.
  Hosts own provider verification and integration; no public credit setter is added.
- Add [bounded host-authorized budget increases](docs/funding-credit.md#bounded-host-authorized-allocation-increases)
  after exact credit confirmation. Preserve spent totals, receipts and lifetime
  slots; exact grant replay changes nothing. Fixed installation ceilings and
  original accepted amounts bound every grant.

### Changed

- Check dependency selectors and tracked workspace lockfiles offline with
  `make dependency-pins-check`, including native macOS CI. Adopt the reviewed
  Shared Tooling snapshot and verified parser setup; use compatible registry
  requirements without changing locked selections.
- Select ic-testkit 0.18.3 for the unpublished local IC harness alongside the
  new memory contract. Keep earlier validation artifacts bound to their
  original dependency graphs.
- Add an [operator retirement runbook](docs/retiring-installations.md) for exact
  objects, references, uncertain effects, balances and continuing billing before
  hard-cut reinstalls. Document overlapping-release retention, cleanup receipts
  and interrupted publication using existing owners
  ([#6](https://github.com/dragginzgame/ic-blob-storage/issues/6)); actual provider
  deletion/billing and consumer adoption remain separate work.

### Testing

- Add opt-in `make test-funding-receipt-resources` for receipt-populated confirmation
  and restoration. Keep digest uniqueness in the existing funding owner and cover
  index corruption, bounded grants and rollback after complete synchronous writes.
- Cover repeated local IC dispatch after synthetic host credit confirmation,
  accounting-write rollback, receipt/authority refusal and fenced restoration.
  These substitute checks do not qualify deployed provider credit evidence.
- Add opt-in `make test-hard-cut` using a hash-pinned pre-cut Wasm to prove actual
  upgrade refusal preserves stable bytes and old obligations. Allow an explicit
  installed Chromium-family executable for serial media evidence and validate
  credential-free delivery without assuming cache-dependent GET counts.

### Fixed

- Verify retained evidence on macOS using the reviewed checksum helper's Perl
  fallback. Add an offline `make evidence-check` for hashes and refusal checks
  without compiling Rust or making provider requests.
- Enforce release identity and publication refusals under Apple's Bash, and stop
  validation after a failed prerequisite even when a cache exists. Run focused
  CI checks independently and retain failed fixtures for diagnosis.
- Adopt Shared Tooling's fixed formatting hook and automatic release recovery
  across newer committed fixes. Verify older receipts at the selected release
  commit before validating the next increment
  ([Shared Tooling #5](https://github.com/dragginzgame/shared-tooling/issues/5)).
  Retain actual failed release-gate logs across retries.

## [0.14.12] - 2026-10-05

### Changed

- Adopt the reviewed Shared Tooling release workflow. Patch, minor and major
  releases share validation and reconcile an interrupted attempt at its saved
  version when the same target is rerun. Push only the selected branch and tag;
  keep package publication and artifact cleanup separate.
- Add repository-local pre-commit formatting and Cargo manifest sorting, with
  matching independent checks. Install hooks once per clone and prepare the
  pinned formatter before running them. Hook setup resolves physical path
  aliases ([Shared Tooling #1](https://github.com/dragginzgame/shared-tooling/issues/1)).
- Replace manual release staging/commit/push steps with the common runner and
  explicit saved-release recovery. Version preparation updates workspace lock
  entries without reselecting dependency versions.

## [0.14.11] - 2026-10-05

### Added

- Add an opt-in restoration profile for occupied funding and read stores under
  the ordinary fixture envelope. Exact terminal, prepared and uncertain funding
  histories, interrupted read reservations and completion high-water identities
  survive actual same-release upgrades with mutation fences preserved. Retain
  local requests, callback traps, results and resource measurements.
- Extend occupied-history profiling to 100/1,000/10,000 lifetime funding
  intents and up to 1,024 occupied reads across 32 tenants. Preserve complete
  accounting, selected exact funding requests and every read target through
  actual same-release upgrades; verify atomic batch rollback, separate tenant
  and global capacity, operator authority, stale counters and restore fences.
  The private fixture declares explicit funding/read ceilings and admits local
  history through maintained owners; ordinary tests retain four intents and one
  read. No provider dispatch, production layout change or default scale workload.

### Changed

- Qualify the maintainer-selected ic-memory 0.25.9 and ic-testkit 0.15.8 graph
  through focused funding/read, standalone continuity/history/snapshot and
  ordinary/scale restoration checks. Scoped actual Rust 1.88 native and Wasm
  compilation passes. Retain upstream source comparison and prior artifact
  identities; no local adapter or production restore optimization is introduced.

### Fixed

- Restore seven vendored Shared Tooling guides through the pinned snapshot
  refresh helper after documentation banners changed their recorded bytes.
  Preserve repository-owned documentation banners and the original source
  revision, manifest hashes and file set.
- Point three retained consumer-feedback references at their exact pre-removal
  Git source rather than the retired local backlog file.

## [0.14.10] - 2026-10-05

### Fixed

- Update release-gate bootstrap fixtures for snapshot verification before
  dependency fetching. Verify that snapshot or fetch failure stops validation
  without changing release files or discarding retained build artifacts.
- Refresh the reviewed Shared Tooling snapshot to 0.1.0. Adopt the Bash 3.2
  snapshot-verifier fix and package-relative LOC classification with nested
  workspace members excluded from parent counts. Correct the README's LOC
  command scope to include every Cargo workspace member.
- Align dependency documentation with the selected management-types 0.11.0,
  ic-memory 0.25.5 and ic-testkit 0.15.4 graph. Correct release setup to verify
  the shared snapshot before fetching dependencies, preserving original
  qualification records and their artifact bindings.

### Added

- Extend opt-in service restoration profiling to 32 tenants with overlapping
  request IDs and 16/64 MiB multi-chunk manifests. Actual same-release upgrades
  preserve accounting, exact selected manifests, permissions, references and
  receipts, while refusing foreign callers and fenced mutations. Retain all six
  local workloads, requests, checkpoints and the initial bounded-input failure;
  default CI work and production contracts are unchanged.

### Changed

- Upgrade management-canister types to 0.11.0, select ic-memory 0.25.5 and
  update the test harness to ic-testkit 0.15.4. Verify current-instance recovery,
  interrupted history reads, retained liabilities and actual snapshot refusal
  on this dependency graph. Public APIs, persisted layouts and bounded
  IC-history acquisition are unchanged.
- Retain the transitive powerfmt update to 0.2.1. Its declared Rust floor of
  1.79.0 remains below the workspace's 1.88.0 MSRV.
- Adopt a reviewed Shared Tooling snapshot with the canonical engineering
  baseline, its linked guides and portable tools. Keep service-specific
  instructions in the local AGENTS overlay and verify snapshot integrity offline.
- Adopt the shared requirements to review GitHub repository descriptions and
  enumerate removed functions, methods and types in cleanup reports.
- Report Rust file and test-attribute counts for every Cargo workspace member,
  including canisters and test crates, with offline metadata discovery.
- Remove superseded local feedback/integration queues. Use GitHub issues as the
  tracker, retain historical source references and keep supporting evidence with
  its existing owner.
- Prepare synthetic population manifests off-canister through the existing
  Caffeine manifest builder. The private fixture accepts the complete bounded
  declaration and returns no duplicate requests. Its one current installation
  record declares tenant and object-byte ceilings; ordinary callers retain their
  two-object, two-tenant, ten-byte limits and the existing 4 KiB decoder bound.

## [0.14.9] - 2026-10-05

### Fixed

- Remove the mandatory Unreleased section and changelog section-order checks
  from release preparation. Finalize one current draft in place while preserving
  historical notes and rejecting duplicate, empty, conflicting or mismatched
  release drafts.

### Changed

- Allow an undated draft to omit its patch number or use an undecided label until
  release preparation assigns the selected version and date. Align governance and
  release fixtures with the Dragginzgame convention: latest notes first, no extra
  notes queue and no deployment gate for changelog presentation.

### Added

- Add a consumer-owned shared-tooling feedback record with evidence, proposals
  and adoption actions, linked from agent instructions and the handoff. Record the
  reviewed upstream engineering baseline's exact source identity and distinguish
  implementation, publication and consumer adoption. Local architecture and
  validation remain authoritative; no automation or mutable sibling build
  dependency is introduced.

## [0.14.8] - 2026-10-04

### Fixed

- Use the shared current installation encoder when funding transport tests
  upgrade their storage fixture. Preserve accepted/refunded cycle accounting,
  callback uncertainty and restored fences without an obsolete input reader.

### Added

- Add an opt-in PocketIC resource profile for 100, 1,000 and 10,000 lifetime
  operations, mixing uncertain uploads, live references, released objects with
  continuing liabilities and cancelled reservations. Measure the maintained
  synchronous store-opening path under normal application limits; preserve
  checkpoints, failures, accounting, receipts and restored mutation fences.
  Fixture-only per-memory read counters help attribute restoration costs.
  Default CI does not populate these workloads.
- Share bounded Node, dedicated-worker and owned Chromium memory sampling between
  1/8/32 MiB preparation and existing successful/lost-reply media journeys.
  Retain fresh profiles, partial observations and observer failures. These local
  substitute measurements do not qualify deployed Caffeine or million-object use.

### Changed

- Give the unpublished durable-storage fixture one explicit installation record
  with a bounded object ceiling. Ordinary tests retain their two-object envelope;
  all installation/reopen callers use the current record without a fallback reader.
- Reuse each tenant's activation generation within the existing synchronous
  restoration totals scan. Remove repeated enrollment reads while preserving
  every permission's generation check, accounting and recovery fences. No
  persistent cache, storage layout or public contract changes.
- Remove the media fixture's numeric byte bridge and duplicate browser decode.
  Keep exact native-download byte equality, public-response SHA-256, original
  metadata and browser delivery checks. The GLB delivery context's sampled Node
  RSS falls from 2.29 GB to 252 MB locally; production upload/download code is
  unchanged by this fixture simplification.

## [0.14.7] - 2026-10-04

### Added

- Add opt-in populated browser journal profiling through strict commits and
  graceful process restart. Preserve exact uncertain/cancelled histories and
  capacity/replay refusals at 675, 5,000 and 10,000 rows. Retain the earlier
  20,000-row attempt that exceeded the fixture deadline; no default CI workload,
  state counter, schema change or provider effect is introduced.
- Requalify complete GLB/WebP delivery and PNG/JPEG lost-reply/control-interruption
  recovery with matching rebuilt 0.14.6 CLI, standalone Wasm and browser tools on
  ic-memory 0.25.0 and ic-testkit 0.14.11. Verify the installed release, original
  roots/chunks/whole bytes/cache headers, recovered maps and reference liabilities;
  recovery dispatches no additional PUT. Retain hashes, logs and original profiles
  as local/substitute evidence, without claiming deployed provider qualification.

### Changed

- Simplify synchronous upload restoration: remove the temporary set of every blob
  root and reuse the decoded permission's admission time for attestation validation.
  Keep durable index, tenant/request uniqueness, accounting and recovery-fence
  checks. Corrupt duplicate bindings and pre-admission attestations still refuse
  without repair; storage layouts and public contracts remain unchanged.
- Refresh the acceptance plan to reflect retained live trials, current same-release
  recovery and local media qualification, while keeping consumer adoption,
  production serving and provider retirement obligations explicit.
- Clarify that IndexedDB point lookups and count do not guarantee constant-cost
  operations. Keep journal capacity checks and distinguish measured browser
  workloads from service-canister or million-object qualification.
- Move the completed 0.14.6 handoff narrative into retained history and keep the
  current handoff focused on release identity, qualification and remaining work.

## [0.14.6] - 2026-10-04

### Fixed

- Avoid a second full-body array copy when the patched Caffeine SDK constructs
  its Blob. Keep the snapshot before MIME detection awaits, exact selected bytes,
  upstream hashing/chunking and one-shot preparation handles. Chromium mutation
  checks and substitute SDK payload/lost-reply/budget checks pass.
- Send bounded base64 strings through the native-to-browser body bridge instead
  of serializing every byte as a Playwright argument element. Preserve the 64 KiB
  raw-frame bound, exact original bytes, SHA checks and SDK preparation before
  certificate intent. Binary-byte and existing Chromium failure/recovery checks pass.

### Added

- Add opt-in launcher/CDP/SDK profiling with retained original profiles and
  zero provider effects. Instrumented local preparation/refusal observations for
  1/8/32 MiB bodies improve from 2.99/22.88/86.41 seconds to 0.08/0.48/1.83 seconds.
  These measure the client bridge before issuance, not successful upload latency
  or Canic execution; worker/process peak memory and populated-store scale remain open.

### Changed

- Update the workspace ic-memory dependency to 0.25, locking 0.25.0.

## [0.14.5] - 2026-10-04

### Fixed

- Return typed `native-control` refusals for unknown/non-string continuation
  phases and unserializable native results. Check phase and final replies through
  one 8 MiB boundary; preserve context shutdown, profiles and original claims.
- Validate native local-verification permissions through the core's canonical
  validator before body access or queries, including invalid service principals.
  Preserve explicit role/scope refusals, byte bounds and original open-file ownership.
- Fence retained browser journal handles when IndexedDB invalidates or closes
  their connection. Use the same close owner as explicit shutdown, return typed
  `store-closed` refusals, and refuse a handle invalidated during startup checks.
  Preserve existing transactions and claims; opening missing storage still refuses.

### Changed

- Consolidate native saved-request and exact installation decoding into one
  bounded helper. Preserve command-specific byte/work/type limits, no skipped
  fields, exact single-argument inputs and separate authenticated reply contracts.
- Snapshot browser bootstrap input once and reuse one validated scope and trust
  root for host jobs and the private payload. Remove the repeated local validation
  while preserving independent worker checks before storage access.

## [0.14.4] - 2026-10-04

### Added

- Qualify source installation from clean Git release 0.14.3 with a fresh native
  prefix and pinned npm directory, without Git discovery. Exact installed tools
  and matching Wasm complete original-cache-root GLB/WebP and recover PNG/JPEG
  lost replies without another PUT. Consumer-owned installation remains separate.

### Fixed

- Keep browser shutdown and native argument/control serialization failures inside
  the redacted refusal boundary. Preserve the original startup/control failure
  during cleanup, release the asset server even when Chromium close rejects, and
  reject invalid native frames without writing or consuming a phase. Preserve
  profiles and uncertain claims; add no retry or replacement history.
- Give the serial publication fixture one 120-second session deadline for native
  original/recovery intents, browser bootstrap and subprocess ownership. Preserve
  shorter single-step bounds and the whole-fixture timer. Retain effective limits
  with completion/recovery evidence; production timeout defaults are unchanged.

## [0.14.3] - 2026-10-04

### Changed

- Let the existing local publication fixture consume an explicitly selected,
  hash-bound two-file media set. Derive installation byte limits from the bodies,
  retain original MIME through native/SDK preparation and serve the uploaded
  metadata. Keep authored PNG and synthetic cases on the same workflow.
- Extend browser delivery checks to JPEG/WebP images and complete GLB bytes with
  structural inspection. Retain independent whole-file verification, canonical
  URLs, selected-origin CORS and explicit local image CSP checks. Private consumer
  media stays outside the repository; game rendering and deployed serving remain
  separate acceptance.

### Added

- Exercise valid byte-bounded Candid skipped-value and type-table refusals at
  standalone query/update, manifest, certificate and installation boundaries.
  Preserve owner state on rejection and failed reinstall, include valid controls
  and check dense near-limit manifests reach the domain count refusal. Document
  the subsumed header ceiling and limits of decoding-work coverage.
- Add offline machine-readable `blob-storage --version` and document one selected
  source checkout for native installation, pinned browser builds and retained
  artifact identities. Verify a fresh-prefix CLI installation with the actual
  SDK/native snapshot handoff, then qualify an isolated source copy with fresh
  npm dependencies and no Git metadata. Use its exact installed CLI, matching
  browser bundles and Wasm for cached media completion and lost-reply recovery
  without another PUT. Retain build-profile/toolchain provenance; downstream
  adoption remains separate.
- Qualify the concurrently selected ic-memory 0.24.10 graph with matching Wasm
  initialization, same-release stop/start/upgrade fences, obligation-preserving
  recovery and stale-snapshot refusal. Retain its source/artifact identities
  separately from earlier media publication observations.
- Exercise native map-output refusal after completed uploads, then recover a
  complete map through fresh signed queries without another upload. Preserve the
  occupied output, exact completion history and overlapping-reference cleanup.
- Record representative frozen consumer-byte completion, lost-final-reply recovery
  and tail-corruption refusal. Qualify an eight-chunk model and actual image
  decoding locally; reference release retains physical/billing liabilities.
- Preserve explicit original `Cache-Control` metadata through native bindings,
  transfer descriptors, browser snapshots and the same SDK preparation. Qualify
  all four retained consumer roots/headers/leaves, local cached model completion,
  lost-image-reply recovery and corruption refusal. Changed or omitted required
  hints refuse before certificate intent; omission still adds no cache header.

### Fixed

- Refuse predictably impossible publication transfer budgets before saving
  certificate intent. Use SDK-owned chunk sizes and the rebuilt manifest to
  check tree/chunk request count, largest chunk and total body bytes. Preserve
  exact gateway checks for later envelope overhead, existing claims and lost-reply
  recovery; return the specific refusal through the worker without dispatch.

## [0.14.2] - 2026-10-04

### Added

- Exercise ordinary PNG image loading from canonical native download URLs under
  an explicit local Content Security Policy. Allowed anonymous images decode
  with readable canvas pixels; blocked images produce an enforced `img-src`
  violation without reaching the provider substitute. Cover small/multi-chunk
  completion and lost-reply recovery; native whole-content verification remains
  authoritative and deployed consumer serving policies remain unqualified.

### Changed

- Give browser media fixtures one helper for downloaded-byte decoding, direct
  URL checks and CSP image checks. Reuse frozen body expectations and the native
  target owner instead of reconstructing download routes or duplicating sampling.
- Retain private bounded browser request traces when fixture assertions fail,
  alongside original profiles, journals and native evidence. Keep failed attempts
  and their exact source/bundle identities in the probe ledger.

### Fixed

- Fetch locked Rust dependencies before the CI, validation and release gate's
  offline checks. Dependency updates no longer require manual cache preparation
  for that gate; failed fetching stops before validation or version mutation.
  Keep scoped checks offline, preserve versions/build artifacts and test both
  ordering and failure handling with isolated Cargo substitutes.

## [0.14.1] - 2026-10-04

### Added

- Exercise distinct valid PNGs through the maintained native/Chromium publication
  driver, whole-file verification and tenant downloads. Compare downloaded bytes
  with frozen originals and decode dimensions/pixels in Chromium. Qualify local
  success, lost-reply recovery without another upload and corruption before
  attestation; consumer media and deployed serving guarantees remain open.
- Extend local publication acceptance to a PNG with distinct full/partial chunks,
  lost final-chunk recovery and corruption beyond the first chunk. Exercise
  signed second-reference retention, download after first-reference release,
  final descriptor refusal and unchanged physical/billing-liability accounting.
  Fresh maps become incomplete after release; historical retain success remains
  an observation of the original operation.
- Fetch the canonical native download URLs directly from Chromium under the
  local substitute's selected-origin CORS policy. Check PNG MIME, length,
  digest, dimensions and pixels without credentials; another origin cannot read
  the response. Saved public URLs remain accessible after logical release even
  when service descriptors refuse. Deployed serving/cache/CSP remains unqualified.

### Changed

- Refresh the locked ic-memory dependency to 0.24.3 and ic-testkit to 0.14.2;
  keep package/dependency versions inherited from the root workspace.
- Give publication fixtures one body-based manifest/freezing path. Derive byte
  budgets and retained accounting from the selected files; preserve the existing
  synthetic restart/corruption journeys and compare their downloaded bytes too.
- Let the shared HTTPS/HTTP2 substitute validate bounded ordered chunks against
  frozen bodies and SDK manifests. Derive transport limits/request counts from
  selected files and pass exact reference identities to the download fixture.
  Compare large byte buffers using the existing length/first-difference checks
  to keep failure diagnostics bounded.
- Refresh release/status documentation for 0.14.0 and distinguish its released
  driver from unfinished consumer adoption.

## [0.14.0] - 2026-10-04

### Added

- Return native `next_frame` guidance from checked publication phase results and
  original source paths. Start each execution with fresh status at index zero;
  recover original observations/handoffs before considering setup, and advance
  only on authenticated exact completion/reference evidence.
- Add Chromium `driveSession(nativeControl)` to follow native guidance through the
  existing transfer/verifier/map owners. Bound the loop by the selected step
  budget, validate current peer replies, exclude concurrent browser jobs and stop
  control waits when the context closes. Keep process/key selection and explicit
  restart with the caller; no automatic retry or new durable journal.
- Require the exact current complete map, matching native final report and exit
  zero before the browser driver returns `complete`. Keep stopped outcomes with
  their final native result; bind inventory/installation, ordered indices and
  neutral lease/serving facts at the peer boundary. Cancel pending final-exit waits.
- Add `startPublicationSession` for an explicitly selected native binary, key-file
  arguments and environment. Own private bounded pipes, one outstanding phase,
  abort/deadline shutdown and the final report plus exit status. Add the focused
  `test-browser-native` target; no key discovery or process restart is inferred.

### Changed

- Centralize every Cargo dependency version and local dependency path at the
  workspace root. All members, including unpublished fixtures, inherit the
  workspace package version; preserve existing pins, features and target scopes.
- Give all indexed native phases one ordering guard while preserving durable
  unattempted facts for phases that own claims. Share startup/current ready checks
  and one execution gate between manual browser jobs and session driving.
- Keep browser request correlation monotonic when the driver follows manual jobs;
  request IDs remain execution metadata, never effect identity or replay authority.
- Run the coordinated local IC/Chromium journeys through the maintained native
  subprocess helper. The fixture observes effects and completion rather than
  forwarding phase commands through a separate Rust control proxy.
- Update ic-memory to 0.24.0, ic-testkit to 0.14.1 and direct
  ic-management-canister-types to 0.10.0. Core/standalone/CLI compile without
  adaptation; fresh standalone lifecycle/recovery and snapshot refusal retain
  their behavior.

### Fixed

- Remove the installation test's obsolete requirement that the harness and
  library versions differ. Preserve installed-Wasm release verification,
  wrong-service rejection, unchanged state on refusal and exact CLI carrier
  installation after workspace version inheritance.

## [0.13.0] - 2026-10-03

### Breaking

- Require the original `publication-binding.json` when reopening a Chromium
  profile. One frozen launch binding identifies signer, origin/profile, scope,
  trust root, journal, executable bundles and original native session (or explicit
  browser-only operation); missing or changed history refuses
  before Chromium starts. Old profiles remain with their original launcher and
  bundles; no inferred binding or conversion.
- Identify native publication-session intent as
  `ic-blob-storage/publication-session:retained-browser-handoffs`, with explicit
  source-session, nullable browser selection and original transfer provenance.
  Old session intents are not converted or accepted by
  the new reader; retain their original binaries and phase journals. Service
  endpoint and stable layouts are unchanged.

### Added

- Add a native `transfer` phase that saves a bounded, synchronized handoff after
  validating the original signed setup. Native-bound launchers require that phase
  directory; repeated or recovered handoffs request certificate recovery only.
  Missing browser history refuses without starting another upload. Existing
  IndexedDB claims remain the sole browser effect owner.
- Add `publish-session --browser-selection` to retain original session/profile/port,
  signer and bundle fingerprints, and journal/project/bucket selection before phases.
  Recovery requires the same selection. The launcher checks the original intent,
  ready record and input hashes before Chromium; a native-bound profile cannot
  reopen in browser-only mode. This adds provenance, not dispatch or completion authority.
- Persist a private, bounded launch binding before opening Chromium, with exclusive
  creation and file/directory synchronization. Store signer and bundle fingerprints,
  never signer keys. Preserve the record on launch failure and require exact
  immutable inputs on restart; runtime budgets remain per execution.
- Add `publish-session --source-session` to recover original setup and verifier
  observation paths from a retained same-release run. Validate frozen inputs,
  actors, installation/release, gateway and trust root before creating output;
  retain unused sources through status-only restarts and refuse conflicting,
  partial or missing provenance without authorizing replay.

### Changed

- Native sessions with a browser selection return transfer descriptors only through
  the retained transfer phase. The launcher refuses direct-upload bypasses and
  checks original/current native binding, index, control frame and recovery source
  before delegating to the existing worker. Native-only and browser-only operations
  retain their distinct supported contracts.
- Own the complete browser launch selection before filesystem awaits, preventing
  caller mutation of profile, origin or bootstrap during launch. Use that one
  snapshot rather than cloning bootstrap again.
- Bound the complete native session intent before output allocation so startup
  cannot write a record that its same-release recovery reader cannot consume.
- Give source selection one owner and persist resolved paths before phases.
  Record ordering refusals in the existing step history so absence of a phase
  directory cannot be mistaken for permission to start again. Setup, verification
  and attestation claims remain authoritative; recovery inherits no completion cursor.
- Share immutable host configuration/release decoding between fresh observations
  and retained session history. Refresh release/dependency documentation.

## [0.12.0] - 2026-10-03

### Breaking

- Remove the superseded `workflow::reads::download::describe` API and public
  operational descriptor wrapper/error surface. Use the maintained `handle` with
  `DownloadRequest`/`DownloadResponse`; historical inspection retains its contract.
- Replace the ambiguous numeric marker on native upload bindings with the frozen
  `ic-blob-storage/upload-inputs:original-preparation` layout identity. Single/batch
  preparation and session reopen share one strict reader; update producers together.
- Identify the immutable installation layout as
  `ic-blob-storage/installation:platform-anchor` independently of its exact release;
  replace `InstallationBindingError::Schema` with `Format`. This stable-format hard
  cut requires retirement before fresh reinstall. Existing artifacts and live
  owners remain on their original release; no conversion, replay or migration.
- Retire the local-only `blob-fixture-reference` save/inspect command, its JSON
  format and append-only filesystem-lock journal. Reference preparation, signed
  submission and receipt recovery use the maintained native commands. Preserve
  existing artifact files; no conversion, deletion or effect replay is performed.

### Changed

- Serve download metadata directly from the existing retained descriptor under
  the checked installed scope. Remove the discarded server HTTP target and scope
  clone; clients keep the canonical target builder and approved gateway origin.
- Use the same UTF-8 byte bounds for browser worker and publication metadata;
  refuse empty/oversized manifests and oversized hints before journal access.
  Snapshot selected upload bytes once inside the shared job helper while retaining
  caller-mutation protection and bounded backing-buffer ownership.
- Keep one semantic exported/deployment Candid equality check independent of
  canister installation; remove the duplicate generated-text comparison.
- Separate the current release/operating handoff from implementation history and
  correct recovery documentation to describe independent current-instance proofs.
- Clarify contributor policy: maintain one unversioned/V1 model and identify
  incompatible frozen formats explicitly, without reusing a discriminator or
  adding compatibility readers. Endpoint request/response layouts remain unchanged.
- Remove duplicate fixture reference journeys while retaining signed tests for
  immutable receipts, conflicts, settlement and restored inspection. Align the
  PocketIC harness MSRV with the workspace after removing its file-lock requirement.

## [0.11.0] - 2026-10-03

### Breaking

- Require `preparation` in native upload/publication bindings, containing the
  original optional `content_type`/`filename` strings. Preserve omitted arguments
  and empty strings through frozen inputs and session transfers; reject null,
  unknown fields and oversized UTF-8 hints. Replace the current v1 input contract
  directly, without compatibility readers. Service wire/state generations remain
  unchanged; original journals and outstanding obligations must still be retained.

### Added

- Add a native Playwright Chromium bridge around the maintained browser host:
  explicit persistent profile and fixed asset origin, bounded selected-body reads
  and digest checks, chunked process handoff, finite redacted failures and profile
  retention on close/deadline. Keep service phases and completion/retry authority
  with the native session and original journals.
- Add an optional configured-verifier phase to `publish-session`: observe exact
  provider bytes, bind the original upload and raw digest before one attestation,
  then advance only on authenticated live-reference evidence. Recover original
  observations through read-only history checks without another GET/submission;
  retain all partial claims and stop on corruption or uncertain effects.
- Add a maintained browser worker host and private-port bootstrap with explicitly
  selected Ed25519/secp256k1 SDK identities. Validate signer pairs, exact uploader,
  bindings and budgets before opening the existing IndexedDB journal; terminate
  on close/abort/deadline while retaining original history. Replace fixture-only
  bootstrap/control with this host and worker entry.

### Changed

- Share browser certificate-binding and gateway-budget rules across the client,
  durable journals and worker; reject malformed worker bindings before store access.
- Pair each frozen publication input with its raw digest and use typed native
  setup/preparation/completion outcomes for control decisions. Keep JSON reports,
  durable claims and original-operation recovery unchanged.
- Remove the repeated retained-content lookup when serving a download descriptor;
  preserve tenant authority, restore fences and asynchronous read rechecks.
- Correct current release/dependency and persisted-owner documentation while
  retaining historical probe evidence.
- Use one native upload-identity formatter for attestation, certificate assessment,
  history and local verification. Share manifest reply bounds across setup,
  verification, provider observation and attestation submission; preserve each
  command's content limit, authority checks, error mapping and JSON shape.
- Remove repeated scope/tenant validation from reference-receipt inspection;
  reuse the exact request loader that also serves reference submission.
- Bound private job/root snapshots to selected byte views before cloning, and
  share worker configuration/job predicates between direct and hosted boundaries.

## [0.10.0] - 2026-10-03

### Added

- Add passive complete installation memory requests and `LIBRARY_VERSION` for
  embedding hosts. Standalone uses the shared seventeen-request inventory and
  compiled library identity without selecting a second runtime or allocator.
- Add `publish-map`: reverify the frozen batch once, authenticate installed
  configuration with the operator and inspect tenant enrollment, exact verifier
  digests and current first references. Save bounded query evidence; emit a
  complete media map only when every file passes. Preserve original indices,
  identities, content digests and explicit provider targets without updates,
  provider requests, retries or a publication/retention lease.
- Add `publish-file-status` for serial publication: use the same installed-state,
  enrollment, verifier-digest and live-reference checks for one original frozen
  index, with four bounded queries and retained evidence. It emits no batch map.
  Local two-file Chromium/PocketIC journeys exercise completion at one active
  reservation, browser restart after a lost chunk reply and corruption blocking
  the next transfer; existing dispatch journals and accounting remain authoritative.
- Add `publish-session`: retain one validated frozen batch across bounded setup,
  per-file completion checks and final-map phases. Recheck only the selected body
  before setup, require exact confirmed references before advancing, and recover
  original signed setup journals without redispatch. Local native/Chromium cases
  cover process interruption, lost upload replies, tampering, corrupt downloads
  and idle deadlines. Browser transfer and verifier tools remain separate owners;
  the complete production headless publisher is still unfinished.
- Add a browser publication worker over a trusted private MessagePort. Bind its
  signer, service, tenant, project, bucket, origins and budgets before jobs; reuse
  the existing SDK and IndexedDB journals for one active upload, inspection,
  certificate recovery and cancellation. Return redacted results without signed
  envelopes or service-completion claims. Real dedicated-worker journeys cover
  serial completion, lost replies across browser/native restart and cancellation
  with corrupt observation; native parent automation remains unfinished.

### Changed

- Select the `ic-memory` 0.15 and `ic-testkit` 0.11 dependency families; the
  workspace lockfile resolves 0.15.7 and 0.11.0 respectively.

### Fixed

- Keep the serial browser publication plan in its consuming test module so
  shared browser helpers compile without dead-code errors in the storage suite.
- Open native inputs nonblocking on Unix, then validate the same descriptor as
  a regular file before reading. Share the check with streaming body snapshots
  so FIFOs without writers cannot hang before validation or transport deadlines.

## [0.9.0] - 2026-10-03

### Breaking

- Remove the standalone crate's public `dto` forwarding namespace. Import
  `HostConfigurationView` and `HostFailure` directly from
  `ic_blob_storage::dto::configuration`; the core owns these types with no
  compatibility alias. Update standalone handlers and PocketIC consumers together.

### Added

- Add bounded `publish-prepare-batch`: verify the complete frozen batch once,
  then reuse the maintained serial setup workflow and independent per-file signed
  journals. Stop on blocked, pending or failed setup; recover only the exact original
  file with `publish-prepare-resume`. Retain finite request/deadline budgets and
  capacity checks without certificate, provider transfer or publication effects.
  Active reservations still limit progress; single-file setup and recovery
  independently reverify the complete batch.

### Changed

- Share CLI canonical parsing and batch limits across commands while retaining
  role-specific authority checks. Consolidate signed claim, durable packet/intent
  persistence, one-shot dispatch and bounded response storage for upload setup,
  references, verifier attestations and gateway controls.
- Centralize transient permission-view construction. Retain the heap reference
  model and its independent durable accounting comparisons; it is not a service
  persistence or restore implementation.

## [0.8.0] - 2026-10-03

### Breaking

- Require the host's immutable IC installation version in the current v1
  configuration record and installation candidate. Add operator-only
  `blob_resume_current_instance`, backed by bounded replicated IC management
  history and a sealed same-execution proof. Same-release upgrades remain fenced
  until this proof succeeds; every journal, reservation and unresolved obligation
  survives activation. Snapshot loads, state replacement and incomplete/unknown
  history refuse recovery. The standalone host fences management-version gaps,
  including stop/start and old heap restoration. Previously active owners qualify
  ambiguous execution gaps with one independent preflight before mutation; restored
  owners still require explicit operator recovery. Capture ingress caller before
  the await and reject history invalidated by intervening execution/management.
  Cross-release transitions remain
  reinstall-only; existing installations cannot upgrade across this hard cut.

### Added

- Add browser `createPublicationUpload` for a frozen native file. Verify the
  saved body digest, exact metadata/leaves and rebuilt Caffeine SDK root before
  certificate intent; reuse the existing certificate, transfer and durable gateway
  journals. Local single-file frozen-batch journeys cover upload, independent
  verification, attestation, download, release and interrupted/corrupt outcomes
  without redispatch. Add opt-in `test-browser-publication` for offline frozen-input
  refusal and snapshot checks; complete batch publication remains unfinished.
- Add indexed `publish-prepare` and `publish-prepare-resume` for complete frozen
  batches. Authenticate tenant and uploader independently, check the selected
  root/capacity and reuse signed admission/preparation journals. Recovery binds
  exact original scope and packets, never resubmits a claimed update, and can
  complete only a previously unclaimed preparation. No certificate or provider
  transfer occurs; partial claims, retired content and fences remain blocked.
- Add authenticated `publish-check` for complete frozen batches. Reverify saved
  bodies and request packets before networking; retain bounded capacity/content
  queries and replies, original discovered IDs, partial failures and explicit
  recovery/reuse/retirement blockers. Separate history, bytes, leaves, concurrency,
  suspension and restore fencing. No ID allocation, reservation or provider calls.
- Add offline `publish-inputs` for a frozen, explicitly bound upload inventory.
  Reuse the existing installation/manifest validators and streaming body verifier;
  check input hashes, unique operation/object/reference identities, one tenant/
  provider scope and aggregate fresh-installation object/leaf/byte capacity.
  Save serial verified snapshots and original request files; retain partial failure
  evidence, refuse overwrite and emit a batch summary only after every file passes.
  This prepares inputs without observing live capacity, allocating IDs or uploading.
- Prepare a separate frozen 0.7.0 trial packet and exercise real Caffeine SDK
  preparation plus native batch verification for 1 KiB and 10 MiB files. Retain
  the original 0.6.0 owner's failed upload, balances and liabilities.
- Complete live 1 KiB and ten-chunk 10 MiB Caffeine uploads on a separate 0.7.0
  owner, independent whole-byte verification, accepted verifier attestations and
  authenticated tenant downloads matching the original snapshots. Retain exact
  requests, browser journals, financial observations and failed preparations.
  Logical reference releases retain all physical bytes and billing liabilities;
  production consumer acceptance, deletion and billing cessation remain open.

### Changed

- Raise the configurable browser journal ceiling to 1,000,000 lifetime attempts,
  independent of the 4,096-file native batch bound and service capacity. Preserve
  immutable capacities and cancelled/dispatched history. Actual Chromium checks
  retain 675 rows across restart; one million populated rows remains unqualified.

### Fixed

- Remove the unused Canic build-lock artifact; production code and dependency
  resolution remain independent of downstream frameworks. Record consumer-owned
  adapter/Toko hard-cut actions and separate provider deletion/billing evidence
  requirements in the service-gap review.
- Independently check the selected compiled host release in the PocketIC
  installation carrier test. Make supplies the current workspace version;
  deployment reviews supply their selected version. Catch stale preserved build
  artifacts after release-file version changes and document rebuilding and
  qualifying deployment bytes. The live trial catches a 0.6.1 artifact before
  provider exposure, then qualifies 0.7.0 and corrects only the empty new owner.

## [0.7.0] - 2026-10-02

### Breaking

- Replace the certificate policy's fixed 1 KiB/single-object trial gate with
  validated installation resource limits, allowing larger and multi-file uploads.
  Admission still enforces object size, tenant/global bytes, lifetime history,
  leaves and concurrency. Explicit uploader trust, exact permissions, one-time
  exposure and restore fences remain enforced.
- Remove `RestrictedUploadEnvelope`, `UploadExposureHostEvidence::trial_bounds`
  and `UploadExposureBlocker::TrialBounds` from the public policy, DTO, Candid
  and native JSON without a compatibility path. Consumers must update their
  wrappers and configure their own resource limits. Cross-release installation
  changes remain reinstall-only after obligations are preserved or discharged;
  the existing deployed 0.6.0 trial remains unchanged.

### Added

- Verify actual local canister certificate issuance for a 10 MiB object and a
  second multi-chunk object under one installation. Exercise configured size and
  capacity rejection, caller/trust checks, replay refusal, revocation, retained
  liabilities and restore fences; update native assessment and Candid checks.
- Document consumer sizing for Toko Miner's recorded asset envelope, including
  separate lifetime history, overlapping-release bytes and client/read budgets.
  Record the remaining wrapper, publisher and delivery qualification work.
- Retain the actual-service 1 KiB upload packet, exact native/browser bindings
  and persistent IndexedDB history across browser restart. Record the approved
  payment-link expiry update and first mainnet-verified standalone certificate.
  Caffeine rejects the tree upload with HTTP 403 for insufficient owner balance;
  preserve its certificate, browser claim and exposure before any chunk or retry.
- Reconcile a 4.1T gross payer top-up and the approved daily allowance increase
  from 1T to 5T with expiry unchanged. Verified readback shows 4.0996T remaining
  in the payer, 1T allocated to the owner and nonzero gateway credit with zero
  reported usage. Preserve bounded requests, failed checks and budget accounting;
  internal credit allocation is not another donor debit. A complete live upload
  and independently verified download remain unqualified.

## [0.6.1] - 2026-10-02

### Added

- Record the first isolated mainnet standalone trial: exact installation and
  configuration readback, reconciled cycles-ledger deposit and Cashier credit,
  verified payer settings, shared gateway/balance inspection and an approved
  payer-to-canister link. Retain signed requests, failed observations and budget
  accounting in the Caffeine probe ledger.
- Document the bounded payment-link expiry experiment. Exact terms read back,
  but gateway credit remains zero before and after the candidate expiry, leaving
  expiry enforcement inconclusive. Live upload/download, project/bucket acceptance
  and billing cessation remain unqualified.

### Changed

- Exercise the maintained standalone trial template through the declared/exported
  Candid contract, offline CLI carrier and actual PocketIC installation. Check
  exact readback, wrong-service refusal and controller denial; retain optional
  installation evidence and a frozen 0.6.0 artifact packet for trial review.
- Update the workspace dependency and lockfile to ic-memory 0.15.3.

## [0.6.0] - 2026-10-02

### Breaking

- Require HTTPS and browser request-stream support for upload transport. Send
  each snapshotted SDK PUT payload through one immediately closed stream with
  `duplex: 'half'`; HTTP/1.x fails without a buffered fallback. Trusted fetch hooks
  must preserve the stream.
- Adopt the accepted restricted upload contract: certificate issuance requires
  an explicitly installed trusted uploader, a matching local namespace, a
  one-tenant/one-object/1 KiB lifetime envelope and the current durable owner.
  Provider spending caps, replay-charge guarantees and operational old-backup
  recovery are outside this contract. Exact permission checks, one-time exposure,
  uncertainty accounting and inspection-only restoration remain enforced.
- Add required `trusted_uploader` to standalone installation, shared installation
  candidates and configuration readback; persist it in the current v1 record.
  Replace certificate blocker DTOs and native JSON with the current local facts.
  `installation-check` now requires `--trusted-uploader`. These changes require a
  minor release and reinstall after obligations are preserved or discharged.
- Move complete installation input into the core as
  `dto::configuration::ServiceInstallationInput`; remove the standalone's
  `HostInstallationInput` without an alias. Standalone Candid and consumers use
  the shared passive DTO; lifecycle ownership remains with the host.
- Require explicit project and bucket in offline upload-input JSON and the browser
  certificate binding. Retain them in the current v1 journal before issuance;
  remove independent transfer namespace options. Reopening with changed values
  refuses without dispatch.
- Require `upload-inputs --installation` with the complete current installation
  carrier. Reject inconsistent service, namespace, project or trusted uploader and
  enforce the candidate's object/manifest bounds before producing upload files.
  Preserve exact init bytes and their hash; this offline check grants no installed
  authority or provider provisioning guarantee.

### Added

- Add an offline standalone trial configuration envelope with one 1 KiB lifetime
  object/reference, retained cleanup capacity and a fully reserved service cycle
  attachment allocation. Document isolated role/account preparation and the
  proposed Cashier cycles-ledger deposit route without deploying or funding it.
  Retain provider-returned deposit metadata and the fresh payer's signed
  AccountNotFound observation separately from anonymous authorization refusal.
- Extend the complete standalone/browser rehearsal with a lost final upload
  reply, supported owner stop/start and independent verification without another
  upload dispatch. Cover withdrawal before late completion, explicit reference
  release and exact attestation replay that leaves the released reference inactive.
  Uploads and both native reads now use one TLS HTTP/2 gateway with normal native
  certificate validation; unrelated roots and a refused stream preserve failed
  observations without attestation or automatic read retransmission.
- Reusable bounded IndexedDB intent storage for the browser certificate and gateway
  clients. Explicit creation/reopening, immutable lifetime capacity, strict atomic
  commits and validated history preserve cancellation and uncertain requests across
  tabs and reloads. A missing journal refuses rather than silently starting fresh.
- Journal-only Chromium checks (`make test-browser-store`) cover browser-process
  restart, concurrent claims, retained tombstones, request bounds and corrupt
  history; the upload fixture now exercises the same maintained store.
- Complete local standalone/browser rehearsal (`make test-browser-standalone`)
  exercises actual installed certificate facts, SDK upload, independent native
  verification, distinct-verifier attestation, tenant download and reference release.
  A separate corrupt-body journey refuses completion without erasing exposure or
  liabilities. Both use a local gateway substitute, with no live provider effects.

### Changed

- Explicitly enable the native CLI's reqwest TLS and HTTP/2 features. CLI-only
  builds previously lacked HTTP/2; dependency versions, memory composition and
  platform certificate verification remain unchanged.
- Mitigate the reproduced hidden Chromium chunk PUT replay using streamed bodies.
  Owned HTTP/2 connection-close, immediate receipt-close and refused-stream cases
  retain uncertainty with one arrival; buffered fetch/XHR controls repeat. The
  actual standalone rehearsal recovers pre-header loss without another upload.
  Add opt-in `test-browser-transport`; retain failed/corrected experiments and
  provider/browser compatibility limits without claiming exactly-once billing.
- Clarify browser budgets as guarded fetch-dispatch/body bounds. A retained local
  Chromium diagnostic observes a transparent repeated chunk PUT after a pre-header
  connection reset despite disabled SDK retries; exact wire-attempt counts and
  replay charges remain unqualified. Preserve the failed experiment separately.
- Offline `installation-check` now writes complete `installation.candid` with
  its hash alongside the exact original configuration. Actual local installation
  accepts these bytes and independently rejects an incorrect service binding;
  offline validation still grants no platform or provider authority.
  CLI help now describes the complete init output.
- Adopt the maintainer's ic-memory 0.15.2 dependency update. Review its addressed
  IcyDB lint feedback and unchanged read contract; qualify current installation,
  typed growth-refusal rollback, exact retry and fenced restoration locally.
- Make the offline SDK/native handoff rehearsal accept an explicit body size up
  to its existing 10 MiB bound, so the first 1 KiB standalone trial can use the
  same preparation, snapshot, repeat and corrupt-source checks. Invalid sizes
  refuse before creating output; the default remains 10 MiB.
- Remove the unused browser refusal branch for the deleted managed fixture;
  retain current browser cancellation, concurrency and gateway failure checks.

## [0.5.0] - 2026-10-02

### Breaking

- Upgrade the public `ic-memory` re-export to 0.15.0. Direct runtime growth
  returns typed results; generic stable-structure memory keeps its trait contract.
- Remove the Canic adapter crate, managed fixture and integration suite completely.
  Consumer frameworks now own their wrappers and integration tests.

### Added

- PocketIC growth-refusal coverage checks whole-state rollback, a populated
  neighbor, exact storage-only retry and fenced restoration.
- Offline `installation-check` validates complete proposed configuration, project,
  verifier and release through the shared installation owner before preserving
  exact inputs. It refuses malformed/extra Candid, inconsistent limits and existing
  output without deploying, allocating memory or qualifying provider facts.
- Offline `account-link-inputs` generates reviewable Cashier Candid with explicit
  canister, payer, caller, positive daily limit and expiry. It preserves exact
  bytes and hashes in a fresh directory without signing, linking or funding.
- Reusable private browser upload composition binds the patched Caffeine SDK to
  the admitted certificate owner/root and existing gateway journal. It fixes
  serial transfer and disabled retries, requires explicit traffic/namespace inputs,
  and refuses wrong roots or malformed namespace values before issuance.

### Changed

- Lower the library, standalone and CLI MSRV from 1.98.1 to 1.88.0. The unpublished
  PocketIC harness declares 1.89.0 for native file locking. Pin development Rust,
  Clippy and rustfmt to 1.99.0 independently of those compatibility requirements.
- Adopt Rust 1.99's assertion diagnostics and result-use lint without suppressions;
  recovery tests express the same 24-hour delay using the supported seconds API.
- Remove downstream Canic dependencies and build/test targets from this repository.
  Library and standalone validation no longer require Canic releases or tools.
- Browser and SDK fault fixtures use the same upload composition; the build checks
  both SDK peer pins. Existing reload, cancellation, lost-response and cleanup
  behavior is preserved, including the fixture's post-response abort timing.
- Refreshed public Caffeine source/npm evidence and recorded the remaining trial
  gates separately from local client guarantees; provider host facts stay false.
- Refreshed anonymous deployed Cashier metadata and public pricing observations.
  Both match retained replies; historical guide pricing differs in request-unit
  labels, so no conversion or enforced financial guarantee is inferred.

## [0.4.16] - 2026-10-01

### Added

- Opt-in `test-canic-browser` connects the pinned Caffeine SDK and existing browser
  client to actual managed consumer admission and uploader preparation. Competing
  tabs claim one certificate request; refusal sends no gateway traffic, and exact
  uncertainty survives reload before explicit cancellation and tenant withdrawal.
  Unexposed reservations are released, with cancelled history retained after restore.

### Changed

- Browser composition cases share one bounded Rust control/process helper, including
  child termination on failure. The README reflects completed managed endpoint and
  Canic 0.110.49 hook adoption; provider/recovery qualification remains open.

## [0.4.15] - 2026-10-01

### Added

- Verified `upload-inputs` also exports `certificate-binding.json` for the existing
  Caffeine browser certificate client, deriving exact identities, root and opaque
  permission bytes from the maintained Rust contract without manual JS encoding.
- Opt-in `test-sdk-inputs` checks real pinned Caffeine preparation, a 10 MiB native
  snapshot/browser binding, matching SDK preparation after source edits, and
  repeat/corrupt-source refusal with all network calls blocked.
- Managed fixture certificate endpoint uses Canic's public plain-record rejection
  hook and the shared synchronous issuance workflow. Provider prerequisites still
  refuse exposure; its Candid declaration matches standalone.

### Changed

- Managed blob queries, updates and lifecycle now select bounded Candid decoding;
  local IC checks cover malformed/oversized/type/header input, manifest boundaries
  and occupied-owner rollback.
- Both direct Canic dependency pins now select published 0.110.49, which provides
  the certificate rejection and decoding hooks previously tested through frozen
  local source. The storage core remains independent of Canic.
- Documented local Canic development with frozen source and isolated dependency
  overrides; integration checks require no live Fleet deployment.
- Persistent Canic integration feedback and handoff reminders are required by
  AGENTS.md.

### Fixed

- Managed operator boundary coverage now distinguishes Candid decoding-work
  limits from the 4 KiB byte limit. Small skipped fields reach scope validation;
  valid but over-budget input and oversized input refuse without changing owners.

## [0.4.14] - 2026-10-01

### Added

- Offline `upload-inputs` converts Caffeine's prepared manifest and explicit
  original bindings into admission/preparation and first-reference read requests.
  It saves a complete root-verified `body.bin` snapshot; corrupt bodies leave
  private partial output without usable requests. IDs are explicit and full-width.
- Offline `reference-inputs` generates exact retain/release, status and download
  Candid from the original saved permission and explicit reference/operation IDs.
  It retains permission bytes and hashes without allocating identities, renewing
  expiry, dispatching or granting liveness/retry authority.
- Native `admit-upload`, `prepare-upload` and `revoke-upload` persist the exact
  signed request before one local service update. `upload-permission` and
  `upload-manifest` recover the original retained records without redispatch,
  renewing expiry or issuing a certificate. Tenant and uploader roles stay distinct.
- Shared signed upload setup journeys through standalone and Canic recover lost
  admission and pending preparation, reject corrupt manifests before dispatch,
  verify generated snapshots after source edits, cancel unexposed reservations
  and preserve accounting/history through fenced restore.
- Native `download` authenticates the exact tenant's replicated live-reference
  descriptor, fetches one bounded provider body and writes a usable `body.bin`
  only after complete EOF/root verification against original service metadata.
  Failed bodies remain private partial files; redirects, automatic retries and
  content decoding are disabled, and existing output is never overwritten.
- Signed managed completion-to-download coverage exercises verified file output,
  corruption, truncation, oversize, redirects, released references and fenced
  restoration over labelled local exposure/content substitutes. Signed standalone
  unconfirmed/restored refusals issue no provider GET. Real uploads still require
  the existing provider, recovery and framework qualification.
- Generated reference inputs now drive signed retain/release and receipt/status
  recovery. A lost retain reply is recovered without resending; a second reference
  still downloads after the first is released. Final release stops downloads while
  physical/billing liabilities and original receipts survive fenced restore.

### Changed

- Signed `verify-upload` and offline snapshots share one bounded local-file
  verifier, keeping the same open source handle and complete EOF/root checks.
- Native and canister upload clients use the same public permission/declaration
  preflight validators and bounded reply decoders; no second metadata/tree contract.
- Tenant download and verifier observation share the maintained streaming HTTP
  verifier and private artifact helper. The superseded observation-only module
  is removed; exact verifier statements and receipt recovery retain their behavior.

## [0.4.13] - 2026-10-01

### Added

- Operator-only `blob_funding_preparation_assessment` shares the existing funding
  policy across both adapters. It reports exact local capacity, retained identity,
  unresolved allocation and restore fences alongside missing provider, recovery,
  activity and spendability evidence; it reserves nothing and calls no provider.
- Signed native `funding-assessment` preserves full-width proposed amounts and
  optional target, validates exact bounded replies and grants no payment or retry
  authority. Both-adapter journeys preserve occupied uploads and complete stable
  bytes during refusals and restored inspection while the Cashier is stopped.
- Native `sync-gateways`, `cancel-gateway-sync` and `revoke-gateway` delegate to
  existing shared service handlers. Each saves exact canonical request, signed
  intent and outcome in a fresh private directory before one update; interrupted
  or existing claims refuse without resubmission.
- Shared signed gateway journeys through standalone and Canic adapters distinguish
  acknowledgments from lost/pending replies, require exact cancellation, retain
  invalid-reply pending work and preserve occupied owners through fenced restore.
  Current status never becomes a historical receipt or retry/deletion authority.

## [0.4.12] - 2026-10-01

### Added

- Managed application restore tests preserve unresolved retain/release outbox
  entries after committed service effects, in both upgrade orders and repeated
  same-release upgrades. Exact intents, tombstones, receipts, reference liveness
  and liabilities survive; restored application mutations and acknowledgment
  recovery refuse through its fence without clearing uncertainty.
- Native `inspect-account` observes an explicitly selected balance or payment
  relationship through the existing shared service update. Exact operator/account
  scope, independent balances, signed arbitrary-width relationship fields and
  reported absence/errors remain observations without credit or retry authority.
- Signed native account journeys exercise both standalone and managed adapters,
  malformed/oversized replies, foreign identity/scope/trust refusals and fenced
  same-release restore over the existing query-only Cashier substitute.

### Fixed

- Native Agent transport no longer inherits ic-agent's automatic HTTP 429/503
  retries. The configured no-retry client now owns HTTP handling; a network
  regression checks one service submission under either backpressure response.

## [0.4.11] - 2026-10-01

### Added

- Standalone signed reference submission exercises typed unknown/unconfirmed and
  fenced refusals. Lost/pending replies remain unresolved when receipt inspection
  refuses, with saved intent preserved and no resend or synthetic completion.
- Existing application/outbox probe now runs as a canister tenant against managed
  storage through shared admission/reference/descriptor clients. Callback traps,
  use guards, suspension cleanup and a delayed-publication cancellation race keep
  original intents and tombstones. Receipt recovery succeeds under a service
  mutation fence; restoring the application preserves its own fence and history.
- Native `submit-reference` saves an exact tenant command and signed intent before
  one retain/release update. Existing or interrupted claims refuse; pending/lost
  replies reconcile through saved receipt inspection without automatic resend.
  Recorded transition failures remain distinct from successful reference changes.
- Managed signed tenant journeys cover reference submission/recovery, liveness,
  release at capacity during suspension, historical replay without resurrection
  and fenced restore beside signed verifier completion. Last release preserves
  physical bytes and billing liabilities; source bytes/exposure are local substitutes.
- Five managed endpoints delegate certificate assessment, gateway sync,
  cancellation/revocation and account inspection to shared service workflows.
  All twenty-nine implemented service method types and modes match standalone.
- Managed PocketIC journeys check real certificate blockers, uploader/controller
  boundaries, exact failed-sync cancellation, passive account observations,
  unchanged unrelated owners and retained pending work after fenced restore.
  An async gateway update accepts valid Candid at 4 KiB and rejects one byte more.
  Provider replies remain explicitly labelled local substitutes.
- Common signed native CLI exercises managed status, funding/upload history,
  certificate assessment and local byte verification. Wrong identity/scope/trust,
  changed permission and corrupt/short/long files refuse; historical reads remain
  passive after all-owner fenced restore, without provider-completion authority.
- Managed signed verifier journeys observe an authenticated plan, fetch labelled
  local bytes and submit one retained attestation. Acknowledged, lost and pending
  replies recover exact receipts without resend; foreign verifiers, damaged
  observations and fenced new fetches refuse. Raw local request/response and
  signed dispatch evidence can be retained in fresh report directories.

### Changed

- Focused Canic tests build the native CLI, existing query-only Cashier and
  bounded application/outbox probe alongside the managed artifact. Shared
  subprocess diagnostics name the failed command. No provider endpoint or paid
  effect is added to the tests.
- Native fixture and managed verifier cases share one HTTP fault proxy and byte
  source; the superseded fixture-local proxy module is removed.
- Managed certificate issuance remains unwired: pinned Canic's default Fleet
  guard requires a Result reply, conflicting with Caffeine's plain-record contract.
  Supported framework rejection and decoder controls remain acceptance gates.

## [0.4.10] - 2026-09-30

### Added

- Five managed Canic endpoints delegate verification plans/manifests, verifier
  attestation/receipt inspection and reference-qualified download descriptors to
  shared service workflows. All twenty-four managed method types and modes match
  standalone Candid.
- Managed PocketIC journeys cover verifier-only completion, exact attestation
  replay, downloads through live tenant references, release during suspension,
  retained physical bytes/billing liabilities and fenced same-release restore.
  Exposure state and file bytes are explicit local substitutes; no provider
  certificate or live Caffeine effect is exercised.

### Changed

- README now presents a concise project overview, prototype status, setup and
  development commands with emoji and tables. Detailed CLI and PocketIC examples
  moved into linked operator/verifier and local-tool guides.

## [0.4.9] - 2026-09-30

### Added

- Shared operator-only installation readback binds the actual service and caller,
  preserves exact configuration/project/verifier/release and reports restoration
  fencing. Standalone delegates to this handler with unchanged Candid.
- Controlled Canic artifact exposes nineteen managed blob endpoints for
  configuration, tenant enrollment/suspension, upload admission/revocation,
  status/history/discovery, manifests, references/capacity and passive operator
  status/funding history/outcome. All delegate to shared service workflows;
  native Candid comparison checks exact method types and modes against standalone.
- Managed PocketIC journey preserves an admitted, prepared upload and capacity
  through same-release upgrade while refusing mutations. It checks operator,
  tenant, uploader, outsider, Root and controller authority and valid Candid at
  the 128 KiB update boundary, including oversized inter-canister refusal.
- Managed suspension/cleanup journey cancels an unexposed reservation, preserves
  cancellation history and accounting through fenced restore, and checks private
  reference/history boundaries and passive operator inspection.

### Changed

- Managed fixture enrollment now uses the maintained blob tenant endpoints;
  adapter-owned state access and one passive generic failure wrapper preserve
  standalone reply types without duplicating authority or workflow logic.

## [0.4.8] - 2026-09-30

### Added

- Unpublished Canic composition library supplies explicit memory declarations, caller
  guards and synchronous installation/restoration over the shared service owner.
  Linking registers no endpoints, lifecycle or memory. The owning artifact uses
  Canic directly; the core and composition library remain Canic-free.
- Bounded managed application-argument extraction requires the authenticated
  two-argument carrier and explicit typed installation policy, project and verifier.
  Participants bound their own platform copy before decoding, bind the service to
  its actual principal and retain Canic's validated release identity. No defaults
  or replacement configuration are accepted. Failed installations preserve an
  enrolled tenant, all stable owners and neighboring memory.
- PocketIC composition fixture using published Canic 0.110.48 covers Prepared ingress
  and inter-canister refusal, activation, Fleet admission, tenant denial for
  outsider/Root/controller callers, rejected-upgrade rollback and retained
  installation/neighbor memory with all four owners fenced after upgrade.

### Changed

- Local validation builds the managed fixture through Canic's supported CLI.
  Its isolated resolver-2 workspace follows the host validator while the main
  workspace retains resolver 3 and the existing allocator. Its ignored lockfile
  is seeded from the repository lock and projected offline before build/lint.

### Fixed

- Managed Candid export now follows every application endpoint declaration. A
  native official-parser check validates endpoint names, modes and argument shapes,
  covering an omission that actual endpoint calls alone did not detect.

## [0.4.7] - 2026-09-30

### Added

- Shared named service memory requests and grant assembly let embedding hosts use
  their existing ic-memory runtime without duplicating the sixteen-store mapping.
  Default-runtime access requires prior host bootstrap. Composition tests preserve
  populated owners and neighboring application memory under different placements;
  missing grants refuse without initialization or repair.
- Shared immutable installation owner validates service/resource configuration,
  provider project, verifier and compiled release before memory allocation. It
  preflights all seventeen grants, retains one bounded configuration record and
  restores every owner synchronously into inspection-only fences. Tests preserve
  populated accounting and reject missing or invalid retained state without repair.
- Focused PocketIC coverage verifies installation rollback, caller/verifier
  authority, corrupt-state refusal, project byte bounds and repeated fenced
  upgrades against the refactored standalone canister.

### Changed

- Standalone delegates configuration persistence and service construction to the
  shared installation owner, removing its private record/conversion implementation.
  It uses the shared grant mapping while retaining allocation policy, authenticated
  endpoints and synchronous lifecycle. Public DTOs, Candid, stable keys, schemas,
  limits and restore fences remain unchanged.

## [0.4.6] - 2026-09-30

### Added

- Standalone lifecycle regression coverage preserves full installation and retained
  metadata through stop/start and repeated same-release upgrades. All four owners
  remain fenced, and operational refusals preserve stable memory.
- Native `blob-storage reference-receipt` and `reference-status` sign separate
  tenant queries using saved original boundary requests. Historical success,
  recorded failure and absence remain separate from current liveness and restore
  fences, with exact scope validation and no mutation, provider call or retry
  authority. Shared request encoders reuse the maintained reference rules.
- Signed reference inspection tests preserve receipts through release, settlement
  and fenced upgrade while current liveness changes. Production standalone
  unknown/unconfirmed content remains a typed refusal.
- Native `blob-storage funding-outcome` signs one query for the exact original
  funding operation, offer and optional target. JSON preserves absence, unknown
  attachment, exact refund, structured provider response, conservative
  reconciliation and found-record restore fences without claiming provider
  credit or authorizing another payment. A shared validated request encoder and
  bounded reply decoder retain typed refusals and full-width amounts.
- Signed PocketIC coverage joins funding history to exact outcome inspection and
  checks passive uncertain/refund records through fenced upgrade. Standalone
  absence remains distinct from unknown fence information and scope/trust refusals.

### Changed

- Consolidated acceptance and Canic parity summaries around current implementation
  and source-bound evidence. Durable standalone configuration and signed native
  transport are recorded as implemented; provider, operational recovery, managed
  adapter, operator mutation, consumer and retirement qualification remain open.
  Historical capability records remain explicitly dated evidence.

## [0.4.5] - 2026-09-30

### Added

- Native `blob-storage upload-history` signs one operator query over retained
  upload identities and local lifecycle states. Explicit filters, full-width JSON,
  saved scoped cursors, scanned-row counts and restore fences preserve empty
  filtered progress without automatic pagination, provider calls or retry authority.
- Shared bounded upload-history request/reply codec checks complete identity,
  scope, ordering, root/object uniqueness, selected states and forward continuation.
  Refusals stay distinct from empty pages. Signed PocketIC coverage exercises
  cancelled history, real scan/page bounds and saved continuation after upgrade
  without changing stable state.

## [0.4.4] - 2026-09-30

### Added

- Native `blob-storage certificate-assessment` signs one uploader-only query and
  compares the returned permission with saved original intent. JSON retains every
  missing prerequisite and full-width identity; unprepared, revoked and fenced
  refusals remain distinct. No certificate issuance, provider call, reservation
  or retry authority is introduced, including when the blocker list is empty.
- Shared bounded certificate-assessment request/reply codec validates the complete
  permission and rejects malformed, oversized, foreign or duplicate-blocker replies.
  Signed standalone PocketIC coverage exercises exact intent, root trust,
  revocation and upgrade fencing while preserving stable state.

## [0.4.3] - 2026-09-30

### Added

- Standalone `_immutableObjectStorageCreateCertificate` update and uploader-only
  `blob_upload_certificate_assessment` query delegate to shared permission/exposure
  checks. Assessment returns the original permission and missing prerequisites
  without changing state or reserving issuance. Certificate updates refuse while
  provider pre-charge limits, namespace binding, replay charging and recovery
  readiness remain unqualified. PocketIC covers authentication, malformed input,
  unchanged reservations, stop/start, revocation and upgrade fencing.
- Actual PocketIC management snapshot tests demonstrate that rollback can bypass
  upgrade fencing, revive revoked permissions and lose later upload records and
  reservations. Standalone certificate issuance still refuses after rollback;
  labelled fixture facts demonstrate how falsely asserted recovery readiness could
  permit repeated simulated exposure. Retained evidence documents the unsupported
  snapshot activation path; these tests do not implement operational recovery.
- Retained official Caffeine source capture and recovery review records. Reviewed
  backend hashes and npm 1.1.2 integrity remain unchanged despite a newer upstream
  revision. Source/local evidence supplies no new deployed-provider guarantees.

### Changed

- Shared passive certificate-assessment DTOs and exposure-blocker conversion replace
  the fixture's duplicate wire enum and mapping. Issuance rechecks current authority;
  a successful assessment cannot override later permission changes.
- AGENTS.md, service contract, roadmap and handoff retain one authoritative storage
  owner with local durable journals. External journal/controller machinery and extra
  metadata calls are deferred until a concrete use case justifies them. Existing
  certificate and recovery prerequisites remain enforced.

## [0.4.2] - 2026-09-30

### Added

- Native `blob-storage submit-attestation` validates a complete successful provider
  observation and submits its exact statement once. A durable exclusive claim,
  statement copy, signed request, request ID and artifact hashes precede the update;
  incomplete, failed or mismatched observations reject. Existing claims refuse
  resubmission, including after interruption or a lost reply. Bounded acknowledged
  replies use the shared exact-statement decoder; pending and uncertain results
  remain for explicit historical receipt recovery. No provider fetch, automatic
  retry, polling or regenerated observation time occurs.
- Signed PocketIC coverage checks intent before transport, competing native
  submitters, damaged artifacts and receipt recovery after a local proxy drops an
  actual replica acknowledgment. Provider exposure/content remain labelled fixtures;
  this does not qualify standalone exposure or deployed Caffeine behavior.

## [0.4.1] - 2026-09-29

### Added

- Verifier-only `blob_verification_plan` binds the installed Caffeine owner/project
  and original declaration to an exposed unfinished upload. Standalone and the
  durable fixture share the handler; wrong roles, bindings, phases and restored
  owners reject without changing state.
- Native `blob-storage observe-upload` makes one explicitly bounded provider GET,
  checks the complete stream against original root/metadata/length, and durably saves
  the exact attestation statement. Fresh run artifacts retain request intent,
  responses, failures and hashes; existing or interrupted runs cannot be overwritten
  or automatically resumed. Partial, corrupt, oversized, encoded and redirected
  responses reject. No service update or on-canister file hashing occurs. Local HTTP
  and signed PocketIC evidence includes process interruption and explicit fixture
  attestation/recovery; deployed provider qualification and native dispatch remain open.
- Authenticated `blob-storage upload-attestation` compares a saved exact statement
  with immutable service history, reporting matched, conflicting or absent evidence.
  It preserves full-width identities and restore fences without resending the
  statement, fetching provider content or authorizing retries. Signed PocketIC
  coverage recovers a discarded acknowledgment through reference release,
  settlement and restoration while preserving service state and the saved intent.
- Shared bounded attestation reply handling checks the expected service, namespace,
  permission, verifier and receipt chronology. Mutation acknowledgments must match
  the complete saved statement; malformed replies and service refusals never become
  absent receipts. Historical receipt lookup preserves conflicting statements.

## [0.4.0] - 2026-09-29

### Added

- Explicit verifier trust for upload completion: shared handlers authenticate the
  installed verifier and exact exposed upload, retain its first attestation with
  accounting and the first reference, and support immutable receipt inspection.
  Exact replay never reactivates released references; conflicting statements and
  restored mutations reject. Verifier-only manifest inspection supplies the
  original declaration. Attestations establish trusted observations of content
  availability, not future retention, payment credit or billing cessation.
- Authenticated `blob-storage verify-upload` checks a bounded local file against
  the service's exact retained manifest using the existing native streaming
  verifier. Full-width identities and original metadata remain bound; corruption,
  truncation, changed permissions and unprepared uploads reject. It sends no
  gateway request or attestation and does no on-canister file hashing.

### Changed

- **Breaking:** standalone installation now requires
  `completion_verifier`, validated before allocation and on restore. The current
  v1 host and confirmed-lifecycle schemas retain explicit verifier authority and
  evidence without a compatibility reader. Cross-release transitions remain
  reinstall-only; same-release receipt recovery remains available through fences.
- The standalone test target builds its consumer fixture explicitly.

## [0.3.0] - 2026-09-29

### Added

- Offline Caffeine SDK fault probes retain synthetic request/reply evidence for
  single/multiple chunks, ignored resume hints, non-complete status, lost final
  response, HTTP failure and traffic exhaustion. Native Rust verification checks
  original bytes and rejects corruption/truncation; uncertainty blocks replay.
  `make test-sdk-probe` is opt-in and makes no network or paid provider calls.
  Retained local artifacts are hash-checked by `make probe-check`; local evidence
  does not establish deployed completion, retention or billing behavior.
- Persistent Caffeine probe ledger and independent qualification policy. The
  `caffeine-probe` tool records intent before bounded anonymous source requests,
  retains raw responses and hashes in separate immutable runs, and verifies
  artifacts without treating failed/incomplete capture as provider qualification.
  Local HTTP evidence covers response limits, errors, redirects and interrupted
  records. Provider cooperation is no longer a prerequisite; supported behavior
  still requires evidence and explicit limits, with paid trials separately scoped.
- Authenticated `blob-storage funding-history` reads one bounded page through the
  shared service API and funding decoder. Exact scope, descending operation order,
  full-width amounts, refunds, uncertainty and restore fences survive JSON output;
  saved cursors bind the complete original scope before any network request.
  Native boundary tests and signed HTTP subprocess coverage exercise refusals,
  pagination and restoration without payment, automatic traversal or credit claims.
- Native `blob-storage status` authenticates to the shared service endpoint with
  an explicit PEM identity, operator and full installed scope. IC mode pins the
  SDK's IC root; local mode requires a separately trusted root and loopback URL.
  Verified query signatures, bounded transport/decoding, exact scope checks,
  decimal-string counters and separate restore fences preserve observation-only
  semantics. Signed HTTP subprocess evidence covers rejected trust/identity/scope,
  operator refusal and passive restored inspection. This unpublished CLI adds no
  mutations or provider calls; native dependencies stay out of Wasm builds.
- Standalone operator-only `blob_inspect_account` uses a shared handler for one
  bounded replicated Cashier balance or payer-relationship query. Exact installed
  scope and all restore fences are checked before dispatch and after the await;
  replies reuse the maintained provider encoders/decoders. Independent balances,
  signed relationship figures, missing relationships and provider errors remain
  distinct. No attached cycles, retry, funding, account change or cached authority.
  Native and PocketIC evidence covers linked payers, full-width values, caller and
  account mismatches, malformed/oversized replies, restore fencing and unchanged
  local accounting. Deployed Cashier behavior remains unqualified.

### Changed

- **Breaking:** the private browser gateway contract now requires
  `maxTotalRequestBytes` in configuration and retained scope. Both the hook and
  atomic store claims enforce aggregate outbound body bytes; uncertain requests
  stay charged to that budget. Consumers and the IndexedDB fixture use the current
  contract directly, with no old-scope fallback. Chromium/PocketIC regression
  coverage preserves cancellation, reload and competing-tab fences.
- Local reference recovery now uses the shared `blob_reference_receipt` API and
  bounded library decoder against standalone and durable storage hosts. The v1
  fixture journal retains independent full-width upload, object, lifetime and
  first-reference identities; changed original arguments conflict within the same
  operation slot. Service refusals remain distinct from missing receipts and
  recorded failures. Removed the transient fixture's private receipt endpoint,
  DTO, conversion and superseded tests; subprocess coverage follows saved intents
  through release, settlement and fenced restoration. Transport remains local
  PocketIC; no production authentication or mutation dispatch is introduced.

## [0.2.24] - 2026-09-29

### Changed

- **Breaking — requires a minor release:** standalone installation now takes
  `HostInstallationInput { configuration, project }`. The explicit Caffeine project
  is validated before allocation, retained in the current v1 configuration record,
  revalidated during restore and included in operator configuration readback. No
  project is inferred from a tenant, payer or local namespace. The previous host
  init/schema is replaced directly; cross-release transitions remain reinstall-only.

### Added

- Shared tenant-only `blob_reference_status` query in standalone and the durable
  storage fixture, replacing the fixture's private boolean endpoint. Exact upload
  and reference bindings return current local liveness and the restore fence,
  independently of historical retain/release receipts. The replicated reference
  client now validates bounded status replies. Native and PocketIC tests cover
  independent full-width IDs, changed bindings, suspension, release/settlement,
  passive reads and inspection after both service and client restoration.
- Standalone `blob_download_descriptor` update delegates to the existing shared
  exact-live-reference workflow using the installed project mapping. It returns
  original metadata only, with no provider/body call, read-session allocation or
  serving access for inactive tenants, unconfirmed content or restored owners.
  PocketIC coverage includes bounded inputs, project validation/restore corruption,
  an actual canister client and the existing confirmed-reference fixture journey.
  The standalone test target now builds its local consumer fixture too.
- Shared tenant-scoped `blob_lookup_content` query in the standalone host and both
  admission fixtures, replacing the private discovery endpoint. Indexed discovery
  reuses the maintained history conversion, preserving independent full-width
  upload, object, incarnation and first-reference identities through every local
  lifecycle phase. Responses echo the request and report restore fencing even for
  absence. The inventory tool validates those bindings and reports later fences;
  native and PocketIC tests cover isolation, passive reads, suspension, restoration,
  cleanup/settlement and actual inventory subprocesses.

## [0.2.23] - 2026-09-29

### Added

- Shared tenant/root-scoped `blob_reference_capacity` query in the standalone host
  and both admission fixtures. Correlated responses preserve unknown/foreign/
  unconfirmed absence, lifetime reference slots, reserved cleanup receipts and
  restore fencing. Replaced the private fixture DTO and conversions; inventory
  inspection validates exact replies and blocks on a later observed restore fence.
  Native and PocketIC tests cover caller isolation, suspension, cleanup at capacity,
  settlement, restoration, bounded ingress and inventory subprocesses.
- Shared tenant-only `blob_upload_capacity` query in the standalone host and both
  admission fixtures, reusing maintained global/tenant headroom calculations.
  Responses bind the tenant scope and report enrollment, object/metadata limits,
  lifetime history, concurrent slots, byte headroom and the restore fence. Native
  and PocketIC coverage includes full-width values, shared contention, cleanup,
  continuing billing, bounded ingress and passive suspended/restored inspection.
  Replaced the private capacity DTOs and conversions; the inventory tool consumes
  the shared response and reports restored services as blocked despite spare quota.
- Standalone `blob_upload_status` query using the shared exact-upload handler and
  maintained DTOs. Tenant-only history stays available through suspension and
  fenced restoration without granting current reference liveness or upload retry
  authority. PocketIC tests cover independent full-width identities, passive query
  and replicated execution, cancellation, caller isolation, changed original
  arguments and bounded ingress. The generated deployment Candid includes the query.

### Fixed

- Removed unnecessary `Result` wrappers from inventory test reply helpers so strict
  Clippy also passes when library unit-test targets are included.

## [0.2.22] - 2026-09-29

### Added

- Shared operator-only `blob_revoke_gateway` update in the standalone host and
  storage fixture, bound to the installed service, namespace, Cashier and payer.
  Removal invalidates older sync and read observations even for an absent member;
  it preserves occupied read slots, upload obligations and funding accounting.
  Repeated calls are fresh revocation decisions, not automatic receipt replay.
- Native and PocketIC evidence for caller/scope rejection, absent/final-member
  removal, stable-write rollback, delayed sync replies, remove/re-add during reads
  and restored mutation fences. The fixture's private removal command is replaced
  by the shared endpoint, with a separate local fault hook using the same handler.
  No provider credentials, deletion or billing state changes.
- Shared `blob_sync_gateways` and `blob_cancel_gateway_sync` operator updates in
  both hosts. Refresh persists its pending identity before one bounded replicated
  Cashier query; failures retain pending work, and cancellation targets the exact
  observed sequence. Standalone limits are 30 seconds and 64 KiB per reply, with
  independent decoder and membership bounds. No attached cycles or automatic retry.
  The fixture's private cancellation command is removed. Native and PocketIC tests
  cover malformed/oversized/rejected replies, overlap, stale cancellation, delayed
  callbacks, write rollback and fenced restoration. `make test-standalone` now builds
  the local gateway source fixture too; deployed-provider qualification remains open.

### Fixed

- Updated operator gateway-query tests for passive malformed, oversized and empty
  fixture replies. Client validation rejects each without changing pending sync
  state or source journals; caller, scripted-effect and restore refusals remain covered.

## [0.2.21] - 2026-09-29

### Added

- Shared operator-only `blob_funding_history` query in the standalone host and
  storage fixture. Bounded descending pages recover exact local operation IDs,
  attachment amounts, optional target balances and transport phases, with full
  service/namespace/Cashier/payer binding and an explicit restore fence. The host
  returns at most 32 entries; cursors grant no retry, credit or freshness authority.
- Native and PocketIC coverage for full-width attachments/refunds, caller/scope
  rejection, pagination, interrupted writes and retained uncertainty across restore.
  The fixture's private history endpoint, DTOs and conversion are removed; its
  related summary query now uses the shared operator scope. No funding dispatch
  endpoint or deployed-provider guarantee is introduced.
- Shared operator-only `blob_funding_outcome` query in both hosts, replacing the
  fixture's private outcome endpoint and conversion. Exact lookup checks the full
  original scope, operation, offer and optional target balance. Responses preserve
  transport phase, reported balance components, provider errors, decoder categories
  and restore fencing. Shared reconciliation keeps accepted or uncertain attachments
  unresolved independently of reported success. Native and PocketIC tests cover
  altered intents, absent/transport-only observations, delayed callbacks, write
  rollback and retained outcomes after restoration against local substitutes.
- `ReplicatedFundingClient` for single-call history and exact-outcome inspection
  from a pinned canister operator. Bounded reply validation checks scope, original
  amounts, descending cursors and refund/reconciliation consistency while retaining
  absence, service refusals and restore fences. No attached cycles, automatic
  retries, extra journal or lifecycle hooks. Native and PocketIC tests cover
  malformed observations, caller isolation, reply limits and both host restorations.

## [0.2.20] - 2026-09-29

### Added

- Standalone canister host with explicit installation configuration, seventeen
  exclusive ic-memory grants and synchronous assembly of the shared stores.
  A bounded host record retains configuration and package-release/service binding;
  upgrades load it without replacement inputs and leave every owner fenced.
- Tenant, upload admission/revocation, manifest preparation/inspection and reference
  endpoints delegate to existing shared workflows. Operator configuration readback
  grants no controller authority. The maintained Candid exposes typed requests even
  with custom bounded decoding; a schema check ignores explanatory comments.
- Focused standalone build and PocketIC targets. Actual Wasm tests cover the 10 MiB
  manifest path, caller isolation, stop/start, retained state and fenced mutation,
  failed-install rollback, ingress bounds and foreign/missing/release-mismatched
  restore rejection. Provider effects, operational recovery and the Canic adapter
  remain unfinished; this host does not yet provide a complete storage journey.
- Shared operator-only `blob_local_status` query, exported by the standalone host
  and storage fixture. Explicit service/namespace/Cashier/payer binding and matching
  owner configurations precede a synchronous snapshot of maintained upload totals,
  funding allocation/outcomes, bounded gateway membership and read occupancy.
  Separate restore fences remain visible; inspection makes no provider calls or
  claims about provider credit, platform liquidity or readiness. Native and PocketIC
  checks cover caller/scope rejection, full-width accounting, cancellation history,
  refunds and uncertain funding, pending sync and interrupted reads across restore.
- Shared `blob_upload_history` query for tenant-scoped or operator-wide bounded
  discovery of retained operations. Responses preserve independent full-width
  upload/object/incarnation/first-reference identities, current cleanup state and
  the restore fence. Scope-bound cursors advance through empty filtered pages;
  callers cannot choose work limits. The standalone host scans at most 64 rows
  and returns at most 32 entries per call. The fixture's private scan DTOs and
  endpoint are replaced by the shared handler. PocketIC tests cover isolation,
  cancellation, suspension, cursor rejection, fresh sweeps and continuing billing
  after physical deletion; history remains passive through restoration.

## [0.2.19] - 2026-09-29

### Added

- Shared scoped tenant enrollment API and handlers for operator compare-and-set
  updates and tenant/operator inspection. Requests bind service, namespace and
  tenant; responses retain that scope and report the restore fence. Suspension
  preserves lifetime capacity and obligations, while reactivation advances the
  existing permission generation. Lost replies require inspection before choosing
  another command; inspection does not prove historical execution.
- The storage fixture now exports `blob_update_tenant` and `blob_tenant` through
  the shared handlers, replacing its private enrollment endpoints and conversion.
  PocketIC coverage checks caller isolation, malformed/stale preconditions,
  capacity during suspension, enrollment write rollback and fenced restoration.
  Production adapter/lifecycle ownership and provider qualification remain open.
- Replicated tenant client with explicit executing canister and pinned tenant scope,
  bounded waits/replies and no automatic retry or query-to-update fallback. Update
  acknowledgments must match the existing model's exact generation/state transition;
  inspection preserves absence and restore fences without claiming historical execution.
- Local operator-client evidence uses the existing consumer fixture's bounded store
  to retain one command before dispatch. Success, unusable replies and callback
  interruption cannot reopen dispatch; restoration preserves the command for
  inspection. Stale activation state and generation are rejected without changing
  service storage, and rejected commands remain retained across client restoration.
  This fixture is not a production operator journal or identity provider.

## [0.2.18] - 2026-09-29

### Added

- Shared host configuration input and bounded Candid decoder. Explicit service,
  operator, payer, namespace, resource, billing, funding allocation and read-session
  inputs reuse existing model validation before state allocation. The host's actual
  service identity must match. Invalid limits are rejected without defaults or
  clamping, and balances and byte budgets preserve their full numeric width.
- Shared synchronous assembly of upload, funding, gateway and read-session stores
  with explicit host memory grants. All store envelopes are validated before
  allocation; any occupied grant refuses fresh installation, and restoration never
  initializes a missing store. The fixture's separate initialization paths are
  removed. Changed funding/read limits reject without modifying retained bytes;
  restored stores remain inspection-only.
- Targeted coverage for malformed/oversized configuration, role bindings, resource
  relationships, funding/read budgets and occupied/missing memory grants. A combined
  PocketIC journey preserves upload obligations, uncertain funding, pending gateway
  sync and interrupted read occupancy through same-release restoration, with
  mutations refused afterward. Production adapters and deployed Caffeine
  qualification remain open.

## [0.2.17] - 2026-09-28

### Added

- Browser gateway transport guard around Caffeine's existing fetch hook. Caller-owned
  storage commits bounded request fingerprints before dispatch, atomically with
  cancellation checks. A retained execution fence prevents competing tabs or reloads
  from restarting a claimed transfer; failed or uncertain requests cannot be retried.
  HTTP responses record history without confirming provider completion or billing.
- Chromium/PocketIC coverage for gateway journal write aborts, lost responses,
  failed observation writes, oversized replies, request capacity and cancellation
  before claim/after dispatch. Reload and certificate recovery preserve gateway
  history. Tests use a local gateway substitute and fixture IndexedDB store;
  production persistence, reconciliation and deployed qualification remain open.
- Browser admission now exercises signed ingress through the existing consumer
  canister fixture, which persists asset intent and admits as the actual tenant.
  The browser then prepares the manifest directly as the uploader. Unrelated
  identities and direct uploader admission are refused. Candid encoding and reply
  validation reuse the Rust types; no JavaScript service schema is duplicated.
  This is local integration evidence, not a production Toko endpoint or identity flow.
- Browser reference journeys now attempt consumer registration after gateway outcomes
  and explicitly persist consumer cancellation before tenant withdrawal. HTTP success
  cannot publish unconfirmed content; withdrawal preserves exposed reservations and
  browser request history. Shared authorization probes run once across the fault
  scenarios instead of repeating the same checks for every transport interruption.

## [0.2.16] - 2026-09-28

### Added

- Reusable browser certificate client with caller-owned authentication, explicit IC
  trust and an atomic durable-intent store contract. Issuance saves the exact signed
  request before dispatch; recovery verifies its historical reply without reissuing,
  and cancellation remains permanent. Saved permission, network trust, uploader,
  service, method and root are checked before accepting observations.
- Browser upload composition now uses the actual Caffeine 1.1.2 package and its
  supported IC SDK 5.4.0 through the existing agent hook. A pinned, hash-checked
  patch exposes network-free preparation and per-client transport/cancellation/
  retry controls; upstream still owns hashing, chunks, certificate extraction and
  HTTP formats. Preparation snapshots caller bytes and uses immutable, single-use
  handles. The private fixture applies the patch to a generated package copy and
  verifies the pinned source hashes before building.
- Bounded conversion of Caffeine's prepared manifest into the service's existing
  declaration, reusing metadata and root validation without another tree builder.
  Browser evidence now supplies the actual prepared JSON to tenant admission and
  uploader preparation in PocketIC before receiving the permission; tenant-only
  preparation is refused. File bytes remain in the browser.
- Chromium/PocketIC coverage now imports the reusable client and Caffeine package.
  Tests cover competing tabs, lost replies, cancellation, changed saved bindings,
  invalid proofs, single-use handles, failed gateway requests without retry and
  abort before issuance/after the tree. Decoder tests reject oversized, malformed
  and inconsistent manifests. Gateway responses remain local substitutes;
  production consumer authentication/storage, durable gateway-effect coordination
  and deployed Caffeine qualification remain open. The browser package is private.

## [0.2.15] - 2026-09-28

### Added

- Headless certificate evidence through PocketIC's real HTTP v4 ingress API, using
  an actual signing identity and the official IC agent. Tests extract the raw
  certificate, verify its signature/delegation/time and bind the saved request ID,
  uploader, service and expected root. Forged, unrelated, malformed, oversized and
  rejected-response proofs cannot become upload certificates.
- Saved ingress-request recovery obtains the original certified reply without
  reissuing or changing service state. Historical proof survives local revocation
  but does not renew permission or authorize a gateway retry. New cryptographic
  dependencies belong only to the unpublished native test harness; production
  browser integration and deployed Caffeine acceptance remain open.
- Opt-in Chromium certificate/IndexedDB evidence using the pinned current IC JS
  SDK. A bounded two-slot fixture commits the original permission and signed
  envelope before dispatch, serializes competing tabs, and preserves cancellation
  through a held response, tab closure and reload recovery. Aborted writes send
  nothing; stale, forged or oversized proofs cannot change retained intent.
  `make test-browser` runs the local test without downloading dependencies. This
  is browser integration evidence, not a production journal or provider upload.

## [0.2.14] - 2026-09-28

### Added

- Shared guarded upload-exposure workflow with exact permission binding and
  independent host checks for pre-charge limits, provider namespace, replay charging,
  recovery eligibility and durable commit. Missing or stale observations block
  writes; commit rechecks actual uploader, preparation, activation, deadline, phase
  and restore fencing. Preview results cannot authorize a later mutation.
- Exact tenant/uploader exposure-history recovery and local IC evidence for
  permission-write rollback, traps before update completion, and lost committed
  acknowledgments. Repeat exposure is rejected, while revoked uncertain uploads
  remain charged and inspectable through restore. The storage fixture now uses
  the shared gate with explicitly labelled provider-evidence substitutes. Actual
  evidence acquisition and deployed issuance remain open; no provider effect,
  new stable schema or additional memory grant is introduced.
- Shared Caffeine certificate-response handler resolves a root to its original
  permission, authenticates the actual uploader and commits guarded exposure
  before returning the reviewed plain `upload`/`blob_hash` reply. Linking exports
  no endpoint. Local IC tests cover default refusal, malformed/foreign requests,
  evidence mismatch, write/reply rollback, lost acknowledgment and fenced restore.
  Provider acceptance, certificate replay and pre-charge enforcement remain
  unqualified; the test adapter uses explicitly configured substitute evidence.

## [0.2.13] - 2026-09-28

### Added

- Replicated upload-manifest client with explicit actor, tenant and service bindings.
  Preparation validates the bounded declaration before dispatch; inspection recovers
  the original declaration. Calls send once, attach no cycles and never retry
  automatically. Only the actual admitted uploader may prepare; the tenant may
  also inspect. Query execution and changed bindings are refused.
- Durable uploader intent in the local application fixture, exercised by a separate
  uploader canister. Exact intent and dispatch state survive unusable replies and
  callback traps; uncertainty blocks redispatch until inspection finds the accepted
  declaration. Typed refusals remain recorded, and unprepared observations cannot
  clear uncertainty. Bounded history, conflicting intents, stale observations and
  fenced upgrades are covered. A three-canister journey joins tenant admission,
  uploader recovery, first-reference registration and cleanup. Exposure/completion
  remain labelled substitutes; browser integration and provider qualification are open.
- Uploader cancellation now retains a permanent local tombstone, original intent
  and uncertain/accepted/refused preparation history. It blocks redispatch without
  claiming tenant revocation or freeing service capacity. Recovery and delayed
  acknowledgments preserve cancellation; saved intent cannot reopen it. Local IC
  races cover tenant withdrawal during a held uploader reply, unexposed reservation
  cleanup, exposed late completion and reference release under suspension, plus
  cancelled history through fenced upgrade. This extends the same application fixture.

## [0.2.12] - 2026-09-28

### Added

- Shared tenant upload revocation through `blob_revoke_upload` and the replicated
  permission client. The full original permission, including uploader and expiry,
  is checked before mutation. Unexposed cancellation releases reservation bytes;
  possible exposure and confirmed references preserve their storage and billing
  obligations. Replies distinguish first withdrawal from exact replay and require
  positive revocation evidence. Suspension permits cleanup; restore fences it.
- Consumer cancellation now retains a revocation intent before actual IC dispatch.
  Lost or unusable acknowledgments remain pending until exact inspection; typed
  refusals remain recorded. Local IC evidence covers callback and stable-write
  rollback, late completion, caller/permission isolation and uncertain withdrawal
  through upgrade. The storage probe's private revoke endpoint is replaced by the
  shared contract. This withdraws local issuance permission, not a provider object;
  provider qualification remains open.
- Shared uploader manifest preparation and exact declaration inspection through
  `blob_prepare_upload` and `blob_upload_manifest`. Full original permissions and
  raw declaration bounds precede conversion; root/length validation uses the
  existing Caffeine algorithm. Equivalent reordered retries preserve the first
  metadata and leaves. Tenant/uploader recovery remains available after expiry,
  revocation, exposure and fenced restore. Bounded reply decoders reject changed
  permissions and malformed declarations. The storage fixture now uses this
  contract, including write-trap recovery. This establishes declaration consistency,
  not stored bytes, provider completion or a production uploader client.

## [0.2.11] - 2026-09-28

### Added

- Authenticated upload-status inspection through shared DTOs, a durable handler
  and a bounded replicated IC client. Replies bind the complete original upload
  and distinguish reservation, possible exposure, confirmation and cancellation.
  Historical confirmation survives release, settlement and restore; it does not
  establish current reference liveness or authorize retrying an uncertain effect.
- Fresh-upload registration in the local consumer fixture uses the first
  reference created by completion, with no extra retain or receipt. Intent can
  be persisted before admission. Cancellation preserves uncertain
  uploads for reconciliation and exact cleanup, including late completion under
  suspension. Native and PocketIC evidence covers stale observations, conflicting
  identities, interrupted callbacks, unchanged storage during registration and
  fenced upgrades. Provider completion remains labelled test-host work;
  production application integration and operational recovery are still open.
- Canonical tenant upload admission and exact permission inspection now share
  library DTOs, durable handlers and a bounded replicated client. Recovery binds
  uploader and original expiry as well as every upload field; replay cannot renew
  permission or reserve additional capacity. The consumer fixture persists that
  permission before actual IC dispatch, retains typed refusals and reconciles
  uncertain acknowledgments by inspection without resending. The storage probe's
  private admission endpoint is replaced. Native and PocketIC tests cover changed
  permissions, expiry, suspension, reply limits, write/callback rollback and
  upgrade fencing. Manifest preparation, revocation transport and deployed-provider
  qualification remain open; no certificate or paid provider effect is added.

## [0.2.10] - 2026-09-28

### Added

- Shared authenticated reference-receipt inspection with library-owned DTOs and
  an explicit replicated IC client. Exact original upload, object, lifetime,
  reference and operation arguments bind the returned historical success or
  failure. Absence uses an explicit wire variant so malformed optional data
  cannot silently become a missing receipt. Bounded decoding, actual tenant
  identity and service targeting precede disclosure; no mutation fallback,
  automatic retry or attached cycles are introduced.
- The durable storage probe uses the canonical receipt query in place of its
  private lookup. Native and two-canister PocketIC evidence covers independent
  full-width identities, recorded failures, conflicting arguments, reply limits,
  caller isolation, atomic rollback and receipt retention through release,
  suspension, settlement and fenced upgrade. Historical success and absence
  remain distinct from current liveness, publication and retry authority.
- Canonical retain/release updates now use the same exact command and shared
  durable handler as receipt recovery. The explicit replicated client sends once,
  validates the returned operation and preserves recorded inner failures and
  replay status. Fresh retains require active enrollment; cleanup and exact
  replay remain available under suspension and receipt pressure, while restored
  owners reject mutation. The private mutation endpoint/DTO/conversion is removed.
  Local IC evidence recovers a committed mutation after an unusable reply without
  changing accounting on retry, and preserves rollback, isolation and cleanup.
  Production consumer intent persistence, publication transactions and outboxes
  remain application-owned integration work.
- A separate local consumer canister now exercises existing-content registration
  through the shared retain, descriptor and receipt clients. Its bounded stable
  intent reserves cleanup identities before dispatch; atomic publication,
  dependency checks and tombstones prevent cancelled registrations from reviving
  assets. Interrupted retain/publication/release callbacks retain repairable
  evidence, and upgrade preserves unresolved work under an inspection-only fence.
  Native and PocketIC tests cover these races, exact payload conflicts and cleanup
  at lifetime capacity. This is an application substitute, not a Toko adapter,
  production client journal or fresh-upload journey. Make targets include it.

## [0.2.9] - 2026-09-28

### Added

- Bounded durable read sessions with separate global/per-tenant slot and reply
  buffer budgets. Three host-granted memories retain exact chunk/reference/gateway
  intent, active accounting and a never-reused local sequence. Shared admission
  checks current authority and chunk range before committing; exact callback
  completion releases only its own reservation and rechecks disclosure authority.
  Revocation, elapsed time and dropped tickets do not release occupied capacity.
  Restore validates retained rows/counters and remains inspection-only.
- The local read fixture now uses the shared durable journal in place of its
  temporary busy flag. Native and PocketIC evidence covers independent count/byte
  limits, stale callbacks, exhaustion, authority invalidation, atomic admission
  and callback failures, transport rejection and retained occupancy after upgrade.
  Allocator and existing upload/funding/gateway schemas are unchanged.
- Shared asynchronous chunk reads now combine durable admission, one host call,
  current callback authority and exact manifest-leaf verification. Peer/root/index,
  reply budget, length and hash must match before bytes are returned. Verification
  reads the selected leaf from the bounded immutable manifest without rebuilding
  its tree or copying operation histories. The local IC fixture fetches actual
  chunks, bounds decoding and uses bulk byte decoding; malformed/corrupt replies
  settle occupancy without disclosure, while callback traps retain it through
  upgrade. The private probe reuses the existing locked `serde_bytes` dependency.
  Deployed Caffeine transport, resource sizing and operational recovery remain open.
- Operational download descriptors bind the current service owner, explicit
  provider project/local namespace and exact live tenant reference to the stored
  root, length and original hash metadata. Suspended/restored instances refuse
  delivery while passive inspection remains available. The shared workflow emits
  the reviewed Caffeine direct-blob request target with bounded, escaped fields;
  no default project, origin, provider call or canister body transfer is introduced.
  Source/package review confirms the current client download fields; provisioned
  mappings, approved HTTP policy and production authenticated delivery remain open.
- Library-owned descriptor request/response/error DTOs and one shared endpoint
  handler replace the private fixture descriptor schema. An explicit replicated
  client calls `blob_download_descriptor` once from the actual tenant canister,
  bounds Candid decoding and checks the complete original reference, owner,
  project, declared size and hash metadata. No credentials, server-selected URL,
  cycles attachment, method fallback or automatic retry is introduced. Local
  two-canister evidence covers caller identity, query refusal, limits, suspension
  and restored service refusal. Browser delivery and publication/reference
  coordination remain open.

## [0.2.8] - 2026-09-28

### Added

- Explicit replicated IC transport for the canonical Cashier gateway-list query.
  It checks service/Cashier binding and execution mode, sends once with a bounded
  timeout and no attached cycles, and checks reply size before handing the owned
  buffer to the decoder. Ordinary platform costs still apply. PocketIC exercises
  the query-only endpoint, wrong bindings, non-replicated refusal, rejection,
  oversized replies, callback rollback and restore fencing. Current public
  Caffeine source/package/interface observations remain unchanged; a live
  replicated Cashier call and deployed-provider guarantees remain unqualified.
- Shared scoped gateway root observations compose current durable membership,
  matching owner configuration, stored object bindings and both restore fences.
  Bounded indexed reads return local phases without tenant/operation details,
  preserve pending uncertainty and reject revoked callers even for empty batches.
  Native and PocketIC evidence covers independent fences, corrupt indexes, caller
  isolation, membership replacement and all cleanup phases. These observations
  grant no provider deletion, completion, retry or read-session authority.
- Shared read-authority capture/recheck binds the original tenant, exact live
  reference, object lifetime, root and selected gateway. A durable invalidation
  counter prevents gateway removal/re-addition or unchanged-list syncs from
  reviving old reads; tenant suspension/reactivation also invalidates them.
  Membership and its counter commit together, with exhaustion blocking reads
  while preserving revocation. Native and held-reply PocketIC tests cover these
  checks, write rollback and both restore fences. Bounded session admission,
  one-shot completion and production read transport remain separate work.
  The internal v1 gateway record is replaced directly; cross-release installs
  remain reinstall-only, with no compatibility reader or allocator change.

## [0.2.7] - 2026-09-28

### Added

- Durable gateway membership and pending sync state through one explicit
  host-granted memory. The bounded v1 record reuses existing gateway validation
  and revocation rules: operator edits invalidate older replies, invalid lists
  leave pending state intact, and empty membership remains possible by removal.
  Scope, operator and installed limits are checked on access/restoration; restored
  registries remain inspection-only. Native and PocketIC coverage includes full
  record bounds, stale replies, write rollback and retained pending syncs through
  upgrade. Membership alone grants no provider callback or recovery authority.
- Shared durable gateway sync now binds the canonical Cashier query to its pending
  identity and applies encoded replies through the existing bounded decoder.
  Authority, restore fencing and request/source/token checks precede parsing;
  failed replies preserve the pending attempt for explicit cancellation or a valid
  response. Native and PocketIC tests cover decoding refusal, stale correlation,
  atomic reply-write rollback and restoration. The labelled probe uses this same
  workflow; authenticated provider transport remains integration work.
- Shared async gateway query orchestration checks the durable attempt on polling,
  releases the registry borrow across host transport, and revalidates on completion.
  Transport/source failures retain pending state; delayed replies cannot overwrite
  operator edits or a newer sync. PocketIC covers real delayed local calls, callback
  write traps and fenced restoration. Transport is explicitly host-supplied; the
  local scheduling substitute does not qualify Caffeine's deployed query endpoint.

## [0.2.6] - 2026-09-27

### Added

- Shared guarded funding dispatch now owns first-attempt admission, durable
  marking, post-write platform liquidity checks, one canonical unbounded call and
  callback settlement. Host observations are acquired synchronously on polling;
  missing holds refuse without mutation. Liquidity refusal records a positively
  unsent call, while callback failure preserves uncertainty after remote acceptance.
  PocketIC exercises this complete handler against the labelled local Cashier
  substitute, including malformed replies, delayed replies with duplicate refusal,
  and write traps. Production evidence acquisition and deployed-provider
  qualification remain open.
- Shared first-attempt admission binds fresh host observations to the complete
  retained request. It recognises that request's own reservation without charging
  it twice or requiring another history slot, while preserving older acceptance,
  external uncertainty and restore fences. Only an exact unattempted intent can
  receive its first durable marker; uncertain and terminal intents cannot retry.
  Blocked calls preserve storage and reservations. Native and PocketIC coverage
  exercises full history, identity exclusion, missing evidence and restoration.
  Marking sends no call; qualified host evidence and post-write platform liquidity
  checks remain required when composing production dispatch.
- Shared funding preparation checks exact scope and identity, current journal
  obligations, history capacity and the restore fence alongside independently
  supplied host evidence. Unknown provider qualification, recovery, spendability
  or complete account activity blocks reservation; external clearance cannot
  override local liabilities. The update re-reads current state synchronously,
  accepts no saved preview and leaves blocked requests unchanged. Native and
  PocketIC coverage includes changed state, scope/caller isolation, full history
  and fenced restoration. Host evidence acquisition and production dispatch
  remain separate requirements; preparation sends no provider call.

## [0.2.5] - 2026-09-27

### Added

- Scoped funding summaries expose complete local attachment totals, lifetime
  history capacity and the restore fence without scanning intent rows. Shared
  policy keeps earlier accepted or reserved amounts unresolved after later
  refunds or unsent attempts. Native and PocketIC cases cover scope/caller
  isolation, rollback, actual IC acceptance and fenced upgrade inspection.
  A clear local summary does not establish complete provider-account activity,
  verified credit or payment authority.
- Durable funding observations retain structured Cashier replies and exact
  transport accounting together. Operator outcome lookup reports balances,
  errors or missing replies separately from conservative reconciliation needs;
  accepted cycles still require independent credit evidence. Original account,
  attachment and optional target are checked against the shared call before
  recording. Exact replay cannot refund twice or overwrite a different reply.
  Native and PocketIC coverage includes bounded record widths, response/phase
  validation, callback rollback, mismatched identities and fenced upgrade reads.
  The v1 intent schema is replaced directly under the reinstall-only contract.
- Explicit shared Cashier transport sends the canonical top-up request once and
  captures its exact unbounded-call refund before decoding. Call cost excludes
  the attachment; same-message liquidity checks preserve the full offer. Replies,
  rejects and accepted cycles remain distinct. PocketIC connects the durable
  journal to a local Cashier substitute, covering zero/partial/full acceptance,
  malformed/error replies, receiver and callback-write traps, caller isolation,
  blocked retries and fenced upgrade. This supplies local IC transport evidence;
  deployed-provider qualification, account-wide activity and credit reconciliation
  remain required before production payment admission.
- `StableFundingJournal::history` provides bounded operator-only discovery of
  original funding intents and current local outcomes, newest first, without
  requiring a saved request. Service/Cashier/account/namespace checks bind each
  query and continuation cursor. Reads preserve reservations and remain available
  under the restore fence. Native and PocketIC cases cover full-width identities,
  pagination, changing observations, caller isolation, rollback and upgrade;
  discovered requests confer no payment, retry or unfencing authority.

### Changed

- Reduced the 704-object PocketIC release-history test's overhead by queuing
  independent updates as separate IC messages and reading bounded diagnostic
  windows. Every reply, measurement sequence, caller and instruction bound is
  checked; full history, exact retries, restart and cleanup coverage remain.
  Fixture content preparation uses the existing manifest builder to avoid
  duplicate leaf hashing. No service API, allocator or dependency change.

## [0.2.4] - 2026-09-27

### Added

- `StableFundingJournal` persists exact local attachment intents, first-attempt
  markers and terminal transport observations through two host-granted `ic-memory`
  stores. Full offers are reserved before attempts; pending/uncertain intents block
  later reservations. Exact replay cannot repeat an attempt or refund twice, and
  conflicting outcomes reject. Incremental accounting shares the existing funding
  reconstruction rules, retaining accepted and unresolved amounts independently
  of provider credit. Reopen checks complete bounded history and totals without
  repair, then fences every mutation. Native and PocketIC cases cover corrupted
  history, overflow, interrupted writes and every phase through upgrade.
- `CashierTopUpRequest` owns canonical `account_top_up_v1` encoding. Its method,
  explicit account, optional target balance and exact attachment are bound to the
  retained intent. Request inspection
  survives fencing; changed arguments and mismatched transport service/target
  reject without releasing reservations. Independent Candid vectors and PocketIC
  cover request encoding, correlation and upgrade preservation.

### Changed

- Updated the test dependency `ic-testkit` to 0.10.1.
- Made the pre-1.0 policy explicit in `AGENTS.md`: 100% hard cuts, complete
  removal of superseded contracts and no compatibility or migration paths.
  Same-release recovery and obligation-preservation requirements remain in force.

Provider authentication, spendability and operational recovery remain open;
no payment dispatch is enabled.

## [0.2.3] - 2026-09-27

### Added

- `StableUploads` persists tenant enrollment, root claims, exact upload permissions,
  manifests, confirmed lifecycles, individual references/receipts and maintained
  quota totals through host-granted `ic-memory`. Synchronous IC updates commit
  related writes together. Heap and stable owners share admission, reference,
  cleanup capacity and settlement rules. Completion preserves charged bytes;
  logical release, physical deletion and billing cessation remain separate.
  Reference mutations touch individual rows without copying complete histories;
  bounded manifest records use variable-size pages.
- `StableTenantEnrollments` preserves activation generations and suspension;
  `StableRootClaims` maintains immutable root/object bindings and a reverse index.
  Both are incorporated into the durable upload owner. Fresh installation rejects
  allocated memory. Reopening validates configuration, records, indexes and totals
  without repair, then fences mutations. Retained local state alone cannot authorize
  operational recovery.
- Durable indexed content discovery preserves independent upload/object identities
  and original metadata. Reference-qualified descriptors require the consumer's
  exact live reference. Tenant and operator history/cleanup scans enforce separate
  scan/result bounds and scope-bound cursors; empty filtered pages advance, and
  continuing billing remains visible after physical deletion.
- Durable admission/reference capacity queries use maintained counters and shared
  model arithmetic, preserving lifetime history and reserved cleanup receipts.
  Bounded operator root observations retain exact identities, pending/retired states,
  duplicate positions and malformed inputs. Suspended/restored reads remain
  available without granting provider callback, deletion, retry or mutation authority.
- `reference_receipt` inspects an exact tenant reference operation without
  applying it. Reads share mutation authority/payload checks and preserve original
  success or typed failure through suspension, full history and settlement.
  Historical success is distinct from current reference liveness.
- `blob-fixture-reference` saves bounded, explicit reference intents in a locked,
  identity-keyed local journal, then inspects their exact receipt on a selected
  local probe. Writes sync the file and directory before acknowledgment; exact
  retries recover the same record and changed payloads conflict. Full journals
  preserve recovery, and interrupted files are never automatically discarded.
  Native and executable/PocketIC cases cover writer termination, lost local
  acknowledgment, missing receipts, failures, release, settlement and stop/start.
  The tool sends no mutations, allocates no identities and grants no restore
  authority; local filesystem persistence is not production service recovery.
- An unpublished storage probe exercises the shared durable owner through actual
  IC callers. PocketIC cases cover partial-write rollback from admission through
  settlement, caller isolation, bounded reads, capacity and same-release upgrades
  into the mutation fence. Native cases additionally cover heap/stable agreement,
  corrupt or missing records, full-width identities and retained history.

Real Caffeine integration, durable provider-call intents, read sessions and
operational recovery remain unfinished. Provider facts in the probe are labelled
substitutes; this release does not qualify the service or Canic retirement.

## [0.2.2] - 2026-09-27

### Added

- `blob-fixture-inventory` connects prepared inventories to local admission,
  content-discovery and reference-capacity queries. It checks manifest/root
  consistency and recomputes totals before querying, preserves per-asset reference
  demand, and separates invisible, unfinished, live and retired content. Typed
  failures produce no partial report. Actual executable/PocketIC journeys cover
  isolation and unchanged service state. Queries are sequential observations,
  never upload permission, reservations or a resumable operation journal.
- Tenant-scoped `admission_capacity` reports remaining lifetime objects, concurrent
  uploads, manifest leaves and byte headroom from the shared admission accounting,
  plus enrollment and per-object metadata/size limits. It includes reservations
  and continuing billing, preserves lifetime history after cancellation/settlement,
  and remains inspectable during suspension. Native and bounded PocketIC queries
  cover scope isolation and lifecycle accounting; observations reserve nothing.
- `prepare_upload --inventory ... --snapshot PARENT_DIRECTORY` saves the exact
  hashed bytes and completed inventory in a fresh private directory. Duplicate
  roots share one saved body; later source replacement cannot alter that copy.
  Ordinary failures remove only the current attempt, and repeated runs preserve
  prior snapshots. Files are synced before success, but this is not a crash-durable
  transaction or operation journal. Saved files still need verification before
  later effects; no provider calls, library dependencies or allocator changes.
- `prepare_upload --inventory` performs a bounded offline multi-file dry run.
  It validates all declarations and aggregate work limits before reading sources,
  preserves separate asset mappings while grouping identical roots, and reports
  source versus distinct-blob byte/leaf totals. It rejects duplicate asset IDs,
  unsafe paths, symlinks and nonregular or wrong-length sources. No report is
  emitted until every file succeeds; source failures identify the asset and path.
  These are local content statistics, not live capacity, reservations or uploads.
- `CaffeineManifestBuilder` prepares a bounded ordered chunk manifest and both
  content identities in one streaming pass through the shared hasher. It reserves
  leaf capacity before accepting bytes and preserves state on rejected appends.
  The local `prepare_upload` example reuses the service metadata validator and
  emits original metadata, computed root/digest and leaves only after clean EOF.
  Independent vectors and a 10 MiB PocketIC admission/descriptor journey cover
  client preparation without relaying file bytes through the canister. No provider
  call, raw-digest admission requirement or allocator change is introduced.
- `retained_content_descriptor` checks confirmed completion and an exact live
  consumer reference in one owner read. Released/unknown references and mismatched
  object incarnations disclose no descriptor, even if another reference keeps the
  blob live or an old successful retain receipt replays. Native and PocketIC cases
  cover caller isolation, suspension, stop/start and cleanup. The observation adds
  no receipt or reservation; trusted publication and release coordination remain
  consumer workflow requirements.
- Tenant-scoped `content_descriptor` views recover the first validated metadata,
  exact object identity and current lifecycle. Admission retains one bounded
  metadata set per lifetime permission; reordered retries, suspension, cancellation
  and settlement preserve it. Actual IC caller, client-verification and capacity
  tests cover the private query adapter. This is not certified browser publication.
- `CaffeineRootVerifier` binds a trusted root, exact length and original hash
  metadata before processing a bounded stream. Clients can verify downloads
  without a supplied raw digest, leaf list or canister byte relay, reusing the
  existing Caffeine hashing implementation. Prefixes remain unverified until
  finalization. A local `verify_download` example requires successful EOF and
  rejects corruption, truncation, excess bytes and late read errors. This is a
  verification primitive, not a production HTTP/browser client or provider proof.
- The local `verify_download` example accepts an optional output path. It stages
  bounded bytes privately, verifies clean EOF and the root, syncs the file, then
  publishes without replacing an existing destination. Failures remove normal
  staging residue; symlinks and racing output creators cannot be overwritten.
  A 10 MiB independent-vector CLI check covers saved bytes and rejection paths.
  This is local file publication, not authenticated descriptors, browser delivery
  or a crash-durable transaction. Only the example adds an already-locked tempfile
  development dependency; library dependencies and the allocator are unchanged.

### Changed

- Resource checks now include maximum metadata retention and report
  `.tmp/descriptor-resources.json`. With retained headers, the 704-object workload
  uses 64 KiB more allocated Wasm memory; retain/release peaks measure 5.43M/5.51M
  instructions. These local costs include allocation/memory-access effects and
  do not establish production limits. The default allocator remains unchanged.
- The private readback fixture uses Candid's bulk byte decoder with unchanged
  wire types, reply limits and manifest verification. A 1 MiB read falls from
  about 234M to 116M measured service instructions; allocated Wasm memory is
  about 1.1 MiB higher but stays flat across repeated reads. The default Rust
  allocator is unchanged. Operator-only, bounded diagnostics and
  `make test-read-resources` cover full chunks, malformed replies, held-slot
  denial and recovery. These fixture costs do not qualify production delivery.
- Reference requests validate a private single-reference transition and cleanup
  receipt capacity before publication, removing the full lifecycle-history copy
  on each mutation. Public APIs, retained failure receipts and exact retry
  semantics are unchanged. A new PocketIC workload checks 256 simultaneously
  live references, all 511 receipts, stop/start and cleanup at capacity;
  `make test-admission-resources` also writes `.tmp/reference-history.json`.
- The unpublished admission probe uses operation-specific Candid inputs through
  the same shared handlers. It no longer sends or decodes the entire command
  enum for every mutation. Byte, work, type-header and skipped-value limits stay
  enforced, with actual-caller rejection, retry and capacity-cleanup coverage.
  Library APIs and production endpoint contracts are unchanged.
- Resource reports separate runtime entry, Candid header/value decoding and
  workflow counters. The 704-object workload's final admission drops from
  5.43M to 3.11M measured instructions, and peak retain from 9.20M to 5.43M.
  These local observations include IC memory-access charges, exclude reply and
  persistence/provider costs, and do not establish a production capacity limit.

## [0.2.1] - 2026-09-27

### Changed

- Admission keeps private global/tenant reservation and manifest-leaf totals
  alongside successful transitions, removing scans of retained upload history.
  Root ownership uses a bounded object-identity index instead of a reverse scan.
  Confirmed-object usage now maintains every global/tenant counter through one
  mutation path, including receipts for rejected lifecycle operations. Root-only
  exposure indexes the original operation and rechecks its authority; rejected
  admissions cannot create a lookup entry.
  Exact retries, cancelled history and separate physical/billing charges remain
  unchanged; failed admission consumes no index or quota capacity.

### Added

- Read-only reference capacity reports separate unused reference identities,
  unreserved receipts and cleanup reservations, including the number of fresh
  distinct retains that fit. The shared owner authorizes disclosure by tenant.
- A 704-object PocketIC workload retains manifests for 288 MiB of synthetic media
  through four reference generations, exhaustion, retries, stop/start and cleanup.
  Completion/deletion/billing controls are explicitly operator-only substitutes.
  `make test-admission-resources` also emits `.tmp/release-history.json`, separating
  pre-workflow and workflow instructions from allocated Wasm memory.
- Tenant-authorized content discovery returns the original upload operation and
  current reservation/lifecycle through the retained root index. Foreign and
  unknown roots disclose no object; suspended tenants retain inspection access.
  Native overlapping-release tests cover exact receipt recovery, stale reads,
  history exhaustion with cleanup and separate physical/billing settlement.
  PocketIC checks real caller isolation and passive discovery across stop/start.
- A two-tenant PocketIC workload filling 256 lifetime operation slots, checking
  cleanup, isolation, exact retries, stop/start and instruction/memory budgets.
  `make test-admission-resources` now writes both single-upload and history reports.
  Measurements retain the before/after tradeoff: fewer explicit scans do not
  establish uniformly lower total call costs, and this workload uses 64 KiB more
  allocated Wasm memory with the indexes.
- Independent accounting checks across interleaved tenant transitions, rejected
  mutations, zero-byte uploads, confirmation, deletion, settlement and retries.
  A 64-object, three-tenant audit covers all confirmed usage fields and totals above
  `u64`; an actual IC test exercises the final permission at 256-operation capacity
  through failed authority checks, stop/start and one-shot exposure.

## [0.2.0] - 2026-09-27

### Added

- Canonical service upload metadata: exactly one `Content-Length` must match the
  reservation; malformed names/values, duplicate names ignoring case and ambiguous
  length spellings reject before manifest mutation. Configuration must accommodate
  the largest object's length header. Native and actual IC checks preserve quota
  and prior manifests on rejection and accept equivalent reordered retries.
- Bounded manifest authorization for direct browser-to-Caffeine uploads, with
  exact uploader, activation and deadline checks. Manifest binding is distinct
  from provider completion; exposed uncertainty keeps its reservation. No file
  chunks pass through service admission. The 10 MiB local admission, preparation,
  retry and exposure sequence measures about 4M instructions, excluding provider
  transport and persistence.
- Bounded decoding, pre-conversion manifest checks and operator-only resource
  observations in the local admission probe. Focused PocketIC checks enforce
  instruction/memory budgets and emit a report through
  `make test-admission-resources`. The metadata-only boundary accepts at most
  16 KiB per command and separately bounds decoder work, skipping and type headers.
- Explicit global and per-tenant budgets for retained manifest leaves. Admission
  reserves capacity from declared size before catalog mutation; retries consume
  nothing extra, and cancellation or settlement cannot refund retained history.
- A local IC adapter exercising the shared owner with actual caller/time bindings.
  Covers role isolation, 10 MiB manifests, exact retries, suspension/expiry,
  stop/start continuity and atomic rejection of unsupported upgrades. It emits
  no certificates or provider effects; durable service recovery remains open.
- Shared service manifest budgets derived from the admitted object size, with
  explicit metadata limits and portable chunk counts. Empty uploads reject before
  reservation. Independent Caffeine vectors and the PocketIC journey now cover
  the 10 MiB media boundary, oversized rejection, exact chunk retries and retained
  recovery history; fixture admission and journal validation share one envelope.
- Bounded operator-managed tenant enrollment in the shared admission model.
  Suspension blocks fresh uploads, exposure and reference retains while preserving
  exact receipts, cleanup and liabilities. Reactivation invalidates old uploader
  permissions; stale enrollment updates conflict. Reference operations now verify
  project/service/namespace context at this boundary. The 0.2 plan specifies the
  remaining consumer registration/release transaction and outbox contract.
- Project-authorized upload admission over the shared catalog, binding an exact
  operation to an uploader and issuance deadline. Root-only certificate lookup
  resolves retained permissions; changed retries, wrong callers and repeated
  exposure reject. Passive lookup cannot renew authority. Revocation cancels only
  unexposed reservations; escaped uploads and failed consumer registration retain
  their accounting and references. This is a transient service model, without
  certificate transport, durable state or provider retry guarantees.
- Service configuration candidate validation for explicit service, operator and
  payer identities, Wasm-compatible metadata counts and consistent object,
  tenant, upload and gateway limits. Reference budgets reserve enough minimum
  receipt history for retain/release. Construction grants no installed authority
  or provider effects and defines no stable schema.
- A finite 0.2 delivery plan and current Toko development source review, separating
  consumer upload limits, browser authorization and asset-registration failures
  from the completed 0.1 foundation. Current status now summarizes milestones
  instead of repeating historical implementation notes.

### Changed

- Hard-cut the mandatory service byte-append workflow, streaming verification
  state and raw-digest verdicts. `UploadRequest` no longer requires a raw digest;
  service configuration names retained manifest capacity explicitly. Independent
  content/read verification primitives remain available, with digest binding owned
  by their integrity fixtures. Provider size, replay and completion guarantees
  remain prerequisites for live issuance, not claims established by a manifest.

## [0.1.19] - 2026-09-27

### Added

- Account-scoped Cashier audit query encoding with an explicit positive page
  bound, event filter and opaque cursor. Replies use the original method/target
  and the stricter requested/decoder count limit. Independent Candid vectors
  cover every filter and full-width cursor values; no automatic pagination or
  payment reconciliation is inferred from audit text.
- Bounded Caffeine payment-relationship reply inspection with explicit expected
  storage owner and payer. Preserves signed limits, period spend and bandwidth
  counters without inventing spending authority; absent reports, binding errors
  and all advertised provider failures remain distinct. Independent Candid
  fixtures cover decoder bounds and optional-field semantics.
- Library-owned Cashier balance, payment-relationship and gateway-list query
  encoding with explicit targets and account bindings. Replies can now be handled
  against the original request, rejecting wrong methods or sources before parsing.
  Gateway application also requires the exact registry scope and pending token.
  The local balance and gateway workflows retain their request across the await;
  independent Candid vectors and IC revocation/recovery checks cover both paths.
- Query-only relationship and gateway inspection in the controlled local source. PocketIC
  covers exact signed amounts, missing/error reports, owner/payer mismatches,
  denied callers, unchanged journals and restoration fences. Gateway queries
  refuse scripted effect modes; consumed or revoked sync tokens cannot reapply
  their replies. This adds no deployed transport, account-link action or paid operation.
- Source-backed installation proposal separating Toko tenants, storage owners
  and Cashier payers, with existing-obligation disposition and concrete gates
  for shared production handlers. Current package/interface metadata rechecked;
  no live installation, account or provider effects are selected by the proposal.

## [0.1.18] - 2026-09-27

### Added

- Bounded Caffeine ledger-deposit notification response decoder, based on the
  refreshed deployed Cashier Candid. It preserves reported credit, arbitrary-width
  block indices and all advertised errors, rejecting invalid amounts and malformed
  or over-budget replies. Independent Candid fixtures cover boundaries and unknown
  variants. Reports do not establish operation correlation or settle uncertain
  payments; no ledger transfer, notification or automatic retry is introduced.
- Shared bounded attachment accounting for sequential funding journals. Original
  full offers must preserve the installed reserve before refunds are applied;
  accepted and unknown transfers retain their allocation. Exact refunds and
  proven unsent offers stay separate, with typed capacity/sequencing/overflow
  failures. The funding fixture now uses this library model instead of owning
  another accounting loop; native and IC recovery checks cover the integration.
- Query-only exact funding lookup and unpublished `blob-fixture-funding-lookup`
  CLI. Full original requests must match retained intents; absent, pending and
  observed states remain distinct from denied/conflicting/failed lookups. IC and
  subprocess tests recover discarded ingress results without retransmission and
  preserve callback uncertainty and old-backup fences. This inspects the local
  experiment, not Cashier credit or permission to repeat a payment.

### Fixed

- Local funding now preserves all four advertised Cashier error categories in
  its journal, operator status and CLI JSON, including the reported unauthorized
  principal. Previously only internal errors survived classification. Actual IC
  tests preserve refunds and uncredited acceptance independently of each error,
  reject wrong-route ledger reports and unknown errors, and retain uncertainty
  after callback traps and fenced restore. Unpublished fixture schema remains v1
  with reinstall required across releases; published library APIs are unchanged.

## [0.1.17] - 2026-09-26

### Added

- Unpublished local `blob-fixture-refresh` command with passive dry-run and an
  explicit balance refresh. Requests bind service, namespace, source, account,
  configuration revision and next attempt; stale or consumed requests cannot
  dispatch another read. This is a PocketIC tool, not a production provider client.
- Separate JSON action and post-status outcomes: a failed diagnosis cannot replay
  the action or erase an acknowledged completion. Actual subprocess/IC tests cover
  stale previews, pending reads, rejected callers, failed observations, fenced
  restores, malformed acknowledgements and query-only dry-run enforcement.
- Local `blob-fixture-sync` command with passive preview and exact service,
  namespace, source, edit revision and next-sequence admission. Revocations,
  including absent-member revocations, invalidate old previews; consumed requests
  cannot dispatch again. Balance refresh and sync share action/status reporting.
  PocketIC covers held replies, reentrant replacement, old archives and live
  callback upgrades without restoring authority or repeating a completed action.
- Additive funding admission assessment preserves missing recovery, spendability
  and activity alongside known blockers. Reserve arithmetic requires known funds
  and retains the complete request; existing admission API behavior is unchanged.
- Passive `blob-fixture-funding-preview` command binds sender, peer, operation ID
  and amount. It reports identity reuse, journal capacity, unknown spendability,
  unverified credit and restore fences without consuming an intent or transferring
  cycles. PocketIC covers refunds, callback traps, exhausted history, added gross
  cycles and update-only method rejection. No operator funding action is exposed.
- Explicit installed attachment budgets for the local funding fixture. Original
  intents reserve the full offer atomically; exact refunds and proven enqueue
  failures release only the corresponding allocation. Accepted or unresolved
  attachments remain charged through restore. Preview revisions change even after
  full refunds. Gross cycle top-ups and incoming receipts cannot replenish this
  budget; execution fees, provider credit and production spendability remain separate.
- Additive liquidity policy checks the full attachment against platform liquid
  cycles, call costs, positive operating slack and explicit other liabilities.
  The local funding fixture rechecks after intent persistence and records liquidity
  refusals as unsent operations, without fabricated refunds or reusable identities.
  Passive previews expose cost/liquidity observations but cannot authorize dispatch;
  PocketIC covers fee-only rejection, operating holds and changed funds with an
  unchanged allocation revision. Production credit and recovery remain unqualified.

### Fixed

- Local funding refusals no longer execute callback trap controls when no call
  was sent. Their consumed identities and released attachment allocations survive
  restore; genuine callback traps still retain the full uncertain attachment.
  PocketIC covers refusal-history exhaustion and zero/full-refund callbacks.
- Funding previews now report the same maximum attachment bound enforced by
  update admission, independently of resource and provider blockers.

## [0.1.16] - 2026-09-26

### Added

- Exact-release Caffeine verification checkpoints bind manifests, content digests,
  lengths and streaming hash state. Reconstruction validates counters, leaf
  boundaries, canonical buffering and an accidental-damage checksum. Protected
  checkpoints remain private; they grant no freshness or provider authority.
- Shared operator diagnosis preserves recovery/provider blockers, unknown or
  uncertain funding and outstanding work. Complete funding-history assessment
  keeps transport acceptance separate from provider credit; later refunds cannot
  clear older obligations.
- Shared balance-threshold assessment reports exact shortfalls without inventing
  spendable funds. Operator diagnosis explicitly blocks unknown spendability.
  The existing complete reserve API retains its signatures and behavior.
- Unpublished `blob-fixture-status` client queries explicitly selected local
  PocketIC instances. JSON preserves unknown values, exact decimal amounts,
  separate catalog charges and recovery fences. Blocker checks and typed read
  failures have distinct exit codes; queries cannot fall back to updates.
- Scoped local balance observations persist exact service/namespace/source/account
  intents and bounded response history. The shared Caffeine decoder distinguishes
  zero, malformed amounts, mismatched accounts and provider failures. Revision
  changes, dispatch-based expiry and restoration prevent stale use. Validated
  diagnostic billing limits remain bound to their original configuration revision.

### Changed

- Authority, controlled-source and funding fixtures restore into permanent
  inspection-only fences. Validation retains catalog identities, receipts,
  reservations, private verification state, byte charges and pending operations.
  Old journals and late callbacks cannot resume uploads or payments. Restored
  funding receivers reject cycles before acceptance.
- PocketIC coverage now includes held gateway/balance replies, older in-flight
  journals, callback/upgrade rollback, repeated restores, lifetime capacity and
  separate physical/billing release. Actual CLI subprocesses prove passive reads,
  caller/target checks, update-only method rejection and configured diagnosis with
  unknown spendability. Fixture schemas are hard cuts with reinstall across releases.

Production provider integration, both service adapters, operational recovery and
whole-canister snapshot safety remain unqualified. These local fixtures and
read-only diagnostics do not establish Canic removal readiness.

## [0.1.15] - 2026-09-26

### Added

- A connected local PocketIC upload/deletion journey uses shared tenant policy,
  upload reservations and lifecycle accounting with current Caffeine method shapes.
  It covers certificate admission before possible exposure, interrupted uploads,
  exact request replay, cross-tenant rejection, logical release, real inter-canister
  deletion callbacks and separate billing cessation. Mixed invalid deletion batches
  roll back on the IC; revoked gateways cannot apply delayed confirmations.
  Upload completion and final billing evidence remain explicit local substitutes.
- The local journey now binds certificate admission to actual content verified
  against its reserved Caffeine manifest and raw digest. Corrupt/truncated bytes,
  digest replacement, unauthorized verification and pre-exposure completion are
  rejected without releasing reservations. Explicit metadata and up to six 1 MiB
  chunks are verified across separate messages with tenant-only progress. Exact
  chunk retries do not advance hashing twice; rejected chunks preserve the prefix,
  and a final raw-digest mismatch cannot be reset by replaying admission. Bounds
  are local fixture limits, not production upload limits or restart guarantees.
- Local readback now fetches one chunk through a real inter-canister call and
  verifies its exact length/hash against the admitted manifest before returning
  bytes. Tenant/reference/gateway checks run before and after the await. Held
  replies are rejected after release or gateway revocation/re-addition; corrupt,
  truncated, oversized and malformed replies return no bytes. One bounded read
  slot prevents overlap, with exact callback cleanup and no automatic retry.
  The controlled source is a provider substitute, not a Caffeine HTTP adapter.
- The transient authority fixture rejects unsupported upgrades in both lifecycle
  hooks, preserving obligations instead of discarding heap journals. PocketIC
  covers stop/start continuity, rejected upgrades (including skipped outgoing
  hooks), held callbacks and retained billing/root history. An actual read-callback
  trap rolls back slot cleanup and leaves further reads blocked through stop/start
  and elapsed time. Upload journal restoration and old-snapshot safety remain open.
- The local gateway source persists a bounded journal through host-owned ic-memory:
  bindings, one retained leaf, read state and lifetime call intents/results. It
  restores synchronously into a permanent inspection-only fence. PocketIC covers
  maximum retained data, exhausted history, missing/older stable journals, skipped
  outgoing hooks and unresolved callbacks. No restored counter authorizes new
  effects; this is fixture evidence, not provider or production recovery.
- Raw content digests can be parsed from exact 32-byte boundary inputs, with
  typed length errors and no implied content verification or tenant authority.
- The authority fixture atomically archives all three catalogs through ic-memory,
  retaining cancelled/settled roots, reservations, release receipts, byte liabilities,
  original manifests and observed verification progress. Exact read intent is
  recorded before dispatch; callback traps and rejected deletion batches roll back
  archive writes with live state. Operator-only inspection reads stable memory.
  The archive cannot resume hashing or authority; unsupported upgrades still reject.
- Gateway registries expose read-only allocated/pending sync sequence observations,
  without exposing reusable tokens or granting reconstruction authority.

## [0.1.14] - 2026-09-26

### Added

- Shared funding-transfer accounting distinguishes exact unbounded-call refunds,
  proven enqueue failures and unknown outcomes. Pure reconciliation policy keeps
  accepted cycles separate from provider credit and retains the full attachment
  when transfer evidence is missing. Invalid refunds are rejected without clamping.
- The local funding fixture now uses the shared model and policy. PocketIC covers
  a real insufficient-cycles enqueue failure with no callback refund, retained
  history across upgrades and rejection of reused identities. Fixture starting
  balances are explicit; live Caffeine credit reconciliation remains unqualified.
- Bounded decoding of Cashier audit-download responses using the refreshed deployed
  Candid interface. Opaque CSV, reported counts and optional cursors are preserved;
  provider errors stay distinct from empty pages. Independent wire fixtures cover
  resource bounds and malformed pagination. Audit rows do not yet establish credit
  or authorize retries.

## [0.1.13] - 2026-09-26

### Added

- Local PocketIC funding experiments capture exact call refunds separately from
  Caffeine reply decoding, covering zero/partial/full acceptance, provider errors,
  malformed replies and rejects. Callback traps retain unresolved intent and
  block another payment. Host-owned ic-memory journals restore synchronously
  across same-release upgrades, preserving payment identities, receipts and
  lifetime limits. Receiver traps roll back acceptance and receipts with a full
  refund; failed upgrades preserve journals and unresolved-payment blocking.
  The bounded unpublished fixture uses ic-testkit's PocketIC
  export; it does not qualify live Cashier credit or recovery from old backups.

## [0.1.12] - 2026-09-26

### Changed

- Replaced the direct `ic-stable-structures` dependency with `ic-memory` 0.14.3,
  matching Canic and IcyDB. The crate re-exports `ic_memory`, including its exact
  stable-structures substrate. Memory bootstrap, allocation grants and bucket
  configuration remain owned by the integrating host; no blob stores are declared.
- Tenant upload usage reads now scan only that tenant's ordered operation range,
  avoiding full service history scans while retaining all reservation and liability
  accounting. No cached counters or additional upload index are introduced.

### Added

- Bounded tenant pages for unsettled confirmed objects, including physically
  deleted and zero-byte objects whose billing obligations remain unresolved.
  Tenant-owned root indexing prevents other tenants from consuming scan budgets
  or appearing in cursor metadata; usage reads share the same index.
- Native pagination, accounting and upload-transfer checks plus a PocketIC
  release/deletion/billing observation journey with real caller isolation.
  Provider confirmation facts remain explicitly substituted by the fixture.
- Native memory composition checks for passive library linking and host-owned
  handles shared through the re-export, with isolated cells and preserved host
  configuration. These do not implement or qualify blob persistence or recovery.
- `scripts/dev/cloc.sh`, copied from Canic and adapted to this repository's crate
  names, plus `make cloc`. Reports Rust runtime/test file LOC and test function
  counts under `crates/`; requires optional developer tools `cloc` and `jq`.

## [0.1.11] - 2026-09-26

### Added

- Transient upload admission sharing one owner's catalog, root history and
  tenant/global capacity. Reservations bind exact request IDs, service, tenant,
  namespace, object incarnation, first reference, raw digest, root and length.
  Concurrent upload limits are separate from retained lifetime history.
- Exact retries return current state without another allocation. Cancellation
  frees byte capacity only before exposure; possibly exposed uploads retain
  reservations until independently confirmed. Confirmation transfers capacity
  into the catalog without double counting or resurrecting released references.
- Native coverage for competing tenants, exhausted bounds, conflicting requests,
  cancellation, uncertain outcomes, completion replay, zero-byte uploads and
  wide byte totals. Logical release, physical deletion and billing cessation
  continue to free separate capacities. No provider effect or persistence is added.
- Tenant-authorized usage and bounded active-upload pages include pending
  reservations. Pages recheck caller/cursor scope, retain continuation through
  terminal history and observe intervening cancellation or completion.
- Gateway root observations distinguish reservations, possible exposure,
  cancellation and confirmed lifecycle state. Namespace isolation and current
  membership apply before disclosure; no observation grants deletion permission.
  Batch reads resolve distinct roots together and scan operation history at most
  once, stopping when all pending roots are found. Duplicates reuse observations;
  confirmed/unknown/malformed-only batches skip history scanning. Temporary maps
  stay bounded by input length and do not add a persistent index.
- PocketIC upload fixtures verify actual tenant/controller isolation, cancellation
  replay, retained uncertain capacity and immediate gateway revocation. Native
  read tests cover page budgets, cross-scope cursors and changing state between pages.

## [0.1.10] - 2026-09-26

### Added

- Bounded in-memory chunk verification progress over an immutable Caffeine
  manifest. Out-of-order reads credit each position once; duplicates cannot
  inflate verified bytes or hide a missing chunk. Every supplied chunk is checked,
  including retries after prior success, and rejected input leaves coverage intact.
  The tracker retains one bit per chunk and no content or retry history; it does
  not claim destination durability, persisted resume or provider completion.
- Independent client-vector coverage for missing-position recovery, repeated
  content, reverse-order delivery and exact partial-chunk byte accounting.
- Ordered manifest-and-raw-digest verification: a chunk must pass its exact leaf
  check before entering the whole-file hash, so corrupt chunks can be retried
  without losing the verified prefix. Finalization requires full length and the
  expected raw digest; skipped/replayed chunks reject without advancing state.
- Bounded missing-chunk pages with independent scan/result limits and exact local
  byte ranges. Empty filtered pages retain continuation, and later pages skip
  chunks verified between calls. Enumeration neither reserves reads nor grants
  completion; tests cover end positions, oversized indices and partial final chunks.
- Actual PocketIC Wasm execution of full-chunk, partial-final-chunk and Unicode
  metadata vectors, including corruption recovery, replay denial, wrong-digest
  and truncated-read rejection. The local fixture measures ordered-append
  instructions against an explicit regression budget; no provider is contacted.

## [0.1.9] - 2026-09-26

### Added

- A test-only Wasm authority probe and unpublished PocketIC harness using
  `ic-testkit`. Real IC caller/controller checks cover tenant reads and release,
  forged bindings, exact release replay and gateway revocation between calls.
  Sample object facts are local substitutes; no provider or persistence is qualified.
- Real inter-canister gateway-sync tests with a controlled local source: overlapping
  attempts reject before effects, revocation survives an old reply, and a completed
  newer sync cannot be overwritten by the earlier response. Malformed, oversized,
  empty-list and rejected replies preserve membership and require explicit retry.
  Shared unpublished fixture types keep host/canister controls in one place.
- `make test-pocketic` builds the fixture and runs it against the explicitly
  provisioned server under managed startup/cleanup. `make test-native` retains
  the core-only suite; `make test` runs both sequentially. Checks/lints now cover
  all workspace packages, with build artifacts retained.

### Changed

- Moved the testkit dev dependency into the actual PocketIC host harness. The
  published core package and its native-only tests no longer pull the simulator
  stack into their dependency graph.

## [0.1.8] - 2026-09-26

### Added

- Bounded streaming Caffeine content hashing and verification: raw SHA-256 and
  the provider's metadata-dependent root in one pass, with 1 MiB provider chunks
  independent of append boundaries. A fixed hash frontier avoids buffering whole
  files or trees; explicit content, append and metadata budgets bound processing.
- Independent client vectors covering exact chunk edges, uneven multi-level trees
  and ECMAScript metadata normalization/order. Tests reject corrupt/truncated bytes,
  metadata mismatch and invalid appends without advancing hash state. Empty provider
  objects remain explicitly unqualified; matching roots do not prove upload completion.
- Bounded Caffeine chunk manifests checked against an expected root, with distinct
  chunk-hash identities and exact per-index byte verification. Individual chunks
  can be checked out of order or retried without changing state. Independent client
  leaf vectors cover reordered/tampered manifests, wrong positions, short/oversized
  chunks and corruption; a valid manifest is not proof of storage or whole-file completion.

### Changed

- Replaced the direct native PocketIC dev dependency with `ic-testkit` 0.10.0.
  Tests use its full `ic_testkit::pocket_ic` re-export and shared harness helpers;
  the locked PocketIC client/server remains 16.0.0. Testkit stays outside the
  production/Wasm dependency graph.

## [0.1.7] - 2026-09-26

Bounded local catalog, tenant reference reads and exact request-result lookup.
Persistence, provider execution and canister adapters remain pending.

### Added

- Tenant-checked reference liveness reads, individually or in bounded batches.
  Released and unknown references report inactive even while another reference
  keeps the object live. Batches preserve order/duplicates and reject unauthorized,
  oversized or mismatched-scope requests without returning partial results.
  Native tests keep physical and billing obligations intact after logical release.
- A bounded transient catalog joining confirmed objects, immutable root claims
  and exact reference receipts. Added lifetime object/reference/receipt limits,
  per-tenant logical quota, separate physical and billing-byte caps, and derived
  service/tenant usage counters. Rejected admissions are atomic; exact registration
  retries cannot reactivate released objects or erase settled history.
- Pending-deletion pagination with independent scan/result budgets and scoped
  forward cursors. Added current-gateway checks on every page and ordered root
  observations that keep unknown, malformed and foreign-namespace inputs explicit.
  Native multi-object tests cover capacity recovery, zero-byte liabilities,
  receipt exhaustion, cross-tenant denial, revocation and late replay. This remains
  local bookkeeping; durable upload reservations and provider execution are pending.
- Per-tenant lifetime object limits across namespaces, including zero-byte and
  settled entries, so byte-free history cannot consume all shared object slots.
  Exact registration replay remains valid at capacity.
- Bounded tenant reference reads across catalog objects and namespaces, preserving
  order and duplicates. Unknown and foreign roots share one rejection; mixed
  unauthorized batches return no partial results or object-binding details.
- Read-only lookup of exact reference-request receipts, including recorded failures
  and results retained after settlement or at capacity. Queries never reapply a
  request or allocate a receipt; original outcomes remain distinct from current
  reference liveness. Shared lookup rules keep mutation replay checks consistent.

### Changed

- Refreshed Canic, Toko and upstream provider evidence and design assumptions.
  Recorded root reuse, reference coordination, history churn and scan costs as
  decisions requiring consumer/provider evidence before production implementation.

## [0.1.6] - 2026-09-26

### Added

- Scoped gateway registries that reject stale, cancelled and replayed sync
  responses. Operator membership edits invalidate earlier syncs, including
  revocation of a currently absent gateway; counter exhaustion never blocks
  revocation. Added pure callback checks for current service/namespace membership
  and native revocation/race tests. Persistence and endpoints remain pending.
- Bounded Cashier gateway-list reply decoding connected to the scoped registry.
  Wrong-scope and stale attempts reject before parsing; malformed, over-budget
  and invalid lists preserve membership and pending state. Added an independent
  Candid fixture and native reply-to-callback revocation coverage.
- Bounded Cashier account-balance reply decoding with requested-account checks,
  validation of every amount and distinct provider failures. Independent Candid
  fixtures and native readiness tests distinguish failed reads from a real zero
  balance and preserve recovery fences. Funding and balance replies share one
  private balance schema; live queries remain pending.

### Changed

- Recorded Caffeine's independent-deployment support question and the concrete
  provider evidence still needed for the full service journey. Updated the
  handoff to the verified 0.1.5 release and preserved its source-bound evidence.

## [0.1.5] - 2026-09-26

Local blob lifecycle, ownership bindings and request replay handling.
Persistence, endpoint authentication and provider execution remain pending.

### Added

- Bounded binary-root batch parsing for future liveness requests. Input order,
  duplicates and per-entry errors are preserved; raw entry and byte limits
  reject oversized batches before allocating results. No liveness lookup or
  callback endpoint is implemented by this parser.
- A transient confirmed-object lifecycle model with bounded, idempotent reference
  bookkeeping. Final release, physical deletion and billing settlement advance
  separately; premature confirmations and reference reuse reject without mutation.
  Native transition tests cover accounting and rejection without mutation.
- Explicit service/tenant/namespace/object/incarnation bindings on lifecycle
  references and confirmations, with rejection before mutation or replay handling.
  Added pure direct-tenant access checks and native cross-scope denial tests.
- Bounded local request receipts for reference mutations: exact retries return
  original success/failure, conflicting request-ID reuse rejects, and receipt
  reservations preserve capacity to release every active reference. Receipt
  replay still requires the caller's current access check; durability is pending.
- Bounded Caffeine response decoding that preserves structured funding errors
  and separates upload completion reports from verified storage. Added private
  wire types, independent Candid fixtures and malformed/over-budget reply tests.
- Immutable, bounded root claims that reject reassignment across tenants,
  namespaces and incarnations, retaining original associations after settlement.
  Native composition verifies delayed confirmations cannot delete a newer object.

### Changed

- Reviewed Canic's lifecycle design independently and documented the proposed
  persistence boundaries, separating logical release, physical deletion and
  continuing billing obligations. Provider qualification remains open.
- Added source-bound provider recovery probes and integration requirements for
  upload completion, structured funding errors and delayed root-only deletion
  callbacks. Client/codec substitutes do not qualify the deployed provider.

## [0.1.4] - 2026-09-25

Blob-storage billing validation and gateway primitives extracted from Canic,
plus provider-interface evidence. Persistent workflows, provider execution and
canister adapters remain pending.

### Added

- Safe conversion of billing amounts from Candid `nat`/`int` values into Rust
  cycle amounts, plus strict positive decimal funding input. Typed errors reject
  malformed or out-of-range total, prepaid, promotional and ledger amounts.
- Validated billing-configuration candidates combining Cashier principal,
  funding thresholds and gateway bounds that fit 32-bit Wasm on every host.
- Bounded gateway lists and transient membership: ordered deduplication, separate
  raw/distinct limits, idempotent add/remove and all-or-nothing sync replacement.
- Pure funding-intent admission that rejects recovery fences, outstanding or
  uncertain payments, missing configuration and full-request reserve violations.
- Native boundary, rejection/recovery and Candid composition tests connecting
  validated inputs to gateway limits, funding admission and readiness diagnostics.

### Changed

- Recorded Toko's provider defaults, the deployed Cashier's Candid interface and anonymous
  gateway/pricing observations. Corrected the compatibility review: both gateway
  names are advertised and the newer top-up wrapper is Candid-compatible.

## [0.1.3] - 2026-09-25

Release-test reliability and documentation cleanup; storage-service scope is unchanged.

### Changed

- Release-helper tests now show a clearly labeled fixture notice and one success
  summary, with case diagnostics and retained logs on failure.
- Isolated release-test cases, consolidated overlapping preparation checks, and
  replaced permissive log matching with exact release-effect assertions.
- Corrected Make help to reflect that releases preserve the build cache.
- Consolidated duplicate extraction/provider planning documents and replaced
  accumulated handoff history with current status, preserving source evidence.

## [0.1.2] - 2026-09-25

Release and publication tooling fixes; storage-service scope is unchanged.

### Changed

- Release and publication commands retain build artifacts; cleanup is available
  only through an explicit `make clean`.
- Enabled crates.io publication and removed the B1 ownership/readiness blocker.
  Publication still validates the clean tagged release and its source receipt.
- Added the public repository URL to the crate's published metadata.

## [0.1.1] - 2026-09-25

Content-verification and billing-policy foundations. Storage workflows and
provider integration remain pending.

### Added

- Separate raw SHA-256 content digests and Caffeine provider root identities,
  with canonical hash parsing, typed malformed-input errors and byte conversion.
- Incremental raw-content verification with exact offsets, declared-length and
  digest checks, bounded memory, and unchanged state after rejected chunks.
- Validated funding limits and pure reserve-protected funding decisions that
  never substitute a partial top-up.
- Read-only billing readiness with typed balance failures, blockers, warnings
  and recovery-fence reporting.
- Native boundary/vector tests and source-bound evidence for the first core
  primitives; service workflows and provider qualification remain pending.
- Official Mops registry evidence confirming backend package 1.1.1 and matching
  source hashes for the pinned Caffeine integration.

### Fixed

- Release preparation now preserves undated historical changelog entries while
  rejecting competing future drafts, allowing 0.1.1 after the recorded 0.1.0.

## [0.1.0]

Initial repository scaffold, dependency setup and extraction planning.

### Added

- Independent Rust 2024 library workspace with Rust 1.98.1, an MIT license,
  repository governance and isolated build output.
- Pinned Candid, Serde, SHA-256, typed-error, IC CDK and stable-storage
  dependencies, with a locked dependency-fetch command and offline native/Wasm
  compilation checks.
- Native-only PocketIC 16 integration-test dependency, verified local server
  provisioning and an overridable server path that prevents automatic downloads
  during tests.
- Maintainer patch, minor, major and exact-version release tooling, including
  release previews, validated version preparation, rollback on failure,
  source-bound release records, atomic branch/tag pushes and separate registry
  publication commands.
- Canic extraction and capability inventories covering storage lifecycle,
  gateway administration, billing, operator commands and diagnostics, with
  replacement acceptance requirements before Canic removal.
- Draft service and acceptance contracts for standalone and Canic-managed
  deployments, including tenant isolation, quotas, paid-effect recovery,
  restore fencing, deletion, billing cessation and installation retirement.
- Caffeine integration baseline for client 1.1.2 and backend reference source
  1.1.1, with package-integrity verification, client hashing observations and
  documented provider-interface and qualification gaps.

### Implementation status

- Storage APIs, provider workflows, clients and canister adapters are not yet
  implemented. The dependency and tooling checks qualify the scaffold only.
- Provider qualification and the B1 service contract remain open. Registry
  publication is disabled, and Canic functionality has not been removed.
