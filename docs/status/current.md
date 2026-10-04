# Current status

Date: 2026-10-04

## Released baseline

Released **0.13.0** is at `31c1a36`, with validated source
`ce93a83ef5e3b9b2c83b51d0d512c6682fac189f`. Read
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

Released 0.13.0 retains original native setup/observation/transfer sources, exact
native/browser selection and the profile binding before Chromium access. Native
transfer handoffs are passive provenance; the existing browser journal owns paid
claims. Repeated/recovered handoffs request certificate recovery only. Missing or
changed history refuses without replacement, redispatch or an inherited cursor.
Old contracts stay with their original binaries/artifacts; no conversion exists.
The [source](../evidence/caffeine-probes/README.md#native-session-original-sources--2026-10-03),
[binding](../evidence/caffeine-probes/README.md#browser-profile-launch-binding--2026-10-03),
[selection](../evidence/caffeine-probes/README.md#native-session-browser-selection--2026-10-03)
and [handoff](../evidence/caffeine-probes/README.md#native-browser-transfer-handoffs--2026-10-03)
records retain their exact checks, failures, artifacts and qualification limits.

## Current work

Completed changes are grouped in the undated **0.14.0** changelog draft.
Cargo and the release receipt remain at 0.13.0 until maintainer release preparation.

Native phase events now include derived `next_frame` guidance. Fresh status starts
at index zero; original observation/handoff recovery precedes setup, and only
checked completion/reference evidence advances the cursor. All indexed phases
share one ordering guard while retaining their required unattempted facts.
The Chromium bridge's `driveSession(nativeControl)` follows those native decisions
through the existing transfer/verifier/map owners. It checks current ready/input
binding and phase replies, bounds steps and metadata, excludes concurrent jobs,
and stops pending control/final-exit waits on context close/deadline. Correlation IDs remain
execution metadata; no new dispatch journal, persisted cursor or retry owner exists.

The new `startPublicationSession` helper owns the explicitly selected native
subprocess/private control transport: bounded split/coalesced UTF-8 records, one
outstanding phase, cancellation/deadline, and final report plus checked exit.
The driver now owns completion acceptance: exact current map, original input hashes,
all ordered indices, no blockers, neutral lease/serving facts, matching final report
and exit zero. `native_result` accompanies complete/stopped results. The caller
selects keys/binaries, consumes only a complete driver result and chooses an
explicit same-release restart. Native Rust remains the authority for verification
and references; the browser checks the private peer's passive output contract.
There is no automatic process discovery/restart or upload retry. A complete map
is a current observation, not a serving lease or completed consumer asset transaction.
Service wire, native intent/source/profile and stable layouts are unchanged.

Seven native session unit checks, seven retained native IC cases, twenty actual
Chromium boundary cases and strict affected Clippy pass. The actual IC/Chromium
driver restart completes two files with four PUTs/four GETs and 3,072 retained
physical/liability bytes, without another upload. Corrupt content stops before
attestation or the next file with two PUTs/one GET and 1,024 retained bytes.
The maintained manual verifier/observation recovery journey also passes.
The [driver evidence](../evidence/caffeine-probes/README.md#native-phase-guidance-and-browser-driver--2026-10-03)
retains fresh 0.13.0 CLI/Wasm/bundles, profiles, journals and all validation attempts
under `.tmp/session-driver-01`. These are local substitutes and synthetic bytes.

Ten real child-pipe boundary checks pass, including incomplete output, unsolicited
records, contradictory exit, cancellation and a final report whose child never
exits. The two coordinated IC/Chromium journeys now use the maintained subprocess
helper directly; the Rust fixture only observes results and independent downloads.
They preserve the same request/accounting totals and no repeated upload. The
[native control evidence](../evidence/caffeine-probes/README.md#private-native-subprocess-control--2026-10-04)
retains `.tmp/native-control-01` artifacts and all attempts, including sandboxed
Node output failures and the missing evidence-parent refusal before provider access.
Current configuration guards are pipe-tested; actual IC journeys bind their exact
retained integrated bundle. No endpoint, stable, native intent or profile cut is added.

The maintained driver now accepts one native control handle with ready/phase/finish;
the interim separate caller completion check is removed from current consumers and
examples. Eleven child-pipe and twenty-one actual Chromium boundary checks pass.
Changed/incomplete maps, blockers, lease claims, mismatched/failed final results
and lost final replies refuse; a pending final wait still owns the browser gate
and cancellation reaches the native child. Both direct IC/Chromium journeys pass
with the same request/accounting totals and no repeated upload. The
[completion evidence](../evidence/caffeine-probes/README.md#one-publication-completion-boundary--2026-10-04)
retains exact sources, artifacts, outcomes and original history under
`.tmp/driver-final-01`. The phase driver has no public pre-exit completion state.

Maintainer dependency edits are preserved: ic-memory 0.24.0, ic-testkit 0.14.1
and direct ic-management-canister-types 0.10.0. Core/standalone/CLI checks pass
without adaptation. Fresh standalone Wasm passes all four lifecycle/recovery
cases and the snapshot rollback refusal under the new dependencies. Logs, intent
and Wasm remain under `.tmp/management-types-010`. The recovery owner is the only
production consumer of management types; its request/history decoding and fail-closed
classification still use the maintained platform API. No recovery schema changes.
The publication journeys use unchanged retained 0.13.0 CLI/Wasm and the retained
test harness; their results are separate from the fresh recovery-artifact checks.
No full CI or deployment qualification is claimed.

All eleven Cargo members now inherit the root package version and dependency
declarations. CLI/test-only version pins and local dependency paths have one
workspace owner; members keep their existing features, targets and publish policy.
Eight unpublished fixtures inherit 0.13.0 instead of carrying independent 0.0.0
labels. Before/after Cargo metadata confirms unchanged external packages,
per-member dependency settings and resolved features. Native workspace checks
with all targets/features and Wasm workspace checks with all features pass.
Captures remain under
`.tmp/workspace-manifests-01`; this does not change frozen runtime artifacts.

The installation carrier regression exposed an obsolete assertion that harness
and library package versions must differ. Removed that assertion; installed-Wasm
release readback, independently selected release, wrong-service refusal with
unchanged state and exact carrier installation remain checked. Both installation
module cases pass with the current 0.13.0 artifacts. Reproduction, rebuild and
focused rerun logs remain under `.tmp/installation-version-01`; no full CI run.

## Remaining product work

- Consumer adoption of the callable driver and native subprocess helper, with
  explicitly selected native/browser identities, binary/trust inputs, same-release
  restart and exact original history retention.
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
one-runtime memory composition, current formats/recovery, original preparation
hints and adoption of native guidance/callable driving/subprocess control. Siblings remain read-only;
no upstream message is authorized.

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
