# Current status — 2026-10-27

Released baseline: `2d130afc`, Blob 0.49.3. The accepted pending batch targets
**0.50.0** because it changes persisted records and removes a public Candid trait.
Package versions remain 0.49.3; only notes select the next release. The
[released handoff](https://github.com/dragginzgame/ic-blob-storage/blob/2d130afc/docs/status/current.md)
retains the earlier private RPC/journal Wire cut and its qualification.

## Accepted storage cut

All 34 Candid-derived model types now use Serde with published Wire 0.7.18.
The existing `model::record_codec` owner supplies the 17 independently stored
encodings through `ic_storage::adapter::Storable`. Each has an explicit storage-only
frame identity; nested models do not add envelopes. Installation format is now
`ic-blob-storage/installation:wire-records`. There is one codec, with no Candid
reader, migration, alternate writer or automatic reset.

The current record byte ceilings and collection layout bounds remain unchanged.
Wire policy includes the complete frame: allocation allowance 256 times the byte
ceiling, nodes eight times it, depth 32. These are finite codec admission units,
not measured heap limits or equivalents of the retired Candid work quota. Large
manifest profiles (1,954 leaves and 4,096 headers) round-trip within the unchanged
public admission envelope. Full-width counters, identities and typed outcomes
remain covered. Candidate installation uses the same fallible encoder before any
store is opened.

Host grants, allocation ledger, memory keys, key ordering and substrate ownership
are unchanged. Keys identify slots, not payload compatibility. `ReadSessionLimits`
loses its public Candid trait; consumers must remove such bounds. Required external
provider/management Candid, application endpoint frames, saved command documents,
canonical content identities and exact operation/retry policy remain unchanged.
No TypeScript client regeneration or new framework adapter is introduced.

## Retained installations and consumers

Old Candid stores refuse the new image. Apply the
[retirement runbook](../retiring-installations.md) using the original binary/tools;
retain provider objects, uncertain effects, balances, access and billing evidence.
Reinstall is not deletion or settlement. Never clear a ledger, discard an old
snapshot or activate stale state to get past format refusal.

Two retired opt-in tests asserting old-format compatibility are removed. Their
historical proof remains with the released source/evidence. The replacement
`standalone_record_cut` regression takes an explicit old Wasm/hash, populates it
through real endpoints, and proves upgrade refusal preserves stable bytes, the
old heap owner and registered physical/billing obligations. A fresh 0.49.3 image
was retained before editing and exercised here; the new test remains opt-in for
other runs that lack that pinned input. Same-release interruption/recovery stays
supported and independently tested.

Canic owns adopting the supplying release and qualifying its dedicated/embedded
services. No sibling is changed. This library cut does not establish application
login, deployed-provider availability, physical deletion or billing cessation.

## Qualification

Evidence, exact inputs, failed development attempt and logs:
`/tmp/blob-stable-wire-20261027/`. Published dependency versions are unchanged;
the lock only adds Core's explicit edge to already-selected Wire 0.7.18.

Passing on Linux: 342 core unit tests; 30 selected core integration cases; strict
Clippy for Core, standalone and PocketIC callers; fresh CLI and seven Wasm builds.
Native record coverage rejects retired Candid, wrong identity/direction,
truncated/trailing/oversized input and allocation/node/depth violations. Installed
standalone (75) and storage (122) cases pass on the probe-unified artifacts,
including callback/write rollback, references, funding, lifetime capacity,
restoration and uncertainty. The populated old-store refusal test passes too.
The initial unit-test compile failure was a shadowed test function, corrected
before these passing runs; its log remains retained.

Rust 1.91 checks Core and standalone with all features for Wasm. The default
standalone artifact is built separately to avoid probe feature unification;
its nine focused installed lifecycle/snapshot/certificate/refusal checks pass,
recorded in `default-*.log`. Dependency pins, Platform boundaries, normal deployment
exclusions, Rust/manifest formatting and documentation links pass. Full CI,
Binaryen variants, native macOS and live provider qualification were not run.
Historical/resource opt-in cases without their own selected inputs remain skipped.

Same compiler/profile default-artifact comparison (Rust 1.99.1, Cargo release,
unchanged published graph except Core's Wire edge): linked standalone Wasm
3,536,889 → 3,192,335 bytes; deterministic gzip-9 750,892 → 698,940 bytes.
The probe-unified artifact has separate measurements. These are artifact sizes,
not IC instruction, throughput, heap or deployed-provider measurements.

The [native Agent advisory](https://github.com/dragginzgame/ic-blob-storage/issues/102)
remains unresolved. Required provider Candid and frozen historical input tests
remain intentional; this is not a Candid-free resolved graph or deployment.
Nothing staged, committed, pushed, released, published or deployed by this task.
