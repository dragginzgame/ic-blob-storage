# Current status — 2026-10-24

Released baseline: `c7b420d` (0.46.0). Pending notes target 0.47.0 because the
Contracts feature default changes. Package versions remain untouched. Incoming
Wire 0.7.3 lockfile edits and the earlier fixture-library cleanup are preserved.
Older adoption and qualification detail is retained in [history](history.md).

## Completed Blob-owned cleanup

- Private Candid query/update helpers and reply limits have one owner in the
  unpublished PocketIC library. No authored Rust allow attributes remain.
- CLI invocation retains the first Clap result and passes selected arguments to
  online/offline handlers. Production no longer reparses argv or clones the
  subcommand. Test entrypoints exercise the same grammar and command conversion.
  Domain checks remain with their owners; no second argument model or CLI DSL.
- Setup dispatch and recovery share SetupIntentRecord for authority fields.
  Saved JSON keys and optional observational metadata behavior are retained;
  signatures, original request bytes and no-redispatch rules remain authoritative.
- Contracts has no default features. SDK/HTTP prepared-manifest and upload-reply
  users must explicitly enable provider-json. CLI/PocketIC already do; native
  wire-schema independently enables its JSON dependency. Canister features and
  endpoint encodings are unchanged.
- Replaced this handoff's accumulated current/historical narratives with this
  active record, retaining the complete prior text and evidence links in history.

## Reviewed and retained

- Frozen Candid projections support actual old-frame rejection tests and
  independent external provider replies. They are not a production fallback.
  Splitting their shared shape solely to rename ownership would duplicate it.
- Funding and upload cursor files retain JSON. They carry different scopes and
  continuation rules; common scalar parsing already has one owner. Moving them
  to Wire would break saved files for little deletion benefit.
- Heap admission/catalog/lifecycle models remain test-support-only accounting
  comparisons and fault-injection fixtures, not a parallel production store.
  Reusing durable transitions would compromise the independent comparison.
- Keep public installation access until consumer migration. Read-only Canic
  inspection still finds InstalledJournal and JSON schema reconstruction; Canic
  can adopt ServiceWorkflow::ACCESS and typed schema projections. No sibling edits.

## Validation

Strict CLI/Contracts Clippy all targets/features and dependency/deployment
boundaries pass. Native CLI tests pass (114 Blob, 7 probe); minimal Contracts
unit tests pass (118), as do explicit provider-json tests (10).
Rust 1.91 checks and the standalone release Wasm build pass. All seven installed
publish-preparation tests pass, including lost replies, tampered journals and
recovery without resubmission. The first installed invocation omitted BLOB_CLI_BIN
and failed before exercising recovery; the corrected explicit-artifact run passes.
Formatting and documentation links pass. Full CI and macOS were not rerun.
Logs: `/tmp/blob-owned-cleanup-*.log`. Earlier fixture-library qualification is
in `/tmp/blob-audit046-*.log`. No new provider, macOS or production-capacity
qualification is claimed. No commit, push, release or deployment was performed.
