# Frozen pre-cut standalone image

`manifest.json` pins a retained pre-0.14.10 preflight image actually compiled as
0.14.9 with ic-memory 0.25.5. It is not asserted to be the tagged 0.14.9 binary.
The source base, dirty-source patch hash and
[historical preflight record](../../../docs/evidence/release-preflight-01410.json)
bind its origin. Do not replace its digest with a fresh build or introduce an
old decoder into the current service.

The binary is an operator-retained artifact, not committed fixture data. To run
the opt-in regression with the exact retained bytes and a fresh report:

```sh
BLOB_PRE_CUT_STANDALONE_WASM="$PWD/.tmp/v01410-preflight-01/artifacts/ic_blob_storage_canister.wasm" \
BLOB_HARD_CUT_REPORT="$PWD/.tmp/my-hard-cut-report" make test-hard-cut
```

The report's parent must exist; the report itself must not. A different image
refuses before installation. Missing historical evidence is a missing check, not
permission to synthesize an equivalent image. Default CI does not run this case.

The test reinstalls only an empty local fixture before creating old obligations:
a released 1 KiB object and a pending 10 MiB manifest. It tries the new image,
requires typed canister rejection, compares every stable byte and checks old
configuration, exact admissions and conservative accounting remain accessible.
The raw trap identifies the failing bootstrap stage; prose is not a test oracle.
Failed reports are retained and retries require new directories. Same-release
recovery remains covered separately. No live owner or provider effect is used.
