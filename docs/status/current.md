# Current status

Date: 2026-10-19

## Source and scope

This batch starts at released 0.35.0, commit
`583947545a3afde00c4be68dcc6bca1caec8daee`, in the isolated
`codex/wire-owned-api-cut` branch. Package versions, manifests and Cargo.lock
remain unchanged. Pending breaking notes are 0.36.0. The earlier private-control
CBOR migration is already in the base. No release, publication or deployment ran.

The [archived handoff](handoff-2026-10-18.md) retains earlier evidence and
consumer prerequisites; its release labels qualify only its recorded source.

## Implemented boundary

All 31 owned methods and standalone lifecycle inputs now use method-bound
IC Wire. The two finite tenant contracts retain their exact frames; the other
29 methods and lifecycle use published Wire 0.4.4 native Serde with an explicit
identity table and frozen positional field/variant order. See
[the service protocol](../service-wire.md) for exact limits and schema obligations.
There is no old-packet detection, fallback or migration lane.

Standalone, storage probe, replicated Client, CLI and installed callers use the
same contract. Platform ingress length is checked before native argument copying.
Wrong method/direction and malformed packets refuse before workflow effects.
The CLI preserves passive document inputs separately from dispatched Wire frames
and signed IC envelopes, including observation, reference and retry artifacts.
Private consumer-probe controls retain their independent CBOR protocol and explicit
control-call helper. Provider certificate/management/Cashier Candid and Caffeine
HTTP bodies/chunks remain external contracts. Core production logic, persisted
records and journals are unchanged. Cross-release transitions require retirement
and reinstall; same-release restoration fences and exact retries remain required.

## Focused evidence

Linux evidence is retained under `/tmp/blob-wire-*`; failed attempts remain beside
corrected runs. Codec/CLI checks pass: 122 Contracts unit cases, 121 CLI unit cases,
focused package integration/doctests, 67 Core upload cases and 17 operator inventory
cases. Tests include full-width manifest values, independent positional projections,
canonical bytes, wrong identities, truncation and bounded refusal.

Installed standalone qualification covers all 75 default cases across the surface
run and the corrected 19-case recovery rerun. Thirty replicated-client cases pass.
The real Chromium standalone upload/verification/attestation/download/release
journey and direct browser certificate-intent calls pass against local substitutes.
Three native-verifier browser cases pass for completion, lost-reply recovery and
corruption refusal. The native/browser offline handoff passes with a 1 MiB body;
launcher tests pass. Strict Clippy, Rust 1.91 host/Wasm checking, release Wasm builds,
formatting, dependency pins/boundaries and local documentation links pass.
The owning Wire repository independently qualifies 1 MiB finite/native/generated
TypeScript exact-byte agreement and hostile packet admission.

Earlier sandbox loopback failures, wrong signed-envelope artifact names, private
control/service helper confusion, installation fixture encoding and verification
query identity failures are retained. Their corrected owning checks pass. These
are local integration results, not deployed-provider observations or performance
claims. No native macOS, full local CI, live provider or Canic qualification ran.

## Consumer and release work

Canic owns its wrapper, lifecycle and installed adoption in its repository.
Public DTO Candid traits used by record/external contracts remain separate work in
[#84](https://github.com/dragginzgame/ic-blob-storage/issues/84); this hard cut does
not claim those traits or external transports have disappeared. Required hosted
checks and maintainer review precede integration. Release, retirement disposition,
package publication and deployment remain separately authorized operations.
