# Fixture decoding, transport selection and session resolution

2026-10-16. Blob HEAD: `c348af88e7840a710629be39528e53081ea7163c` (0.29.0),
with the preceding CLI cleanup already dirty. Before-images and raw logs remain
under `.tmp/review-validation/fixture-convergence0291/`. Incoming Metrics 0.11.5
is preserved; Build 0.9.0, direct Host 0.21.0, public Memory 0.43.0, Testkit
0.48.0 and PocketIC 16.1.0 remain selected. Own versions stay 0.29.0; compatible
pending notes stay 0.29.1.

Published `ic-testkit-candid` 0.48.0 supplies fixture-only reply decoding; its VCS
receipt names `ae7c5be65dacf5faf2e48599614723d0ba4bb2c7`. Wire, work, skipping
and type-table budgets remain explicit per caller. Header bounds equal the
existing admitted wire ceilings; they do not exclude previously admitted inputs.
Native codec diagnostics are discarded into the existing typed fixture errors.
Blob retains all semantic bindings and query/update policy. No new dependency
enters the public library, Contracts or production Wasm graph.

Fixture commands share the existing transport target and common parsing in the
model. Status still permits anonymous simulated callers; inventory requires its
caller to equal its usable tenant. Loopback/nonzero-port admission and command
flags remain explicit. Session recovery updates the original frame's selected
path after the same pinning checks, preserving retained formats and refusal order.

Linux checks pass: 23 fixture units and eight session units; strict fixture
library/binary and CLI lint; warning-denied fixture Rustdoc; actual Rust 1.91
fixture/CLI checks. Existing cases additionally cover extra Candid values and
unusable inventory callers. Fresh selected Wasms/CLI pass eight installed fixture
cases and all seven installed publication-session cases, including lost replies,
path tampering, selected-byte changes and retained browser handoffs. Tool admission,
dependency pins, formatting and documentation links pass; source/input/artifact
hashes are retained with the logs.

Installed results use local substitutes. Full CI, native macOS, archive/release
qualification and deployed-provider/Canic acceptance were not run. No sibling
edits, index/ref changes, own-version changes, release or paid effects ran.
