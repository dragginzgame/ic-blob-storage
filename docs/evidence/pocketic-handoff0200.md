# PocketIC ownership and Host 0.9 hard cut

This record retains initial Shared 0.2.0/Host 0.9.0 qualification. The
[coordinated follow-up](coordinated-hardcut0200.md) owns the final Shared 0.2.2,
Host 0.9.1, Testkit 0.26, Memory 0.32 and Metrics 0.3 graph; earlier results
are not relabelled.

Pending **0.20.0** replaces the maintained developer tool setup contract. Shared
Tooling's five-tool bundle no longer provides `.tools/ic/bin/pocket-ic`; Testkit
owns explicit server setup and offline admission. Run `make install-tools` before
validation. `make install-testkit` / `make testkit-check` provide the focused setup
and check. All PocketIC Make test callers default to Testkit's admitted absolute path,
including native, browser, resource, standalone and old-image refusal targets.
Explicit environment/Make `POCKET_IC_BIN` overrides retain caller-owned byte
admission and pass unchanged to the harness. The default never selects a server
from the old shared path. The harness's
per-instance owned servers and instance-before-server drop order are unchanged.

The consumer adapter reads the single Testkit selection from Cargo.lock and
delegates CLI installation to the canonical Cargo installer, then calls the
published CLI's setup/check. It owns neither download assets nor compatibility
policy. Missing/ambiguous lock selections and owner failures stop dispatch;
ordinary checks never provision. Failure collection includes the selected CLI,
Testkit server candidates and consumer routing fixtures, preserving old prefixes.

## Source and retained installation

Current implementation base is `9a5173d365d2c11889ea2e813c9645ab366fbffe`, following
released 0.19.3 `a2081319dee7e23e20e6ce587437edba73549258`. Package metadata/receipt
remain 0.19.3; only the pending changelog moves from 0.19.4 to 0.20.0. Service
APIs, DTOs and durable formats do not change. No named function, method or type
is removed; the retired PocketIC checker files were already absent here.

The canonical exporter adopts the same 93 paths from clean committed Shared
0.2.0 `8140e3dd1b44409d682c721889ab702f438c6a17`. The prior
[hook](shared0370194.md) and [exception](shared0380194.md) records keep their inputs.
Direct Host artifacts/fs 0.9.0 use unchanged packaged Rust sources from 0.8.10;
their registry archives match the lock and source matches owner
`715854b47b888eefb6fc0fca76d9c99f55099150`. Testkit 0.25.5 retains private Host
0.8.10; those values do not cross the direct Host type boundary. The approved
incoming Memory 0.31.11/private Metrics 0.2.20 selections are preserved. Their
changes affect owner tooling, not packaged runtime sources. All seven reviewed
package archives and their Rust/original manifests match their owner commits.

Before explicit setup, `.tmp/pocketic-handoff-01/setup-intent.json` records the old
active bundle and every receipt-covered hash. The new offline checker correctly
refuses that six-tool selection; the raw failure log survives. Explicit official
five-tool setup and Testkit 0.25.5 CLI/server setup succeed. All old recorded bytes,
pins and receipts remain unchanged. The new bundle contains no PocketIC. Testkit's
real check passes with `PATH=/nonexistent`, using its retained authenticated bundle.

## Verification and limits

Raw logs, package source comparisons, 13,960 frozen compiler input hashes and
tool readback are retained under `.tmp/pocketic-handoff-01/`. The local runtime
[intent](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/intent.json) and
[summary](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/summary.json), followed
by the [final adapter intent](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/final-adapter-intent.json)
and [final summary](caffeine-probes/local/2026-10-09-tooling-hardcut0200-01/final-adapter-summary.json),
bind compiled 0.19.3 to the current graph and developer tooling:

- All 242 native-host cases pass in each phase, including two actual standalone installation
  carriers and the Metrics restoration read window, through the new handoff.
- Strict affected CLI/harness lint and actual Rust 1.88 native/Wasm compilation
  pass. Shared five-tool fixtures pass Bash 5 and Linux-built Bash 3.2.57, including
  old-bundle refusal and retention. Consumer routing verifies admitted path and
  argument forwarding, owner/test exit statuses and no test dispatch after refusal.
- Changed shell lint, failure archive round trips, actual offline tool admission,
  dependency declarations, formatter/hook checks, links, snapshot and evidence
  consistency are recorded separately. An initial source inventory used the wrong
  VCS metadata filename and is retained before corrected source verification.
- The final adapter preserves explicit overrides after the initial default-only
  qualification. Its separate source manifest and intent retain the earlier
  phase. The first override fixture exported an undefined Make variable as empty;
  that refused default dispatch. The retained failure precedes the correction,
  which relies on Make's existing command-line/environment export behavior.
- Two native-host runs budget eight explicit local probe GETs. Local substitutes
  do not qualify deployed Caffeine; successful temporary body/request files are
  not separately archived. Source assertions, logs, binaries and hashes survive.
  Owned local server scopes close; there are no live/paid effects or external cleanup.

The Testkit setup/check prerequisite passed Linux/Intel/Apple Silicon in
[owner 0.25.4 CI](https://github.com/dragginzgame/ic-testkit/actions/runs/37901828971).
Published 0.25.5's provisioning source is unchanged; its separate exact-source
CI remains queued at this read. Adopted [Shared](https://github.com/dragginzgame/shared-tooling/actions/runs/37916384666)
and [Host](https://github.com/dragginzgame/ic-host-tooling/actions/runs/37917141870)
CI are queued. [Released Blob 0.19.3 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37910255607)
passes MSRV/Linux/both macOS hosts, qualifying #37/#38's earlier fixes only.
#37/#38 are closed with that exact-source evidence; #39 receives the prepared
adoption and retains its outstanding acceptance.
Dirty 0.20.0 still requires full release validation and committed native acceptance
under [#39](https://github.com/dragginzgame/ic-blob-storage/issues/39).
No sibling edit, agent commit, push, release or publication occurs.
