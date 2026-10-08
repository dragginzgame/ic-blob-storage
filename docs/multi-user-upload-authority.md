# Multi-user upload authority

The maintainer selected project-approved per-upload authority for
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31). Pending 0.19.0
removes the installed single-uploader restriction. This is a public API, CLI,
Candid and installation-format hard cut; released 0.18.5 retains its former
contract. Package versions remain unchanged until maintainer release preparation.

## Reuse the existing authenticated project permission

An enrolled tenant/project principal already calls upload admission with an
exact `UploadAdmissionRequest`: service, tenant, namespace, original operation,
object/reference, root, declared bytes, uploader and exclusive expiry. The shared
parser rejects a caller other than that tenant. The durable owner reserves quota
and preserves this full original permission, rather than trusting later browser
input. The project's application session/capability owner must authenticate the
user and its `AssetsManage` decision before making this call. Blob does not verify
an application session or infer it from controller status.

The existing retained permission is the uploader grant.
Each project may approve two different browser principals through two separately
bound original permissions. A grant applies only to its enrolled tenant, exact
service/namespace, original object/root/operation, size and deadline. It grants
no tenant, operator, verifier or gateway authority. Anonymous and management
principals remain invalid. No global uploader registry, wildcard capability,
new stable journal, browser bearer secret or application authentication dependency
is needed.

Certificate assessment and issuance must use the same retained permission and
manifest. The actual ingress caller must equal its uploader; the installed local
service/namespace and full object binding must match; the tenant and original
operation must remain eligible, unrevoked and unexpired; all existing capacity,
mapping, exposure-intent and restore checks remain mandatory. Certificate
resolution and exposure validate this retained grant before assessing
the independent host prerequisites. There is no uploader-trust Boolean or
service-wide uploader list. Browser input cannot replace the stored grant.

## Revocation and uncertainty

Reuse the tenant's existing exact permission revocation. Before possible exposure,
revocation denies certificate issuance and releases only the releasable reservation.
After exposure or uncertainty it cannot retract an escaped ingress certificate,
erase provider bytes or release continuing accounting obligations. Expiry has the
same limitation. Replays preserve the original operation and permission; they
cannot authorize a fresh certificate/effect under another identity. Restored owners
remain fenced until the existing independent continuity proof permits recovery.

The project controls its own user/session revocation policy. Blob enforces
the retained per-upload permission state; it does not duplicate the application
authentication owner or maintain a separate service-wide user registry.

## Hard-cut surface and acceptance

This changes certificate trust and removes the single installed uploader contract.
Pending 0.19.0 requires cross-release retirement and reinstall.
`ServiceInstallationInput`, `ServiceInstallationCandidate` and `HostConfigurationView`
no longer have `trusted_uploader`; standalone initialization refuses skipped
fields and extra arguments. `UploadIssuerAuthority` and its accessors are
removed. The policy/DTO no longer have `TrustedUploader`, and native
`installation-check` refuses the removed `--trusted-uploader` flag as unknown.
Per-batch tools derive uploader identity from their original upload permissions.
The immutable installation has a distinct format discriminator; the former
single-uploader format refuses without opening or replacing service owners.
There are no compatibility re-exports, dual modes or record migrations.
Workspace versions and release effects remain maintainer-operated.

The [local implementation record](evidence/multi-user0190.md) retains passing
two-signer ingress/certificate, cross-user refusal, permission binding,
expiry/revocation, shared quota, uncertainty and restored-owner checks. Frozen
old-image refusal preserves every stable byte and existing obligations.
Existing verifier trust stays separate.
Canic owns thin mounting/DTO propagation and its neutral managed fixture; Toko
owns real session verification and application capability decisions. Browser and
deployed-provider acceptance must follow separately from local substitutes.
