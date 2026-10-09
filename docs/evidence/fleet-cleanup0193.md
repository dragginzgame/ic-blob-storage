# Optional fleet tooling cleanup — 0.19.3

The consumer no longer vendors or schedules the unused fleet reporters. Fleet
inventories remain owned by Shared Tooling; local workspace LOC and tool
setup/check retain their existing owners.

## Reviewed inputs and disposition

Released Blob HEAD is `59230ee017dcbf9ff38c19f389b0bb03e750852a`
(0.19.2), with validated source `4a7639b9186515d332220ff02b423866ba02cc38`.
The starting working tree was clean. Adopt committed Shared Tooling 0.1.34,
`3d33cd250fcae7dbe5cabe44b2abd6b2c91a1822`, using its canonical exporter from
an isolated clean checkout. Dirty upstream installer/policy work is excluded.
[Shared #83](https://github.com/dragginzgame/shared-tooling/issues/83) supplies
the formerly missing optional-report guard and selection guidance.

Caller review found only the two consumer fixture invocations, local help/README
advertising and the common report target. Remove the four files and manifest
records together with those consumer callers. The canonical guarded target and
synthetic command fixture remain unchanged. Shared task documentation explicitly
runs fleet reporting from Shared Tooling. Preserve historical evidence and the
collector's old failure-root prefixes so previous failures remain retainable.
No sibling is edited or invoked by the absent-report path.

cloc 2.10 measures **640 removed code lines** in four files (701 physical lines,
including comments/blanks). This is reduced vendored consumer footprint and two
fewer dedicated CI fixtures; no canister instruction, binary-size or compilation
speed improvement is measured. The snapshot selects 93 files, previously 97.

## Removed symbol inventory

These are deletions of consumer copies, not moves or upstream removals. Canonical
fleet implementations and coverage remain in Shared Tooling.

| Former file | Removed definitions |
| --- | --- |
| `scripts/dev/cloc-tooling.pl` | `usage`, `capture`, `read_file`, `write_file`, `safe_path`, `is_linked`, `in_scope`, `load_snapshot` |
| `scripts/dev/cloc-siblings.sh` | `usage`, `print_row` |
| `scripts/ci/test-cloc-tooling.sh` | No shell functions/types; generated synthetic Rust `main`, `runtime_builder` |
| `scripts/ci/test-cloc-siblings.sh` | No shell functions/types; generated synthetic Rust `library`, `inline_test`, `application`, `smoke`, `generated`, `another_test` |

All listed definitions served optional fleet reporting or its removed fixtures;
they have no product owner or replacement in Blob. Local `scripts/dev/cloc.sh`,
pinned cloc, checksum/receipt helpers and all product/release checks remain.

## Focused verification and limitations

Retained raw logs and readbacks are under `.tmp/continuation0193-01/`:

- Reduced snapshot and dependency declarations pass; Cargo manifests, lock and
  release receipt remain byte-identical to released HEAD.
- Common command and local LOC fixtures pass under Bash 5 and Linux-built
  Bash 3.2.57. They qualify substitutes and Linux execution, not native macOS.
- Actual offline `make tools-check` and local `make cloc` pass. The workspace
  report still covers the complete twelve-member roster.
- Actual omitted `make cloc-tooling` refuses with Shared Tooling/explicit-selection
  guidance. The canonical fixture verifies no dispatch on omission and continued
  explicit-selection behavior. Make's fixture plan and local help have no retired
  callers; JavaScript/Rust/browser/provider behavior is unchanged.
- Changed shell syntax/ShellCheck, local documentation links and whitespace pass.

At the final observation, [released 0.19.2 CI](https://github.com/dragginzgame/ic-blob-storage/actions/runs/37901765903)
passes MSRV; Linux, Intel macOS and Apple Silicon are still running. It does not
qualify this dirty cleanup. [Upstream 0.1.34 CI](https://github.com/dragginzgame/shared-tooling/actions/runs/37900620129)
passes Linux, Apple Silicon and lint; Intel macOS is still running.
[#37](https://github.com/dragginzgame/ic-blob-storage/issues/37) remains open for
committed consumer native acceptance. No new native macOS, full CI/release gate,
Rust build, provider probe, paid effect, commit, push or publication is claimed.
Pending 0.19.3 is compatible tooling cleanup; package metadata remains 0.19.2.
