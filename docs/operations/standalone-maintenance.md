# Standalone maintenance

This runbook covers the experimental Blob host's existing native admin commands:
inspect, drain, stop, upgrade, start and explicitly activate. They are qualified
against a local IC gateway. Fresh provisioning, live deployment, schema migration
and Caffeine effects are not qualified by this procedure. The reference application
has its own stop/drain/upgrade/start lifecycle; this CLI does not manage it.

The [design](../design/ic-blob-storage-design.md#interrupted-management-operations)
owns the lifecycle contract. The [CLI](../../crates/ic-blob-storage-cli/src/main.rs)
owns argument validation. Commands below are an operator sequence, not an automatic
deployment script: complete and inspect each stage before preparing the next.

## Inputs and custody

Use an existing installation and independently reviewed inputs:

| Input | Required meaning |
| --- | --- |
| Endpoint and canister | The exact installed target; HTTPS, or loopback HTTP for local qualification |
| `manager.pem` | Ed25519 PKCS#8 identity matching the installation's manager; also a platform controller for stop/upgrade/start |
| `root.der` | Independently trusted network BLS root key; never discover trust from the endpoint being checked |
| Root subnet principal | Independently established root subnet identity paired with that key |
| `installation.wire` | Original bounded Wire `blob_init` request with the expected immutable configuration |
| `current.wasm` | Reviewed uncompressed artifact currently installed before this maintenance attempt |
| `next.wasm` | Reviewed state-compatible replacement, uncompressed and at most 1 MiB; compatibility is the operator's responsibility |
| State version | Expected durable record format, currently `1`; this is not the crate release version |
| Request parent | Existing private operator-owned directory for new immutable operation records |
| Custody root | One durable operator-controlled root shared by every manager of this target |
| Checkout | Trusted Blob checkout, Makefile and execution environment; its admitted tooling snapshot must match the CLI binary |

Preserve both artifacts, installation inputs, all operation records and the shared
custody root. The root binds dispatch claims to network, canister, operation and
attempt. Changing paths or deleting a claim is not recovery. Even a partial or
empty claim bars redispatch. Native inputs must be bounded regular files; keep
identity material private. Retain any `.NAME.pending` publication stage; it is
not disposable output. [Native custody rules](storage-inspection.md) also apply
to these records. Legacy recovery records never enter this checkout.

Build and install the CLI with `make blob` after the normal `make install-tools`
preparation; `blob` is then available through Cargo's bin directory on PATH.
For this maintenance procedure, use the checkout artifact reported by Cargo,
then set the following Bash
variables to the reviewed paths and target. Placeholder paths are not created here.

```bash
blob_cli=$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "blob" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one Blob CLI artifact") end' target/blob-cli-artifacts.jsonl)
blob_inputs='/absolute/operator-owned/inputs'
blob_records='/absolute/operator-owned/attempt-records'
blob_custody='/absolute/operator-owned/shared-custody'
blob_checkout='/absolute/trusted/ic-blob-storage'
blob_target=(
  --url 'http://127.0.0.1:REPLACE_PORT'
  --canister 'REPLACE_CANISTER_PRINCIPAL'
  --identity "$blob_inputs/manager.pem"
  --root-key "$blob_inputs/root.der"
  --root-subnet 'REPLACE_ROOT_SUBNET_PRINCIPAL'
  --installation "$blob_inputs/installation.wire"
  --wasm "$blob_inputs/current.wasm"
  --state-version 1
)
blob_dispatch=(--custody "$blob_custody" --checkout "$blob_checkout")
"$blob_cli" maintenance inspect "${blob_target[@]}"
```

Inspection authenticates the module hash before and after a replicated management
read and compares installation, configuration and format. Success reports current
observations, not snapshot continuity or approval to activate. For an active host,
select exactly the reported maintenance attempt plus one as `blob_attempt`; do not
advance it on retry. If already drained, recover its existing records and stage
instead of starting this sequence again.

## Execute one maintenance attempt

Every `prepare-*` creates a new record and signs a request without submitting it.
Requests expire after two minutes: prepare each immediately before its dispatch,
not all five in advance. Every `dispatch-*` admits tooling, takes the shared target
lock and records its irreversible claim before sending the original packet once.
It may return an error because the request is pending or its reply is uncertain.
In that case, use only the corresponding `observe-*`; do not repeat dispatch.

Set `blob_attempt` to the selected positive integer. Keep this value, target array
and original `--wasm` unchanged throughout the attempt, including after upgrade.

### Drain admission

```bash
"$blob_cli" maintenance prepare-drain "${blob_target[@]}" \
  --attempt "$blob_attempt" --record "$blob_records/drain"
"$blob_cli" maintenance dispatch-drain "${blob_target[@]}" \
  --record "$blob_records/drain" "${blob_dispatch[@]}"
"$blob_cli" maintenance observe-drain "${blob_target[@]}" \
  --record "$blob_records/drain"
```

Require the original drain acknowledgment. Historical acknowledgment alone does
not describe current admission; stop preparation rechecks the exact drained
attempt. Draining blocks new admission while inspection, cleanup and historical
replay remain available. It does not delete objects or settle provider exposure.

### Stop runtime execution

```bash
"$blob_cli" maintenance prepare-stop "${blob_target[@]}" \
  --attempt "$blob_attempt" --record "$blob_records/stop"
"$blob_cli" maintenance dispatch-stop "${blob_target[@]}" \
  --record "$blob_records/stop" "${blob_dispatch[@]}"
"$blob_cli" maintenance observe-stop "${blob_target[@]}" \
  --record "$blob_records/stop"
```

Require **both** `original=acknowledged` and `platform=Stopped`. `observe-stop`
can exit successfully for a stopped canister whose original outcome is unavailable,
pending or rejected. That observation does not permit upgrade; upgrade preparation
requires the authenticated original stop reply. Do not start a stopping canister
to make a timeout disappear. Ordinary Blob inspection cannot run while stopped.

### Upgrade the stopped host

```bash
blob_upgrade=(--stop-record "$blob_records/stop" --next-wasm "$blob_inputs/next.wasm")
"$blob_cli" maintenance prepare-upgrade "${blob_target[@]}" \
  "${blob_upgrade[@]}" --attempt "$blob_attempt" --record "$blob_records/upgrade"
"$blob_cli" maintenance dispatch-upgrade "${blob_target[@]}" \
  "${blob_upgrade[@]}" --record "$blob_records/upgrade" "${blob_dispatch[@]}"
"$blob_cli" maintenance observe-upgrade "${blob_target[@]}" \
  "${blob_upgrade[@]}" --record "$blob_records/upgrade"
```

Require the original upgrade acknowledgment and matching replacement artifact on
the stopped target, even for a same-Wasm upgrade. The command signs an ordinary upgrade, runs the source hook and loads the
existing stable cells. Both modules must support the same layout. See the
[stable storage contract](../design/stable-storage.md). Upgrade arguments are fixed to
the unit Wire `blob_post_upgrade` request. There is no reinstall, skipped-hook,
snapshot restore or automatic downgrade option.

### Start and verify reopening

```bash
blob_start=("${blob_upgrade[@]}" --upgrade-record "$blob_records/upgrade")
"$blob_cli" maintenance prepare-start "${blob_target[@]}" \
  "${blob_start[@]}" --attempt "$blob_attempt" --record "$blob_records/start"
"$blob_cli" maintenance dispatch-start "${blob_target[@]}" \
  "${blob_start[@]}" --record "$blob_records/start" "${blob_dispatch[@]}"
"$blob_cli" maintenance observe-start "${blob_target[@]}" \
  "${blob_start[@]}" --record "$blob_records/start"
```

Require `Start observed` with the exact attempt and `active=false`. This checks the
original start acknowledgment, running platform, replacement artifact, configuration
and reopened record format. Start does not activate Blob. Missing evidence keeps
the procedure at this stage; running state alone cannot replace the acknowledgment.

### Explicitly activate

```bash
blob_activate=("${blob_start[@]}" --start-record "$blob_records/start")
"$blob_cli" maintenance prepare-activate "${blob_target[@]}" \
  "${blob_activate[@]}" --attempt "$blob_attempt" --record "$blob_records/activate"
"$blob_cli" maintenance dispatch-activate "${blob_target[@]}" \
  "${blob_activate[@]}" --record "$blob_records/activate" "${blob_dispatch[@]}"
"$blob_cli" maintenance observe-activate "${blob_target[@]}" \
  "${blob_activate[@]}" --record "$blob_records/activate"
```

Require `Activation observed` with the same attempt and `active=true`. Dispatch
checks the original stop, upgrade and start claims; the host rechecks manager and
attempt at execution. A later drain invalidates the current activation observation.
After completion, future fresh inspections use the replacement as `--wasm`.
Observing this attempt's records continues to use its original target inputs.

## Interruption decisions

| Observation | Action |
| --- | --- |
| Pending request or transport failure after dispatch | Preserve records and custody; use the same `observe-*` without submitting anything again |
| Unknown, forgotten or expired request | Keep the operation unresolved; expiry and missing status do not prove nonexecution or permit a renewed request |
| Partial preparation or dispatch claim | Preserve it; no automatic cleanup, repair, alternate directory or custody root |
| Definitive upgrade rejection | Keep the old host drained; retain the failure and custody; no retry, reinstall or automatic reactivation |
| Artifact, configuration, predecessor or attempt mismatch | Stop progression and inspect the mismatch; do not change bound inputs to force acceptance |
| Running, stopped or matching hash without the original acknowledgment | Preserve the distinction between current state and original outcome; do not advance on state alone |

Observations can be repeated against original records while the network retains
their outcomes. The tool does not persist an authenticated reply receipt, and later
stages re-read predecessor request statuses. Completing an earlier command therefore
does not guarantee recovery after the network forgets it. No supported history
compaction, obligation import or arbitrary state restoration procedure exists.
Unrestricted controllers can bypass this management arrangement; local readiness
and module hashes are not a snapshot-continuity proof.

## Evidence and limits

`make test-management` exercises the actual binary, signed requests and authenticated
local gateway responses. The [interruption tests](../../qualification/ic-blob-storage-tests/tests/management/mod.rs)
cover custody and refusal boundaries. The [populated lifecycle test](../../qualification/ic-blob-storage-tests/tests/management/lifecycle.rs)
checks shared references, closing exposure and reservations across the whole sequence.
Its different-artifact case adds only a Wasm custom section: it does not prove a
schema migration or arbitrary replacement compatibility. The
[capacity measurements](../design/standalone-capacity.md) separately cover cleanup
and reopening at the finite 64-slot history limit. Cleanup does not reclaim those
lifetime slots. These witnesses make no live-provider or sustained-use claim.
