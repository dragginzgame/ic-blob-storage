# Entrypoint-based Shared Tooling adoption

Date: 2026-10-12. Pending Blob notes remain 0.22.11; package versions and the
finalized receipt remain 0.22.10. HEAD advances during the session to
`ca482fae6bff8efa14a8c309d00cf9fb32ee6feb`, preserving user-owned delivery.

## Reviewed source and selection

The adopted committed source is
`8e8cc21265272e0234326c53a8c7bf9e7d8d77df`, matching GitHub main at inspection.
Its changelog describes pending 0.7.0 entrypoint selection; its committed
`VERSION` is still 0.6.6, recorded unchanged by the canonical exporter.
Dirty sibling edits and release preparation are excluded.

A clean isolated clone previews and refreshes the consumer. The initial explicit
entrypoints retain all 105 existing payloads, as permitted by the upstream
adoption guide. The exporter derives transitive companions and publishes
entrypoint annotations separately from the payload integrity records. Five
payloads change: exporter, verifier, adoption/Make guides and snapshot task.
No additions or retirements occur. Narrowing these roots requires a separate
caller/ownership review; this adoption preserves current obligations.

Future refresh reuses recorded entrypoints without selection arguments.
Extensions use `--add-entrypoint`; a complete `--entrypoint` set replaces the
roots. Old `--file` / `--add-file` options refuse. The selected commit requires
individual `--retire-file` selections for obsolete owned files, backed by exact
prior source/hash/mode/index/path checks. No consumer-owned files, tool
installations, evidence, parent directories or sibling source are removed.

## Focused qualification

Canonical actual preview/apply and snapshot integrity checks pass. Repeat
preview with no roster arguments derives the same 105 files, all unchanged.
The old CLI selector refuses. The selected source's isolated retirement suite
passes on Bash 5.2 and genuine Bash 3.2.57 on Linux, including old-manifest root
adoption, ownership/local-conflict refusals, companion closure, malformed or
undeclared roots and interrupted-deletion retry. These are private substitute
repositories, not actual consumer retirements or native macOS acceptance.

Actual consumer Make routing, complete offline tool admission, declarations,
root formatting and hook preservation checks pass. Executable pins remain
unchanged; no tool setup is performed. The exporter/governance integration
suite remains producer-owned and is not added to Blob's CI roster.

Incoming Cargo.lock updates now select Host 0.14.5, Memory 0.35.14, Metrics 0.7.1
and Testkit 0.34.2. They are preserved. The initial comparison against the prior
held lock detects this change; it does not authorize relabelling the previous
637 native tests or Rust 1.88 results. The earlier qualified graph remains in
[its original record](shared066-host014402211.md). This tooling adoption runs
no new product compilation, PocketIC instance, Wasm or complete gate.
The Metrics probe/report adaptation remains
[Blob #60](https://github.com/dragginzgame/ic-blob-storage/issues/60), and native
tooling/retention acceptance remains
[Blob #59](https://github.com/dragginzgame/ic-blob-storage/issues/59).

Raw source identity, previous snapshot, preview/apply/reuse, refusal and fixture
logs remain in `target/review-validation/shared-entrypoints02211/`. No staging,
commit, version mutation, release execution, sibling file edit, publication,
deployment, paid provider effect or artifact cleanup is performed by this work.
