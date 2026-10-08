# Publication helper repair

Date: 2026-10-08. Compatible pending **0.18.1**, based on completed **0.18.0**
`704b8ebf6bea85a715e465e32e34758b601852ec`. Cargo metadata and the release receipt
remain unchanged by this repair. The reported missing-selector error stopped
before invoking Cargo publication.

## Supported command and retry

Ordinary `make publish` selects contracts then core. Optional `PUBLISH_PACKAGE`
still selects one member. Only canonical crates.io is used. The existing clean
working tree, release hashes/receipt, exact sole parent, annotated tag and shared
release lock still admit publication; no dirty-source bypass or release runner
is introduced. The [release guide](../releasing.md#publication-and-deployment)
records the fixed flow and separate clean-tag steps for the already tagged 0.18.0.

For each selected package, bounded registry readback distinguishes exact 404 from
successful version rows; transport/HTTP/JSON failures refuse. An existing version
must be non-yanked, with a valid checksum. Its downloaded archive must match that
checksum and its embedded VCS record must match clean release HEAD and the owning
package path. Only then can retry skip an upload. Inputs and a fresh Cargo target
remain in the unique printed publication directory. Failed archive/source checks
retain readback and do not dispatch a replacement upload.

Standard [Cargo publication](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)
owns uploading and index polling. Contracts failure stops before core. There is
no progress journal, custom polling or automatic retry after an uncertain reply.
An explicit retry reads the registry first. Dry-run calls Cargo verification but
never uploads; core requires the exact contracts version already on the registry.
A contracts dry-run does not supply it. Paired offline payload verification stays
with `make package` and its isolated target.

## Evidence and limits

Two complete isolated adapter fixture runs pass, including existing release,
receipt, failed-read, parent/tag and lock guards. Final focused publication cases
also pass after assigning a fresh Cargo target to each publication run:

- Default and individual ordering, core dry-run after indexed contracts, and
  unpublished-contract dry-run refusal without any upload.
- Contracts/core rejection followed by retry; lost replies after either successful
  upload followed by registry reconciliation; a completed retry dispatches no upload.
- Transport, HTTP, malformed JSON, wrong package/checksum/source/path, dirty source
  and yanked-row refusal before the affected upload.

All Git, Cargo upload and registry effects use isolated substitutes. The final
focused driver uses the maintained fixture functions without altering their
logic; its private copy and raw logs are retained in `.tmp/publication-0181-01`.
ShellCheck, Bash/Perl syntax, evidence checksum and local documentation link
checks pass. An initial direct documentation-check invocation omitted required
arguments and printed usage; the maintained Make target supplies them and passes.
This is not a live registry upload or a full CI/release gate. Release CI was still
in progress at the single retained inspection. No commit, tag, push, publication,
deployment or paid effect was executed by the agent.

An external lockfile edit during this work selects private Metrics 0.2.11; it is
preserved and captured separately. The [package/Host checks](host050-package0180.md)
remain bound to the original Metrics 0.2.9 / compiled 0.17.2 graph. Publication
substitutes do not qualify that dependency change.

## Cleanup inventory

Removed `test_missing_publish_selection` from `scripts/release/test-release.sh`:
its required-selector rejection is obsolete now that default publication succeeds.
Default-success cases replace that requirement; `test_invalid_publish_selection`
retains rejection of unsupported package selectors. No Rust function, method or
type is removed. No sibling files or frozen earlier evidence records are edited.
