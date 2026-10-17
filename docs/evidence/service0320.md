# Owned service CBOR cut — pending 0.32.0

Date: 2026-10-17. Source starts at released 0.31.0 commit
`d2b5877e1449c434feacb2cb9b0fa9811a6810a2`; implementation is the current working
tree. Own Cargo metadata remains 0.31.0. Incoming maintainer selections settled
at Memory 0.46 and Build 0.12.1 and are preserved. No Git index/ref, release,
publication, sibling, deployment or paid effects ran.

## Implemented contract

All 31 Blob-owned `blob_*` methods use exact bounded CBOR for input and output;
standalone init uses the installation DTO and post-upgrade uses CBOR unit/null.
The provider certificate alone retains its mandated Candid request/reply.
Provider/management traffic and stable record codecs retain their original owners.
The [wire guide](../service-wire.md) owns representation, limits, host/Canic actions
and recovery. The same endpoint catalog supplies mounting and method metadata;
`service.did` describes only the provider subset. Core clients and native saved
service packets use this codec. JSON installation proposals retain their original
bytes and hashes alongside CBOR outputs. Provider account-link files stay Candid.

Ciborium 0.2.2 and stable-structures 0.7.2 were already locked. This task adds
four local dependency edges: Contracts/Ciborium, core/stable-structures and
Serde in standalone/storage probe. The only external selection differences from
the released base are incoming Memory 0.46 and Build 0.12.1. Memory's removed
blanket substrate export is replaced by the direct collection dependency and
supported host backing exports. Its new exclusive leases remain host-owned;
a composition fixture proves a second live open refuses and dropping its first
collection permits reopen with the retained value. No collection-format conversion
or ledger reset is introduced.

## Focused qualification

Raw logs and scripts are retained in `.tmp/review-validation/service0320/`.
The [probe summary](caffeine-probes/local/2026-10-17-service0320-01/summary.json)
binds the final graph, source hashes, actual Wasms/CLI/server and selected results.
Original [intent](caffeine-probes/local/2026-10-17-service0320-01/intent.json),
[restore correction](caffeine-probes/local/2026-10-17-service0320-01/attempt2-intent.json),
[incoming graph intent](caffeine-probes/local/2026-10-17-service0320-01/attempt3-intent.json),
[qualified intent](caffeine-probes/local/2026-10-17-service0320-01/qualified-intent.json)
and [browser intent](caffeine-probes/local/2026-10-17-service0320-01/browser-intent.json)
remain separate records. These are local substitutes, not live provider observations.

- Contracts: 115 passing library cases, including full-width u128/principals,
  old Candid/malformed/trailing/unknown/duplicate refusal, depth/output bounds and
  catalog codec ownership. Native account cases also preserve signed/arbitrary-width
  Int/Nat through canonical decimal CBOR fields.
- CLI package default all-target cases: 127 pass (121 main, five probe, one
  native-files). The provider account-link pair passes again after restoring its
  original Candid filename. No extra unique-case count is claimed for that repeat.
- Core store/grant cases: 11 pass. Native memory composition: two pass.
- Fresh Wasms: standalone, storage, consumer, authority, funding and gateway-source.
  A separate default-feature production standalone rebuild avoids probe feature
  unification. Explicit PocketIC 16.1.0 archive, decompressed executable, SHA-256
  and version admission passes without setup/download; it does not qualify a
  separately installed Testkit CLI tool record.
- Selected surface cases: 36 pass, comprising 35 installed cases and one static
  Wasm/catalog/provider-schema inspection. They cover CLI installation, ingress,
  certificate authority, signed queries/references, account reports, manifest
  recovery, observation/attestation lost and pending replies, repeated upgrade,
  fences and replicated tenant clients. One Chromium case exercises ten scenarios
  with actual signed CBOR admission/preparation and retained provider Candid.
- Strict affected-owner all-target/all-feature lint, actual Rust 1.91 native/Wasm,
  warnings-denied core/Contracts Rustdoc and browser tool admission/build pass.
  Browser/native preparation passes using the fresh CLI, strict JSON proposal and
  opaque CBOR packets. Final formatting/declaration/documentation checks are retained
  alongside the focused logs; they do not constitute the complete CI gate.

Final installed successes are under attempts `20261017T075634156601Z` and
`20261017T075935036165Z`; each retains its exact artifact hashes and commands.
The former's observation failures are superseded only by the latter's passing
observation cases. Earlier successful Memory 0.45 attempts retain their own graph.

## Failures and limits

Development logs remain distinct: initial codec/caller compilation, nested macro
Candid-export ordering, arbitrary-width account Serde mismatch, lint and test
harness admission failures. Attempts `20261017T073326858064Z`,
`20261017T073417621418Z`, `20261017T073517328826Z` and
`20261017T073734912867Z` retain the probe-init, missing CLI selection, old empty-DIDL
post-upgrade guard and signed test caller failures before their fixes. The later
`20261017T073924089163Z` successes apply to the earlier graph only.

Incoming Memory 0.46 first failed on removed exports; caller fixes and strict
checks follow in the Memory-labelled logs. A native composition attempt refused
its duplicate live handle, then passed after explicit drop/reopen. Three HTTP
fixture tests initially failed because the sandbox refused localhost binding;
the same scoped package run passed with approved loopback access. Final observation
cases initially tried Candid-decoding the new statement and now use bounded CBOR.
No failed attempt was relabelled or deleted.

Pinned historical image producers retain an isolated Candid test owner and their
original artifact names. Their opt-in tests compile but were not executed here.
Full CI, optimized Wasm, actual package archives, native macOS, deployed provider
acceptance and Canic's own adapter/caller tests remain unrun. Core library linking
still exports no endpoints or lifecycle hooks. Existing retirement/reinstall and
same-contract restore/reconciliation rules remain required.
