# Caffeine probe ledger

This is the entry point for every Caffeine investigation. The maintainer selected
independent qualification on 2026-09-29; direct provider cooperation is unavailable
as a planning assumption. Release acceptance depends on demonstrated behavior and
explicitly reviewed operating limits, not obtaining an answer from Caffeine.
Provider correspondence remains useful evidence if it becomes available.

## Recording rule

Before a probe, record its question, evidence class, source revision or deployment,
actor and owner/project/account bindings, exact requests, request/byte/time/cycle
budgets, stop conditions and expected cleanup. Distinguish zero attached cycles
from unknown provider charges. Paid trials require an explicitly selected isolated
account/installation and an authorized budget; this plan is not that authorization.

After each request, retain its outcome before issuing the next. Record failures,
timeouts, redirects, truncation and inconclusive results as well as success. Never
erase an uncertain operation or replay it merely to get a cleaner result. Record
raw response bytes where safe, hashes, timestamps, decoder/source versions,
observations, interpretation and what the result cannot prove. Secrets, upload
certificates and private account data belong in controlled storage; put a redacted
manifest and the accountable artifact location here, never credentials. Hashes
of locally recorded responses establish file integrity, not provider signatures.

Every run gets a new directory/ID and an index entry below. Completed artifacts
are not edited; corrections or repeated experiments get new linked entries.
An interrupted run with no summary stays incomplete. Update the capability table
and handoff after a material result, and mark conflicting/drifted evidence for
review instead of treating previous observations as permanent guarantees.
"Constant tracking" means this is part of every investigation, not an unattended
poller or recurring paid job. Routine reruns of the same local test need not copy
all logs here; link their maintained test and record material changed findings.

## Current operating decisions and experiments

2026-09-30 planned local verifier submission evidence: extend the signed PocketIC
observation journey with the native one-shot `submit-attestation` command. Retain
the existing fixture owner/project, fixed test verifier identity and ten-byte local
HTTP substitute. Check missing/failed/changed artifacts before signing, intent and
signed-envelope persistence before one update, no repeat submission, and historical
receipt recovery. A local HTTP fault endpoint will exercise uncertain transport
without a deployed provider. Bound each command to 30 seconds and one update;
no live Caffeine, attached payment or provider cleanup is involved. Test artifacts
live in temporary controlled directories; maintained scenarios and material outcomes
are retained here. These tests cannot establish deployed provider availability,
retention, deletion or billing behavior.

On 2026-09-29 the maintainer explicitly selected a configured external verifier
trust model. The verifier must independently retrieve complete content from the
installation's owner/project binding and verify original metadata/root/length.
Its signed canister call attests observed content availability, not guaranteed
future retention, billing cessation or paid-operation identity. The new receipt
path retains all physical/economic obligations and existing exposure/restore gates.
This is an explicit service trust decision, not newly discovered Caffeine behavior.

| Capability / question | Experiment and decisive evidence | Current limit until demonstrated |
| --- | --- | --- |
| Source/interface drift | Capture official revision, package metadata, source/Candid hashes; compare to the reviewed baseline before behavior changes | A matching interface/source is not proof of the deployed implementation |
| Upload completion | Isolated known files spanning empty/single/multiple chunks; capture actual tree/chunk responses, interrupt the final response, then independently download and verify complete bytes/metadata against the expected root | Browser progress/hash alone cannot confirm completion. Verified reads establish observed content availability, not future retention or a trusted canister fact. Keep the current service completion gate until the evidence bridge and narrower semantics are implemented/reviewed |
| Resume and retry charges | One variable per trial: duplicate tree, duplicate chunk, interrupted chunk, completed root. Correlate request logs with isolated audit/account observations; wait through billing aggregation | Assume repeats can cost money. No automatic uncertain paid-effect retry; bounded manual disposition must preserve prior liability |
| Funding | Exact offered/accepted/refunded transport evidence plus provider audit correlation, including deliberately lost response | Never infer exact credit from aggregate balance movement. Uncorrelated outcomes stay uncertain and block automatic retry |
| Deletion and billing | Release one isolated root; record authenticated callbacks, subsequent availability and account observations over the applicable billing interval | Failed GET is not proof of deletion. Preserve physical/economic obligations separately; no unsupported deadline for billing stop. Retain immutable root history to prevent stale-callback reassignment |
| Old-backup recovery | Local interruption matrix first; then isolated live evidence tied to records outside the restored state, including delayed callbacks/certificates | Old backups remain fenced. A restored counter cannot authorize new work; reopening service requires surviving independent evidence and an implemented recovery decision |

These limits retain current safety behavior. They do not silently remove Canic
parity, authorize a new completion state or weaken existing acceptance cases.
If an unavailable guarantee makes a feature impossible, explicitly revise that
feature's supported contract and acceptance case before enabling it. Verification
work stays off the storage canister where possible; no return to mandatory full
file hashing in the canister is implied.

## Probe command

```sh
cargo run --offline --locked -p ic-blob-storage-cli --bin caffeine-probe -- \
  public-source docs/evidence/caffeine-probes/runs/NEW-RUN-ID
cargo run --offline --locked -p ic-blob-storage-cli --bin caffeine-probe -- \
  verify docs/evidence/caffeine-probes/runs/NEW-RUN-ID
```

The parent directory must exist and the run directory must not. The command
persists `plan.json`, then each `request-N.json` before its HTTPS GET. It retains
`response-N.body`, its byte count/SHA-256/status/outcome in `response-N.json`, and
a final `summary.json`. Each source fetch uses the commit observed in request 0,
not another moving branch read. Runner version and a source/manifest/lockfile
fingerprint are recorded. Four requests maximum, 1 MiB per response, 30 seconds
per request, no redirects or application retries. Only official public GitHub/raw
source and npm endpoints are selected by the command. It sends no credentials,
gateway upload, account query, cycles or payment. File/directory sync is required;
failure stops the run. No existing run is resumed or overwritten.

`verify` checks recorded response hashes and structural completion; failed and
incomplete runs remain labelled. It does not promote source captures to behavioral
qualification. Review source/package drift separately; never update dependency
pins just because latest metadata changed. Native tests use a local HTTP substitute
to exercise intent-before-request, HTTP errors, refused redirects and body limits.
`make probe-check` verifies public-source `runs/` directories offline and is part of
the repository validation gate. Failed/incomplete experiments remain valid records;
corrupt, orphaned or structurally inconsistent recorded evidence fails the check.

Local SDK scenarios use the same pinned patched package as the browser fixture:

```sh
BLOB_BROWSER_NODE=/path/to/node24 make test-sdk-probe \
  BLOB_SDK_PROBE_REPORT=.tmp/NEW-SDK-RUN
```

The report's parent must exist and its directory must be new. This opt-in target
uses installed dependencies, with no downloads, sockets or provider calls. Its
supplied agent response, gateway and in-memory store are substitutes, explicitly
identified in `plan.json`. The SDK prepares and transfers the files; the test
does not reconstruct its upload protocol. The Rust verifier checks captured bytes
against the SDK root and original metadata, outside the storage canister.
The bundle/verifier hashes, request fingerprints, raw synthetic response bodies,
verification outputs and results are retained. `failure.json` preserves failed
cases; interrupted captures without a summary are incomplete. Files in the
material local runs below are covered by `local/SHA256SUMS` in `make probe-check`.
That checksum check establishes artifact integrity, not scenario correctness or
provider authenticity; rerunning the opt-in test establishes current local behavior.

## Run index

The `verifier-observation` local experiment is planned for 2026-09-29 against the
current workspace. Tests use the maintained Caffeine target encoder and streaming
verifier, a fixed PocketIC verifier identity, fixture owner/project and loopback HTTP
substitutes. Record intent before one bounded GET; exercise complete, corrupt,
truncated, oversized, non-200, redirected and encoded responses. Test refused service
plans, interruption/no-clobber behavior and a verified statement carried to receipt
recovery through an explicit fixture attestation. Per invocation: one service query,
one provider GET maximum, 30 seconds each, explicit content bound at most 1 GiB,
64 KiB service Candid. No deployed provider/account, credentials or attached cycles;
no external cleanup. Stop on failure and retain failed outcomes; these local cases
do not qualify the deployed provider or authorize live reads.

The `attestation-recovery` local contract check is planned for 2026-09-29 against
the current workspace and `storage_attestation_cli` PocketIC test. A fixed test
tenant signs bounded receipt queries; the fixture operator is the explicit verifier.
The plan saves the exact statement before a local attestation, discards its reply,
and inspects absence, acceptance, changed intent, wrong verifier/root trust and
settled/restored history. Exposure, content and deletion/settlement facts are
labelled substitutes. Each CLI query is limited to 30 seconds, 256 KiB HTTP and
4 KiB Candid; no query retries. No deployed account, gateway request, provider
charge or attached cycles are involved. Stop on failure; only temporary local
canisters/files require cleanup. The completed result is indexed below.

| ID / date | Class and target | Status / evidence | Conclusion and obligations |
| --- | --- | --- | --- |
| verifier-submission / 2026-09-30 | Native artifact validation and signed PocketIC through a bounded local fault proxy; fixture exposure and ten-byte local content | [Maintained journeys](../../../tests/pocketic/tests/storage_observe_cli/mod.rs) pass (4.49 seconds): normal acknowledgment, dropped real acknowledgment and forced HTTP 202. Native CLI tests and strict affected Clippy pass; historical settled/restored recovery (5.28 seconds) and signed standalone status/history regression (6.46 seconds) pass | Proxy compares the exact signed wire body with saved intent before forwarding one update. Competing submitters send once; incomplete, failed or changed observations reject before claiming. Killed provider reads remain undispatchable. A dropped acknowledgment stays uncertain; 202 stays pending. Both recover the exact immutable receipt through signed queries without resending. Request IDs, source/root/artifact hashes and outcomes are retained in temporary controlled test runs; scenario definitions remain in source. The pinned ic-agent 0.49.2 implementation was inspected locally for one-call transport, certificate verification and disabled retries. No live provider/account, attached cycles, external charges or cleanup obligations; deployed retention/billing/exposure remain unqualified |
| historical-recovery / 2026-09-26 | Source inspection, substituted HTTP replies, Candid codecs and anonymous metadata | [Existing record](../caffeine-recovery-review.json), indexed retrospectively; not rerun | SDK progress is insufficient; root-only callbacks and payment correlation need conservative handling. No paid effects were recorded in that review; no fabricated modern run metadata |
| historical-deployment / 2026-09-25 | Anonymous deployed interface/gateway/price observations | [Existing record](../caffeine-deployment-observation.json), indexed retrospectively; use its own timestamps/scope | Interface reachability is not upload, payment or billing qualification |
| public-source-01 / 2026-09-29 | Anonymous public source/registry capture; four recorded GET invocations, no account/paid effects | [Plan](runs/2026-09-29-public-source-01/plan.json), [summary](runs/2026-09-29-public-source-01/summary.json); captured, integrity checked | Official commit `78781961e52b8c9c874becd473402950429d4818`; both backend hashes match the prior review; npm 1.1.2/integrity unchanged. This runner had no application retries but still inherited reqwest protocol retries; actual wire request count was not measured. Preserved as captured; current runner explicitly disables transport retries |
| public-source-02 / 2026-09-29 | Repeat public-source capture after explicitly disabling HTTP transport retries; four requests maximum | [Plan](runs/2026-09-29-public-source-02/plan.json), [summary](runs/2026-09-29-public-source-02/summary.json); captured, integrity checked | All four response hashes match 01. New runner/source fingerprint retained; 01 and its retry limitation remain unchanged. No provider effects |
| local-sdk-01 / 2026-09-29 | Pinned patched SDK with substituted certificate-agent reply, gateway and in-memory intent store | Failed before runner initialization: ESM bundle attempted dynamic require of Node `tty`; no run directory or requests. Fixed bundle with Node `createRequire` | Planned six scenarios did not start. No network/account access or cleanup obligations |
| local-sdk-02 / 2026-09-29 | Same local substitutes; independent native Rust content verification | [Failure](local/2026-09-29-sdk-02/failure.json): first verification subprocess timed out; partial request/reply records retained | Node synchronous subprocess input did not reach EOF in this environment (reproduced with `cat`); asynchronous closed input works. No completed case, network/account access or cleanup obligations |
| local-sdk-03 / 2026-09-29 | Same local substitutes, asynchronous verifier subprocess | [Plan](local/2026-09-29-sdk-03/plan.json), [summary](local/2026-09-29-sdk-03/summary.json); all six scenarios pass | SDK sends all chunks despite `existing_chunks` and returns successfully for a non-complete status. Lost final reply stays uncertain despite verified bytes; replay blocked. Aggregate budget stops before the next request. No real network/account access or cleanup obligations |
| verifier-contract / 2026-09-29 | Local model/CLI and PocketIC; fixed test verifier principals and substituted content observations | 60 upload-store and two codec tests pass; all 32 standalone cases pass (141.72 seconds), plus `storage_completion` (5.05 seconds) | Exact authority/permission binding, immutable receipt replay, four stable-write rollback cuts, late revocation, release/settlement and fenced restoration pass. The signed CLI verifies 10 MiB against original metadata without mutating service state. Local canisters only; zero provider requests/attached cycles, no external cleanup or deployed availability claim |
| attestation-recovery / 2026-09-29 | Local saved-intent decoder/CLI and signed PocketIC receipt queries; fixed test tenant and fixture verifier | `storage_attestation_cli` passes (5.33 seconds), including a deliberately discarded successful update acknowledgment | Absent, matched and conflicting evidence remain distinct. Changed permission/verifier and wrong root trust reject; settled/restored receipts remain immutable. Queries preserve stable bytes and the saved intent. This discards a test acknowledgment, not a simulated network packet; content/exposure/settlement are substitutes, with no deployed provider availability claim. No external requests, charges or cleanup obligations |
| verifier-observation / 2026-09-29 | Native loopback HTTP substitutes and signed PocketIC installed-plan/statement journey; fixture exposure only | 14 native CLI tests pass, including multi-chunk streaming and HTTP corruption/EOF/size/encoding/redirect refusals. `storage_observe_cli` passes (4.00 seconds); standalone completion/plan refusals pass (5.37 seconds) | Intent exists before GET; complete verification saves the exact statement without dispatch. Killing the subprocess mid-body leaves no statement/summary and forbids reuse of that run. Explicit fixture dispatch accepts the independently checked statement and receipt recovery matches. Service bytes stay unchanged during observation. Local artifacts are temporary test outputs; maintained tests retain the scenario definitions. No real provider/account access, charges or cleanup obligations; no deployed qualification |

Other retained investigations are indexed here without inventing missing request
logs or replaying their effects. Use each record's own dates and evidence classes:
[client observations](../caffeine-client-observations.json),
[Mops verification](../caffeine-mops-verification.json),
[funding review](../caffeine-funding-review.json),
[contract refresh](../caffeine-contract-refresh.json),
[installation review](../caffeine-installation-review.json),
[browser reuse](../caffeine-browser-reuse.json), and
[gateway/account transport review](../caffeine-gateway-transport.json).
The older records retain their original schemas; they are research history, not
backward-compatible product state or a claim of modern capture completeness.

The maintained Chromium/PocketIC suite also passed its ten scenarios with the
aggregate-budget contract (49.12 seconds). The initial sandbox run could not bind
the local server; the loopback-enabled rerun passed. This uses actual IC certificate
and IndexedDB behavior with a local gateway substitute. It does not turn the SDK
probe's synthetic certificate into deployed-provider evidence.

## Next trial — upload/resume, prepared scope, not dispatched

Use an isolated explicitly selected owner/project/payer and diagnostic authorization
surface; standalone does not yet expose the complete real certificate/completion
journey. Preserve original bytes outside the canister. Reuse the pinned upstream
file preparation/transfer code and independently check reads against expected
roots/metadata. Record all SDK-internal requests: a high-level upload call is not
one HTTP request. No SDK retry may bypass the total request/byte budget.

1. Record baseline configuration, current interface/source hashes and available
   account/audit evidence. Stop on binding/version drift, unresolved pre-existing
   obligations or insufficient budget evidence. Do not fund or link accounts as
   an implicit setup step.
2. Upload one known three-byte file; retain every request/response and the exact
   certificate correlation securely. Download and verify it independently.
3. Upload a second known file of 1 MiB + 1 byte. Deliberately discard the final
   chunk response at the client, recording that loss. Inspect current availability
   and the documented resume response. Record each explicit repeated tree/chunk
   request separately; stop if the response cannot be interpreted under the pinned
   contract. Do not turn this experiment into an automatic production retry rule.
4. Download and verify the second file, then capture account/audit observations
   and compare complete versus lost-response paths. If charges cannot be correlated,
   record that result as unresolved. Preserve both roots and their obligations for
   a separately planned deletion/billing observation; do not erase trial records.

Proposed client bounds for this trial: two object roots, at most two certificate
updates, eight read-only service/account observations, 24 gateway HTTP requests,
10 MiB upload and 12 MiB download traffic, 30 seconds per request and 15 minutes
total. Stop at the first exhausted bound, unexpected binding, uncertain new payment
or insufficient charge evidence. These are traffic/time ceilings, not a financial
guarantee. The exact account, operator, certificate authority, provider spending cap
and funded cleanup owner remain unselected. Resolve them in a recorded run plan
before any live effect; this draft is not permission to deploy or spend.

Deletion/billing and long-retention experiments have separate observation windows.
No guessed wait interval or quiet balance sample can establish final settlement.
