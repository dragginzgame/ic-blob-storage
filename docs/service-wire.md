# Blob-owned service protocol

All 31 owned methods use one IC Wire request/response contract. Standalone
installation and same-release post-upgrade inputs also use method-bound Wire.
The 0.36.0 hard cut rejects old CBOR and Candid service packets,
with no detection, fallback or compatibility lane. The root manifest selects
published Wire 0.5.0 or later in that family; Cargo.lock retains the exact selection. Wire itself
owns no Blob DTO, endpoint, platform dependency or provider transport.

## Boundaries and ownership

| Boundary | Encoding |
| --- | --- |
| All owned service requests and replies | Canonical Postcard in a method- and direction-bound IC Wire frame |
| Standalone initialization | Request frame for `blob_install`, containing `ServiceInstallationInput` |
| Same-release post-upgrade input | Request frame for `blob_post_upgrade`, containing unit; no replacement configuration |
| Provider certificate, outgoing Cashier/provider and IC management | Their existing external Candid contracts |
| Caffeine HTTP upload/download and 1 MiB chunks | Existing provider contract; bodies never pass through the Blob RPC codec |
| Offline permissions, declarations, reference commands and statements | Bounded Wire value documents with identity `0x424c4f420036ff10`; never service-method frames |
| Private consumer/storage-probe controls | Private Wire values using `document`; storage controls retain argument/reply tuple envelopes |
| Signed IC transport envelope | IC transport CBOR, containing the exact application frame |
| Stable records and accounting journals | Existing persisted encoding; no migration |

`endpoints::SERVICE_METHODS` supplies method names, modes and codecs from the
same catalog that mounts endpoints. Linking Core, Contracts or Client exports
no endpoints or lifecycle hooks. `service.did` describes only the provider
certificate method. Wasm omits extraction machinery; the native exporter owns
provider schema generation.

## Schema and admission

The eleven-byte Wire header binds the literal u64 identity and direction
(request 0, response 1) before decoding. The explicit identity table in
[the codec owner](../crates/ic-blob-storage-contracts/src/wire/mod.rs) is frozen;
identities are not computed from Rust names or catalog positions. Unknown and
provider method names refuse.

The two existing tenant contracts retain their finite derived schemas, explicit
field/variant IDs, `BytesRef<29>` principal views and exact original frames. Each direction
allows 512 encoded bytes, 1024 value-allocation bytes plus codec scratch, 32 nodes
and depth 8. Native tenant DTOs are projections through `wire::tenant`; the generic
native helpers do not replace those projections.

Wire 0.5 supplies schema and serialization independently of deserialization.
Generic service/document decoders explicitly require `Deserialize<'de>` and
accept input-backed `BytesRef`/`TextRef`; callers needing independent ownership
can retain `DeserializeOwned`. The finite tenant contracts borrow principal bytes
from the input or native scope. Their public types carry a lifetime and encode
projections convert from references; the existing native helper functions still
return owned frames/DTOs. Field IDs, bounds and canonical bytes are unchanged.

`DownloadResponse<Text = String>` and its headers have one field layout. Native
CLI descriptor decoding uses `download::reply::decode_borrowed` with `TextRef`
into the retained authenticated reply; owned async clients use the same validator
and the default owned text. These are ownership forms of the same current format,
not separate readers for old formats. Persisted records and retained retry frames
remain owned. Standalone upload input remains owned so its ingress frame can be
released before the continuity await: the measured borrowed async experiment
increased cost and retained heap. See [the measured adoption](evidence/borrowed-wire0390.md).

Provider body chunks do not enter a Blob Wire RPC. Caffeine HTTP and async read
transport ownership remain unchanged; generic document callers can use bounded
`BytesRef` for chunk views without adding a new endpoint or provider protocol.

The other 29 methods and lifecycle packets use Wire's existing native Serde API.
Their declared field order and variant order are the frozen positional schema.
Reordering or changing a DTO is a protocol change, even if the Rust names remain.
There are no named-field extensions, unknown-field skipping or optional old-schema
projection. Structs and equivalent tuple projections can have identical bytes;
Serde `deny_unknown_fields` does not add named-field checks to Postcard.
Full-width u128/u64 values, principal bytes, ordered leaves and exact header strings
round-trip. Arbitrary-width provider integers retain their canonical decimal-text
serializer; roots, content hashing and provider certificate bytes are unchanged.

Native requests admit 4 KiB ordinarily, 128 KiB for `blob_prepare_upload`, and
16 KiB for `blob_install`, including the Wire header. Each native traversal admits
at most 64 MiB of allocation charge, 262,144 nodes and depth 32. Responses admit
at most 4 MiB; clients apply their existing narrower reply and transport limits.
These are codec ceilings, separate from domain counts, quotas and liabilities.
The standalone raw adapter checks platform argument size before copying native
request bytes. The existing finite tenant adapter's CDK transport buffer remains
separate from finite value admission.

Wrong method/direction, noncanonical values, truncation, trailing bytes and
exhausted resource budgets refuse before workflow effects. Response encoding
failure traps, preserving synchronous rollback without promising rollback of
external effects committed before an await. A valid frame grants no authority:
actual actor/service capture, tenant policy, bindings and continuity checks remain.

## Native preparation and browser handoff

`installation-check` accepts the strict JSON proposal, validates it offline, and
emits passive `configuration.wire`, complete `installation.wire` and hashes.
Only `installation.wire` is the standalone init argument. Offline upload/reference
preparation retains passive `.wire` documents. Native dispatch encodes the selected
method after validating those documents; a document is never sent directly to an
owned endpoint. Browser fixtures receive Wire bytes for direct service calls and
retain passive permission bytes for the independent certificate-intent binding.

Dispatched application packets use `request.wire`, `response.wire`,
`service-response.wire` and query-specific `-args.wire`/`-reply.wire` files.
`signed-request.cbor` remains the actual IC envelope. Reference submission also
retains passive `reference.wire` for receipt inspection: an apply frame cannot be
relabelled as a receipt request. Observations retain passive `permission.wire`
and `statement.wire` alongside the verification `request.wire`; attestation
dispatch creates its own method-bound `request.wire`. Save original signed bytes for uncertain updates;
never regenerate or redispatch them as codec recovery. Old run directories retain
their original tools and disposition; their packets are not current authority.

## Host and consumer adoption

The standalone host and storage probe mount raw native methods with exact method
selection and manual Wire replies. Tenant emitters keep their finite custom
codecs. Provider registration and Candid export remain in one separate expansion.
Replicated Client, native CLI and installed-test callers use matching method-bound
request and reply codecs. No wrapper may fall back to CBOR when Wire rejects.

Consumer frameworks own their wrappers, installation/lifecycle and integration
qualification in their own repositories. Canic must adopt these native emitters
and callers before claiming this release. Blob's local substitutes do not qualify
Canic or the deployed provider. The 0.38 application trait cut removes public
application DTO Candid implementations without changing Wire bytes. External
provider types retain Candid; historical inputs and persisted private probe
records use frozen local schemas. Production stable records remain unchanged.

Cross-release transitions remain retirement and reinstall only. Resolve provider
objects, uncertain effects, balances and continuing billing under
[the retirement contract](retiring-installations.md) before reset. Same-release
backup, restore fences, exact retry identity and receipt inspection remain required.

## Passive values and saved artifacts

`document::encode` and `document::decode` use published Wire's native codec with
a separate document identity (`0x424c4f420036ff10`, request-direction envelope).
The expected DTO is selected by the CLI command or private control boundary;
this is not a service-method identity or a claim of per-method control binding.
Native values carry no Rust type or field names; byte-identical schemas are
indistinguishable. A unit field contributes no payload bytes. Treat documents as
caller-selected input data and validate their bindings, never as authority.
Document frames and service frames cannot be substituted for one another. The
caller byte ceiling is capped at 4 MiB, traversal depth at 32, nodes at 262,144
and allocation admission at 64 MiB. Full-width integers and principals retain
their values; field/variant order belongs to the frozen native schema.

New offline permission, manifest, configuration, reference, download and
attestation documents use `.wire`. Browser fixtures carry those exact opaque
values from the Rust owner; they define no second codec. There is no Ciborium
implementation or direct dependency in Blob. IC Memory owns its persisted
metadata codec, and IC transport/certificate CBOR remains external.
Never rewrite old saved directories into this format or replay old uncertain
effects. Preserve their original release/tools for reconciliation and retire
installations before the cross-release reinstall. Persistent records and
journals are not changed by this application-protocol cut.

Platform 0.4 owns the application Principal without Candid traits. Its binary
Serde representation preserves the literal Wire vectors. External Candid,
Agent/PocketIC and persisted-record principals retain Platform's explicit
`ExternalPrincipal`; those conversions do not introduce alternate endpoint codecs.
