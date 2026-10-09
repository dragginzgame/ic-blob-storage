# Coordinated 0.20.0 dependency and tooling hard cut

The final approved graph selects Host artifacts/fs/process/tools 0.9.1,
Testkit 0.26.0, public Memory 0.32.0, private Metrics 0.3.0 and PocketIC 16.1.0.
Shared Tooling's canonical exporter refreshes the same 93 files from committed
0.2.2 `ee48bb37c98c771e77b92fd891f0757d8c1c8b99`. Package versions and the release
receipt remain 0.19.3; pending notes remain 0.20.0. Consumers must rebuild against
the new public Memory type identity. Memory runtime source and Metrics arithmetic
are unchanged; no durable schema conversion or alias is introduced. Existing
cross-release retirement/reinstall and same-contract recovery obligations remain.

The [initial handoff](pocketic-handoff0200.md) retains Shared 0.2.0 and the earlier
dual Host graph. Testkit 0.26 now shares Host 0.9.1 with direct consumers; it owns
server setup, admission and compatibility. Its checker releases the decoded
executable buffer before byte admission. Host simplifies durable publication
without changing permissions and releases captured pipes at EOF. Blob adds no
parallel implementation, lifecycle owner or named function/type deletion.

Shared 0.2.1's adopted IC installer processes a final row without a newline.
Its fixtures cover installation and refusal of changed final-tool versions for
all three synthetic host selections, under Bash 5 and actual Linux-built Bash
3.2.57. Shared 0.2.2's optional CI installer is outside this snapshot selection;
its updated canonical documentation is included without adding unused callers.

## Exact source and local qualification

Raw captures are retained under `.tmp/pocketic-handoff-01/`. All seven selected
registry archives match locked checksums; all 233 packaged Rust/original manifest
files match their recorded owner commits. `package-sources-coordinated.json`
records those identities. The final manifest freezes 13,919 build inputs and the
selected lock; they remain unchanged through qualification. The
[intent](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/coordinated-intent.json)
and [summary](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/coordinated-summary.json)
bind final results and exact executed artifacts:

- All 242 native-host cases pass, including actual standalone installation and
  Metrics restoration. Strict affected CLI/harness lint and actual Rust 1.88
  native/Wasm compilation pass.
- Explicit lock-selected Testkit 0.26 CLI setup succeeds. Its real offline check
  passes with `PATH=/nonexistent`; the authenticated server bytes are unchanged.
  The earlier CLI, six-tool bundle, all receipt-covered bytes and failed candidates
  remain retained. Default Make callers obtain the checked path; explicit binary
  overrides retain caller-owned byte admission and per-instance ownership.
- Snapshot, actual offline tools, declarations, formatting/hooks, links, shell
  lint, failure archive round trips and evidence consistency have separate logs.

The intermediate Host 0.9.1 run reports 242 passes, but concurrent incoming
Memory/Metrics/Testkit edits fail final source consistency. It is not attributed
to the final graph. A subsequent command continued after a failed pre-dispatch
assertion and was interrupted during Wasm builds before tests; zero probe GETs
or PocketIC launches occurred. [The interruption record](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/interruption.json)
retains this limitation. The final run uses a fresh intent and unchanged inputs.
Six unrelated broad-range Windows bindings remain at their incoming selections;
the intermediate Cargo rebindings are retained separately.

Four completed native runs used 16 explicit local probe GETs in total. No live
provider requests, deployed paid effects or external cleanup. Successful temporary
HTTP request/body files were not separately archived; assertions, logs, executed
binaries and hashes survive. Owned local scopes close. These local substitutes
do not qualify deployed Caffeine, consumer frameworks or production measurements.

Exact selected [Shared](https://github.com/dragginzgame/shared-tooling/actions/runs/37918955655),
[Host](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37919257776) and
[Testkit](https://github.com/dragginzgame/ic-testkit/actions/runs/37917989870)
CI remain queued at the readback. Released Blob 0.19.3's green native CI closes
#37/#38 only. [#39](https://github.com/dragginzgame/ic-blob-storage/issues/39) retains
the changed consumer's committed native acceptance; full release gates remain
outstanding. No sibling edit, agent commit, push, release or publication occurs.
