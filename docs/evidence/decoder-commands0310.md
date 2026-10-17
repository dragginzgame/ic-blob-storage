# Query-command, Candid and URL convergence — pending 0.31.0

Base is clean `831874938b2ba92653d828221f61b0496def6b53`, subject 0.31.0;
package metadata remains 0.30.0. The complete before archive, graph, attempted
commands and failures remain under `.tmp/review-validation/decoder-commands0310/`.
Incoming Memory 0.45.0, Host 0.24.0, Testkit/Candid 0.50.0 and Build 0.11.1 are
preserved. Only Contracts' direct edge to already locked percent-encoding 2.3.2
is added; external package/source/checksum rows are unchanged.

## Ownership and preserved behavior

The private operator owner shares passive query help, parse/query sequencing,
error targets and exit codes. Funding lookup/preview retain their own parsers,
JSON payloads, method and authority binding. Action/update flows remain separate.
Contracts supplies explicit Candid configuration for itself, core and CLI;
finite header bounds, absent-header defaults and zero/bounded skipping remain
caller choices. Authentication, byte limits, exact argument exhaustion and errors
remain at their boundaries. No decoder setting implies provenance or retry power.
The percent encoder replaces handwritten byte/hex iteration while preserving
Unicode, spaces, query delimiters, unreserved punctuation and the sha256 prefix.

## Incoming caller repairs and remaining qualification

Memory 0.45 removes SchemaMetadata and range objects. Requests now pass only
owner/key; the same fixture exclusions are expanded into numeric IDs. Existing
grant, neighboring-store and explicit host-runtime tests retain their purpose.
This is a public Memory identity/ledger hard cut: earlier installations require
accountable retirement/reinstall, with no migration or automatic state clearing.
PR #78's HTTP reader calls are applied against Host with native test-support;
Testkit 0.50 no longer supplies those functions. Scope/byte/deadline policy stays.

315 focused cases pass: 110 Contracts, 47 provider, four grant, two public memory
composition, 127 CLI/probe/subprocess and 25 fixture-library cases. New pure
command tests prove no query on help/invalid input and preserve blocked/failure
codes, wide values and original targets. URL vectors retain exact escaping.
Strict default-feature native affected-owner Clippy passes; fixture checks select
library/binaries, excluding installed targets. Actual Rust 1.91 native checks,
public/storage/consumer Wasm checks and warning-denied public Rustdoc pass.
Formatting, dependency declarations, docs links and diff checks pass.

The initial all-feature/all-target compile exposed incoming incompatibilities;
a mechanical Memory edit also briefly failed syntax and was corrected. All
attempt logs remain. Installed-test targets still fail E0061 because Testkit
requires an explicit DecodeLimits argument. They did not reach PocketIC startup;
previous installed proofs do not qualify this graph. Optional archive verification
also fails E0308 on Build's Host 0.23 versus direct Host 0.24 types (#69).
Prepared issue/comment Markdown remains at the raw evidence owner; automatic
approval review rejected posting their internal failure/source details to GitHub.
Neither was posted or retried. The starting commit has no matching hosted runs
at inspection. No full CI, actual archives, native macOS, Canic/provider operation,
sibling edit, package-version change, Git/index/ref effect or release ran.
