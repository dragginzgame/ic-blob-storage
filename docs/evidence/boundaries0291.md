# Funding, recorded-query and browser cleanup

Source base: `f0555981ed6dda5d19f4fbf2b679964276f2f9a6`, plus the incoming
dirty preparation captured in `.tmp/review-validation/boundaries0291/before/`
and `diff-before.patch`. This receipt covers the three accepted local cleanups;
earlier probe/signer changes and incoming Cargo selections are not this task's delta.

Funding assessment/outcome share positive intent conversion; outcome/history
share phase projection. Authority, host-evidence unknowns and refusal mapping stay
local. Recorded native queries share argument/intent/reply persistence with exact
existing filenames and intent fields, including the setup record's absent retry
field. Transport failure keeps partial evidence; raw replies precede decoding.
Browser JSON equality retains property ordering and serialization semantics;
certificate byte equality stays separate. Session fields use the existing exact
required/allowed-name check after the existing structured-clone boundary.

The live graph selects Memory 0.44.0, Build 0.10.0, direct Host 0.22.1, Metrics
0.11.5 and Testkit/Candid 0.48.0. The incoming manifests and lock are preserved.
Live library test compilation fails on removed global-runtime entrypoints in
`stores/grants` and its tests (`funding-tests.log`). Public Memory 0.44 adoption
requires a minor release; the unchanged pending 0.29.1 heading is not suitable
for that complete graph. No own-version or release effects ran.

Focused live CLI tests pass 29 publication and two setup cases, including partial
query evidence, exact roles, binding, budget and recovery refusals. Strict CLI lint
passes. Browser publication
input/mutation checks, real Chromium journal persistence and 22 launcher/session
boundary cases pass. Initial sandbox socket/tool-admission failures and successful
loopback reruns remain in separate logs; no deployed provider was contacted.

`compatible/` retains the same cleanup sources with HEAD's Memory 0.43/Build 0.9
dependency selections. Only this isolated copy reconciles its fixture dependency
edges. Its 40 funding cases, strict library lint and fresh standalone Wasm build
pass. These results do not qualify the live Memory 0.44 graph.

Three installed PocketIC cases pass with that fresh Memory 0.43 Wasm and the
current live-graph CLI: batch preflight across restore, complete live-reference
mapping, and indexed preparation with distinct signed claims and recovery without
redispatch. The fixture harness is built from `compatible/`; the explicit Testkit
server passes offline admission. Input/artifact hashes bind this mixed selection.

Raw logs, before-images, selected Cargo inputs and artifact hashes remain under
`.tmp/review-validation/boundaries0291/`. Full CI, native macOS, archive
qualification, Canic and deployed-provider acceptance were not run.
