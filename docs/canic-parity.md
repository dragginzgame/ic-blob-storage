# Consumer integration feedback

Canic owns its wrapper and integration tests outside this repository. This list
records current actions; it does not restore a Canic dependency or adapter here.
The maintainer defers this adoption until useful repository-local work is complete.
No sibling edits or upstream messages have been made.

## Integration feedback

| Action | Required consumer work | State |
| --- | --- | --- |
| Shared installation input | Import `ic_blob_storage::dto::configuration::ServiceInstallationInput`; the former standalone input is removed without an alias. Authenticate installation and validate actual service/compiled release before allocation. | Open; consumer adoption unverified |
| Uploader trust and restricted issuance | Supply explicit immutable trusted uploader and verifier, tenant-approved exact permission and one-tenant/object/reference/1 KiB limits; adopt current certificate blockers without a controller fallback. | Open; consumer adoption unverified |
| Memory composition and lifecycle | Resolve one ic-memory package identity across the host, framework and database. This library now resolves 0.15.2. Supply explicit grants, restore synchronously and preserve every owner's fence; framework activation cannot establish freshness. | Open; consumer composition unverified |
| Browser project and history | Supply complete reviewed installation.candid to native upload-inputs; its proposed service/namespace/project/trusted uploader must match. Supply project/bucket in the original native upload-input JSON and immutable certificate binding; transfer derives them from that binding. Choose a project matching installation and accepted by the SDK HTTP-header check, preserve owner/project/root through both downloads, and use the maintained bounded journal without automatic retries. | Open; live consumer environment unselected |
| Cancellation and transport limits | Preserve explicit reference-release intent when publication is cancelled: tenant withdrawal still permits reconciliation of already exposed/stored bytes. Late confirmation must be followed by exact release; historical attestation replay does not reactivate it. Uploads require HTTPS, request-stream support and HTTP/2 or HTTP/3; trusted fetch hooks must preserve the one-shot stream without buffering or retry. Browser journal budgets bound fetch dispatches, and cannot guarantee wire attempt counts or provider replay charges. | Deferred; consumer cleanup/transport review unverified |
| Gateway trust and native HTTP2 | Keep uploads, verification and tenant reads bound to the same reviewed owner/project/gateway. Native clients must enable HTTP2 explicitly and preserve normal platform TLS validation; the CLI now does so independently of harness feature unification. Anonymous source-listed gateway TLS/CORS metadata does not establish authenticated streaming, provisioning or economics. | Deferred; live bindings and consumer validation unverified |

Local library/standalone evidence and next steps are in the
[current handoff](status/current.md#consumer-integration-feedback). These actions
do not gate library publication or authorize live provider effects.
