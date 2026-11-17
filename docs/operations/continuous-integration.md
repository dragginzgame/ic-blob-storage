# Continuous integration

[The repository workflow](../../.github/workflows/ci.yml) runs on pull requests,
pushes to `main`, and manual dispatch. It has read-only repository
permissions, pinned external action revisions and no deployment credentials.

- Linux owns `make ci`: tooling/pin admission, non-mutating formatting, strict
  product and qualification Clippy, local Rust/Node behavior, Rustdoc and links.
  `make dependency-pins-check` uses Toko's declaration checker for both Cargo
  workspaces, workspace inheritance, tracked lockfiles and GitHub action pins.
  Private qualification versions remain independent; the exact tooling pin needs
  no exception. This is declaration admission, not transitive dependency or npm
  verification; locked preparation and runtime checks remain separate.
  `make build-host build-reference-app` also checks and builds the production
  Wasm boundaries. Setup prepares the compiler from `rust-toolchain.toml`, pinned
  common tools and both locked Cargo graphs. `check-browser` owns the browser's
  locked npm installation; setup does not repeat it.
- macOS runs the existing reference, Caffeine integrity, local service, CLI and
  example tests. This covers native custody on macOS; it does not claim installed
  canister behavior. Linux owns common formatting and lint instead of repeating
  them in a host matrix.
- A separate Linux job validates the workflow and runs Gitleaks once over full
  checked-out Git history, including documentation-only changes. The admitted
  Toko snapshot owns the installers and version/checksum catalog. Findings fail
  the run with fully redacted output. `.gitleaksignore` names only five reviewed
  historical findings: public simulator trust material and deterministic test
  seeds. New occurrences require review; no file or detector is excluded.

CI deliberately selects setup targets instead of `make install-tools`, which
also provisions the optional simulator. Installed PocketIC suites, Chromium,
live-provider qualification, funding and deployment remain separate operations.

Each job restores tool downloads through the installed Toko
[`cache-tool-downloads` action](../../vendor/toko-tooling/.github/actions/cache-tool-downloads/action.yml)
after snapshot admission and before setup. It saves after successful setup,
before checks, with writes restricted by the shared action to default-branch
pushes. Restored archives still pass the normal checksum/version admission;
cache hits are not test evidence. This caches authenticated tool downloads,
without caching provider custody or validation results. Hosted cache behavior
and any speed improvement require an observed CI run.

Failed shell setup/check steps use [the bounded output wrapper](../../scripts/ci/run-step.pl).
It retains up to 64 KiB of sanitized command output plus phase/tool/status metadata
per failed step, caps console output and marks truncation or missing diagnostics.
Toko's Rust installer already prints its selected attempt's `install.log` on
compilation failure, so that output is captured even before tests start. Unprinted
files are omitted; the wrapper does not scan hidden directories, tool stores,
homes, raw environment or provider custody. Known sensitive fields, credentials,
private-key blocks and URLs are removed before either printing or retention.
This is for the credential-free repository CI commands, not arbitrary payload logging.

The shared Toko tool-evidence collector retains full failed tool candidates and
does not supply this consumer's redaction/size contract. The
[Blob action](../../.github/actions/retain-failure-logs/action.yml) therefore selects
only its whitelisted bounded diagnostic files, packages them with Toko's archiver
and delegates uploading to the shared
[`retain-failure-evidence` action](../../vendor/toko-tooling/.github/actions/retain-failure-evidence/action.yml).
It passes `prepared-archive` and `collection-log`; this shared branch never scans
tool stores or repackages the archive. A failed archive remains private and is
not published as complete evidence; its collection log can still be uploaded.
The shared uploader owns the **14-day** retention period (previously seven days).
The Blob wrapper links any available artifact and reports incomplete collection
or upload separately. Neither can turn the original command failure into success.
Failures before shell capture (for example checkout or setup-node) remain in the
action's hosted log; missing evidence is reported explicitly.

`perl scripts/ci/test-run-step.pl` checks early setup failure with an explicitly
printed hidden attempt log, sanitization, bounds, missing output, collector
refusal and original status preservation. The workflow/security job owns that
small regression. `perl scripts/ci/test-workflow.pl` executes the actual Blob and
shared action run bodies for archive selection, missing/redirected/oversized
evidence, archiver failure, shared completeness and upload-result reporting. It
also checks cache ordering, preserved job coverage and one browser installation
owner. This focused wiring fixture runs in Linux after host tools are prepared;
generic Toko cache/upload regressions stay upstream. Both local commands require
the checkout root; the wiring fixture uses the prepared host `yq` and `jq`.
Local execution does not qualify hosted cache or artifact upload/retrieval, or
native macOS execution; those require the corresponding hosted run.

The initial workflow resolves [BlobStorage #2 — Complete hosted CI wiring](https://github.com/dragginzgame/ic-blob-storage/issues/2).
[Release 0.100.11's first hosted run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/43859650612)
passed all three jobs on `f61c414`, including native macOS and production Wasm.
