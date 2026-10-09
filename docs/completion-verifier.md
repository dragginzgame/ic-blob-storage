# Completion verifier for a browser application

This reference uses an **application-operated private native worker**, running
the existing `blob-storage` CLI. The application operator owns the worker,
its signer, availability and evidence retention. Blob owns permission/content
validation and attestation acceptance; the browser uploads bytes and reports
progress. A successful browser PUT cannot complete an upload by itself.
This implements the operational recipe portion of
[#33](https://github.com/dragginzgame/ic-blob-storage/issues/33); a deployed worker
and two-user application/provider acceptance are still unqualified.

## Configure the worker before installation

Choose a dedicated verifier identity and install its principal as the immutable
`completion_verifier`. Keep its PEM only in the operator's private worker key
store, outside browser bundles, upload inputs and retained run artifacts.
Provision and back up that store under the application's normal secret policy.
Changing the installed verifier requires the supported retirement/reinstall
procedure; an unavailable worker must not be replaced with browser self-attestation.

Configure the exact IC API origin, Blob service principal, service namespace,
verifier principal and explicitly approved provider gateway origin on the worker.
Bind these to the deployment record, including the provider owner/project and
tenant mapping. Jobs do not choose these trusted settings. The signer must match
the installed verifier and all service/namespace bindings must agree.
Provider read budgets and worker concurrency remain operator policy; the CLI's
byte/deadline limits do not establish provider charges.

Use the reviewed CLI binary and a private evidence parent directory on storage
that survives worker restarts. Only the worker can write observation/submission
artifacts. Local hashes protect against accidental corruption; they do not
authenticate a directory rewritten by another principal.

## Run one exact permission through existing commands

The application backend authenticates the user and correlates the job with the
exact Blob service, tenant, original permission and prepared manifest. It passes
the original binary `permission.candid` through a bounded job input. The worker
does not accept a browser's replacement digest, root, gateway or tenant assertion
as authority. The signed Blob verification plan supplies the installed
owner/project and original declaration; a conflicting job must stop.
The application-owned multi-user delegation boundary remains
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31).
Its [project grant boundary](multi-user-upload-authority.md) is implemented in
0.19.0 and remains separate from verifier trust. Canic/Toko grant integration and
the real two-user application/provider journey still require acceptance.

For each admitted permission, select a new observation directory and run the
[existing observation command](operator-guide.md#observe-provider-content).
It authenticates the verifier, obtains the original plan and independently reads
the complete provider object. It rejects redirects, encoded/partial responses,
truncation, excess bytes and wrong original root/metadata. Only a complete
successful observation produces the saved statement. Local `verify-upload`,
browser progress and a previously saved body cannot substitute for this read.

Inspect the completed observation, then run
[submit-attestation](operator-guide.md#submit-an-attestation) once against that
same service/namespace with the same signer and original run directory. The CLI
claims and syncs exact submission intent before dispatch. Keep the original
permission, observation, signed request and any accepted receipt correlated in
the application's job record. Report completion only from the accepted statement
or its exact historical receipt; command exit status alone is insufficient.

## Restart, uncertainty and availability

Retain failed and interrupted directories separately. A partial observation is
not a statement; no upload or completion update is issued from it. Another read
needs a separately approved budget and new run directory, preserving the failed
attempt. Do not overwrite files to turn an incomplete observation into success.

If attestation dispatch is pending/uncertain or the worker restarts after claiming
it, use the existing `upload-attestation` lookup with the original saved statement
and independently configured verifier. A matched receipt recovers that exact
acceptance; conflict fails. Absence, ingress expiry or an empty claim does not
authorize deleting the claim or resubmitting. Preserve the original operation
and follow the existing reconciliation contract. No second retry engine or
distributed exactly-once guarantee is supplied by this recipe.

Monitor worker availability and pending jobs. During an outage, show uploads as
awaiting verification; preserve exposed-upload/provider obligations. Do not mark
them ready, delete provider objects or manufacture receipts. Application asset
publication follows the accepted Blob reference, under the application's separate
release transaction. Verifier observation does not prove future availability,
physical deletion or billing cessation.

## Acceptance still required

Use two actual application users with authenticated tenant/job bindings. Verify
success, corruption/truncation/metadata mismatch, foreign service/tenant/permission,
conflicting statements, interrupted reads and lost attestation replies. Confirm
recovery keeps the original upload and cannot create a duplicate reference.
Record signer provisioning, worker restart/availability and retained evidence.
Local substitute tests, deployed-provider observations and application acceptance
remain distinct; this recipe performs none of those live effects.

The [two-user standalone browser record](evidence/multi-user-browser0192.md)
exercises this CLI observation/submission boundary with two Chromium uploaders
and a separate configured verifier. It uses fixed test keys and a local provider;
production signer/worker availability and real application acceptance remain open.
