# Compatible continuation after 0.19.0

Reviewed base: `29bc6e078bb455750d4a3435178885fc4808a562`, released 0.19.0.
Package versions and release receipt remain unchanged. The pending 0.19.1 notes
cover compatible tooling/dependency maintenance and a provider contract review;
no public service or durable schema changes occur.

## Selected inputs

The clean isolated Shared Tooling source exports the existing 97-file selection
at committed `9af82393c620e486578febed74a648523725c234` (0.1.31). The sibling's
newer dirty changes are excluded. Installer reuse compares all validated host
records while retaining exact installation pins/receipts; comment and row-order
changes do not download again. Versions, checksums and active links still refuse
drift. The IC matrix and active executable identities remain unchanged.

Preserved incoming lock changes select Metrics 0.2.16, TOML
1.1.8+spec-1.1.0 and parser 1.1.5+spec-1.1.0. Released HEAD already contains
all four Host 0.8.4 packages, despite the preceding implementation handoff's
0.8.3 development graph. Testkit 0.25.3, Memory 0.31.8 and PocketIC 16.1.0
remain selected. Earlier qualification retains its own source/graph; it is not
relabelled with today's inputs.

## Focused local qualification

Raw capture: `.tmp/maintenance0191-01/qualification.json` and companions.
Explicit locked/offline cache preparation succeeds without changing the lock.
Snapshot/declaration/offline IC checks, shell lint and installer regressions
pass. The installer fixture also passes under Linux-built Bash 3.2.57;
simulated Darwin assets are not native macOS qualification.

`make test-native-host` passes 242 selected cases: CLI/probe, contracts/examples,
actual standalone installation/Candid and Metrics restoration. The first
sandbox attempt passes 107 CLI cases but fails three local socket binds with
`PermissionDenied`; its complete log remains separately retained. The permitted
loopback run passes on the same selected graph.

Thirty-four native gateway cases pass, including actual-caller/scope checks,
independent restore fences and broken-root-index refusal. Three additional
existing standalone PocketIC cases pass: exact certificate CLI intent/refusals,
installed-verifier/exposure/restore authority, and two distinct signing keys
using direct ingress under one signed project tenant. These are local IC results,
not real Toko sessions or a deployed Caffeine journey.

Rust 1.88 checks core/contracts, standalone, CLI and the private integration
harness on native targets, plus core/contracts/standalone on Wasm. Strict
affected all-target/all-feature Clippy passes. The 841 frozen build inputs remain
unchanged across these final checks. Tested CLI/Wasm/harness hashes are retained;
compiled package identity is 0.19.0. No full local CI/release gate runs.

At the final hosted observation, released Blob's exact CI remains successful.
Shared 0.1.31's ordinary CI is still in progress. Its separate
[Cargo-install assessment](https://github.com/dragginzgame/shared-tooling/actions/runs/37891957851)
passes Linux and fails both macOS receipt checks after installation; the owner
already records the Cargo `dev`/`debug` profile-spelling correction for pending
0.1.32 in [Shared #65](https://github.com/dragginzgame/shared-tooling/issues/65).
That assessment script/workflow is not in Blob's selected snapshot or gate;
dirty owner corrections are not imported. Neither this failure nor local receipt
replay supplies native qualification of the corrected owner assessment.

## Provider and consumer boundary

The [GC review](../gateway-gc-contract.md) and its
[ledger capture](caffeine-probes/local/2026-10-09-gc-contract0191-01/summary.json)
record authoritative application-side sources and synthetic Candid controls.
The source bytes are unchanged from the earlier review. A separate web cache
miss is retained as a failure, rather than reported as source evidence.
Missing/rejecting/malformed callback retention and billing semantics are absent
from these sources. A root-only prune acknowledgement cannot authorize physical
or financial settlement without the provider's meaning and original correlation.
Keep [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) open.

The verifier recipe now reflects implemented 0.19.0 project grants. Canic/Toko
still own mounting, session/capability checks, signer/worker operation and the
actual two-user browser/provider/publication journey under
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#33](https://github.com/dragginzgame/ic-blob-storage/issues/33). No sibling file,
live provider call, payment, deployment, commit, push, release or publication
occurs. No Rust function, method or type is removed.
