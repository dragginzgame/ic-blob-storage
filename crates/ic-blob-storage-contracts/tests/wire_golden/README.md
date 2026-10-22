# Frozen Blob Wire vectors

These 139 cases in 56 JSON files cover all 27 owned method request/reply pairs
and standalone install/post-upgrade inputs. The explicitly approved 0.41 hard cut
removes the four verifier methods, verifier installation/readback fields and the
intermediate upload phase. Native frame identities change from `0036` to `0041`;
remaining payload values retain their meaning except the documented registration
state. Tenant finite-Wire bytes are unchanged. Inputs were reviewed before a
one-off typed recapture; the test has no regeneration or update mode.

The original 159-case, 64-file capture at `e6710ca`, later qualified at `3c02d95`,
remains in [the delivered pre-cut source](https://github.com/dragginzgame/ic-blob-storage/tree/ba779305b66acc87bf7af1e4f5a4a4f3ee5f24c7/crates/ic-blob-storage-contracts/tests/wire_golden).
There is no compatibility reader for that earlier contract.

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
