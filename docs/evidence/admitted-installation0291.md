# Shared publication installation and recovery inspection

2026-10-16. Clean starting Blob HEAD: `c348af88e7840a710629be39528e53081ea7163c`
(released 0.29.0). Pending notes select compatible 0.29.1; own versions stay
0.29.0. Cargo inputs are unchanged: Build 0.9.0, direct Host 0.21.0, public
Memory 0.43.0, Metrics 0.11.4, Testkit 0.48.0 and PocketIC 16.1.0.

Each native publication invocation shares one admitted offline installation
candidate, original Candid packet and constrained manifest ceiling. Admission
checks the candidate's model, not authenticated installed identity. Every file
still independently binds its service, namespace and project; retained packet
comparisons, manifest/body hashes and installed checks remain. The canonical
prepared-manifest decoder already validates the declaration, so its immediate
second validation is removed. Recovery reports project the existing attestation
inspection without changing conflict, settled-history or no-redispatch handling.
No library API, durable schema, CLI option or output contract changes.

Linux checks pass: 119 CLI and five probe cases; all-target/all-feature strict
CLI lint; warning-denied Rustdoc; actual Rust 1.91 all-target/all-feature CLI
check. The existing batch refusal case additionally checks a later file's
service, namespace and project before output. Fresh standalone/storage Wasms
and native CLI pass 19 selected installed publication/setup/session cases and
the signed-attestation lost-reply/conflict/restored-history case.

Before-images, input/source/artifact hashes and raw logs remain under
`.tmp/review-validation/admitted-installation0291/`. Final logs use `*-final.log`,
`clippy-pass2.log`, `installed-publication.log` and `installed-attestation.log`.
The earlier missed packet projections and needless-borrow compiler/lint failures
remain beside passing reruns. Installed execution uses local substitutes, not
the deployed provider. Optional archive-verifier compilation is covered, but
actual archive qualification, full CI, native macOS and Canic acceptance were
not run. No sibling edits, index/ref changes, release or paid effects ran.
