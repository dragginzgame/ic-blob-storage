# Initial IC Wire adoption — pending 0.32.0

Date: 2026-10-17. This starts over the existing pending CBOR/Memory 0.46 batch,
not clean released HEAD. The incoming content snapshot is retained at
`.tmp/review-validation/icwire0320/before.tar`; no incoming work was reverted.
Own package versions remain 0.31.0. No sibling edits, Git index/ref, release or
publication effects ran.

## Implemented slice and publication boundary

Tenant inspection/update now use bounded IC Wire contracts in Contracts, the
standalone, storage probe and replicated tenant client. Explicit field/variant
identities and distinct method/direction envelopes preserve the native DTOs,
authority/CAS policy, uncertainty, journals and restore fences. The service catalog
selects IC Wire for these two methods. Neither accepts Candid or pending CBOR
frames. Provider Candid, persisted records and other owned methods are unchanged
by this slice. [The protocol guide](../service-wire.md) records the current state.

The maintainer selected the first publication as 0.3.0. Tracked manifests declare
the 0.3 registry line; root Cargo.lock is byte-identical to this turn's starting
snapshot. Crates.io resolution returned `no matching package named ic-wire found`.
A final approved registry readback still found no package. No registry checksum,
path lock or published-package qualification is fabricated.
A prior attempt separately retained the sandbox DNS refusal before approved
registry access established the package's absence.

Development uses an ignored private consumer copy and captured read-only upstream
source from HEAD `226a11fce6d5cdd59a77d9a006c91f5a46c58b42` plus its observed
working-tree files. That source still calls itself 0.2.2. The raw
`upstream-source.json` records every captured file's SHA-256; the copy remains
fixed while its owner prepares publication. Only the private consumer root catalog
selects the captured path. Its separate lock adds the candidate Wire/derive,
Postcard 1.1.3, cobs and embedded-io packages; no such override enters tracked
manifests or the root lock. These results do not qualify published IC Wire 0.3.

## Focused candidate qualification

Raw commands, logs, source manifests and separate failed attempts remain under
`.tmp/review-validation/icwire0320/`. The
[intent](caffeine-probes/local/2026-10-17-icwire0320-01/intent.json) and
[summary](caffeine-probes/local/2026-10-17-icwire0320-01/summary.json) bind local
scope, artifact hashes and results. No deployed provider requests, paid effects
or external account/object/billing obligations were created.

- Seven tenant-filtered Contracts cases pass: three existing semantic reply cases,
  three new IC Wire cases and one existing history case selected by its name.
  Maximum-width namespace/generation and principal values round-trip. Every
  application refusal retains its meaning. Wrong method/direction, truncation,
  old Candid/CBOR, nonminimal integer, overlong principal and trailing input refuse.
- The catalog codec ownership case and pure replicated-client binding case pass
  independently, giving nine native cases in this slice.
- Strict affected-owner all-target/all-feature lint and actual Rust 1.91 native
  and Wasm checks pass against the captured candidate. Native scope includes
  Contracts, core, standalone, PocketIC support, storage and consumer probes;
  Wasm MSRV scope is Contracts/core/standalone.
- Fresh standalone/storage/consumer Wasms build. A separate default-feature
  standalone rebuild selects production exports without probe feature unification.
- Ten selected surface cases pass in attempt `20261017T082952873298Z`: one static
  Wasm/catalog/provider-schema inspection and nine installed cases. They cover
  CLI installation, admission/restore, actual raw IC Wire rejection and tenant
  policy/client cases. The existing native CLI artifact is reused only for its
  unchanged CBOR installation carrier and is retained by hash; it has no direct
  tenant command. Replicated clients exercise IC Wire request and reply traffic.
- Installed checks preserve controller denial, exact CAS conflicts, failed-write
  rollback, lifetime capacity, restore fences, lost/short replies, callback traps
  and original retained commands without redispatch. PocketIC 16.1.0 archive,
  decompressed executable, version and hashes are explicitly admitted before use.
  Managed local fixture lifetimes complete without external cleanup obligations.

The first candidate compile exposed a now-ambiguous tenant conversion error type;
its existing closure now names the native failure explicitly. Original lint
failures (conversion-helper documentation/attributes and two test expressions)
are retained separately and repaired. No failed attempt was relabelled as success.

## Remaining work

Resolve and qualify the actual published 0.3.0 package and its registry lock first.
Then convert the other 29 owned methods and lifecycle inputs, including bounded
manifest/header/history/account projections and controlled native saved packets.
Retire the superseded direct CBOR owner when no owned boundary needs it; Memory's
independent persisted codec remains upstream-owned. Freeze identities and bounds
only after that coordinated conversion. No per-method format fallback is added.

Full CI, published dependency/package archives, optimized Wasm, native macOS,
deployed provider behavior and Canic's own wrapper/caller qualification remain
unrun. The earlier [CBOR receipt](service0320.md) retains its original source scope.
