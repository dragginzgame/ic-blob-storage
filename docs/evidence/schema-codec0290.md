# Inventory schema, stable record codecs and native query transport

Date: 2026-10-15. Starting released base:
`e65834ce6e61b38954c6a411edd7417cf0090e5a` (0.28.0), including the existing dirty
0.29.0 batch retained in `.tmp/review-validation/schema-codec0290/before.tar.gz`.
Earlier work and the separate unpublished Host candidate are not this task's
delta. Cargo requirements, selected lockfile and package versions are unchanged.
Raw logs, before-images, source hashes and artifact sizes live in that directory.
`final-source-inputs.json` binds 958 source/configuration inputs; final checks
confirm no unrelated source drift or dependency reselection.

## Ownership and preserved boundaries

Contracts' `upload::inventory` owns passive prepared-report, asset, totals,
blob, claim and header data shared by the preparation example, download verifier
and fixture observer. JSON fields/order, strict unknown/duplicate-field refusal
and original header order are retained. Paths remain passive `PathBuf` values,
preserving producer serialization; the observer never resolves/opens them.
Each consumer still bounds input and independently checks totals, hashes,
asset/root bindings and its own limits. Decoding is not service authority or
proof of provider completion. The example's source-opening/snapshot policy is
unchanged and does not claim containment against concurrent replacement.

The private model codec implements common Candid serialization and bounded
decoding. Each owner supplies its existing byte/work/skipping/type/header limits
and stable-tree layout. All 31 record schema bodies in the eight affected record
files are byte-for-byte unchanged (`frozen-record-declarations.json`). This
source comparison covers declarations, not persisted-image or deployed-provider
qualification. Manifest records retain the 64 KiB codec ceiling with an unbounded
stable-tree page layout. No record layout, memory key, discriminator, authority,
retirement, fence or restore decision changes.

Installation writes now check the same 16 KiB ceiling already declared by
`Storable::BOUND` and checked on decoding. The focused installation regression
checks current Candid bytes and refuses oversized encoding. The common borrowed
encoding path avoids explicit record cloning; no allocation or speed claim is
made. Existing malformed-record and same-release recovery checks remain.

A native query helper owns the unchanged 30-second call deadline and
Timeout/Transport mapping. General queries, batch checks, preparation and
publication mapping delegate after selecting their already configured agent.
Operator/tenant routing, overall deadlines, query budgets, evidence writes,
response validation and update dispatch remain with their existing owners.
The descriptive runner fingerprint includes the extracted query source.

## Focused evidence

- `library-tests.log`: 349 library/model/service cases pass, including malformed
  records, bounded manifests, installation encoding and restore fences.
- `contracts-tests.log`: 107 Contracts cases and 21 preparation/download example
  cases pass. `inventory-tests.log`: 17 observer cases pass; unknown fields are
  now checked at every nested shared-schema object as well as the report root.
- `cli-tests.log`: 119 command cases, five probe cases and one verifier case pass.
- `clippy-rerun.log`, `rustdoc.log`, `msrv-native.log` and `msrv-wasm.log`: strict
  affected-package lint, warning-denied Rustdoc and actual Rust 1.91 native/Wasm
  checks pass. Initial import/test-trait and numeric-literal failures remain in
  `check.log` and `clippy.log`, alongside the corrected results.
- `installed-standalone.log`: 21 selected installed cases pass, covering actual
  Candid exports, operator/tenant queries, preparation and lost-reply recovery,
  stable restoration, gateway decisions and refused replacement installations.
- `installed-storage.log`: 35 selected durable cases pass, covering funding
  reservations/credit/receipts and rollback, lifecycle and read restoration,
  plus the operator-owned restoration-read attribution window.
- `standalone-wasm.log` and `storage-wasm.log`: fresh release Wasms are built
  separately, keeping probe test-support features out of the production build.
  `before-wasm.json`/`after-wasm.json` bind hashes and physical sizes; selected
  preceding standalone/storage bytes are retained locally for comparison.
  Standalone size changes from 4,123,379 to 4,126,529 bytes (+3,150); storage
  probe size changes from 5,165,460 to 5,172,942 (+7,482). These are ordinary
  unoptimized Cargo Wasms; source reduction does not establish binary savings.

These checks cover the selected registry graph and local substitutes. Full CI,
archive/release acceptance, native macOS, deployed providers and Canic delivery
remain separate. No sibling source, Git/index, release, publication or paid effect
was changed or executed. Cross-release retirement/reinstallation and same-release
interruption recovery remain required.
