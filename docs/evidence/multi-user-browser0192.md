# Two-user browser qualification after 0.19.1

Base: released `5f2d171b1b01e0dc3b0047f00c194a9afe874642`; validated source
`f0192506d0a71a3f4116220db1f6d901947ba74a`. Package identity and receipt remain
0.19.1. The compatible pending 0.19.2 batch changes CI/tooling and test support,
not public DTOs, certificate policy or durable schemas.

## Current inputs and maintained behavior

The released lock already selects all four Host 0.8.5 packages, Memory 0.31.9,
private Metrics 0.2.16, Testkit 0.25.3 and PocketIC 16.1.0. These differ from
the earlier 0.19.1 development record; that record keeps its original graph.
No Cargo selection is changed by this continuation. Offline cache preparation,
current standalone Wasm/CLI builds and strict affected Clippy pass. The changed
private harness also compiles with actual Rust 1.88.

Shared Tooling adopts the existing 97-file selection at committed 0.1.32,
`635a39a9dd5f8d021fa9c9196b591e00521a7e02`. The owner dirty optional-fleet
changes are excluded. Existing setup/check and the reviewed PocketIC bundle
remain active. Tool-command fixtures, snapshot/declaration/browser prerequisites
and workflow actionlint pass. Push CI groups now include the source SHA;
only newer PR revisions cancel old review runs. This follows
[Shared #80](https://github.com/dragginzgame/shared-tooling/issues/80) and protects
queued native qualification without adding runner capacity.

## Failure and repair

The first new Chromium case refuses the initial preparation control frame:
the SDK's internal `maxChunkBytes` field is forwarded into a strict Rust DTO
declaring only hash, byte length and manifest JSON. No permission is prepared,
certificate exposed or provider PUT/GET issued. The reader's original complete
JSON frame was not retained; the rejected-field diagnostic, source/bundle and
input hashes survive in `.tmp/continuation0192-01/`.

The producer now explicitly projects those three declared fields. The strict
consumer is unchanged, and SDK preparation/hashing remains its canonical owner.
This fixes the existing single/serial harness boundary as well as the new case;
no provider decoder or permissive JSON fallback is introduced. Fresh attempt-two
directories preserve the failed attempt separately.

## Actual local browser and IC results

The [intent](caffeine-probes/local/2026-10-09-multi-user-browser0192-01/intent.json),
[corrected-attempt intent](caffeine-probes/local/2026-10-09-multi-user-browser0192-01/attempt2-intent.json)
and [summary](caffeine-probes/local/2026-10-09-multi-user-browser0192-01/summary.json)
bind the final results and request budgets. Raw qualification, source inputs,
artifacts, journals, CLI operations and refusals remain under
`.tmp/continuation0192-01/qualification.json` and companions.

Two independent Chromium processes use fixed test-only signing keys 42 and 45
against one actual standalone service. A separately signed project tenant 43
grants each exact permission; installed verifier 44 independently reads the whole
body and submits the existing attestation. Both distinct 1 KiB bodies upload,
survive page reload without another certificate/upload dispatch, verify and
download with matching hashes. A browser opened with the other user's key refuses
before ingress and preserves the original saved intent. The separate existing
signed-IC regression qualifies service-side cross-user root and self-grant refusal;
it is not replaced by the browser's local identity check.

With both references live, logical bytes are 2,048. Releasing the first leaves
the second confirmed and 1,024 logical bytes. Releasing both reaches zero logical
bytes while physical and liability accounting retain 2,048 bytes. Each user's
native evidence and browser journal remain separate; no root is reused.

The new two-user case passes in 8.35 seconds. The two default-key success/corrupt
controls pass in 10.43 seconds; lost-final-reply and withdrawn/late-completion
controls pass in 10.04 seconds. Six upload owners produce exactly **12 local
PUTs and 11 local GETs** across these five cases, matching the recorded budget.
The new two-user portion uses four PUTs/four GETs. Corruption produces no accepted
statement; interruption recovery preserves original intent and obligations.

These are real Chromium, signed IC and native-worker results with an owned
HTTP/2 provider substitute. They do not establish Toko session/AssetsManage
checks, Canic mounting, real media rendering, deployed Caffeine retention,
deletion or billing. Fixed fixture keys are not production signer provisioning.
Owned browsers, TLS/gateway sockets and PocketIC scopes close; there is no live
provider request, paid effect or external cleanup.

## Remaining owners

Keep [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#33](https://github.com/dragginzgame/ic-blob-storage/issues/33) open for the real
application/provider journey. [The GC review](../gateway-gc-contract.md) remains
unchanged: [#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) needs
the gateway's failure/retention policy and physical-deletion meaning before
handler mounting. [#37](https://github.com/dragginzgame/ic-blob-storage/issues/37)
can retire four unused fleet files once the shared optional-reporter guard and
adoption guidance are committed; the current shared Make target still invokes
the reporter unconditionally. No vendored file is patched to bypass that owner.

No sibling file, full local CI/release gate, agent commit, push, release,
publication or paid provider effect occurs. No function, method or type is
removed in this batch.
