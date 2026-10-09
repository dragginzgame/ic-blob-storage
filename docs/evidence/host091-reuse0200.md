# Host 0.9.1 fixture I/O reuse

Readback of the owning main branch still selects published Host 0.9.1,
`4a016053525fa710bc13f3aedbe85a471b78f6ed`. The existing approved pending 0.20.0
graph already selects all four Host crates at 0.9.1 through Testkit 0.26.0 and
direct artifacts/fs dependencies. Cargo.toml and Cargo.lock remain byte-for-byte
unchanged from task entry, preserving the prior hard-cut batch. Implementation
base remains `9a5173d365d2c11889ea2e813c9645ab366fbffe`; package/receipt stay 0.19.3.

The bounded review follows [flow convergence](../../audits/flow-convergence-and-duplication.md)
at Shared `ee48bb37c98c771e77b92fd891f0757d8c1c8b99`, under the local AGENTS
overlay. It covers private inventory file reads, local resource artifact writes,
probe persistence and native/browser file/child boundaries. Service/provider
economics and downstream frameworks are outside this source cleanup.

- `operator::inventory::input::load` delegates its manual bounded reader to
  `ic_testkit::ic_host_fs::read::read_file`. The 8 MiB limit, followed symlinks,
  JSON/domain validation and `InvalidRequest` projection remain. Host owns regular
  file admission, fallible allocation and metadata/stream limits.
- `storage_resources::retain_bytes` delegates open/write/file-sync code to
  `ic_testkit::ic_host_fs::durable::create_new_bytes_with_parents`. Existing raw
  Candid/report bytes, trailing JSON newline and no-replacement behavior remain.
  Host adds atomic complete publication and parent synchronization. Failure stops
  the profile; there is no automatic retry or cleanup of earlier artifacts.

Both adapter functions remain; no function, method or type is removed.
The probe's failed/partial records and phase diagnostics, unverified download
body lifecycle, nonblocking regular-file opening with followed symlinks,
recovery path confinement and prefetched browser output protect different
contracts. The current Host helpers do not replace those contracts directly.
No extra dependency, wrapper framework or sibling edit is introduced.

Raw intent, source hashes, logs, entry Cargo files and exact executed test
binaries are retained under `.tmp/host091-reuse-01/`. Four inventory input cases
and one resource artifact case pass; they verify exact-limit input, oversize,
missing/directory refusal, symlink selection, report framing and preservation of
existing bytes after refused replacement. Strict affected-package Clippy and
actual Rust 1.88 native compilation pass. Frozen inputs and selected graph remain
unchanged. Formatting, snapshot/declarations, documentation links and whitespace
have separate focused checks.

These are local file cases: zero HTTP/provider requests, IC instance/lifecycle
operations or paid effects. Successful temporary file fixtures follow test
cleanup; logs, source assertions and binary hashes survive. Earlier 242-case
runtime evidence is not relabelled. Full gates and committed native macOS
acceptance remain outstanding under #39; no commit, push, release or publication.
