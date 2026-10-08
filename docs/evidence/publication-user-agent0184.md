# Publication request identity — pending 0.18.4

Reviewed 2026-10-08 against released Blob
`c5e204b04a0e9293ee4af0ab83c1c3b3013f5af3` (0.18.3). The maintainer's registry
readback stopped before Cargo publication with HTTP 403. Original readback is
retained at `target/publication.U9xfMV/ic-blob-storage-contracts.json` (empty).
The owning issue is [#30](https://github.com/dragginzgame/ic-blob-storage/issues/30).

Two bounded, sequential, read-only requests to the exact same canonical contracts
0.18.3 endpoint reproduce default curl identity returning 403/empty, then an
identifying application/version/repository User-Agent returning 404 with the
canonical missing-crate JSON. The existing parser classifies that exact 404 as
absent. This is live registry-read evidence, not an upload or provider observation.
Raw headers, bodies and statuses remain in `.tmp/publication-ua-0184-02/`.

The consumer helper now supplies that identity to API, archive and dry-run
dependency reads and disables implicit curl configuration first. No fallback,
retry, credential selection or new publication journal is added. Genuine 403,
transport failure, malformed metadata, yanking and checksum/source mismatches
remain fatal. Cargo remains the upload/index-poll owner.

The complete Blob adapter suite passes with substituted Git/Cargo/registry
effects (`adapter-after.log`, exit 0). The curl substitute requires isolated
configuration and an identifying application; a separate identified-client 403
case verifies zero upload effects. Existing ordered contracts/core, dry-run,
partial-publication/lost-reply reconciliation and archive-source refusal cases
remain in that passing suite. Bash syntax and ShellCheck pass. No full CI/release
gate or native macOS execution was run. No function, method or type is removed
or renamed.

The already-tagged release is prepared independently at
`.tmp/publication-ua-0184-02/release-0.18.3/`: clean detached v0.18.3 with unchanged
payload and a passing released source/receipt/tag check. An external `bin/curl`
wrapper adds the same identifying User-Agent and invokes the original curl
executable with implicit configuration disabled. It does not replace Cargo or
the released publication guards. The wrapper's executable smoke check passes;
the prepared checkout has no source edits. From this repository root, the
maintainer can execute the concrete publication command:

```sh
PATH="$PWD/.tmp/publication-ua-0184-02/bin:$PATH" make -C "$PWD/.tmp/publication-ua-0184-02/release-0.18.3" publish
```

This is registry publication and has **not** been executed by the agent. It uses
the released helper's contracts-before-core flow and exact readback. Preserve
failure artifacts; a remaining 403 or inconclusive read still stops. No tag,
receipt, release version, registry credential or sibling file was changed.
The normal source repair belongs to compatible pending 0.18.4; Blob's incoming
Host 0.6 / Testkit 0.23 Cargo edits remain separate and are not runtime-qualified
by these transport fixtures. Their entry hashes/diff are retained in the raw
directory and remain unchanged by this repair.
