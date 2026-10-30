# Current status — 2026-10-30

## Source and pending batch

Released source is **0.58.0**, `e94f521e`. Workspace package versions remain
0.58.0. Pending **0.59.0** is the approved certificate-assessment hard cut and
publication/test cleanup. No dependency update, sibling edit, staging, commit,
push, release, deployment or paid effect belongs to this batch.

Certificate assessment now returns only the original permission and assessment
time. Retired host-evidence blockers and the unused evidence-binding failure are
removed from current contracts, CLI output and tests. The canonical endpoint
catalog assigns this method Wire identity `0x424c_4f42_0059_0009` in both directions;
retired frames refuse. Endpoint names and Caffeine certificate payloads are
unchanged. Clients must rebuild. Other method identities, stable record layouts,
exact retry bytes and lifecycle semantics are unchanged. There is no dual reader.

Publication preparation passes typed requests to the existing setup executor,
using the same scope, actor and declaration validation as external CLI inputs.
Request files and signed-operation claims are still retained. Recovery validates
original journals before observation; missing, partial or uncertain claims never
authorize resubmission. Retained preparation summaries decode directly into the
existing typed view, preserving syntax/data refusal classifications.

Successful journey authority/source restoration now shares one fixture operation
using Testkit's existing exact rate-limit retry: two total attempts and a
300-second simulated cooldown. Intentional failed upgrades remain direct. No
production retry policy changed.

## Configuration ownership decision

Retain component configuration records. The installation owns configuration;
component records independently check that opened upload, gateway and funding
stores match its bindings and limits. Removing them would weaken wrong-store
admission; replacing them with hashes adds a format and hashing mechanism rather
than eliminating the check. This conditional audit candidate was not justified.
No persistent format, memory allocation or retirement workflow changes here.

## Validation and evidence

Current batch evidence: `/tmp/blob-hardcut059/`. Strict Clippy passes for core,
contracts and CLI (all targets/features), and the journey, standalone and storage
integration targets. Three bounded certificate codec cases, the frozen method
suite, five installed-owner unit cases, four setup cases, eight preparation cases
and two native assessment cases pass. The setup regression compares every
operation's exact encoded arguments and claim path with the external loader, then
proves that the in-process path needs no request file. Malformed summaries retain
syntax-versus-data refusal classification, including trailing data.

Fresh release Wasm builds pass for standalone, authority and source fixtures.
Installed checks pass: 33 journey recovery, four standalone certificate/signed
CLI and seven preparation cases, including lost acknowledgments, corrupt journals
and partial claims without resubmission. Artifact and manifest/lock hashes remain
fixed through these runs. The generated TypeScript check passes for 27 methods
and 137 frozen frames, including malformed, identity, direction, budget and decimal
cases. The restricted nested npm check returned an empty version; the same pinned
Node/npm check and schema target pass with normal subprocess permissions.
Original failed logs remain beside the passing retries. Formatting, local doc
links and diff checks pass. No full gate, new MSRV matrix, native macOS, deployed
IC, real browser upload or provider qualification was run. The preceding handoff and its exact
qualification records are retained in [the 2026-10-29 handoff](handoff-2026-10-29.md).
Historical evidence proves only its captured source/graph, not this pending cut.

The current dependency graph remains Memory/Storage 0.56.0, Platform 0.8.2,
Metrics 0.17.4, Tooling/Host/Build/Testkit 0.70.2 and Wire/derive 0.7.23.
Shared Tooling remains 0.27.3. Browser Platform 0.1.1 and SDK Core 6.1.1/Caffeine
1.1.2 are unchanged. Native transport/certificate tests retain required external
CBOR; application protocols remain Wire.

## Remaining acceptance and owner work

[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retain separate
application/provider acceptance under [the coordinated run](../acceptance-plan.md#next-application-acceptance-run).
A current target URL, deployed Blob principal, configuration and paid budget have
not been selected. Historical principals authorize no fresh run. Registration
proves no stored bytes; GC acknowledgment proves no deletion/billing cessation;
funding acknowledgment is not independent account-credit evidence.

Canic owns its adapter and composition qualification. Its optional metrics storage
work remains with [#707](https://github.com/dragginzgame/canic/issues/707) and
[PR #708](https://github.com/dragginzgame/canic/pull/708), including the recorded
window-label correction. The prior inspection and contribution graph are historical,
not a new claim about current Canic. IC Crates' retired-verifier migration is
recorded in [its owner tracker](https://github.com/dragginzgame/ic-crates/issues/1#issuecomment-6495954765).
No sibling was modified or newly qualified in this batch.

## Invariants retained

Owned schemas remain 1. Cross-release transitions use
[retirement/reinstall](../retiring-installations.md), retaining provider objects,
balances, uncertain effects and continuing billing. Memory 0.56's Wire allocation
ledger rejects prior CBOR; never clear only the ledger with collections retained.
Same-release interruption recovery and exact retries remain supported; restored
instances stay fenced until independent continuity evidence permits activation.
Component bindings, independent heap oracles and frozen-image tests remain because
they check different corruption, implementation and historical-refusal boundaries.
