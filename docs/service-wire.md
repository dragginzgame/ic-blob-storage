# Blob-owned service protocol

The implemented surface uses IC Wire for tenant inspection/update and bounded
CBOR for the other 29 owned methods and standalone lifecycle inputs. The catalog
and endpoint emitters agree on this mixed surface. Completing Wire conversion
would be a coordinated protocol change, not deletion of an unused CBOR codec.
The root manifest selects published IC Wire 0.4; the lockfile owns its exact
selection. Initial candidate-source results remain
[historical evidence](evidence/icwire0320.md), not current registry qualification.

Each endpoint accepts one declared codec with no detection or fallback. Former
Candid requests refuse at owned methods, and CBOR tenant frames refuse too.

## Boundaries and ownership

| Boundary | Encoding |
| --- | --- |
| `blob_tenant` and `blob_update_tenant` request/reply | Bounded IC Wire, method- and direction-bound Postcard frame |
| Other 29 `blob_*` methods | One complete CBOR value |
| Standalone initialization | One CBOR `ServiceInstallationInput` record |
| Same-release standalone post-upgrade input | CBOR unit/null (`f6`), with no replacement configuration |
| `_immutableObjectStorageCreateCertificate` request/reply | Existing provider-defined Candid contract |
| Outgoing Cashier/provider and IC management calls | Existing external Candid contracts |
| Existing stable records and accounting journals | Existing persisted encoding; this cut adds no migration |

`wire::tenant::{Inspect, Update}` own the two IC Wire contracts. Their derived
bounded projections retain explicit field/variant IDs and `Bytes<29>` principals.
Each direction admits at most 512 bytes, 1024 bytes of value heap plus codec
scratch, 32 nodes and depth 8; the actual schema maximum can be tighter. These
codec allowances do not retroactively bound the CDK's initial transport buffer.
Adapters convert admitted values into the existing native DTOs, then run unchanged
scope, actor, tenant and compare-and-set policy. A valid frame grants no authority.

IC Wire's `IW` header binds an exact u64 method identity and direction (request 0,
response 1) before canonical Postcard decoding. Wrong method/direction, nonminimal
values, trailing bytes, invalid variants and excessive principal lengths refuse.
Retain original encoded requests for recovery; do not regenerate uncertain effects.
Response encoding failure traps at the host boundary, preserving synchronous IC
rollback without promising rollback of effects committed before an external await.
The two public Method types can use IC Wire's typed buffers and metadata directly.

`ic_blob_storage_contracts::wire::{encode, decode}` remains the codec owner for unconverted methods, using the
already selected Ciborium implementation. `endpoints::SERVICE_METHODS` exposes
method name, query/update mode and required codec from the same catalog used to
mount endpoints. Linking either library claims no endpoint or lifecycle ownership.
`service.did` now describes only the provider certificate method, not the whole
Blob application interface. Default production Wasm omits Candid extraction;
`provider-candid-export` enables extraction of that provider subset for tooling.

## Remaining CBOR representation and admission

This section describes only the remaining CBOR methods and lifecycle inputs.
The IC method name and signed transport envelope remain unchanged. The application
argument contains the CBOR value directly, without an outer Candid `blob` or
one-element argument tuple. No-argument methods receive unit/null. Responses are
also direct values, including Serde's externally tagged `Result` and enum layouts.

Records use text field names; arrays and vectors preserve order. Principals use
Serde's binary principal representation, with platform identity and service
binding checked separately. Fixed-width integers retain their complete range;
128-bit values beyond CBOR's native 64-bit range use standard bignum tags.
Provider account-report `Int`/`Nat` fields use canonical decimal text, preserving
arbitrary width and sign. Alternate spellings such as a leading plus, leading
zeroes or negative zero reject. Hash algorithms and provider certificate bytes
are unchanged. This format does not promise canonical encoding of every accepted
CBOR value; save original signed bytes and operation identity for recovery.

The standalone CBOR adapter admits at most 4 KiB for ordinary inputs, 16 KiB for
installation and 128 KiB for manifest preparation. Byte admission precedes typed
decoding. The codec bounds collection nesting to 32 and refuses trailing input;
DTO deserializers refuse unknown and duplicate fields. Output is capped at 4 MiB;
clients retain their narrower reply and transport bounds. Domain limits on
headers, leaves, histories, reservations and liabilities still apply independently.
Candid work/type/skipping quotas do not describe CBOR resource admission. Encoded
sizes change: measure the actual CBOR packet against its boundary rather than
reusing a former Candid size estimate.

A decoding failure occurs before workflow effects. A missing, rejected or unusable
reply can still follow a committed update or an external effect. Keep the exact
original intent and reconcile its receipt; codec changes authorize no retry.

## Native preparation and browser handoff

`blob-storage installation-check --configuration proposal.json ...` accepts a
strict JSON configuration proposal. Use integer-preserving tooling for u128
numeric literals. It retains that proposal as `configuration.json`, validates all
bindings and limits, then emits `configuration.cbor`, `installation.cbor` and a
summary with proposal and packet hashes. It does not deploy, allocate memory or contact a provider.

Other service input/output packets now use `.cbor`: permissions, manifests,
reference commands, verification statements, queries and replies. Browser
publication tools carry these permission packets as bound opaque bytes; Caffeine's
certificate request and certified response remain their mandated Candid shapes.
Do not relabel an old `.candid` file or regenerate an uncertain signed operation
in a new codec. Retain old run directories and use their matching released tools
for inspection/disposition. Old records do not become current dispatch authority.

## Host and Canic adoption

Mount owned methods with both custom decoding and custom reply encoding. For
IC CDK adapters, set `decode_with` and `encode_with`, and hide owned methods from
Candid service generation. Use each converted method's IC Wire limits, retain the
remaining standalone CBOR ordinary/manifest limits, and
preserve actual caller/canister capture, auth predicates, method mode, pre-await
intent and post-await continuity checks. The shared declaration callback's
`tenant_inspection` and `tenant_update` categories select the exact IC Wire methods;
`certificate` remains a separate Candid emitter. Export its schema in
the same expansion pass as its registration; an earlier export can miss it.

Controlled tenant callers encode the selected IC Wire method and decode that
same method's response after authenticating the service. The core replicated
tenant client already does this. Other clients still use their current CBOR owner
until converted; the catalog records this explicitly. Never send a response or
another method's request as a tenant command. Canic owns its wrapper macros,
initialization, metadata/export checks and dedicated/embedded integration tests
in its own repository. It must coordinate its endpoint adapters and callers on
this release before claiming integration; Blob's local tests do not qualify Canic.

Cross-release transitions remain retirement and reinstall only. Resolve provider
objects, uncertain effects, balances and continuing billing under the existing
[retirement contract](retiring-installations.md) before reset. Same-contract backup,
restore fences, exact retry identity and receipt inspection remain required.
