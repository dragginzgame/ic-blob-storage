# Published example installation assessment

Blob owns qualification of the frozen published `ic-blob-storage 0.15.1`
`prepare_upload` example. This is an assessment of Cargo example installation,
receipts, retained failures and the checked-in Shared Tooling installer's explicit
package selection. It does not execute the installed example, contact Caffeine,
qualify the current Blob CLI or change package/release selections.

The assessment originates in Shared Tooling's `scripts/ci/qualify-cargo-install.sh`
at revision `a3d4f619bd1999a0d4c084b429bb417b13839807`. The package, version,
example target and debug profile stay frozen so the ownership move preserves the
original contract. The common Cargo binary assessment remains in Shared Tooling.
The adopted snapshot remains authoritative for shared installer/build/store code.

Run the selected adapter checks without registry access:

```sh
bash scripts/ci/test-published-example-install.sh
shellcheck -x scripts/ci/qualify-published-example-install.sh scripts/ci/test-published-example-install.sh
```

The substitute fixture proves path custody, occupied/dangling-root rejection,
retained failures and acceptance of Cargo's `dev`/`debug` receipt spelling. It
does not establish native registry behavior.

After explicitly preparing host tools and the selected compiler, run the native
assessment with a fresh evidence directory:

```sh
bash scripts/ci/qualify-published-example-install.sh /absolute/new-evidence-directory
```

This explicitly selected command may fetch the frozen published package and its
locked dependencies, compile and install executables into its owned evidence
store. It checks receipts, offline reuse, missing-target refusal, failed compiler
dispatch, concurrent Cargo requests and changed executable bytes. The production
shared installer must independently admit and reuse its selection and refuse
corruption without executing the product example. Existing evidence is never
reused or deleted; a retry requires a new directory.

The `Published example installation` workflow runs this assessment on Linux
x86-64, macOS ARM64 and macOS x86-64 and retains evidence on success and failure.
The original owner head `e3888fa772adab829f0b69747caed771a4f6f7c0` passed all three
native jobs in [its native run](https://github.com/dragginzgame/ic-blob-storage/actions/runs/39046360319).
This integration also triggers that workflow on snapshot changes. The
[current adoption receipt](shared023-host0270321.md) records local Linux
qualification with Shared 0.23 separately from that earlier three-host evidence.
