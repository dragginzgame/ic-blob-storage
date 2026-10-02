# Prepare an isolated standalone trial

The [accepted contract](../../../docs/standalone-trial.md) permits one trusted
1 KiB upload. This directory supplies its proposed resource envelope; it does not
deploy, select a payer or provision a Caffeine namespace.

Copy [configuration.args.template](configuration.args.template) into a fresh private
directory and replace every named principal with its reviewed value. Obtain the
actual service principal after separately authorized canister creation. The
operator and isolated payer are explicit; their controller status is insufficient.
Choose the project, trusted uploader and independent verifier separately.

| Proposed setting | Value and purpose |
| --- | --- |
| Lifetime tenants, objects, references | One each |
| Object and physical/liability/logical bytes | 1,024 each |
| Retained chunks | One |
| Headers | Four / 512 framed bytes |
| Receipts | Two, preserving cleanup capacity |
| Active upload / read session | One each |
| Read buffers | 1,024 bytes per session and globally |
| Billing reserve / minimum / target | 2T / 0.1T / 1T cycles; proposed local policy, not provider prices or effects |
| Service cycle attachment allocation | One cycle, entirely reserved; no attachment offer available |
| Gateway reply entries / unique principals | Two / one; drift refuses and needs explicit review |

The funding reserve prevents service attachment offers within this installation.
It does not limit external provider bills, fund an account, revoke certificates or
end retention charges. Proposed provider funding uses the separately reviewed
cycles-ledger route; its existence in the Cashier interface does not establish
account creation, credit, authorization or enforcement semantics.

Encode and validate the **completed** configuration offline:

```sh
didc encode --defs canisters/standalone/service.did \
  --types '(ServiceConfigurationInput)' < "$RUN/configuration.args" \
  > "$RUN/configuration.hex"
xxd -r -p "$RUN/configuration.hex" > "$RUN/configuration.candid"
blob-storage installation-check \
  --configuration "$RUN/configuration.candid" --service "$SERVICE" \
  --project "$PROJECT" --verifier "$VERIFIER" --trusted-uploader "$UPLOADER" \
  --release "$REVIEWED_HOST_RELEASE" --run-dir "$RUN/installation-check"
```

Use the validated `installation-check/installation.candid` as the complete init
argument. Bind it to the reviewed source, Wasm and DID hashes before installation.
Decode independently using `xxd -p` and `didc decode --types
'(ServiceInstallationInput)' --defs canisters/standalone/service.did` through stdin.
Do not pass a filename as didc's hex argument. A package label alone does not bind
an unpublished working-tree artifact to the published release receipt.

The proposal is tested with an explicitly labelled local service stand-in in
[retained evidence](../../../docs/evidence/caffeine-probes/local/2026-10-02-trial-provisioning-01/summary.json).
Its installation bytes are **local-only** and must not be used for deployment.
Current trial role candidates and secret-file locations remain in private
`.tmp/trial-provisioning-01/proposal.json`; no keys are committed or imported into
the global identity manager. Follow the
[provisioning sequence](../../../docs/operator-guide.md#prepare-isolated-trial-provisioning)
before any live action.
