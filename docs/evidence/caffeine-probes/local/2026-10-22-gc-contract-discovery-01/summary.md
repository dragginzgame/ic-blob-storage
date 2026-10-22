# Public GC contract discovery — 2026-10-22

The [intent](intent.json) bounded one web-search call with two queries, three
public GitHub reads, one npm metadata read and one archive read. There were no
retries, provider data requests, payments, deployments or signer operations.
[Checksums](files.sha256) bind retained inputs and outcomes.

- [Search output](search-results.json) returned unrelated cloud-storage billing
  material and the existing Blob announcement. It supplied no authoritative
  Caffeine gateway contract; none of the unrelated providers' semantics is used.
- The [public organization inventory](provider-repositories.json), HTTP 200 with
  no continuation link, contains 25 repositories. No separately named gateway
  server/specification repository was identified. This is not an exhaustive
  code search and does not prove that no public/private gateway API exists.
- [Compiler commit readback](motoko-commit.json) resolved public master to
  `9f8180640732ae182a3161862e0e4e3e4377d9fc`; the successful exact-source
  [primitive capture](prim.mo) contains the functions used by the prior mixin.
  Request records retain UTC, URLs, bounds, status, lengths and SHA-256.
- [Npm metadata](npm-latest.json) still selects SDK 1.1.2. The
  [archive](object-storage-1.1.2.tgz) matches its registry SHA-512 integrity;
  five exact source/license members were inspected without installation.
  `StorageClient.putFile` returns a hash. Its certificate is acquired before
  upload; chunk status is ordinary HTTP JSON. The inspected API supplies no
  provider-signed completion receipt. This does not prove no other API exists.

The experimental primitive section uses a deduplication table and weak references
for liveness/dead enumeration. Pruning removes matching entries only when dead;
it makes no remote storage call. This is source evidence of runtime bookkeeping,
not deployed gateway behavior, physical deletion or cessation of charges.

Historical Canic `d9d70322ade2ba6fbd17520acf93acad66dcf4aa` registered roots as live
while issuing certificates, before provider upload; it did not independently
verify completion. That explains the removed behavior but is not provider proof.
Restoring that trust model would change Blob's current completion contract and
requires the maintainer's explicit disposition; it is not a missing signer fix.

The gateway caller, failure/unknown-root behavior, provider-key association and
deletion/billing evidence remain unresolved. Do not copy the runtime table into
Blob or expose placeholder callbacks. #32 remains open. #33 separately needs an
application-operated verifier and trusted deployment/read-budget inputs under the
current contract; a proposed trust-model change remains undecided. No signer or
live service was selected by this investigation. No external cleanup is needed;
all existing objects, balances, uncertain effects and obligations are unchanged.
