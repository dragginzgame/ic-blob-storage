# GC and credit contract follow-up

Source baseline: Blob 0.41.1, `d4dbfa1006106ae07ce47f207acdd3af294fc7d7`.
This is public source/documentation evidence, not deployed-provider qualification.
The intent files precede their separately bounded requests. No account, provider
object, canister, payment or deployment request occurred; no external cleanup is
needed and existing obligations remain unchanged.

One web search with three queries, two official guide reads, two provider-scoped
GitHub code searches, two exact source reads and two issue searches completed.
The code searches each returned one result, both in `caffeinelabs/skills` at
`d03174cc42ab2833293ba1cb313b7593a611a73f`. The downloaded `Mixin.mo` and
`Storage.mo` are byte-identical to the October 21 capture. Request metadata binds
status, byte counts and hashes; stderr is retained, including empty stderr.

The issue searches returned one unrelated storage-guidance discussion and no
Cashier discussion. Search results were not marked incomplete. This bounded
indexed search is not proof that no authoritative contract exists elsewhere.

The official [storage cost guide](https://help.caffeine.ai/hc/en-us/articles/49362898986644-File-Storage-Costs)
describes retail credit charging and next-day storage-charge exclusion after file
deletion. The [troubleshooting guide](https://help.caffeine.ai/hc/en-us/articles/46899784111380-File-Storage-Troubleshooting)
describes asynchronous removal after references disappear. Neither specifies the
Rust callback key mapping, gateway failure handling or Cashier attribution.
Retail credits must not be equated with exact cycle-credit or deletion receipts.
Raw web-tool responses remain at the local paths/hash identities in the portable
JSON indexes; full help articles and irrelevant search excerpts are not copied
into the maintained evidence tree. No failed or inconclusive result is discarded.

For #32, local Motoko pruning does not establish remote deletion, callback failure
policy or billing cessation. For #34, the public helper's unit-return declaration
must not be confused with the retained Cashier DID's structured result. Blob
already decodes that result. Its balance/message fields alone do not document
whether credit is atomic with the matching successful callback, nor how a lost
callback can be attributed using the audit CSV. Clarifying an existing provider
guarantee may be sufficient; a new receipt service is not assumed necessary.

No runtime change or issue closure is justified by this evidence. Two concrete
provider questions are prepared locally; posting outside `dragginzgame` requires
explicit target/action authorization. Source review cannot replace the missing
provider contract. No paid experiment is authorized or proposed for execution.
