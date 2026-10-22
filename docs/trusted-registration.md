# Trusted registration: 0.41 hard cut

Certificate issuance registers retention intent and the first live reference
**before** returning the Caffeine upload certificate. This restores historical
Canic's trust model, with Blob's tenant isolation, bounded accounting and recovery
safeguards. The maintainer approved this change on 2026-10-22.

## Contract

Admission reserves one exact tenant/service/namespace/object/incarnation/request,
root, length, uploader and deadline. The uploader prepares the original manifest.
Certificate issuance rechecks that permission, current tenant generation,
revocation, expiry, service identity, namespace and host eligibility. It commits
registration, its first reference and both usage totals synchronously. A trap
rolls back the update. Registration releases the *pending reservation* slot, but
retains all logical, physical and liability charges and lifetime identity history.
These slots do not bound transfers that continue after certificate issuance.

`UploadState::Registered` means that retention intent was recorded. It does not
prove transfer success, provider availability, physical storage or future retention.
A live reference authorizes a read attempt; the download path still checks the
original root, exact length and metadata against the bytes actually received.
Corrupt, truncated and overlong responses fail without publishing partial output.
Applications own the decision to publish their asset and handle an unavailable file.

There is no verifier principal, worker, attestation receipt or completion callback.
Registration is automatic within certificate issuance, not a new public “mark
uploaded” endpoint. Controllers and operators cannot issue another tenant's
certificate. Revocation cannot undo an escaped certificate or erase liabilities.
Repeated certificate requests refuse; they do not recreate references. A lost
reply must be inspected using the original signed request and retained journals.
Registration alone cannot reconstruct the certificate reply or authorize a resend.

Releasing the final reference queues logical deletion. Provider deletion and
billing cessation remain separate, authenticated host facts. The unresolved
provider gateway semantics in [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32)
and funding acquisition in [#34](https://github.com/dragginzgame/ic-blob-storage/issues/34)
are not supplied by this trust-model change.

## Consumer changes

- Remove `completion_verifier` from installation candidates, inputs and readback.
- Replace upload-state `Confirmed`/`ExposurePossible` handling with `Registered`.
  Content history uses `Live` and the existing deletion/settlement phases.
  The active-upload filter now returns unregistered reservations only.
- Remove calls to `blob_verification_manifest`, `blob_verification_plan`,
  `blob_attest_upload` and `blob_upload_attestation`. They no longer exist.
- Remove CLI `observe-upload`, `submit-attestation`, `upload-attestation`,
  `--verifier` and `--verifier-identity`, and the session `verify` control phase.
  Keep `verify-upload` for local body integrity and `download` for verified reads.
- Publication maps authenticate the installed configuration, enrollment and exact
  live first references. They explicitly report `provider_upload_verified:false`.
  They are retention observations, not provider availability checks or leases.
  Browser transfer and lost-reply journals remain with the original transport.

Owned native Wire frame identities change from `0036` to `0041`. Update generated
bindings and all callers together. The external Caffeine certificate method name
and Candid response remain unchanged. Private fixture/control schemas change too.

The installation discriminator and persisted registration schema change.
Cross-release transitions require [retirement/reinstall](retiring-installations.md).
No compatibility reader, dual mode or automatic migration is provided. Old saved
runs need their original tools; do not reinterpret their completion evidence or
retry bytes with this release. Same-release synchronous restoration still opens
fenced and validates every retained lifecycle/reference/accounting row.

## Evidence limits

The old Canic parent `d9d70322ade2ba6fbd17520acf93acad66dcf4aa` registers a live root
in `crates/canic-core/src/api/blob_storage/lifecycle.rs::create_certificate`, before
upload. That source review explains the chosen trust boundary; it is not evidence
of deployed Caffeine deletion, billing or availability. Earlier verifier-based
captures remain historical evidence for their recorded versions. Current local
validation and its gaps are recorded in [the handoff](status/current.md).
