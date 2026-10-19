# Blob-owned service protocol

All 31 owned methods use one IC Wire request/response contract. Standalone
installation and same-release post-upgrade inputs also use method-bound Wire.
This is the pending 0.36.0 hard cut: old CBOR and Candid service packets reject,
with no detection, fallback or compatibility lane. The root manifest selects
published Wire 0.4; Cargo.lock retains the exact 0.4.4 selection. Wire itself
owns no Blob DTO, endpoint, platform dependency or provider transport.

## Boundaries and ownership

| Boundary | Encoding |
| --- | --- |
| All owned service requests and replies | Canonical Postcard in a method- and direction-bound IC Wire frame |
| Standalone initialization | Request frame for `blob_install`, containing `ServiceInstallationInput` |
| Same-release post-upgrade input | Request frame for `blob_post_upgrade`, containing unit; no replacement configuration |
| Provider certificate, outgoing Cashier/provider and IC management | Their existing external Candid contracts |
| Caffeine HTTP upload/download and 1 MiB chunks | Existing provider contract; bodies never pass through the Blob RPC codec |
| Offline permissions, declarations, reference commands and statements | Bounded passive CBOR documents, decoded by `document`; never owned service inputs |
| Private consumer-probe controls | Independent bounded CBOR test protocol; explicit control callers |
| Signed IC transport envelope | IC transport CBOR, containing the exact application frame |
| Stable records and accounting journals | Existing persisted encoding; no migration |

`endpoints::SERVICE_METHODS` supplies method names, modes and codecs from the
same catalog that mounts endpoints. Linking Core, Contracts or Client exports
no endpoints or lifecycle hooks. `service.did` describes only the provider
certificate method. Default Wasm omits its optional extraction machinery.

## Schema and admission

The eleven-byte Wire header binds the literal u64 identity and direction
(request 0, response 1) before decoding. The explicit identity table in
[the codec owner](../crates/ic-blob-storage-contracts/src/wire/mod.rs) is frozen;
identities are not computed from Rust names or catalog positions. Unknown and
provider method names refuse.

The two existing tenant contracts retain their finite derived schemas, explicit
field/variant IDs, `Bytes<29>` principals and exact original frames. Each direction
allows 512 encoded bytes, 1024 value-allocation bytes plus codec scratch, 32 nodes
and depth 8. Native tenant DTOs are projections through `wire::tenant`; the generic
native helpers do not replace those projections.

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
emits passive `configuration.cbor`, complete `installation.wire` and hashes.
Only `installation.wire` is the standalone init argument. Offline upload/reference
preparation retains passive `.cbor` documents. Native dispatch encodes the selected
method after validating those documents; a document is never sent directly to an
owned endpoint. Browser fixtures receive Wire bytes for direct service calls and
retain passive permission bytes for the independent certificate-intent binding.

Dispatched application packets use `request.wire`, `response.wire`,
`service-response.wire` and query-specific `-args.wire`/`-reply.wire` files.
`signed-request.cbor` remains the actual IC envelope. Reference submission also
retains passive `reference.cbor` for receipt inspection: an apply frame cannot be
relabelled as a receipt request. Observations retain passive `permission.cbor`
and `statement.cbor` alongside the verification `request.wire`; attestation
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
Canic or the deployed provider. Public DTO Candid traits retained by existing
record/external contracts remain separate work under
[#84 — Remove application DTO Candid coupling after migrating current probes](https://github.com/dragginzgame/ic-blob-storage/issues/84).

Cross-release transitions remain retirement and reinstall only. Resolve provider
objects, uncertain effects, balances and continuing billing under
[the retirement contract](retiring-installations.md) before reset. Same-release
backup, restore fences, exact retry identity and receipt inspection remain required.
