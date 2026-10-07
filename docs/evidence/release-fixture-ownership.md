# Release fixture ownership

Reviewed 2026-10-07 for pending 0.16.1, against released Blob source
`49d6ba76964668a6835c63784a892af905ce10b1` and canonical Shared Tooling 0.1.15
`bfb50bd0884b5e6c5ee9592056531c6108f96d73`. This map addresses
[#22](https://github.com/dragginzgame/ic-blob-storage/issues/22), coordinated with
[Shared Tooling #40](https://github.com/dragginzgame/shared-tooling/issues/40).

`make release-check` runs the canonical runner suite, the actual Make routing
check and Blob's metadata/publication adapter suite. These suites substitute
commit, tag, remote push, Cargo compilation and registry publication effects.
The real-index case imports existing Git objects into an isolated fixture; it
creates no commit. Passing fixtures do not establish a real release or native
macOS qualification.

## Scenario ownership

| Local case or family | Retained obligation or canonical replacement |
| --- | --- |
| `test_dependency_bootstrap` | Blob's ordered snapshot/tools/pins/fetch/check dispatch, early refusal, retained cached dependencies and unchanged metadata. |
| `test_preflight_tools` | Blob's ShellCheck selection, formatter pin and rustfmt admission, including an executable path containing spaces and no early metadata mutation. |
| `test_invalid_changelog`, `test_changelog_content`, `test_changelog_intent` | Blob's `release-data.pl` invocation, saved previous version, finalizer status projection, imported history and frozen receipt behavior. Shared finalizer semantics alone do not prove adapter wiring. |
| `test_finalizer_failure` | Failed finalizer output cannot publish candidate notes; failure after a metadata bump restores the exact Blob files and retains preparation evidence. |
| `test_release` | One patch Make-to-adapter smoke proof, exact receipt, external lock selections, historical notes and retained build artifacts. Common patch/minor/major iteration is owned by the shared suite's `for kind in patch minor major` matrix; all four Make entry points remain covered by `check-release-commands.sh`. |
| `test_failed_guard_read` | Failed Git/metadata reads, including commands that emit plausible bytes before failure, cannot admit Blob preparation, committed checks or publication. No Cargo dispatch or metadata loss follows refusal. |
| `test_rejected_preparation` | Blob gate/metadata/formatter failures preserve source files; successful retry produces a verifiable Blob receipt. |
| `test_preparation`, `test_actual_index` | Exact four-file staging inventory and real index admission, including staged unrelated bytes hidden by restored worktree content. |
| `test_receipt_identity` | A self-consistent receipt and notes cannot replace the saved release date. |
| `test_publish`, `test_invalid_publish`, `test_publication_lock` | Blob's package/registry selection, dry-run dispatch, parent/merge/tag/receipt admission and preservation of another owner's publication lock. |
| `test_selected_commit` | Blob's selected historical receipt and file hashes, exact parent/source/tag identity and publication refusal, even when HEAD has newer notes. Shared callbacks use generic fixture metadata. |
| `test_recovery_integration` | One actual Make-to-adapter descendant recovery proof: the earlier Blob receipt/tag stays frozen; failed next-source validation is retained; retry validates the correct source and produces the next exact receipt. |
| Removed `test_versions` | Shared `test-release-runner.sh` owns patch/minor/major arithmetic and large/invalid version rejection. No Blob arithmetic implementation remains to qualify independently. |
| Removed `test_push_retry` | Shared `lost-$kind-$effect` matrix owns uncertain stage/commit/tag/push reconciliation, one validation/preparation/commit/tag and preservation of original identity, logs and artifacts. |
| Former `test_followup_release` success/resume/missing-tag matrix | Shared `descendant-$next_kind-$outcome`, `descendant-validation-fails` and `remote-descendant-missing-tag` cases own next-kind iteration, exact older resume, missing remote tag and retry state. The gate-failure branch is narrowed and renamed to `test_recovery_integration` for the distinct Blob receipt and validation-log obligations above. |

## Retained fixture support

`finish`, `run_case`, `create_fixture`, `expect_failure`, `fingerprint`,
`assert_unchanged`, `assert_cache_retained`, `prepare_tagged_release` and
`commit_fixture_fix` support the remaining adapter cases. The Git substitute's
`resolve` and `ancestor` functions and command dispatcher remain necessary for
selected historical files, parent/tag corruption and the one actual recovery
integration. Cargo, Perl and awk substitutes retain adapter-specific metadata,
bootstrap and failed-output controls.

No generic simulation framework is extracted and no shared fixture is patched.
The remaining local simulator is deliberate integration support, not a second
production runner. Further extraction requires a proven common support contract
at Shared Tooling and consumer qualification; the original 672 code LOC are not
all removable. This batch removes two named functions, narrows one renamed
function and drops redundant matrix invocations. Source-bound results and native
limitations belong in the current handoff and its linked evidence record.
