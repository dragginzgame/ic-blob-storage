# Current status

Date: 2026-09-25

## Released baseline

The maintainer reports 0.1.3 live. Local Git release is `8d4e228`, from source
`ee3530a`; Cargo and the receipt are 0.1.3. Registry publication was not independently
queried. Release/publication retain build artifacts; only explicit `make clean`
removes them. See [release guidance](../releasing.md).

The release-test cleanup and documentation consolidation are released. The
core provides distinct content/provider identities, incremental raw-byte
verification and pure funding/readiness policy. Provider bindings, persisted
workflows, clients, endpoints and adapters remain unimplemented.

## Current implementation batch

The maintainer requested continued Canic replication. The next numeric billing
slice ports Candid/operator inputs and complete configuration-candidate validation
into `ops::billing` and the model. Balances reject negative/oversized values;
funding amounts must be positive ASCII decimal or bounded Candid numbers.
Whole-balance conversion checks total, prepaid, promotional and ledger amounts
together; a malformed component prevents a valid total from reaching readiness.
Configuration combines a non-special Cashier principal, validated funding limits
and positive gateway bounds that fit 32-bit Wasm even on a 64-bit host. It chooses
no account binding or deployment defaults and installs no persistent state.

The continuation also ports gateway normalization into `model::gateway`.
Nonempty lists reject anonymous/management principals, deduplicate in provider
order and enforce separate raw/distinct bounds. Replacement validates fully
before changing the transient value and retains its original limits. Rejected
lists preserve prior membership; tests cover recovery and repeat replacement.
`GatewayMembership` adds idempotent individual add/remove and replacement; an
operator can empty membership, while empty provider sync input still rejects.
This is validated data, not a persisted gateway registry or callback authority.

Pure funding-intent admission now rejects recovery fences, in-progress/uncertain
effects, absent configuration and reserve violations. It preserves full requests
and offers no expiry-based release. Composition tests distinguish diagnostic
top-up arithmetic from admission. This corrects the decision boundary around
Canic's transient funding guard without implementing locks, persistence, retries
or effects. The source guard and billing workflow still match captured hashes.

Native boundary tests and actual Candid encode/decode composition tests pass,
as do strict Clippy, Wasm compilation, docs and formatting. These are targeted
checks, not full CI or provider qualification. Canic's three relevant source
files plus policy/workflow configuration sources still match the captured
inventory despite unrelated sibling changes. Composition tests connect validated
configuration to gateway bounds and funding/readiness diagnosis.
No sibling edits, dependency updates, paid provider effects or version actions ran.

The maintainer directed provider discovery through Canic, Toko and public sources.
Toko indexed commit `6519b72d2a420564dabaf700fc55f7b8603d9fd3` supplies defaults
`https://blob.caffeine.ai` and Cashier `72ch2-fiaaa-aaaar-qbsvq-cai`. Anonymous
mainnet metadata retrieval and gateway-list/pricing queries succeeded. Retained
[deployment evidence](../evidence/caffeine-deployment-observation.json) shows both
gateway names advertised; `didc` proves the newer top-up wrapper's Candid type
compatibility with the deployed signature. Source differences alone were not
proof of a wire break. No update, payment or private account lookup ran.

[Core evidence](../evidence/core-primitives.md) and
[the capability inventory](../canic-capabilities.json) record partial BLOB-07/10
gateway validation and BLOB-08/09/11/15 billing input coverage. Historical evidence is
preserved; the new batch has separate source hashes. The maintainer requested
the [0.1.4 changelog draft](../../CHANGELOG.md), now consolidated under an undated
0.1.4 heading after empty Unreleased. Cargo and the release receipt remain 0.1.3;
no version transaction, commit or publication ran.

## Authority and next work

The maintainer confirmed Canic's 0.110 human acceptance for work here without
changing Canic's handoff. The continuing local port covers content/numeric
policy, configuration/input conversion, transient gateway validation and
funding-intent admission from supplied observations. It does not implement
a provider wire contract or persisted workflows. Those remain gated by the
[service contract](../service-contract.md). Library publication is separately
enabled; publishing these primitives does not qualify the service.
Agents must not create commits, and sibling repositories remain read-only.

All Canic blob functionality, including operator commands and diagnostics,
must work here before removal there. The [parity contract](../canic-parity.md)
owns the source/removal inventory and installation obligations. A numeric
parser is not the operator CLI; provider sync/funding and both real deployment
journeys remain necessary.

The [provider baseline](../provider-baseline.json) records client 1.1.2 and
backend 1.1.1, verified on 2026-09-25. Candidate deployment/interface discovery
has progressed; server revision, account binding and retry/retention/deletion/
billing guarantees remain unresolved in the [provider review](../provider-review.md).
Next qualify those guarantees and freeze consumer, owners, bounds and restore
contract. Recheck upstream before provider bindings. Caffeine remains unqualified;
the [acceptance plan](../acceptance-plan.md) defines the remaining cases.
