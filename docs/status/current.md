# Current status

Date: 2026-10-04

## Released baseline

Released **0.14.1** is at `6c9875f`, with validated source
`53a4bd9eac712db4770f3ca15cbaa014490ca605`. Read
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

Released 0.14.0 adds native next-phase guidance, the callable Chromium driver
and explicit native subprocess control. One exact current complete map, matching
native final report and exit zero establish driver completion. Native Rust owns
authenticated completion/references; the existing browser journal owns paid claims.
No second dispatch/retry owner, persisted cursor or asset-registration owner exists.
All Cargo members inherit root package/dependency versions and local paths.
The [guidance](../evidence/caffeine-probes/README.md#native-phase-guidance-and-browser-driver--2026-10-03),
[native pipes](../evidence/caffeine-probes/README.md#private-native-subprocess-control--2026-10-04)
and [completion](../evidence/caffeine-probes/README.md#one-publication-completion-boundary--2026-10-04)
records retain exact artifacts, checks, failed attempts and local qualification limits.
The release fixes the stale installation-test package-version inequality; actual
release readback, wrong-service refusal and unchanged state remain checked.

Released 0.14.1 groups the
[distinct-PNG](../evidence/caffeine-probes/README.md#distinct-png-publication-and-verified-decoding--2026-10-04),
[multi-chunk/reference](../evidence/caffeine-probes/README.md#multi-chunk-png-and-overlapping-references--2026-10-04)
and [direct browser-delivery](../evidence/caffeine-probes/README.md#direct-browser-url-delivery--2026-10-04)
checks. These authored local fixtures qualify exact verified bytes, lost-reply
recovery without another PUT, tail-corruption refusal and overlapping-reference
cleanup. Logical release refuses descriptors while preserving physical/liability
bytes; saved public URLs still serve under the substitute. Their original
observations use retained 0.14.0 CLI/Wasm binaries, not a live 0.14.1 installation.
The current 0.14.1 lock selects ic-memory 0.24.5 and ic-testkit 0.14.4; historical
records retain their own exact graph/artifact identities.

## Current work — 0.14.2 draft

The tooling fix adds the existing `deps` target first in the shared CI/validate/
release-verify gate. Locked fetching may use the network, fails before validation
or version mutation and selects no new versions; compilation/tests stay offline.
Scoped `make check` or direct `cargo --offline` still require `make deps` after
dependency changes/cache removal. Isolated actual-Make/Cargo tests cover empty
cache preparation and failed fetching without release-file changes or lost build
artifacts. The real scoped gate (`make ci CI_TARGETS='deps check'`), release-helper
suite, Bash syntax and ShellCheck pass. Full CI has not been rerun.

The [image/CSP evidence](../evidence/caffeine-probes/README.md#png-image-loading-under-csp--2026-10-04)
adds ordinary anonymous PNG loading under an explicit local policy. Allowed images
decode with readable canvas pixels; blocked images report enforced `img-src`
refusal with zero provider GETs. A single fixture helper owns media expectations,
decoding/fetch sampling and these image checks, consuming canonical native URLs.
Native whole-content verification, paid-claim journals and completion/reference
owners remain unchanged. Private bounded request traces survive assertion failures.

All six final local IC/browser journeys pass: small/multi-chunk completion and
lost-reply recovery, tail-corruption refusal and original synthetic restart.
Successful large cases make five PUTs/ten GETs; small cases four PUTs/nine GETs.
No recovery adds a PUT. The initial attempt's origin-only CSP-event assertion
failed; Chromium correctly reported the full canonical URL. Its source/bundle,
log and request trace remain preserved alongside the corrected final checks.
All seven attempts, original profiles/journals and exact bytes remain under
`.tmp/media-csp-01`; the public intent/summary and private source/artifact/result
manifests retain their hashes. Owned processes have exited and temporary TLS keys
are removed; evidence and build artifacts remain.

Fresh CLI/Wasm/harness/browser builds, syntax and Rust formatting pass. Two
installation/Candid cases pass with independently expected compiled release
0.14.1, exact configuration readback, controller denial and wrong-service refusal.
These fresh artifacts use ic-memory 0.24.5 and ic-testkit 0.14.4. Cargo and the
release receipt remain 0.14.1; the undated changelog groups this work under 0.14.2.
No full CI, commit, version mutation, publication, deployment, live provider
request, paid cycle, sibling edit/message or build cleanup occurs. Authored local
policy/PNGs do not establish consumer adoption, real asset transactions or deployed
Caffeine serving/cache/CSP/retention/deletion/billing guarantees.

## Remaining product work

- Consumer adoption of the callable driver and native subprocess helper, with
  explicitly selected native/browser identities, binary/trust inputs, same-release
  restart and exact original history retention.
- Consumer acceptance with real media, complete asset transactions, overlapping
  references and MIME/CORS/cache/CSP/retention behavior. Local substitutes,
  synthetic bytes and authored PNG fixtures do not qualify those guarantees.
- Provider deletion and final billing evidence; surviving inventory and independent
  freshness for any proposed older-backup activation. See
  [service gaps](../service-gaps.md) and [the contract](../service-contract.md).
- Large-inventory synchronous reopen and browser heap/CDP qualification before
  promising million-object operation. Configured ceilings are not measured scale.

## Consumer integration feedback

Canic adoption remains deferred until useful repository-local work is exhausted.
The [feedback list](../canic-parity.md#integration-feedback) records wrapper/lifecycle,
one-runtime memory composition, current formats/recovery, original preparation
hints, adoption of native guidance/callable driving/subprocess control and the
public URL/access-policy distinction. Siblings remain read-only; no upstream
message is authorized.

Consumers must also select and qualify their actual image/fetch origins and CSP
with real assets. The new authored-policy image checks are local evidence only;
the [feedback list](../canic-parity.md#integration-feedback) retains that action.

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
