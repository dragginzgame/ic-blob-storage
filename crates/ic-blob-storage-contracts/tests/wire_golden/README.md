# Frozen Blob Wire vectors

These 159 cases in 64 JSON files cover all 31 owned method request/reply pairs
and standalone install/post-upgrade inputs. They were captured from the schema
at `e6710ca` in PR #91 and verified against 0.37.0 (`3c02d95`) with Wire 0.5
before removing application DTO Candid traits. Each file contains named input
and literal expected hex, with no regeneration path in the test. The retained
files are unchanged from that capture.

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
