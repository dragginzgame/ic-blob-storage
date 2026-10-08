# Tooling failure archive integration

Reviewed 2026-10-08 at released Blob **0.18.4**,
`ab15c39d208608d2b43a555fc417dd85a6759942`, with the existing pending **0.18.5**
Host/Testkit and 84-file Shared Tooling adoption preserved. This continuation
implements [#29](https://github.com/dragginzgame/ic-blob-storage/issues/29).
No Cargo selection, package version, receipt or frozen evidence is changed.

## Collection and transport owners

`scripts/ci/collect-tooling-evidence.sh REPOSITORY TEMP-ROOT NEW-ARCHIVE` owns
Blob's existing explicit candidate/fixture selections. It also selects the new
shared archive/evidence fixtures and its own qualification diagnostics. Missing
inputs yield no payload. The canonical `archive-evidence.sh` owns filename
admission, overlap refusal, IO, partial-output retention and exclusion of Git
metadata. It retains hidden files, modes and final symlinks without following
them; redirected parents refuse. Original inputs are never deleted by collection.

The workflow replaces raw fixture upload with this archive path. The pinned
uploader, ordinary artifact name and **30-day** retention are unchanged. A declared
archive disappearing refuses; an empty selection supplies no upload path. Failed
archive creation publishes no output and keeps partial bytes/diagnostics. Neither
collection nor upload clears the original failed job status.

A separate native qualification creates a synthetic archive containing newline/
colon names, permissions, executable bytes, links and original status records.
Local extraction verifies that contract before retaining its archive and SHA-256.
CI uses the same pinned uploader, requires one positive returned artifact ID,
downloads exactly that ID, and compares the complete archive against the locally
retained digest. No name lookup, old artifact fallback or producer/upload replay
is added. Its synthetic qualification artifact has one-day retention; ordinary
failures retain their original 30 days.

The reviewed official download pin is
`actions/download-artifact@9000827ccba6bdab643e8b6fd33ac0654aef8333`; its exact-ID
interface supports immutable artifacts from upload v4+. The caller also selects
fatal transport digest mismatch. Existing upstream immediate discovery failures
remain separate: no bounded retry mechanism is copied downstream to hide them.
Hosted round trips must pass at the committed consumer source on Linux, Intel
macOS and Apple Silicon before closing #29.

## Local evidence

The focused Make target passes on Bash 5 and Linux-built **Bash 3.2.57**. It checks
empty collection, early installer candidates, late logs/fixtures, filename/mode/
executable/link round trips, Git/unrelated-path exclusion, occupied output,
partial failed writes and redirected parents. The complete script ShellCheck,
snapshot verification and real Make routing check pass. Actionlint accepts the
changed workflow.
The final Bash 3.2 run also uses inherited CDPATH and a symlinked TMPDIR;
fixture paths retain their physical identity before comparison.

The actual collector and checksum run blocks are extracted from the workflow and
executed locally against isolated inputs. Empty/full collection passes; archive
write failure supplies no GitHub output; a corrupted downloaded archive refuses.
The actual identity guard accepts a positive ID and refuses missing, zero,
nonnumeric or multiple IDs. These are local caller substitutes; no GitHub artifact
upload/download, workflow dispatch/rerun, full gate or native macOS run occurred.

Raw inputs, commands, outputs, statuses, a Bash 3.2 proof archive and source hashes
remain under `.tmp/archive-routing0185-02/`; its `qualification.json` binds the
final inputs. Prior runtime artifacts and results keep their original graph and
source identity. No Rust build, sibling edit or named symbol removal occurs.
