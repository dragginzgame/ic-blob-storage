# Current status

Date: 2026-10-19

## Source and scope

The isolated `codex/blob-contract-qualification` branch builds on
[PR #90](https://github.com/dragginzgame/ic-blob-storage/pull/90), `2967298`,
and incorporates main's catalogue/export repair at `628d176`. This batch starts
at merge `e6710ca`. Own package versions remain 0.35.0; pending breaking notes are
0.36.0. Only unpublished fixture dependency edges change in Cargo.lock. No release,
publication, deployment, live provider call or sibling-checkout modification ran.

## Implemented contract

All 31 owned methods and lifecycle use Wire as described in
[the service protocol](../service-wire.md). The new 505 literal vectors freeze DTOs,
enum variants, method directions and lifecycle inputs. Application DTOs have no
CandidType implementation; a compile-time negative guard prevents restoration.
The external provider certificate response retains its trait. Cashier, management,
HTTP bodies and production storage records retain their independent contracts.

Private storage-probe controls use bounded CBOR tuples. Historical image tests,
CLI frozen configuration inputs and saved consumer-probe records use local frozen
schemas preserving their original Candid bytes. These are not production fallback
readers. Old-consumer-to-new tests preserve published/cancelled assets, prepared
manifests, stable service bytes and restoration fences. Cross-release deployment
still requires retirement and reinstall. Same-release retries remain exact.

Attestation submission retains the original passive statement before dispatch,
alongside the method-bound Wire argument and signed IC envelope, preserving
lost-reply recovery. Installed tests exposed and now cover the void-control reply,
manifest test packet and saved statement boundaries.

## Evidence and consumer actions

[Codec evidence](../evidence/wire-codecs/README.md) retains native/IC samples,
source and input identities, Wasm hashes and explicit admission charges. The
isolated Wire candidate reduces 1 MiB control decode instructions by 31.24%; actual
manifest costs are essentially unchanged. Native timing is diagnostic. No linked
size, compressed deployment size or end-to-end transfer saving is claimed.
Blob still selects published Wire 0.4.4; the owning Wire change is uncommitted in
its repository and needs separate publication/adoption after review.

Focused checks pass: Contracts' 122 unit cases and integration/doctests, 505 vectors
and the negative trait guard, CLI 121 cases, Core service 153 cases and consumer
model six cases. The 127 default storage cases are covered by 119 initial passes
and corrected owning reruns; two additional frozen-consumer transition tests pass.
Earlier failures and corrected logs remain under `/tmp/blob-dto-*`. Strict lint, 75 standalone cases, two pinned historical-image refusal tests,
one Chromium certificate case (ten scenarios) and 23 standalone Chromium journeys
pass. Final scope review removes unused private fixture derives; the consumer
probe Wasm remains byte-identical, with storage-control smoke checks repeated.
No full repository suite is claimed.

[Canic #603](https://github.com/dragginzgame/canic/pull/603) owns wrapper and
lifecycle adoption; inspected `4caeec25` still uses Blob 0.35's CBOR helper API.
[#84](https://github.com/dragginzgame/ic-blob-storage/issues/84) tracks the DTO cut.
No Canic suite, native macOS or deployed-provider qualification ran. Required
hosted checks and maintainer review precede integration. Release, retirement,
publication and deployment remain separately authorized operations.
