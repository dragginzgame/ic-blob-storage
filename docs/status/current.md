# Current status

Date: 2026-10-03

## Released baseline

This batch started from released **0.11.0**, at `0d14a26`, with validated source
`cc5c9937e697dd9ca4b9c0d8c7dde73bb916f34b`. Read
[Cargo](../../Cargo.toml), [the release receipt](../release.json) and
[the changelog](../../CHANGELOG.md) for authoritative release metadata.
A repository release does not establish registry publication or deployed behavior.

The core owns tenant policy, permissions/manifests, references, quotas, economics
and durable local journals. Standalone authenticates platform context and delegates
to shared handlers. Linking the library exports no endpoint or lifecycle hook.
Consumer frameworks own their wrappers and composition tests externally.

Configured trusted-uploader issuance and explicit verifier completion are maintained.
Logical release, physical deletion and billing cessation remain separate facts.
Same-release restoration validates every owner synchronously and fences mutations;
only an independent current-execution IC-history proof can resume the current
installation. Older-snapshot activation and cross-release upgrades are unsupported.

Native publication sessions compose retained setup, verification and reference
owners. The browser host/worker and callable Chromium bridge retain the original
profile and certificate/gateway journals; they grant no retry or completion authority.
Native bindings require their exact frozen format and original optional preparation
hints, preserving omission and empty strings. Existing input/state readers have no
compatibility fallback.

## Current work

The simplification batch shares browser UTF-8 bounds and refuses malformed metadata
before journal access. The job helper owns the selected body once, before cloning
metadata. One semantic Candid test replaces the duplicate generated-text check.
The current handoff and roadmap no longer mix historical instructions with active work.

The fixture-only reference save/inspect command, JSON record and append
journal contract are retired. Signed native receipt tests cover its service guarantees;
their journeys retain malformed-ingress checks. Native create-new signed claims
remain authoritative and old artifact files are untouched. Tests for the retired
save/lock mechanism are removed, with no replacement abstraction or reader.
The harness now inherits the workspace MSRV after removing its file-lock requirement.

Focused browser, semantic Candid, native claim/operator and signed reference
recovery checks pass, together with strict affected Clippy and Rust 1.88 harness
compilation. Exact scopes and retained evidence are recorded in
[the local ledger entry](../evidence/caffeine-probes/README.md#simplification-follow-up--2026-10-03).
Concurrent ic-memory/ic-testkit dependency updates are preserved and their tested
identities are recorded in the local evidence; they are separate work.
No version mutation, full CI, deployment or provider effect
is included. The fixture command removal belongs in a minor release.

The format follow-up replaces reused numeric markers with one frozen identity per
current layout: `ic-blob-storage/upload-inputs:original-preparation` for native
bindings and `ic-blob-storage/installation:platform-anchor` for immutable stable
installation records. Current producers and single/batch/session readers converge
on these contracts. Exact release and service checks remain independent. A missing
or mismatched identity refuses without repair, replacement or effect redispatch.
The host grant key still names the same slot, so old allocations are not hidden.

This is a minor native/stable-format hard cut, not a deployment or upgrade plan.
Preserve retained snapshots/journals and original binaries. Existing installations,
including any 0.11.0 deployment, cannot upgrade to this format; retirement and full
obligation disposition precede a fresh reinstall. Consumer adoption and deployed
inventory remain unverified. No V2, dual reader, journal conversion or migration
engine is added. Native preparation, frozen-batch/journal checks, strict affected
Clippy, offline SDK/native handoff, semantic Candid and actual local IC lifecycle/
session recovery pass. The [format evidence](../evidence/caffeine-probes/README.md#frozen-format-identities--2026-10-03)
records exact scopes and unchanged retained-artifact hashes under `.tmp/format-identity-01`.

The descriptor follow-up removes the forwarding public `describe` API, the second
operational view and an HTTP target that the service response discarded. The shared
`handle` checks ingress identity and calls the upload owner's existing operational
checks, then presents the retained descriptor under the same trusted borrowed scope.
Clients retain canonical target construction. Historical inspection remains distinct
from active, unfenced serving. Native/stable frozen layouts and endpoint wire shapes
are unchanged; this public source API cut joins the current minor draft. Twelve
focused unit cases, nine actual local IC cases, semantic Candid equality and strict
core/standalone/harness/CLI Clippy pass. The [descriptor evidence](../evidence/caffeine-probes/README.md#descriptor-serving-owner--2026-10-03)
retains the exact tested artifacts and logs under `.tmp/descriptor-owner-01`, and
explicitly records the missing pre-run intent file. Consumer source adoption remains
unverified; no live provider request or paid effect occurred.

## Remaining product work

- Durable native parent coordination around the session and Chromium bridge:
  retain signer/profile/origin and original setup/verification sources before
  effects, then reconcile exact surviving evidence on restart without redispatch.
- Consumer acceptance with real media, complete asset transactions, overlapping
  references and MIME/CORS/cache/CSP/retention behavior. Local substitutes and
  synthetic image-labelled bytes do not qualify those guarantees.
- Provider deletion and final billing evidence; surviving inventory and independent
  freshness for any proposed older-backup activation. See
  [service gaps](../service-gaps.md) and [the contract](../service-contract.md).
- Large-inventory synchronous reopen and browser heap/CDP qualification before
  promising million-object operation. Configured ceilings are not measured scale.

## Consumer integration feedback

Canic adoption remains deferred until useful repository-local work is exhausted.
The [feedback list](../canic-parity.md#integration-feedback) records wrapper/lifecycle,
one-runtime memory composition, current recovery and original preparation-hint
adoption. Siblings remain read-only; no upstream message is authorized.

The old isolated owner remains frozen at 0.6.0 with stopped original history;
the separate live owner was last verified at 0.7.0. Both retain exhausted lifetime
capacity and original provider/billing obligations. Do not reset or upgrade them
as part of source cleanup.

## Evidence and history

[The probe ledger](../evidence/caffeine-probes/README.md) distinguishes source review,
offline checks, actual local IC/browser execution, substitutes and deployed facts.
The [implementation archive](history.md) retains original commands, failures,
source/artifact identities and superseded next-step notes. Its historical release
and dependency statements must not override this handoff.
