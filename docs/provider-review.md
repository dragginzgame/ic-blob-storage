# Caffeine provider review — 2026-09-25

Verdict: Caffeine remains unqualified for the required service journey. Newly
located official integration source improves the protocol evidence, but does
not establish the deployed gateway/Cashier version or paid-effect guarantees.

## Selected integration baseline

The maintainer requires this service to target the latest official Caffeine
integration rather than inherit Canic's potentially drifted bindings.
[The baseline record](provider-baseline.json) pins the verified target and
separates release/source observations from deployed-provider qualification.

On 2026-09-25 the [npm registry](https://registry.npmjs.org/@caffeineai%2fobject-storage)
reports `latest` as `@caffeineai/object-storage` 1.1.2, published
2026-09-17. The downloaded package archive passes its registry SHA-512 integrity
check. Its `dist/StorageClient.js` SHA-256 exactly matches the upstream artifact
used for the retained client observations. Those observations therefore also
apply to this file in the latest published package, including the empty-tree
failure; they do not establish whole-package or server behavior.

Official `main` still resolves to the commit below. Its backend manifest
declares `caffeineai-object-storage` 1.1.1. A subsequent anonymous query to the
official Mops registry confirms 1.1.1 as its highest published version. The
registry's hashes for `mops.toml`, `src/Mixin.mo` and `src/Storage.mo` match the
pinned official source. See [the retained registry evidence](evidence/caffeine-mops-verification.json).
The Rust service does not install the Motoko package. Gateway/Cashier deployment versions remain
unverified and must not be inferred from either package number.

The registry address and query schema come from
[Mops network configuration](https://github.com/caffeinelabs/mops/blob/ad36ae3b51616b3c39140674c19ad36481301f59/cli/api/network.ts)
and [its Candid interface](https://github.com/caffeinelabs/mops/blob/ad36ae3b51616b3c39140674c19ad36481301f59/cli/declarations/main/main.did).
Reproduction uses ICP CLI 1.6.0 with that downloaded interface:

```sh
icp canister call oknww-riaaa-aaaam-qaf6a-cai getHighestVersion \
  '("caffeineai-object-storage")' --query --identity anonymous \
  --network https://icp-api.io --root-key mainnet \
  --candid /path/to/main.did --output candid
icp canister call oknww-riaaa-aaaam-qaf6a-cai getFileHashesQuery \
  '("caffeineai-object-storage", "1.1.1")' --query --identity anonymous \
  --network https://icp-api.io --root-key mainnet \
  --candid /path/to/main.did --output candid
```

Retained query responses are observations, not portable certified-state proofs.
The checks issue no update calls or paid provider operations. This closes the
backend publication-version gap, not any deployed-provider safety requirement.

Before provider implementation and qualification, recheck the official source
and registry latest, and update this exact baseline plus affected evidence
together if upstream changed. Builds must use reviewed pins, not resolve a
moving `latest` tag. Do not reproduce superseded Canic bindings or add fallback
variants to bridge the differences. Service protocol/state generations stay
v1; upstream package versions are separate identifiers.

## Provenance

Reviewed public repository: `caffeinelabs/skills`, commit
`e5cacdfe5ce55e939edb02980fca800c0c13f421`.

- [Backend Mixin.mo](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/backend/src/Mixin.mo)
  and [Storage.mo](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/backend/src/Storage.mo).
- [Backend manifest](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/backend/mops.toml):
  `caffeineai-object-storage` 1.1.1, Motoko 1.7.0.
- [Frontend source](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/frontend/src/StorageClient.ts),
  [JavaScript artifact](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/frontend/dist/StorageClient.js)
  and [manifest](https://github.com/caffeinelabs/skills/blob/e5cacdfe5ce55e939edb02980fca800c0c13f421/packages/object-storage/frontend/package.json):
  `@caffeineai/object-storage` 1.1.2.

These are official application integration/client sources, not the gateway or
Cashier server implementation. No package was installed, no upstream repository
was edited, and no provider request was issued. The repository's skill files
were not installed or adopted as instructions.

## Differences from Canic's captured contract

Comparison baseline: the Canic revision and Candid/source hashes recorded in
[extraction readiness](extraction-readiness.md).

| Surface | Canic's reviewed source/snapshot | Newly reviewed official integration |
| --- | --- | --- |
| Deletion listing | `_immutableObjectStorageBlobsToDelete` returns `vec text` | Returns `[Blob]`, equivalent to `vec blob`; caps the returned dead-blob slice at 10,000 |
| Gateway registry | `storage_gateway_principal_list_v1` | `storage_gateway_list_v1` |
| Cashier top-up | Optional record with optional account/target balance; structured result | Required `{ account : Principal }`; empty result |
| Refill callback | `_immutableObjectStorageFundFromProjectCycles` emitted by Canic billing adapter | `_immutableObjectStorageRefillCashier`, authorized against the configured Cashier principal |
| Liveness/deletion state | Explicit stable root/pending records | Motoko runtime `Prim.isStorageBlobLive`, `Prim.getDeadBlobs`, `Prim.pruneConfirmedDeadBlobs` and GC |

These differences establish interface differences between inspected sources. They do
not establish which interface any deployed provider accepts, or justify dual
readers/compatibility branches. Freeze one selected, verified contract before
writing Rust bindings. The Motoko runtime's blob/GC integration is not a Rust
storage lifecycle implementation that can simply be copied.

## Client algorithm and completion observations

The client uses 1 MiB chunks, domain-separated SHA-256 chunk/metadata/node
hashes, a binary tree and metadata-dependent roots. It sends a certificate with
the tree, uploads chunks, and constructs a direct read URL. Its visible request
bindings include owner, project, bucket, root and chunk/index. These bindings
do not establish provider-side deduplication, operation identity or receipt
retention across a restored service.

`uploadChunk` decodes a completion flag, but `parallelUpload` discards that
flag. `putFile` returns the root after the chunk requests finish. That client
behavior supplies no independent authoritative completion lookup. Its HTTP
retry loop is not proof that uncertain paid operations are safe to repeat.
`getDirectURL` constructs a URL; it performs no byte verification.

An isolated check executed the pinned artifact's hashing classes with Node
v18.19.1 and WebCrypto, excluding imports, the gateway client and network access.
[Recorded observations](evidence/caffeine-client-observations.json) include
the artifact hash, input and output vectors:

- The provider root for `abc` differs from its raw SHA-256 digest.
- Changing only content-type metadata changes the root for those same bytes.
- Building an empty tree rejects: its empty-input branch encodes hexadecimal
  text as 64 UTF-8 bytes and supplies that to a constructor requiring 32 bytes.

This checks the pinned client algorithm only. It is not a provider test or a
service implementation. Empty-object support needs a verified provider vector
or an explicit service exclusion before A02 can be frozen. Do not copy the
client's empty-input failure into the service as the intended hash contract.

For reproduction, retrieve the linked JavaScript at the pinned commit, verify
its SHA-256 against the JSON, and evaluate the slice beginning at
`const MAXIMUM_CONCURRENT_UPLOADS` and ending before
`class StorageGatewayClient` in a fresh Node VM exposing only WebCrypto,
`TextEncoder` and `Uint8Array`. Use `YHash.fromChunk` and
`BlobHashTree.build` with the JSON input, no headers, or `Content-Length: 3`
plus the recorded content types. The empty case uses no chunks or headers.
This bounded experiment adds no maintained JavaScript tooling to the project.

## Serving and deletion evidence

Caffeine's [storage overview](https://help.caffeine.ai/hc/en-us/articles/46899827144852-How-File-Storage-Works-in-Your-App)
describes browser-to-provider upload, direct file delivery and background
cleanup after reference removal. Its
[troubleshooting documentation](https://help.caffeine.ai/hc/en-us/articles/46899784111380-File-Storage-Troubleshooting)
explicitly says URL holders can retrieve files without a delivery-layer login
check, signed/expiring URLs are unsupported, and deletion is asynchronous.

Consequently the first proposed consumer journey should use public assets.
Tenant authorization must still protect mutations and service metadata, but
must not be described as restricting access to publicly served bytes. This
proposal does not select a consumer or authorize changes to it. MIME and
active-content serving still need separate qualification.

Neither document supplies operation-specific deletion receipts, evidence
retention, final-charge settlement or billing-cessation proof. Waiting for a
documented cleanup interval is not authoritative evidence of those outcomes.

## Implementation gate and next evidence

The follow-up public-source search inspected Caffeine Labs' public repository
listing and the pinned integration repository's storage-related tree. It found
no standalone gateway or Cashier server implementation in that search scope.
That is a bounded search result, not proof that no authoritative source exists.

Caffeine's [File Storage Costs](https://help.caffeine.ai/hc/en-us/articles/49362898986644-File-Storage-Costs)
adds relevant economic context: uploads, ongoing storage and downloads are
charged separately; deleted files stop contributing to daily storage charges
from the next billing day; existing files can continue being served and accrue
storage charges after credits run out. This rules out treating an empty balance
as proof that obligations ended. The article does not define a machine-readable
final-charge/deletion receipt or identify the Cashier cycle-account contract.
Do not map retail credits to canister cycles or invent retention guarantees from
that documentation.

The concrete missing provider evidence is collected in
[the provider evidence request](provider-evidence-request.md). It has not been
sent to anyone; obtaining it needs an authoritative source/deployment reference
or provider-owner evidence.

Keep B1 open. To freeze a Caffeine contract, obtain the intended deployed
gateway/Cashier identity and version, authoritative server interface/source,
retry/completion/retention guarantees, namespace exclusivity, and independent
deletion/billing-stop evidence. Resolve the interface differences above and
prove the selected same-release restore boundary against surviving authority.

The initial Rust slice after B1 should distinguish raw content digests from
provider roots and enforce authority/accounting invariants without effects.
Do not scaffold paid provider calls from the older Candid snapshots while
these gaps remain. If required provider guarantees cannot be established,
record a narrower supported contract or another provider decision before B2.
