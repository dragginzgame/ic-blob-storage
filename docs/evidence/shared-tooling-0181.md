# Shared Tooling adoption for pending 0.18.1

Reviewed 2026-10-08 against Blob HEAD
`114f1b885394941b31388f07bafe6153422efdd9`. Compiled/package versions and the
release receipt remain **0.18.0**; this tooling batch changes no public Rust,
wire, storage or service contract.

## Selected source and ownership

The 81-file snapshot selects committed Shared Tooling
`eeb72e741199bd8574280eacb3542d8379b912f6`: VERSION remains **0.1.24**, with the
committed pending **0.1.25** archive corrections. The former 78-file selection was
`3ecc48e579f6cf6e6ab01a6645d8a250fc8c6934` (0.1.20). A clean private checkout,
with the canonical origin and read-only shared Git objects, exported exact
committed bytes. Dirty sibling bytes were excluded. The source advanced during
review; the initial 0.1.24 snapshot and its checks are retained separately.

Explicit additions are `release-pr.sh`, `archive-evidence.sh` and
`test-evidence-archive.sh` under `scripts/ci/`. The governance roster requires the
PR companion; shared helper documentation requires the archive owner. No shared
copy is patched. The existing CI upload workflow is unchanged; the archive helper
is available but not yet wired into that consumer collector. Its canonical
offline regression is included in the maintained `shared-tooling-tests` target.

Blob retains **direct** release delivery. Actual Make entry points reject a
different inherited or explicit delivery selection before entering the runner;
consumer preflight independently refuses it. PR delivery needs separately adopted
merged-source receipt adapters. Common runner tests own final payload/tag checks,
completed remote reconciliation and tracking-ref races; Blob fixtures own its
receipt, index, publication and early delivery-policy admission. No function,
method or type was removed or renamed in this adoption.

## Focused qualification and retained attempts

Raw inputs, diffs and logs are under `.tmp/shared-tooling-0181-03/`:

- `refresh.log` and `refresh-correction.log`: snapshot refresh/verification,
  including all 81 hashes and executable modes.
- `release-check.log`: Make routing and canonical runner pass, including 15
  real-Git tracking scenarios in private fixtures. The subsequent old Blob Git
  substitute refused the new cached-index command; its failed fixture remains
  at `target/release-tests.p4AVlH`. Production admission was not weakened.
- `adapter-retry.log`: updated Blob adapter suite passes, including all four
  release targets refusing explicit PR/invalid and inherited PR selections before
  Git, validation, metadata mutation or external effects. All commit/tag/push and
  registry effects are substitutes.
- `distribution.log`, `cloc-tooling.log`: changed snapshot distribution and
  tooling LOC fixtures pass. `cloc-siblings.log` retains the missing-cloc failure
  and `/tmp/shared-tooling-cloc-siblings-test.mxBWDx`; the surrounding compound
  command continued to the archive test and returned zero, but log inspection
  caught the failed constituent. `cloc-siblings-retry.log` passes with this
  repository's prepared host-tool PATH; no installation or download was performed.
- `archive.log` retains the original 0.1.24 archive qualification.
  `archive-correction.log` qualifies the committed path corrections: literal
  relative and option-like operands under CDPATH, newline boundaries, parent
  symlinks, existing FIFO refusal, whole-root output rejection, payload bytes,
  modes, final symlinks and retained partial failure output.
- `shell-check.log` and `shell-check-correction.log`: Bash syntax, ShellCheck
  and consumer Perl checks pass. Final snapshot/document links and whitespace
  checks are recorded separately. No full CI/release gate was run.

The incoming staged lock first selected Testkit **0.22.1** / zerocopy **0.8.62**;
it changed externally again to all four Host **0.5.2**, Testkit **0.22.2** and
Memory **0.31.4**, retaining private Metrics **0.2.11**. Both diffs and hashes are
retained. These changes are preserved, not selected by this tooling work. Earlier
native/Wasm evidence remains bound to Host 0.5.1 / Testkit 0.22.0 / Memory 0.31.3;
the new runtime graph has not been qualified by these fixtures.

## Hosted evidence and limits

[0.1.24 owner CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37756978585)
passed Linux, Intel macOS and lint. ARM macOS passed portable/native checks but
failed immediate exact-ID artifact download after upload; the overall run is
failed. [Upstream #63](https://github.com/dragginzgame/shared-tooling/issues/63)
owns that observation, separately from
[archive corrections #59](https://github.com/dragginzgame/shared-tooling/issues/59).
The [selected follow-up run](https://github.com/dragginzgame/shared-tooling/actions/runs/37762726615)
was queued at inspection. Local Linux fixtures do not establish native macOS
round-trip acceptance. No matching hosted Blob result was available for entry
HEAD; these dirty changes have no hosted result.

No real release command, commit, tag, push, registry publication, dependency
upgrade, sibling edit, workflow dispatch or paid provider effect was performed.

## Nested Make correction — 2026-10-08

At Blob source `a505a4cb242facc1c0c2fb37435bea6bae6383cf`, maintainer-run
release validation failed in `delivery-release-patch-inherited`. The original
complete failure log remains
`.git/release-state/validation-failures/20261008T104751Z-3865040-0-ci.log`, with
the failed fixture at `target/release-tests.NXz3mT`.

The parent release runner supplies `RELEASE_DELIVERY=direct` as a Make command-line
selection. Recursive Make inherits that selection through MAKEFLAGS; it overrides
the fixture's `env RELEASE_DELIVERY=pr` refusal input. The substitute release
therefore succeeded when the case required refusal. The earlier standalone
adapter pass did not qualify this nested-Make context.

The consumer fixture now clears MAKEFLAGS, MFLAGS, MAKEOVERRIDES, GNUMAKEFLAGS and
MAKEFILES at entry, matching the existing canonical fixture ownership boundary.
Production Make/runner admission is unchanged. No function, method or type is
removed or renamed.

Raw evidence is `.tmp/release-fixture-env-0181-04/`. `before.log` and
`before.status` reproduce the failure with `MAKEFLAGS=' -- RELEASE_DELIVERY=direct'`
(exit 1); its failed fixture remains at `target/release-tests.Da7nZj`. After the
repair, the complete Blob adapter suite passes under an actual parent Make
invocation selecting `RELEASE_DELIVERY=direct` on its command line (`after.log`,
`after.status`, exit 0). All Git release and registry effects remain substitutes.
Bash syntax and ShellCheck pass. Cargo manifests, lockfile and release receipt
hashes remain unchanged. No full CI or real release was rerun; the maintainer's
complete gate must still validate the repaired committed source.
