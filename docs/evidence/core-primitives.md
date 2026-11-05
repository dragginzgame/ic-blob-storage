# Implementation evidence: current boundary and historical record

The append-only implementation diary was compacted on 2026-11-05 under the
[Shared retention rules](../../rules/engineering-details.md#changelogs-and-artifact-preservation).
Its original text and recorded failures remain in Git; it is historical evidence,
not a current API guide or independently reproducible runtime qualification.
Raw experimental logs, temporary sources and artifacts are not retained here.
A surviving hash or narrative does not prove that those local artifacts exist.

## Conclusions retained

- Blob owns tenant admission, manifests, references, quotas, provider obligations
  and original-operation recovery. The standalone adapter owns mounting/lifecycle;
  downstream frameworks own their integration. See the [service contract](../service-contract.md).
- Trusted registration records retention intent. It does not prove that provider
  bytes exist; downloads verify content. Mandatory verifier/attestation machinery
  described in the old diary has been retired. See [trusted registration](../trusted-registration.md).
- Intent precedes effects, uncertain paid calls are inspected rather than resent,
  and restore fences persist. Logical release, physical deletion and billing
  cessation are different facts; local substitutes establish none of the latter.
  See [retirement](../retiring-installations.md) and the [probe ledger](caffeine-probes/README.md).
- Storage owns backing, allocations and stable collections. Platform owns required
  external IC protocols, Wire owns application DTOs, and Host libraries own
  native mechanics. Older Memory and Canic adapter paths are historical names.
  Current selections and validation belong to [dependency setup](../dependencies.md)
  and the [current handoff](../status/current.md).
- Old synthetic instruction/size and local-browser results describe their named
  fixtures and toolchains. They do not qualify a current release, production
  capacity, Canic integration, provider guarantees or native macOS.

## Historical sections

The links below preserve existing citations without copying the old diary or
claiming its raw artifacts remain available. Each opens the pre-cleanup source.

[Complete historical record](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md).

<a id="cashier-audit-response-decoding"></a>

[Historical: cashier-audit-response-decoding](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#cashier-audit-response-decoding).

<a id="funding-callback-experiment"></a>

[Historical: funding-callback-experiment](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#funding-callback-experiment).

<a id="independent-library-and-memory-015--2026-10-02"></a>

[Historical: independent-library-and-memory-015--2026-10-02](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#independent-library-and-memory-015--2026-10-02).

<a id="ledger-notification-response-decoding-after-0117"></a>

[Historical: ledger-notification-response-decoding-after-0117](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#ledger-notification-response-decoding-after-0117).

<a id="managed-application-outbox-and-publication-race--2026-10-01"></a>

[Historical: managed-application-outbox-and-publication-race--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-application-outbox-and-publication-race--2026-10-01).

<a id="managed-browser-setup--2026-10-01"></a>

[Historical: managed-browser-setup--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-browser-setup--2026-10-01).

<a id="managed-certificate-and-decoding--2026-10-01"></a>

[Historical: managed-certificate-and-decoding--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-certificate-and-decoding--2026-10-01).

<a id="managed-certificate-assessment-and-operator-queries--2026-09-30"></a>

[Historical: managed-certificate-assessment-and-operator-queries--2026-09-30](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-certificate-assessment-and-operator-queries--2026-09-30).

<a id="managed-certificate-framework-review--2026-10-01"></a>

[Historical: managed-certificate-framework-review--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-certificate-framework-review--2026-10-01).

<a id="managed-signed-client-and-local-byte-verification--2026-09-30"></a>

[Historical: managed-signed-client-and-local-byte-verification--2026-09-30](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-signed-client-and-local-byte-verification--2026-09-30).

<a id="managed-signed-tenant-reference-submission-and-cleanup--2026-09-30"></a>

[Historical: managed-signed-tenant-reference-submission-and-cleanup--2026-09-30](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-signed-tenant-reference-submission-and-cleanup--2026-09-30).

<a id="managed-signed-verifier-observation-submission-and-recovery--2026-09-30"></a>

[Historical: managed-signed-verifier-observation-submission-and-recovery--2026-09-30](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-signed-verifier-observation-submission-and-recovery--2026-09-30).

<a id="managed-unresolved-outbox-restoration--2026-10-01"></a>

[Historical: managed-unresolved-outbox-restoration--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#managed-unresolved-outbox-restoration--2026-10-01).

<a id="native-browser-certificate-binding--2026-10-01"></a>

[Historical: native-browser-certificate-binding--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#native-browser-certificate-binding--2026-10-01).

<a id="offline-native-upload-inputs--2026-10-01"></a>

[Historical: offline-native-upload-inputs--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#offline-native-upload-inputs--2026-10-01).

<a id="offline-reference-and-download-inputs--2026-10-01"></a>

[Historical: offline-reference-and-download-inputs--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#offline-reference-and-download-inputs--2026-10-01).

<a id="passive-funding-preparation-assessment--2026-10-01"></a>

[Historical: passive-funding-preparation-assessment--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#passive-funding-preparation-assessment--2026-10-01).

<a id="provider-response-and-root-correlation-fixes"></a>

[Historical: provider-response-and-root-correlation-fixes](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#provider-response-and-root-correlation-fixes).

<a id="published-canic-adoption--2026-10-01"></a>

[Historical: published-canic-adoption--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#published-canic-adoption--2026-10-01).

<a id="shared-native-reference-downloads--2026-10-01"></a>

[Historical: shared-native-reference-downloads--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#shared-native-reference-downloads--2026-10-01).

<a id="signed-native-account-inspection-and-bounded-transport--2026-10-01"></a>

[Historical: signed-native-account-inspection-and-bounded-transport--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#signed-native-account-inspection-and-bounded-transport--2026-10-01).

<a id="signed-native-gateway-controls--2026-10-01"></a>

[Historical: signed-native-gateway-controls--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#signed-native-gateway-controls--2026-10-01).

<a id="signed-native-upload-setup--2026-10-01"></a>

[Historical: signed-native-upload-setup--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#signed-native-upload-setup--2026-10-01).

<a id="standalone-signed-reference-submission-and-refused-inspection--2026-10-01"></a>

[Historical: standalone-signed-reference-submission-and-refused-inspection--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#standalone-signed-reference-submission-and-refused-inspection--2026-10-01).

<a id="streaming-caffeine-identities-after-017"></a>

[Historical: streaming-caffeine-identities-after-017](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#streaming-caffeine-identities-after-017).

<a id="verified-native-tenant-downloads--2026-10-01"></a>

[Historical: verified-native-tenant-downloads--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#verified-native-tenant-downloads--2026-10-01).

<a id="verified-native-upload-snapshots--2026-10-01"></a>

[Historical: verified-native-upload-snapshots--2026-10-01](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/core-primitives.md#verified-native-upload-snapshots--2026-10-01).
