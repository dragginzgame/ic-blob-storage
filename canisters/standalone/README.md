# Standalone host

`ic-blob-storage-canister` explicitly owns the canister endpoints, lifecycle and
ic-memory grants. The shared library still exports no endpoints or lifecycle hooks.
This is an initial host, not yet a qualified Caffeine-backed storage service.

Build with `make build-standalone`; run its local lifecycle cases with
`make test-standalone`. The Wasm is
`target/wasm32-unknown-unknown/release/ic_blob_storage_canister.wasm`.
These commands do not deploy or contact Caffeine.

All owned methods and standalone lifecycle inputs use bounded IC Wire. See [the protocol](../../docs/service-wire.md)
for the exact per-method contract; no codec detection or fallback is provided.
The [provider Candid contract](service.did) describes only
`_immutableObjectStorageCreateCertificate` and is generated from its declaration:

```sh
cargo run --offline --locked -p ic-blob-storage-canister --example export_candid > canisters/standalone/service.did
```

Wasm builds omit the Candid extraction pointer and type-export machinery. Use
the native example above to export the provider declaration; no extraction-only
Wasm build is needed. The former `provider-candid-export` feature is removed.
The interface test compares the provider declaration with `service.did` and
checks every production method name/mode against the shared endpoint catalog.
It also rejects the extraction export in production Wasm.

Production endpoints use the published Platform selected by the workspace lockfile,
without a legacy Platform dependency.
Host inspects compiled function exports to verify names and modes independently
of the native provider schema. The deployment dependency guard rejects legacy
Platform/CDK and native tooling in the normal/build Wasm graph.

Installation takes one explicit
`ServiceInstallationInput { configuration, project }`
from `ic_blob_storage_contracts::dto::configuration`. The offline
[`installation-check`](../../docs/operator-guide.md#check-installation-inputs-offline)
command produces its complete method-bound `installation.wire` packet and hash; no manual wrapper is needed.
Initialization accepts the frozen positional schema in a `blob_install` request
frame. Wrong identities, malformed values and trailing bytes refuse before owner
allocation; named-field extension or unknown-field skipping is not supported.
`configuration` is the shared `ServiceConfigurationInput`, including the actual
service principal, operator, payer, namespace and all resource/billing bounds.
`project` is the explicit Caffeine project mapped to that namespace. It is immutable
and retained in the current v1 configuration record. It must contain 1–256 UTF-8
bytes, with no controls or surrounding whitespace; these are local representation
limits, not proof of provider assignment. Owner is the actual service, never the
payer or tenant. Validation precedes allocation and runs again on restoration.
Cross-release transitions are reinstall-only, without migration or fallback.
Host observation types come directly from
`ic_blob_storage_contracts::dto::configuration::{HostConfigurationView, HostFailure}`;
the standalone crate exports only its provider Candid interface helper.
Management-canister installation authorization remains the platform's responsibility.
There are no deployment defaults, provider namespace provisioning or account changes.
Operator-only `blob_configuration` returns the installed values, package release
and restore fence, including the retained project; an unfenced owner does not imply
provider readiness. Future upload dispatch must use this same provisioned mapping.

`blob_upload_certificate_assessment(root)` is an uploader-only query over the
original retained permission. The canonical root locates that permission; it grants
no authority. Prepared, locally eligible uploads return the exact permission,
assessment time and all missing prerequisites. Unknown roots, wrong callers,
unprepared, expired, revoked or restored work refuse. Inspection changes no state
and never reserves a later issuance attempt.

The native [`blob-storage certificate-assessment`](../../docs/operator-guide.md#certificate-assessment) command signs
this query as the original uploader and binds the reply to a saved binary Wire document
permission. It reports all missing prerequisites and distinct refusal codes without
calling the certificate update or a provider. An empty blocker list remains only
an observation; issuance always rechecks current authority.

`_immutableObjectStorageCreateCertificate(root)` is the matching synchronous update.
It delegates to the shared certificate/exposure handler and traps on refusal;
errors are never encoded as successful provider replies. The maintainer accepted
the [configured contract](../../docs/standalone-trial.md): issuance requires the
exact project-approved uploader permission, local namespace, an admitted/prepared
reservation within the validated installation quotas and a durable current owner.
Larger and multi-file installations use their configured limits. Tenant permission, manifest,
activation, expiry, phase and restore fences are still checked. One committed
issuance leaves possible exposure and can never be reissued after a lost reply.
Each enrolled project admits exact permissions naming individual uploader
principals. The actual certificate caller must match the retained uploader.
Anonymous and management uploaders refuse at admission; controller/operator
status grants no implicit uploader authority. No global uploader list is installed.
No ingress-supplied qualification flags or operator override exist. These are local
checks: provider provisioning, spending caps and escaped-certificate replay charges
remain unproven. No paid trial or account/deployment action follows from installation.

Certificate issuance registers the original root and its first live reference
atomically. No verifier principal, verification-plan endpoint or attestation is
required. Registration means retention intent, not uploaded or available bytes.
Tenant reads still verify the actual content against original metadata and root.
See [the 0.41 hard-cut contract](../../docs/trusted-registration.md) for removed
APIs, Wire identities, original-request recovery and retirement/reinstall rules.
