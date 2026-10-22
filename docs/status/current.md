# Current status

Date: 2026-10-22

## Delivered source and pending work

Released **0.41.0**, tag `v0.41.0`, is on verified GitHub default main at
`d15c64c0f43d06c89286c049c593966a6adc5a2d`. The matching remote annotated tag
is `09474b8e1a6619a860e40716d15dcd30e6096a4d`. Package versions remain 0.41.0.
The next notes are **0.41.1**, covering adoption/acceptance documentation only.
Do not stage, commit, release or deploy as part of ordinary continuation.

Preserve the incoming Cargo.lock change selecting Wire/derive 0.5.21 instead of
released 0.5.20. This pass did not update dependencies or qualify that patch.
The released graph uses Platform 0.6.7, Host/Build/Testkit 0.64.0,
Memory/Storage 0.50.10 and Metrics 0.15.5. Shared Tooling remains the reviewed
125-file 0.26.4 snapshot at `dc1ced7e240e90f0c6daa89fb6248fcb074164eb`.

## Current contract and ownership

The [trusted-registration hard cut](../trusted-registration.md) is delivered.
Certificate issuance atomically registers retention intent and the first live
reference before returning upload authority. There is no mandatory verifier,
attestation endpoint or worker. Registration does not prove uploaded bytes,
availability or provider retention. Actual downloads still verify original bytes.

There are 27 owned Wire methods. Native service/lifecycle frames use the `0041`
family; the two finite tenant frames retain their bytes. Passive documents keep
their separate identity and are not service frames. Installation/state changes
require [retirement/reinstall](../retiring-installations.md), with old tools and
obligations preserved. Same-release recovery keeps exact retries, restore fences,
references, balances and physical/billing liabilities. No dual readers or migration.

Blob owns tenant policy, data, provider economics, references and durable journals,
the standalone adapter and native/browser primitives. Platform owns IC runtime
and external Candid; Wire owns application framing. Tooling owns Host/Build/Testkit;
Persistence owns Memory/Storage. Canic owns mounting/lifecycle/metrics composition
and its tests; applications own login, grants, asset publication and serving policy.
Siblings remain read-only. Do not copy downstream adapters/tests into this repo.

## Consumer acceptance and provider boundary

Canic reports local framework qualification on published Blob/contracts 0.41.0
and Testkit 0.64.0: both exact managed dedicated/embedded build/install/recovery
cases, two-uploader registration/reference journeys, public metrics, strict and
isolated-feature Clippy, installation decoders and three-workspace dependencies
pass. See its [mounted/recovery report](https://github.com/dragginzgame/canic/issues/620#issuecomment-6338568536)
and [metrics report](https://github.com/dragginzgame/canic/issues/621#issuecomment-6338568898).
These are owner-reported local results, not executions in this repo. Evidence
stays at `/tmp/canic-blob041/` with Canic; older 0.40.4 captures retain their graph.
The changes remain uncommitted. Canic still owns default-branch/coherent delivery;
native macOS and MSRV were not rerun. Keep delivery open under
[#444](https://github.com/dragginzgame/canic/issues/444),
[#620](https://github.com/dragginzgame/canic/issues/620) and
[#621](https://github.com/dragginzgame/canic/issues/621).

[Blob #33](https://github.com/dragginzgame/ic-blob-storage/issues/33#issuecomment-6338540006)
is closed as a retired requirement after verified default-main delivery, not as
qualification of an operated verifier. Current #31/#4 acceptance no longer
requires that worker; historical captures retain their original meaning.

[#32](https://github.com/dragginzgame/ic-blob-storage/issues/32) remains blocked on
gateway caller/failure semantics, provider-key association and deletion evidence.
The [October 22 source discovery](../evidence/caffeine-probes/local/2026-10-22-gc-contract-discovery-01/summary.md)
proves local Motoko weak-reference pruning, not remote deletion or billing.
No new provider requests were made. Do not expose placeholder GC callbacks or
connect pruning to physical/billing settlement. The [GC review](../gateway-gc-contract.md)
names the remaining facts needed before shared handlers can be implemented.

[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retains authenticated
project-payer linkage and unique top-up credit evidence. A successful reply or
balance change cannot unblock uncertain paid effects. Canic #494 owns guarded
composition after that contract exists. [#31](https://github.com/dragginzgame/ic-blob-storage/issues/31),
[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5) and
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6) retain actual users,
media-map transactions, serving/CSP/cache, overlapping references and deployed
provider acceptance. The [acceptance plan](../acceptance-plan.md) applies the
released registration contract; local substitutes cannot close deployed acceptance.

## Evidence and continuation

Hard-cut source/artifact identities and focused logs remain under
`.tmp/review-validation/hardcut041/`; Host 0.64 metadata, singleton/deployment
checks, strict Clippy, Rust 1.91 native checks and local installed certificate
proof remain under `.tmp/review-validation/host064/`. Stale native/upload test
repairs retain `.tmp/review-validation/registration-observation-fix/` and
`.tmp/review-validation/uploads-registration-fix/`. These logs retain failed
attempts and their distinct graphs; the released source preserves the older
[detailed handoff](https://github.com/dragginzgame/ic-blob-storage/blob/d15c64c0f43d06c89286c049c593966a6adc5a2d/docs/status/current.md).

This closeout changes only current guidance/issues. Documentation links and
whitespace are checked; no compiler, provider, browser or Canic execution is
claimed for it. Managed tools passed the maintainer's earlier CI admission;
installation remains explicit. No fresh native macOS, installed-IC or live
Caffeine qualification is claimed. Frozen source/provider captures remain intact.
