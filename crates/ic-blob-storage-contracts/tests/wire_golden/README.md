# Frozen Blob Wire vectors

These 140 cases in 56 JSON files cover all 27 owned method request/reply pairs
and standalone install/post-upgrade inputs. The approved 0.42 hard cut changes
native frame identities from `0041` to `0042`, adds the disjoint provider
acknowledgment status amount and funding reconciliation variant. Tenant finite-Wire
bytes and named inputs are unchanged. External provider GC callbacks use Candid
and do not enter this application-Wire inventory.

The existing named inputs were reviewed before one typed recapture. Existing
status inputs add `provider_acknowledged: 0`; one additional outcome covers a
matching positive accepted callback with `ProviderAcknowledged`. The same typed
method registration table owns encoding and roundtrip verification. There is no
regeneration/update mode in the maintained test. Scratch generator/source and
exact Cargo-fingerprint dependency selection stay with the local validation logs.

The earlier 139-case 0.41 capture remains in
[released 0.41.1 source](https://github.com/dragginzgame/ic-blob-storage/tree/d4dbfa1006106ae07ce47f207acdd3af294fc7d7/crates/ic-blob-storage-contracts/tests/wire_golden).
The original 159-case, 64-file capture remains in
[the pre-0.41 source](https://github.com/dragginzgame/ic-blob-storage/tree/ba779305b66acc87bf7af1e4f5a4a4f3ee5f24c7/crates/ic-blob-storage-contracts/tests/wire_golden).
There is no compatibility reader for either earlier native frame contract.

One typed registration per method checks current encoding against frozen bytes
and decodes those bytes back to named fields. It also supplies the expected
method inventory, refusing duplicate registrations and missing owned methods.
Cases include successful values, optional/collection shapes and error replies;
they do not assert that arbitrary fixture identities satisfy service policy.
Nested DTOs are checked at their actual method/lifecycle boundaries, without a
second per-DTO registry or a test-only framing identity.

Focused unit tests independently cover full-width integers, provider decimal
values, malformed/trailing/oversized packets and method/direction refusal.
Historical image and persisted probe schemas stay with their existing fixture
owners and recovery proofs; they do not supply current endpoint fallback readers.
Required external provider Candid is independent of this application Wire suite.

Run `cargo test --locked -p ic-blob-storage-contracts --test wire_golden`.
Changing positional fields, variants or expected bytes requires an explicit
contract review; do not blindly refresh these expectations after a refactor.
