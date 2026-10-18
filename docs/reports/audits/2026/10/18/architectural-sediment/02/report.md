# Architectural sediment follow-up

2026-10-18. Local implementation audit under the root AGENTS overlay and reviewed
Shared Tooling 0.23.0 baseline. HEAD remains `e938a1d`; actual package versions
remain 0.32.2 and the pending batch remains 0.33.0. This pass started with the
[first audit's changes](../01/report.md) already present and preserves them.
Raw before-images, incoming diff and command logs are retained in
`.tmp/review-validation/sediment0330-02/`.

## Boundary and evidence

Blob owns service records, tenant authority, quotas, references, funding and
recovery obligations, passive contracts, the standalone adapter and its CLI.
Public entry points are the core installation/workflow APIs, Contracts' method
catalog and codecs, explicitly mounted standalone endpoints and CLI commands.
Linking the library mounts no endpoints or lifecycle hooks. Consumer frameworks
own their wrappers and integration qualification outside this repository.

IC Memory owns allocation/bootstrap and exclusive leases; IC Storage owns stable
collections and codec traits. IC Wire owns framing for the two tenant methods.
Other owned methods/lifecycle still use CBOR; provider contracts remain Candid.
Host owns native filesystem/process/artifact mechanics; Testkit owns local IC
execution and Candid transport helpers, and Shared Tooling owns common tooling.
The ownership table and read-only sibling identities in the first report remain
the wider evidence record. This follow-up additionally checked published Testkit
0.52/0.53 call implementations and the sibling changelog. Published registry
source, not the mutable sibling, establishes availability of the replacement.
No sibling files were changed or downstream deployed behavior inferred.

## Findings and changes

1. **Incomplete Testkit migration, fixed.** The lock already selected 0.53,
   but twelve native imports and 250 call sites still used its removed
   `CandidCallExt` surface. The first package check reproduced unresolved imports
   and methods before any caller migration. Tests now call Testkit's published
   `query_candid_args` / `update_candid_args` directly. One-result replies are
   decoded as `(T,)` and unwrapped locally; original arguments, explicit callers,
   budgets and application results remain. No local replacement trait, helper
   module, dependency change or production endpoint change was introduced.
   Affected consumers are the existing PocketIC fixture suites, including frozen
   Candid fixtures, not service clients. Compilation covers all native test
   targets; the installed content path exercises accepted and refused results.
2. **CBOR-era tenant test overlap, removed.** In
   `tests/pocketic/tests/standalone_ingress/mod.rs`, the old admission test repeated
   tenant refusal/success coverage now owned by the dedicated Wire test. It now
   focuses on failed-installation rollback. The distinct old-Candid query refusal
   was moved to the Wire case rather than deleted. Stable-byte, tenant and saved
   configuration assertions remain around installation failures. All three
   installed ingress cases pass against freshly built standalone Wasm.
3. **Fixture encoding duplication, removed.** Journey root text, readback and
   admission resource hashes, and publication-check artifact hashes now use the
   already-declared `hex` dependency. SHA-256 calculation, prefixes and file
   ownership stay local. No provider grammar or identity parser was generalized.
4. **Stale active ownership guide, corrected.** `docs/service-contract.md` no
   longer describes pending 0.31/0.32 adoption, Memory 0.36/0.44 as the current
   package identity, or Memory as the stable-collection owner. It points to the
   selected dependency relationship and distinguishes Wire from CBOR ingress.
   Historical release evidence and frozen fixtures remain unchanged.

## Deliberately retained boundaries

The private storage probe's extra manager supports allocation-level observation
and fault injection; replacing it requires a suitable upstream API. Retained-file
adapters preserve failed/partial bytes and exact recovery identity, unlike ordinary
atomic-file publication. Provider decoder budgets test an external Candid contract
independently of owned ingress. Domain identity parsing retains its own grammar
and errors. Snapshot scripts are reviewed Shared Tooling distribution, not forks.
These are necessary integrations, not competing canonical implementations.

Recommendation: keep allocation instrumentation upstream work separate and qualify
any replacement before deleting the local observer. Completing the remaining
CBOR-to-Wire conversion needs a separately scoped protocol cut and consumer
qualification. Neither decision blocks this cleanup; no unresolved decision was
used to weaken retained obligations or supported contracts.

## Verification and limits

- Offline locked check and strict Clippy pass for all PocketIC test targets.
- Fresh release Wasm builds pass for standalone and authority probe.
- Installed ingress: three passing tests; content target: two passing tests.
- Formatting, documentation links, whitespace and stale-call searches pass.
- Root manifest, lockfile and snapshot hashes match the first audit's inputs.
- Initial compile failure was the pre-existing removed Testkit API; it is fixed.
  Intermediate lint failures from caller edits were corrected before handoff.

No full CI, full installed suite, browser run, live provider probe, publication,
release or downstream service qualification ran. Compiling all callers does not
establish runtime acceptance for every fixture scenario. Verdict: local repairs
verified within this scope, with the explicit runtime coverage limits above.
