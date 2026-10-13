# PocketIC startup simplification and current Canic review

2026-10-13, over Blob `4277ed1ec059c1eb36e4bc5e32e7b6b7eb74ecc2` with the
preceding 0.23.1 cleanup still uncommitted. Starting harness/notes and incoming
diff are retained under `.tmp/review-validation/harness0231/`.

The shared test harness delegates server and instance construction to published
Testkit 0.36 `PocketIcStartupConfig::build_instance`. The caller still supplies
its builder and admitted binary. One 30-second deadline covers server readiness
and instance creation, replacing two separate 30-second startup stages. Testkit
owns startup failure cleanup; the retained local field order drops the synchronous
instance before its server. The local optional server field matches the upstream
return type; spawn mode remains explicit. No shared server or fallback is added.

Incoming Host 0.17 exposed a missing test feature: `ic-interface` no longer
implies `candid-service`. Select the latter explicitly in PocketIC dev-dependencies;
published runtime/CLI feature choices are unaffected. Initial compiler failures
and a transient locked-input refusal are retained. Metrics 0.8.4 remains private;
Testkit 0.36 uses Host 0.16 while direct callers select 0.17. These results do not
relabel the preceding Candid size measurements on Host 0.16.

Read-only Canic review checked its dedicated/embedded successful managed-run logs
against its recorded root manifest and all three current lock hashes: all match.
Both cases select published Blob 0.23.0 and a single Memory 0.36.1 per graph.
This is existing Canic-owned local evidence, not a new sibling execution or
application/provider qualification. Canic #527/#444 retain family delivery;
Blob #56–#59 still lack completed matching hosted native acceptance.

Focused qualification passes: workspace all-target/all-feature denied-warning
Clippy, advertised Rust 1.88 native/Wasm checks, four actual local PocketIC case
executions (interface, installation rejection, repeated-upgrade fences and custom
NNS/application topology with signed CLI observations), formatting, dependency
pins, documentation links and runtime-free contracts. Default standalone/probe
Wasm inputs are retained from the preceding cleanup; this change affects native
test startup only. No full CI/release gate, native macOS, provider call, sibling
edit, package version change, staging, commit or release was performed.
