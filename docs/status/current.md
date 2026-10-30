# Current status — 2026-10-30

## Source and pending batch

Released source is **0.59.0**, `cd0ce5bd`. Pending **0.59.1** implements local
CLI cleanup for [#111](https://github.com/dragginzgame/ic-blob-storage/issues/111)
and [#110](https://github.com/dragginzgame/ic-blob-storage/issues/110). Package
versions remain 0.59.0. No staging, commit, push, release, deployment, paid effect
or sibling edit. Keep the issues open until delivered source meets acceptance.

The maintained `prepare_upload`, `verify_download` and `wire_schema` examples
use Clap before opening files, writing outputs or reading stdin. Existing
inventory/snapshot and declaration/claim command forms remain. Help succeeds;
missing, conflicting, malformed or extra arguments are Clap errors. Streaming,
bounds, source custody, verification and create-only output owners are unchanged.
The schema generator's four modes emit the exact previously retained output.
Clap is a native-only development dependency of Contracts, not its runtime.

The native CLI grammar supplies typed paths, principals, URLs, choices and bounded
numbers. Request builders and offline handlers no longer reparse string matches.
Canonical decimal/principal syntax remains with the existing parsers via Clap
extension points. File index zero, command-specific ceilings, role exclusions,
local/mainnet trust policy, secret-redacted JSON errors and exit codes survive.
Production argument acquisition uses `args_os`; paths need no UTF-8 conversion.
Domain parsing of untrusted JSON and request validation remain separate boundaries.
No generic CLI framework, new user mode or fallback was introduced.

Deployment admission now explicitly rejects `clap`, `clap_builder` and
`clap_derive`, with refusal fixtures. The actual standalone normal/build Wasm
graph contains none. Other dependency policy and RPC/storage contracts are unchanged.

## Validation and evidence

Evidence: `/tmp/blob-cli-cleanup/`. All **122** native CLI tests pass, including
local HTTP fixtures. The restricted first run passed 117 and refused local socket
binding in three tests; the subsequent run with local socket access passes all
122, including the two added typed-boundary/native-path cases. Original failed
logs remain. All **24** example cases pass, including help/argument relationships,
streaming verification and retained-output safety. Actual binary invocations pass
help, early refusal before missing-file access, preparation/verification of stdin,
and exact comparison of all four generator outputs against retained artifacts.

Strict Clippy passes for CLI and Contracts across targets/features. Rust **1.91.0**
checks pass for the CLI binary and all three example binaries. Fresh release
standalone Wasm, actual runtime/deployment boundary checks, new deployment refusal
fixtures and shell lint pass. All **13** selected installed CLI cases pass:
upload setup/lost replies, seven preparation cases, account inspection, gateway
controls, funding and two installation cases. The initial account attempt lacked
the explicitly selected gateway-source Wasm; that failure is retained, and the
remaining runs pass after building and supplying the existing fixture. Manifest,
lockfile, CLI and both fixture Wasm hashes stayed fixed. No full gate, macOS, deployed
IC or actual provider/browser upload qualification was run.

The incoming Metrics **0.17.6** lockfile selection was preserved; the only added
lockfile dependency edge is Contracts' existing workspace Clap for native examples.
The captured graph uses Platform 0.8.3, Tooling/Host/Build/Testkit 0.70.3,
Memory/Storage 0.56.0 and Wire 0.7.23. This is not a broad upstream adoption.

Prior release evidence and the certificate hard cut are retained in
[the released 0.59 handoff](https://github.com/dragginzgame/ic-blob-storage/blob/cd0ce5bd/docs/status/current.md)
and `/tmp/blob-hardcut059/`, `/tmp/blob059-setup-followup/`. Historical evidence
remains bound to its captured source/graph. Formatting, links and diff checks pass.

## Remaining acceptance and boundaries

[#4](https://github.com/dragginzgame/ic-blob-storage/issues/4),
[#5](https://github.com/dragginzgame/ic-blob-storage/issues/5),
[#6](https://github.com/dragginzgame/ic-blob-storage/issues/6),
[#31](https://github.com/dragginzgame/ic-blob-storage/issues/31) and
[#34](https://github.com/dragginzgame/ic-blob-storage/issues/34) retain application/
provider acceptance under [the coordinated run](../acceptance-plan.md#next-application-acceptance-run).
A current target URL, deployed Blob principal, configuration and paid budget have
not been selected. Registration proves no stored bytes; GC acknowledgment proves
no deletion/billing cessation; funding acknowledgment is not independent credit.
Canic owns its adapter and composition qualification; no sibling was newly reviewed.

Owned schemas remain 1. Cross-release transitions use
[retirement/reinstall](../retiring-installations.md), retaining objects, balances,
uncertain effects and continuing billing. Same-release interruption recovery and
exact retries remain supported. Restoration fences, independent component binding
checks, heap oracles and frozen-image tests remain. This CLI batch changes no
Wire identity, record codec, memory allocation or recovery policy.
