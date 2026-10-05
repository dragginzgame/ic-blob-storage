# Shared Tooling feedback

This is ic-blob-storage's maintained feedback record for reusable principles and
tools. Product architecture, provider economics, retirement procedures, release
authority and exact validation commands stay local. Shared Tooling's baseline
now owns common engineering and pre-1.0 hard-cut rules; its existing principles
explain canonical ownership, simplicity and maintained behavioral tests.

## Reviewed source

Reviewed on 2026-10-04 against Shared Tooling commit
`956236a3848c2cfae6ae05f5c77e9c37b01b3366` and this consumer's base
`3a5bcadb349756abf8595bd199e3100b7dc61953`, including its current local changes.

The initial review included the maintainer-authorized, uncommitted changelog
convention in Shared Tooling's AGENTS.md. That reviewed SHA-256 was
`7d9a581429414d05711c4bb647dfc9a26557bfe09e515b821cec501babb6fb1e`.

The maintainer then explicitly requested an upstream baseline for all
Dragginzgame repositories. Only Shared Tooling's AGENTS.md was changed. Its new
reviewed SHA-256 is
`f3d549197a3f8bde1140b38a04a10412e20a117ddef54e9410f060cbad8aef87`.
Neither dirty-file identity is claimed to belong to the committed revision.
No declared Shared Tooling snapshot is installed here; local instructions remain
the executable authority and CI does not read the sibling checkout.

Inspected Shared Tooling's AGENTS.md, CONTRIBUTING.md, principle documents,
snapshot guidance, supported-host guidance, LOC script and validation runner.
This is source/document review, not execution or platform qualification of those
tools. The subsequent authorized write changes only the upstream AGENTS.md;
there is no commit, submission, snapshot refresh or portable-tool execution.

## Shared improvements and disposition

| Item | Smallest shared change | Consumer evidence | Disposition |
| --- | --- | --- | --- |
| F1: feedback ownership | AGENTS.md specifies one consumer record, reviewed revision/dirty-source identity, affected owner, symptom, evidence, smallest proposal, disposition and separate acceptance/adoption tracking. | CONTRIBUTING.md defines inclusion and validation but no intake or disposition workflow; provenance records initial Canic/IcyDB contributions. | Implemented in the authorized upstream working file and recorded locally. Additional CONTRIBUTING guidance remains a proposal. |
| F2: artifact preservation and evidence identity | Preserve consumer-owned build/evidence artifacts through release/deployment; separate explicit cleanup. Bind observations to actual source, inputs and artifacts; distinguish evidence classes. | [Development governance](development.md) separates cleanup; [retained media evidence](../evidence/caffeine-probes/local/2026-10-04-released-media-v0146-01/summary.json) freezes matching binaries; [memory measurements](../evidence/caffeine-probes/local/2026-10-04-transfer-memory-01/summary.json) distinguish fixture savings from production cost. | Implemented in upstream AGENTS.md; owned temporary-file cleanup remains allowed. No host/performance qualification claimed. |
| F3: contract propagation through recovery | AGENTS.md clarifies coherent changes: trace producers, consumers, codecs, generated artifacts, persisted data and installation/recovery helpers. Reuse the canonical encoder. | [Funding fixture correction](../evidence/caffeine-probes/local/2026-10-04-storage-funding-upgrade-01/summary.json): one missed upgrade encoder caused five failures after funding assertions passed; the shared encoder fixes all affected tests. | Implemented in upstream AGENTS.md; no new principle or blanket full-suite requirement. |
| F4: offline validation prerequisites | Populate caches for the selected lockfile in an explicit prerequisite phase; preserve lock selection and stop before release mutation if preparation fails. Consumers own exact commands and network authority. | [Dependency setup](../dependencies.md#setup-and-checks), Makefile's deps/ci ordering and isolated dependency-bootstrap tests in scripts/release/test-release.sh. | Implemented in upstream AGENTS.md; no shared Cargo wrapper or automatic network request. |

The baseline also records common scope/authority, canonical ownership, pre-1.0
hard cuts and unversioned/V1 models, caller propagation, test discipline, Cargo
workspace inheritance and changelog rules. Shared Tooling's own validation
commands are explicitly separate from consumer commands. Product-specific
architecture, limits and effect/retirement procedures stay local.

Automatic approval review rejected accompanying CONTRIBUTING.md and principle
index edits because sibling authority named AGENTS.md only. The safe alternative
applied AGENTS.md alone, with self-contained feedback fields and an explicit rule
that consumer choices in linked guides are subject to the baseline. Supporting
clarifications remain proposals: CONTRIBUTING can explain the fields above and
acceptance/adoption workflow; the principle index can clarify baseline versus
local-overlay authority. Neither file changed. No reporter, schema registry, CI
gate or recurring job is needed.

Documentation links and diff checks pass. No portable script suite or host
qualification was run because no shared script changed.

## Local adoption action

L1: replace the current LOC script with a reviewed shared snapshot, rather than
implementing workspace discovery again. The current script fixes its root at
`crates/` and iterates direct children; Cargo.toml also has members under
`canisters/` and `tests/`. The shared script already selects workspace members
from Cargo metadata. This is a confirmed local coverage difference, not a shared
tool defect or a measured performance finding.

Reviewed script hashes:

- Local scripts/dev/cloc.sh:
  `d0bfe298f08dcdddef7d18a5036d7febda631ef7b507e4f07378ab006cc7729e`.
- Shared scripts/dev/cloc.sh at the reviewed commit:
  `97a8f42bf3e8dbd1d89683eacbcf28c551bb87eda995551e2bcfa04e20d3a853`.

Use Shared Tooling's existing declared-file snapshot distribution and verifier
when adopting it. Select a clean reviewed source, update Make's description and
check actual member coverage on this workspace. Review metadata/network behavior
against this repository's offline policy. Do not patch a vendored copy or invent
source provenance for the old script. Adoption remains open; no refresh ran.

## Maintaining and delivering feedback

1. Add or update an item only for a demonstrated reusable gap or adoption issue.
   Include the shared revision or exact dirty-file identity, affected owner,
   observable symptom, focused reproduction/evidence and smallest proposal.
2. Link the item from docs/status/current.md and mention open actions at handoff.
   Keep facts and disposition here instead of duplicating a backlog elsewhere.
3. If the maintainer requests submission or shared edits, use the existing issue
   or a focused contribution. Include consumer policy boundaries, affected hosts,
   required callers and the smallest relevant verification. Recording an item
   alone grants no external write or network-effect authority.
4. After a shared decision, record its issue/commit or rationale. Acceptance is
   not adoption: close the consumer action only after its reviewed snapshot/local
   overlay and relevant verification land here. If resolved locally, record why
   sharing was unnecessary. Never claim unsent feedback was submitted or accepted.
