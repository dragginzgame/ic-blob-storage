# Prepare an isolated standalone trial

This directory owns the deliberately small
[JSON proposal template](configuration.json.template), not a deployment flow or
provider defaults. The maintained
[offline installation command](../../../docs/operator-guide.md#check-installation-inputs-offline)
owns candidate validation and encoding. Follow that command with explicitly
reviewed service, operator, payer, Cashier, project and verifier bindings.
Controller status grants no tenant or payer authority; upload permissions name
their original uploader separately.

| Proposed setting | Value and purpose |
| --- | --- |
| Lifetime tenants, objects, references | One each |
| Object and physical/liability/logical bytes | 1,024 each |
| Retained chunks | One |
| Headers | Four / 512 framed bytes |
| Receipts | Two, preserving cleanup capacity |
| Active upload / read session | One each |
| Read buffers | 1,024 bytes per session and globally |
| Billing reserve / minimum / target | 2T / 0.1T / 1T cycles; proposed local policy, not provider prices or effects |
| Service cycle attachment allocation | One cycle, entirely reserved; no attachment offer available |
| Gateway reply entries / unique principals | Two / one; drift refuses and needs explicit review |

Preserve full-width integer literals using integer-preserving tooling.
`installation-check` reads strict JSON and emits `configuration.cbor`, complete
`installation.wire` and their hashes in a new private run directory. Only the
complete installation packet is a current standalone init argument. The provider
`service.did` describes the certificate endpoint; it contains no installation
schema. Former Candid installation packets are not accepted by the current host.

The funding reserve prevents service attachment offers within this installation.
It does not limit external provider bills, fund an account, revoke certificates or
end retention charges. Account provisioning and verified credit remain separate
provider obligations. The
[provisioning sequence](../../../docs/operator-guide.md#prepare-isolated-trial-provisioning)
owns those steps and their effect authorization.

## Bind the installation to reviewed artifacts

| Artifact | Required binding |
| --- | --- |
| Source and release | Exact reviewed commit, source archive, Cargo/lock/toolchain and receipt; receipt hashes bind release files, not the Wasm |
| Standalone Wasm | Frozen default build and SHA-256; observed compiled release must match the selected release |
| Native CLI and provider DID | Frozen hashes; declared and host-exported provider Candid contracts must agree; check the complete Wasm method surface against the shared catalog |
| Configuration and init | Exact validated CBOR carrier and hash for the actual created service principal; all local stand-in inputs remain local-only |
| Installed readback | Explicit operator inspects service, payer, namespace, project, verifier, resource envelope, release and fence before tenant admission |

The maintained `standalone_installation_cli` PocketIC cases consume this JSON
template through the actual CLI, install the complete carrier and check readback,
wrong-service refusal and authority boundaries. `BLOB_TRIAL_INSTALLATION_REPORT`
selects a **new directory** for retained CBOR init/readback and a summary. This is
local installed evidence, not deployed-provider qualification or live authority.
No mutable `target/` path or package version alone identifies reviewed bytes.
Cross-release replacement requires the
[retirement contract](../../../docs/retiring-installations.md); do not discard the
only records of provider objects, uncertain effects, balances or continuing bills.

## Historical trial evidence

The [first accepted trial contract](../../../docs/standalone-trial.md) and its
[probe ledger](../../../docs/evidence/caffeine-probes/README.md) retain the original
0.6/0.7 proposals, private artifact locations, authorizations, installations,
funding results and unresolved obligations. Those records describe their frozen
release and observation time. Use the matching historical source/tools to inspect
old packets; do not relabel or re-encode them as current dispatch authority.
Current commands and admission rules belong to the operator and protocol guides.
