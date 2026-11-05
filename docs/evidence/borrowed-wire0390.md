# IC Wire 0.5 borrowed-contract adoption

2026-10-20. Baseline: released Blob 0.38.0, source
`837310221129550a925ba057f0a76b3df2329075`, including the incoming locked
Platform 0.4.1 selection. Pending 0.39.0 changes public tenant projection lifetimes,
not runtime formats. Package versions remain 0.38.0. Published Wire 0.5.0 was
already selected; no dependency upgrade, sibling override or new package is needed.

The published archive's migration guide and API were reviewed. Wire supplies
schema/serialization; generic decode requires an explicit input-lifetime or owned
Serde bound. The archive checksum is
`bdfc9e7a6686d78854a2a06f408492313ce10c985a17976e7395fc6cef776479`.
The [machine record](https://github.com/dragginzgame/ic-blob-storage/blob/e23dfec0e06a23ae834c2fd27a9b845818f45c2a/docs/evidence/borrowed-wire0390.json) retains affected package checksums,
compiler identity, artifact/source hashes, sample counts/ranges and frame hashes.
Raw experiment sources, drivers, samples and failed/provisional attempts are
not retained by this summary. The historical record does not establish their
continued local custody or independent reproducibility.

## Ownership and implementation

Finite tenant schemas use bounded `BytesRef<29>` principal views, converting to
owned native DTOs before policy/async work. Generic service/document decoders admit
borrowed bytes/text. One generic download descriptor layout defaults to owned
`String`; the native CLI selects `TextRef` into its retained authenticated reply.
The owned and borrowed forms share metadata/binding validation and refusal codes.
There is no second schema registry, compatibility format or automatic migration.

Records, stable keys, retained retry frames and replicated async results stay
owned. No persisted representation changed: model/record/storage implementations
and all frozen method/lifecycle vectors are unchanged. The explicit before/after
same-schema fixture reads existing enrollment/manifest records, retains exact
retry bytes and obligations, and confirms all restore fences before refusing
mutation. It does not establish cross-release production migration support.

Blob has no Wire chunk-body upload/download endpoint. Upload bodies use Caffeine
HTTP; async read transports own returned buffers. Native HTTP already passes
borrowed slices to verification/output. Those protocols and ownership are retained.
Bounded chunk views are exercised through the generic document API and separate
codec controls; they do not turn a body-transfer path into a Blob RPC.

## Method and reproducibility

All comparisons use Rust 1.99.1 (`58cf8f9c3`, LLVM 23.1.1), default Cargo release,
`wasm32-unknown-unknown`, without a Binaryen pass. Baseline and candidate use
the same captured lock (`7560ef2dcf82b11d11ca7dc0532c0d45940042a51929731275702cb05f1974b6`),
Wire 0.5.0, Platform 0.4.1, Memory 0.49.2, Storage 0.5.8 and Testkit 0.59.0.
The identical native test driver uses local PocketIC 16.1.0 on Linux x86_64.
Later incoming Host 0.31.2 selections are preserved for final native checks;
the measured captures retain Host 0.31.1. Deployment selects neither Host version.

Temporary controller-only probes mark decode, workflow and encode boundaries.
Their sum excludes Platform's initial admission/ingress copy and final reply
append/delivery. Workflow includes actual policy, storage and continuity work.
These are measured application regions, not full transport cost, throughput or
cycles. Instrumentation/code layout can affect absolute IC cost; the fixed 80,000
instruction difference in final descriptor decode is reported, without attributing
it to a proven production cause. No benchmark endpoint is in production source.

Each cold first preparation has one sample; exact preparation replay and exposed,
independently attested descriptor delivery each have ten samples, reported as
medians. Small declares 1 KiB with 24 repeated metadata characters; heavy declares
1 MiB with 700. Body bytes are not submitted to these metadata endpoints.
First-call effects are retained separately; no warmup is discarded.

Allocation-observer builds forward the standard allocator and count requests,
reallocations and requested/deallocated bytes over the same region. Their
instruction counts are excluded from performance comparisons. IC linear pages
are not live/peak heap. Native codec probes count peak additional live bytes,
excluding pre-existing input; they omit metadata validation, transport and storage.
The async retained-heap comparison below is a native model, not an IC heap reading.

Initial shared-target captures suffered stale build-cache mixing and were marked
provisional. Accepted comparisons were rebuilt in distinct target directories
for each captured source; they use the same driver. The historical report records the initial cache, compilation, admission and
test failures; raw logs are no longer retained here.

## Final measurements

| Actual metadata operation | Before instructions | Final instructions | Change |
| --- | ---: | ---: | ---: |
| Small first upload preparation | 1,904,438 | 1,904,430 | effectively unchanged |
| Small exact replay | 1,207,843 | 1,207,835 | effectively unchanged |
| Heavy first upload preparation | 2,021,756 | 2,021,748 | effectively unchanged |
| Heavy exact replay | 1,318,023 | 1,318,015 | effectively unchanged |
| Small download descriptor | 1,383,820 | 1,463,820 | +5.78% |
| Heavy download descriptor | 1,387,645 | 1,467,645 | +5.77% |

Request/reply sizes and every canonical frame hash match before/final. Upload
requests are 269/950 bytes; descriptor replies are 240/921 bytes. Retained retries
use exact original bytes. Endpoint IC allocation totals are unchanged: small
first preparation makes 1,115 allocations and 104 reallocations requesting 57,270
bytes; replay makes 776/56 requesting 38,461; descriptor makes 724/42 requesting
33,956. Heavy totals and all ranges are in the machine record. Metadata operations
observe the same 1,310,720 bytes of allocated linear memory at all four marks.

Native descriptor raw decode allocations fall **6 to 1**. Requested/peak extra
bytes fall **306 to 128** (small) and **985 to 128** (heavy). Retained input frames
already belong to evidence capture; borrowing does not add another retained frame.
The header list and shared validation still allocate. This is a codec allocation
result, not a measured whole native-download or IC speedup.

| Codec-only chunk/text control | Before instructions | Final instructions |
| --- | ---: | ---: |
| Empty body | 4,449 | 83,983 |
| 1 KiB | 16,720 | 93,207 |
| 1 MiB | 17,052,701 | 11,819,364 |

These controls decode, scan every byte and re-encode the same frame. The 1 MiB
total falls 30.69%; its decode region falls 5,315,369 to 1,992 instructions.
The smaller total cases regress; no universal codec/endpoint gain is claimed.
The 1 MiB control observes allocated linear memory up to 4,653,056 bytes
before and 3,538,944 after; these pages are not live-heap peaks.
Raw native 1 MiB decode eliminates two allocations and 1,048,600 requested/peak
extra bytes. The IC control, including output allocation, falls three allocation
requests to one and 2,097,215 requested bytes to 1,048,615. Encoded sizes remain
37, 1,062 and 1,048,615 bytes. Controls compare Wire's owned `Bytes`/`String`
with `BytesRef`/`TextRef`, not a raw element-wise `Vec<u8>` sequence, which has
different node-budget applicability at 1 MiB.

Uninstrumented default standalone linked Wasm falls **3,513,165 to 3,511,992
bytes** (−1,173); deterministic gzip level 9, mtime zero falls **742,223 to
742,177** (−46). No production benchmark probes or allocator observer are linked.
These are unoptimized-by-Binaryen deployment observations, not an O3/Os/Oz matrix.

## Rejected experiment and remaining limits

Borrowing upload metadata across the continuity await cut raw native decode from
six allocations to two, but retained the whole ingress frame: the native retained
heap model rose 402 to 525 bytes (small) and 1,081 to 1,206 (heavy). Isolated IC
first preparation regressed 3.74–3.88% and replay 5.89–6.12%. That experiment was
reverted; the final upload request remains owned. Trial source/results are recorded
separately in the historical machine record, not presented as accepted implementation gains.

The descriptor IC instruction regression remains a qualification/performance
limitation. Its allocation benefit applies to native consumers retaining a reply,
not the deployed descriptor producer. No storage/transport performance guarantee
follows from the codec controls. This pass does not qualify native macOS, installed
Canic, a live provider, full CI or the optimized-Wasm matrix. Cross-release service
transitions still require safe retirement and reinstall.


## Qualification scope

Historical focused native, installed substitute, Chromium, malformed-input and
old-minimum compiler checks passed for that captured graph. These results do not
qualify the current graph; the complete old check inventory remains in Git.
