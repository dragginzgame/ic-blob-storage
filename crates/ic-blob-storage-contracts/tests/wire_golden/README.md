# Frozen Blob Wire vectors

These 505 cases in 172 JSON files were captured from the schema at `e6710ca`,
before removing application DTO Candid traits. Each file contains named input
and literal expected hex, with no regeneration path in the test. The typed
registry covers every application DTO, every enum variant, all 31 owned method
request/reply pairs, and install/post-upgrade inputs. Two distinct struct inputs
exercise absent/present collections and optional data. Values are codec shapes;
they do not assert that arbitrary fixture identities satisfy service policy.

Each case checks current encoding against frozen bytes and decodes frozen bytes
back to named fields. Method inventory equality refuses an unrepresented owned
method. The existing full-width manifest tests independently cover u128 extremes.
Non-method DTO cases use a test-only identity, never a production dispatch ID.

The second test fails compilation if application DTOs regain CandidType.
CaffeineUploadCertificateResponse is the sole external provider exception.
Historical image and persisted probe schemas live at their owning fixture
boundaries; they do not supply current endpoint fallback readers.

Run `cargo test --locked -p ic-blob-storage-contracts --test wire_golden`.
Changing positional fields, variants or expected bytes requires an explicit
contract review; do not blindly refresh these expectations after a refactor.
