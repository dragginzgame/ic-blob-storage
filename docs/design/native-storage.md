# Native OP storage — 0.104.0

Historical contract for release 0.104.0. Current development uses [stable storage](stable-storage.md).

Blob 0.104.0 consumes `ic-storage` 0.76.2's **retained** runtime. It does not use the
separate stable-backed `records` path. The supplier revision is
[`f9fa036`](https://github.com/dragginzgame/ic-storage/blob/f9fa03611ebb8641412a2af0f8620b80909c164f/docs/design/retained-runtime.md).
This is a fresh native layout, not a migration from Storage 0.74.1.
The consumed live probe, its old Wasm and all `.operations/` custody stay untouched;
[Storage #1](https://github.com/dragginzgame/ic-storage/issues/1) remains separate.

## Ownership and records

The embedding host creates one supplier runtime and declares its complete root
roster. Blob supplies `local::persistence::storage_root(id, participant)` and
`callbacks::storage_root(id, participant)` for its own schemas and bounds.
`Access` projects a supplier-issued grant and a host-selected root into that
runtime; it owns no allocator, backing memory or lifecycle. Native applications
pass an `Rc<Runtime>`; Wasm hosts use Storage's singleton `Canister`.
`Service` and `Callbacks` are concrete types; there is no backing-memory generic.
Canic must adopt this boundary in its own adapter, sharing the same runtime with
its catalog and retaining captured caller authorization.

| Owner | Native keys and values | Fixed bounds |
| --- | --- | --- |
| Blob control | `00`: installation, configuration, domain version and maintenance; `01 + upload ID`: immutable upload and admission time; `02 + reference ID`: root and status | 129 entries; 17-byte keys; 16-KiB values; 64 lifetime uploads and references; existing aggregate 64-KiB codec budget |
| Callback receipts | `control`: bounded acknowledged roots and original binding | One entry; 7-byte key; 16-KiB value |
| Reference app | `00`: configuration and installation; `01 + draft ID`: original input, uploader and disposition | 17 entries; 17-byte keys; 4-KiB values; 16 lifetime drafts; existing 64-KiB aggregate and 2-KiB input budgets |
| Enrollment probe | `control`: original configuration, request, attempt and raw outcome | One entry; 7-byte key; 16-KiB value; reply allowance remains 8 KiB |
| Reply-failure probe | `count`: rollback sentinel | One entry; 5-byte key; one-byte value |

IDs are big-endian 128-bit values; application values use Wire payload encoding.
These are domain rows, without generic chunk framing. Historical rows remain;
released references and acknowledged deletion never erase replay protection or
provider exposure. Reopening validates schema/geometry in Storage, then checks
Blob's installation, keys, record identities and domain invariants. Derived
indexes, reference counts and declared-byte counters are rebuilt. The domain
`state_version` remains 1; the new native schema identifies the backing cutover.

Each example host uses bank A. Standalone selects 512 nodes / 256 KiB, reference
app 128 nodes / 256 KiB, callback probe 1,024 nodes / 512 KiB, enrollment probe
16 nodes / 64 KiB and reply probe 16 nodes / 4 KiB. These are backing allowances,
not counts of provider bytes. A composing host owns its own total geometry and
unique root/participant IDs. Blob control uses schema `0x424c4f424f503031`;
callback receipts use `0x424c4f4243423031`.

## Mutations and lifecycle

Local domain transitions remain synchronous. Encoding and ordinary refusals
precede native publication. Blob compares existing rows and writes only changed
values. Unexpected write failure panics inside the supplier's update scope: this
poisons a native host owner and traps the enclosing IC execution. Returning a
Rust `Err` after partial writes is not rollback. No Storage scope spans an await.
The installed encoder-failure probe verifies that an actual late reply trap also
rolls back the native sentinel.

The supported upgrade sequence is:

1. Drain Blob admission and finish runtime calls; the reference app and probes
   independently refuse an upgrade with a call in flight.
2. Stop the canister. Its pre-upgrade hook closes the supplier runtime.
3. Upgrade with **`wasm_memory_persistence = Keep`**, without skipping the source
   hook. The CLI signs these explicit Protocol flags; simulator tests use
   `upgrade_eop_canister`.
4. The target reopens the same root roster and loads domain state. Start it,
   inspect original obligations and separately activate the current Blob attempt.

A missing native directory or incompatible declaration traps, preserving the old
module and its domain outcomes. This does not establish arbitrary controller
rollback/snapshot continuity, cross-layout conversion or recovery from an old
stable image. There is no reinstall or reset fallback.

## Artifact preparation

Use `make build-host`, `make build-reference-app` or the relevant probe target.
The root `.cargo/config.toml` selects Storage's supported stack/static-data/memory
layout for Rust 1.100.0 Wasm32: 1-MiB stack, 36-MiB global base, 40-MiB initial
memory and 128-MiB maximum. The supplier's prepare/admit transform is mandatory;
a plain Cargo Wasm output is not installable native Blob.

`build-native-preparer` compiles the private qualification example once per Make
invocation. `scripts/prepare-native.sh` consumes Cargo artifact records, invokes
`retained_build::prepare` and `admit`, and retains raw/prepared hashes and artifact
records in a fresh `target/native-artifacts/attempt.*` directory. The normal
`target/blob-*-artifacts.jsonl` files select the prepared bytes for every installed
consumer and CLI install fixture. Preparation has no provider or live IC effects.
Embedding hosts must implement the same supplier build contract; endpoint
generation does not prepare a Wasm module or bootstrap Storage.

## Qualification and limits

The native tests use the public supplier host runtime and compare logical root
checkpoints. Installed tests observe actual uploads, references, accounting,
drafts and receipts across Keep upgrades; malformed calls compare those public
projections rather than empty stable-memory bytes. The maintenance suite asserts
signed original outcomes, module/controller status and current admission at each
applicable boundary. These projections do not claim byte-for-byte allocator
identity. Missing-directory tests use disposable modules with no native owner;
no test reconstructs OP state by injecting stable-memory bytes.

Evidence for this work is under `target/storage-review/`. The current handoff owns
which checks have completed. The older capacity report describes its selected
0.74.1 baseline; current native capacity execution is separately retained here.
Chromium, macOS, Canic composition and live Caffeine acceptance require their own
witnesses. D0/provider closure and sustained replay retention remain open.

The targeted predecessor review retained identity/replay, closed admission and
obligation preservation from the
[legacy lifecycle tests](https://github.com/dragginzgame/ic-blob-storage-legacy/blob/25bed1352f3d2b9659f0d2583f50fa9af859ea84/qualification/ic-blob-storage-pocketic-tests/tests/standalone_lifecycle/mod.rs).
Its stable journals, memory injection and restore framework are not reused:
the supplier now owns native memory and this supported lifecycle deliberately
excludes arbitrary image recovery. No old provider state or backing code is copied.
