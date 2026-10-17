# Blob tenant codec comparison

Date: 2026-10-17. This is a local installed PocketIC comparison of released
Candid, the pending CBOR codec and the captured IC Wire tenant adapters. It
measures codec work for six matched inputs, including full-width integers,
29-byte principals, absent enrollment and an application refusal. It does not
measure Blob storage policy, provider behavior or Canic's service.

IC Wire uses 61.55–71.99% fewer codec instructions than CBOR and 95.23–97.91%
fewer than Candid in these cases. Complete request-plus-reply packets are
28.41–60.31% smaller than CBOR and 39.56–76.92% smaller than Candid. These ranges
describe these fixtures, not arbitrary contracts or whole-service savings.

## Results

Packet columns are request / reply bytes, including Wire's 11-byte envelope in
each direction. Instruction columns are the median sum of decode and encode
over ten samples; the identical synthetic handler is measured separately and
excluded from these totals. No warmup sample is discarded.

| Case | Candid bytes | CBOR bytes | Wire bytes | Candid instructions | CBOR instructions | Wire instructions |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Inspect absent | 51 / 149 | 49 / 80 | 34 / 37 | 193,359 | 19,032 | 6,258 |
| Inspect full width | 107 / 214 | 106 / 165 | 90 / 104 | 196,662 | 24,395 | 9,380 |
| Update create | 89 / 158 | 74 / 100 | 36 / 39 | 256,916 | 25,012 | 7,201 |
| Update suspend | 98 / 158 | 94 / 100 | 38 / 39 | 262,164 | 26,970 | 7,554 |
| Update full width | 154 / 214 | 159 / 165 | 103 / 104 | 265,931 | 30,475 | 10,968 |
| Update denied | 98 / 123 | 94 / 12 | 38 / 13 | 260,516 | 17,275 | 5,441 |

The tiny refusal is a useful counterexample to universal size claims: Wire's
reply is 13 bytes and CBOR's is 12. Wire also carries method/direction identity
and checks canonical representations; CBOR does not offer those same guarantees.
Maximum-width varints have a different tradeoff from small integers.

The prior default-release standalone artifacts were reused and hash-checked
against their original receipts. The all-CBOR artifact is 3,332,291 bytes; the
partial Wire artifact is 3,334,330 bytes, an increase of 2,039 bytes (0.0612%).
Only two methods have moved to Wire, so the latter still links CBOR. These are
unprocessed whole-canister sizes, not isolated codec sizes, a fresh optimization
comparison or a size comparison against released Candid.

## Method and reproducibility

The [machine record](wire-bench0320.json) contains every instruction sample,
packet hash/length, compiler/environment, source hashes, selected lock and Wasm
identities. The [intent](caffeine-probes/local/2026-10-17-wire-bench0320-01/intent.json)
predates dispatch, and the
[exact-baseline addendum](caffeine-probes/local/2026-10-17-wire-bench0320-01/exact-baseline-intent.json)
predates the final comparison; the
[summary](caffeine-probes/local/2026-10-17-wire-bench0320-01/summary.json) closes
the local scope. Raw experimental Rust sources, lockfiles, all 36 request/reply
packets, commands and build/run logs remain locally under
`.tmp/review-validation/wire-bench0320/`. The private harness is not a maintained
benchmark target or a production endpoint; no permanent benchmark framework is
added to Blob.

All three codecs run in the same installed Wasm with identical native inputs
and deterministic handlers. Candid uses the released standalone's configuration:
4,096 input bytes, 2,000,000 decoding work, 1,024 skipping work, 64 types and a
4,096-byte header. Its DTO source and derives are copied verbatim from released
0.31.0 commit `d2b5877e1449c434feacb2cb9b0fa9811a6810a2`. The runner proves
byte/native-value equality with the current DTOs. This control does not rebuild
the entire released 0.31 service. CBOR uses the
pending Contracts encoder/decoder. Wire uses the current tenant adapters,
including bounded/native DTO conversion, exact method/direction framing and
canonical validation. Decoder guarantees differ; no weaker Wire fast path runs.

The harness copies raw IC arguments before taking `instruction_counter` samples.
It measures decode, handler and encode separately, and stores counters after the
last sample. Counter overhead remains included. Framework dispatch, argument
copying, transport, reply delivery, actual tenant policy/storage and external
effects are outside the measured regions. This is neither a cycles/throughput
benchmark nor an estimate of a complete Canic request's instruction reduction.

Rust 1.99.0 builds the default release Wasm for `wasm32-unknown-unknown` without
LTO or postprocessing. The native runner uses admitted PocketIC 16.1.0 and
Testkit 0.50. Dependency selections match the prior private candidate graph;
only the two throwaway harness packages are additional lock entries. Captured
inputs remain hash-identical through the run. All 180 selected measured calls
and 180 counter queries complete; every response matches both expected bytes and
native values. An earlier successful 180-call/180-query control used current DTO
derives for Candid. It remains separately retained, superseded by exact released
derives because the new Serde attributes can affect decoding work. Totals across
both runs are 360 measured calls and 360 counter queries. Each owned server/
instance drops normally, with no provider objects, balances or external cleanup
obligations.

The candidate is the same frozen, unpublished IC Wire source described in the
[adoption receipt](icwire0320.md): observed dirty source at upstream HEAD
`226a11fce6d5cdd59a77d9a006c91f5a46c58b42`, still labelled 0.2.2. The maintainer
selected registry 0.3. These are not results for a published 0.3 artifact.

Preparation failures remain retained: the first lock comparison mishandled
multiple versions, regenerated locks drifted, and the native SHA helper initially
required an unavailable formatting trait. These were repaired before initial
dispatch. The exact released DTO control initially omitted its direct Serde
derive dependency; that compile failure is retained and repaired before its run.
No failed installed measurement is omitted or relabelled.
No full CI, release, publication, sibling edit or deployed-provider request ran.

## Feedback for IC Wire

- Add a CBOR consumer control to the existing `ic-wire-canister-evaluation`
  runtime matrix, with bounded principal bytes, `u128::MAX`, optional records and
  short application errors. Include native conversion costs and whole envelopes.
  Blob shows a substantial improvement, but tiny replies and full-width integers
  explain why a single mixed-record Candid comparison cannot predict every case.
  Keep consumer dependencies out of the runtime; generic equivalent fixtures can
  live with that existing evaluation owner.
- Improve schema-admission diagnostics at the host/development boundary.
  `contract::admit` currently returns the same `Error::SchemaLimit` for bytes,
  allocation, nodes, depth and overflowing shapes. A diagnostic helper reporting
  the failing dimension, required maximum and selected allowance would make
  bounded manifest/history adoption easier. Preserve the compact runtime error
  and whole-schema bounds; no weaker admission or hot-path formatting is needed.
- Add a complete native DTO bridge example beside the runtime guide: bounded
  principal bytes, both conversion directions, native application errors and a
  mutating host's trap on reply-encoding failure. The primitive rules already
  exist in the guide; a worked example would reduce repeated integration code
  and clarify which conversions belong to the consumer. Preserve the runtime's
  independence from Candid and business authority.

These are qualification and ergonomics suggestions, not demonstrated runtime
correctness defects. They have not been posted upstream. Published-package and
complete Blob/Canic integration qualification remain separate work.
