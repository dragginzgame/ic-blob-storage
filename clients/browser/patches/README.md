# Caffeine 1.1.2 integration patch

This is a local patch to the published Apache-2.0 `@caffeineai/object-storage`
package, not an upstream release or a copied upload implementation. It changes
the published JS and matching declarations; hashing, tree construction, certificate
extraction, HTTP shapes and direct URLs remain upstream code. No upstream contact
or PR has been made.

The private browser build checks the installed package version and original hashes
against `docs/evidence/caffeine-browser-reuse.json`, copies its dist files and
license to `.tmp/browser/caffeine`, checks/applies this patch there with Git, then
bundles that artifact. Installed npm files are unchanged. There is no download,
fallback to an unpatched package, release step or production deployment in this
build. A new upstream version requires an explicit patch/dependency review.

The extension supplies:

- `StorageClient.prepareFile(bytes, contentTypeHint?, filenameHint?)`, a static,
  network-free call returning a frozen `{ hash, byteLength, manifestJSON }` handle.
  Input bytes are copied before the first await. The internal tree and chunks are
  retained privately, so later caller mutation cannot change the upload.
- `storage.uploadPrepared(handle, onProgress?)`, which consumes that original handle
  once before certificate issuance. Copied/unknown/consumed handles are refused.
  The handle is ephemeral, not a durable recovery record; it grants no permission.
- A constructor options argument with per-client `fetch`, `signal`, `retry` and
  `concurrency`. Our composition explicitly uses `retry: false`, `concurrency: 1`
  and a scoped transport. Original package defaults remain upstream behavior;
  they are not this service's approved retry policy.

Prepare before admission, retain the admitted operation separately, then pass
`createCertificateClient(...).certificateAgent` to Caffeine's constructor. This
uses its existing real HttpAgent injection point; no new certificate callback or
private-method/global-fetch monkey patch is needed.

The tests first exercised unmodified 1.1.2 with that agent. The maintained patched
composition sends the actual prepared JSON through the bounded Rust declaration
decoder, tenant admission and uploader preparation in PocketIC before returning
the permission. It then checks a real local IC certificate, exact manifest
agreement, unchanged bytes after caller mutation, failed tree upload without retry,
abort before issuance and after the tree, and existing competing-tab/lost-reply/
cancellation recovery. HTTP gateway responses are labelled local substitutes.

The patch does not fix every upstream edge case (including the recorded empty-file
tree issue), add service admission or completion, persist gateway-effect intent,
qualify provider charging, or create a production application store. A failed tree
or chunk can still be uncertain. Neither a fresh preparation nor certificate
recovery authorizes replaying it. These prerequisites remain before live transfer.
