# Project-approved multi-user upload authority — pending 0.19.0

The maintainer selected the existing project's exact admission as the uploader
grant for [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31).
Released base is `ab37a018e2050b3b06963938ceb78ebb385bb41c` (0.18.5).
The complete pending changelog is 0.19.0; Cargo and the release receipt remain
0.18.5. The [authority contract](../multi-user-upload-authority.md) describes
scope, consumer responsibilities and retirement/reinstall requirements.

## Authoritative grant and removed mechanism

Authenticated tenant admission already records the exact permission. Certificate
resolution requires the actual uploader, resolves the retained root/operation
index and checks current eligibility. Exposure compares the full retained
permission and manifest before independent namespace/current-owner/durability
assessment, then persists exposure before constructing the reply. These existing
checks now own uploader authority without a global trust flag or user registry.
Expiry, revocation, quotas, uncertainty and restore fences remain in their owners.

Remove `trusted_uploader` from candidate/input/readback and immutable configuration,
the policy/DTO `TrustedUploader` blocker and the installation CLI flag/report field.
Publication tools derive their per-batch signer from original permissions rather
than installed configuration. Standalone initialization allows no skipped fields
or extra arguments; ordinary endpoint decoding retains its bounded allowance.
Regenerated standalone Candid agrees with the actual exported service.

The installation format becomes
`ic-blob-storage/installation:project-upload-grants-platform-anchor-funding-credit-index-renewal`.
The allocation key continues to identify the same memory slot, not its layout.
The former format refuses before service owners open. No compatibility reader,
alias or migration is added. Test-only frozen producers install retired images
solely to prove refusal and preservation; production accepts the current DTO.

Every removed named function, method and type:

| Former module | Removed symbols | Reason and replacement |
| --- | --- | --- |
| `contracts::upload::issuer` (`crates/ic-blob-storage-contracts/src/upload/issuer/mod.rs`) | `UploadIssuerAuthority`, `UploadIssuerAuthority::new`, `UploadIssuerAuthority::uploader`, `UploadIssuerAuthority::permits`, `InvalidUploadIssuerAuthority` | Removed installed single-user authority; existing durable permission/caller checks own each project grant |
| `contracts::configuration` | `ValidatedInstallationInput::issuer` | Installation no longer creates a global uploader authority |
| `ops::service::installation` | `ServiceInstallation::issuer_authority` | No installed global authority remains to expose |
| `tests::standalone_certificate` | `standalone_tenant_permission_cannot_grant_installed_uploader_trust` | Removed obsolete single-user rejection contract; actual two-signer and cross-user refusal tests replace it |

No named symbol is moved or renamed. Other existing transitions remain in place.

## Actual inputs and qualification

Final inputs select Memory 0.31.8, private Metrics 0.2.15, all four Host 0.8.3,
Testkit 0.25.3 and PocketIC client/server 16.1.0. Incoming lock changes are
preserved. Host source is committed `67d031222073f23ad437b45053156e229f86a016`;
Testkit source is `a27967101b68e111f9a717e27977fb41c2fd1818`.
Shared Tooling remains 97 files at `4e274a2219c0b0cc3af68ec65658b373253518fb`.
The earlier Shared adoption retains its original Host 0.8.2/Testkit 0.25.2 graph.

Raw attempts, the exact locked graph/checksums, 936 final input hashes, artifact
copies and `qualification.json` are retained in `.tmp/multi-user0190-01/`.
Every final input hash remains unchanged across qualification. Compiled package
identity is 0.18.5, distinct from the pending release documentation.

- 563 native cases pass: service, contracts, CLI and probe binaries. The first
  attempt retains three sandbox localhost-bind failures; the permitted retry and
  final graph check are separate logs.
- The intermediate full standalone suite passes 73 enabled cases, with 26 opt-in
  cases ignored. Its exact-init decoder was subsequently tightened; it is not
  relabelled as qualification of that later source. That phase's graph was not
  independently frozen. Final focused checks bind the current inputs explicitly.
- 17 final PocketIC cases pass: two separately signed users under one signed
  project, cross-user/tenant refusals, quota/exposure/replay/fence checks, signed
  assessment, exact-init budgets, CLI carrier/Candid parity, two frozen-image
  hard cuts and storage exposure/actual certificate recovery.
- Strict affected Clippy passes after simplifying the two-signer test's repeated
  signed-update calls. Its initial 113-line refusal is retained separately.
- Actual Rust 1.88 checks pass for core/contracts/standalone native and Wasm.
  The real SDK's 1 KiB offline preparation, original snapshot/browser binding,
  repeat and corrupt-source refusal pass with Node 24.21.0/npm 12.2.0.

The frozen single-uploader image is actually compiled as 0.18.4, SHA-256
`50429e44ecbd974213ad844b51a36aeb61b6a7a9a441f33f2406de9cd6897a4e`, from
[its retained source/graph record](host071-tooling0185.md), not asserted to be
the tagged binary. Its upgrade refuses with `Binding(Format)`; every stable
byte, old configuration, exact admissions and conservative accounting remain
unchanged and accessible. The existing pre-allocation-cut 0.14.9 image also
passes its separate refusal/preservation regression. Before/after bytes and
raw trap/readback evidence remain in fresh `upload-authority-cut/` and
`allocation-cut/` directories. Retired images never replace a live owner.

## Limits and consumer acceptance

The signed local case uses fixed test credentials and the explicitly owned IC
emulator root. It establishes the actual standalone ingress/certificate boundary,
not Toko login or `AssetsManage` decisions. Canic owns thin DTO/Candid mounting and
its managed consumer fixture; Toko owns application grants. Browser/provider
acceptance remains separate, so #31 stays open for those remaining obligations.

No full CI/release gate, hosted dirty-source acceptance, commit/push/tag/release,
publication, sibling edit, deployment or paid/live provider effect occurs.
