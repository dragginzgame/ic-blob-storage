# ic-blob-storage

Experimental blob authorization, application references and provider-obligation accounting for the Internet Computer.

This is the independent rewrite of the legacy crate. API and persistence are
experimental; existing installations have no automatic migration. Local
qualification does not establish supported Caffeine enrollment, upload authority
or charge closure.

Use the library directly in a Rust host, or through Canic’s host adapter. Blob
owns grants, references and durable obligations; the host captures callers and
mounts endpoints. Declare `blob.control.v1` in the host’s Storage bootstrap and pass
the granted `RuntimeMemory` lease to `Service::create` or `Service::open`. This crate does not transfer file bytes or claim provider settlement.

## Cargo features

The default `persistence` feature puts bounded Postcard records in IC Storage.
Select `default-features = false` for the in-memory model. Hosts and native schema
generators use the [Wire adapter](../ic-blob-storage-wire/README.md).

## Embedding the Caffeine upload update

Enable `ic-blob-storage/caffeine-certificate` in the embedding host. Mount
`_immutableObjectStorageCreateCertificate` as a synchronous update. Cap the
ingress argument at `caffeine::certificate::MAX_REQUEST_BYTES` before copying
it. Use `certificate::decode_request` as the decoder and
`certificate::encode_reply` as the encoder. In the handler, call
`service.execute(caller, Command::Register(root), now)` with the caller and time
captured from the IC runtime. Return the decoded root only on success. Trap on
decode, registration or encoding failure so the update rolls back atomically.
The SDK obtains the certified ingress response from the replica.

## Catalogue upload decisions

Keep the original upload ID, uploader, object root and installation. After a
lost local reply, an authorized caller can query `Inspection::Upload(id)` and compare the
returned declaration with its original grant. To verify readability, check a
gateway read against the expected root and byte length. Record the time of that
observation. A catalogue requiring provider confirmation of a specific upload
must withhold remote publication. [Blob Storage #6](https://github.com/dragginzgame/ic-blob-storage/issues/6)

## Optional operational metrics

Enable the library’s `metrics` feature and supply one heap store per service
incarnation. A standalone Rust host can own the handle directly.

```rust
use ic_blob_storage::{local::persistence::Service, metrics::MetricsStorage};
use std::rc::Rc;

let store = Rc::new(MetricsStorage::default());
let service = Service::create_with_metrics(memory, installation, input, Rc::clone(&store))?;
assert_eq!(service.metrics(), Some(store.snapshot()));
```

On upgrade, pass fresh storage to `Service::open_with_metrics`. Plain
`create` and `open` leave instrumentation disabled. A Canic adapter can
declare a typed source with `Storage = MetricsStorage`, pass
`store::open::<BlobSource>()` at initialization, and report with
`store::get::<BlobSource>().map(|s| s.snapshot())`. The host controls report access.

Each committed `Service::execute` call contributes one outcome to its command
kind, including ordinary refusals and historical replays. A first successful
registration also records the declared object size in bytes. Retries, attached
references, queries and trapped messages add no size sample. Inspect histogram
bounds and overflow flags when exporting. For storage over time, timestamp the
existing application-authorized `Inspection::Usage` values for reserved, live and
unresolved declared bytes separately. The durable ledger survives upgrades.

## Optional operational logging

Use IC Logging 0.26 for the recorder’s `ic_logging::Level` parameter.

Enable the library’s `logging` feature and bind the host’s recorder after creating
or reopening the service. A Canic host with logging initialized can use its
existing recorder directly.

```rust
let service = Service::open(memory, installation)?.with_logging(canic::logging::record);
```

A standalone Rust host can capture its own `Rc<RefCell<ic_logging::Log>>` and
clock. The host decides retention, timestamps and who may read the shared log.

```rust
let log = Rc::clone(&application_log);
let service = service.with_logging(move |level, source, message| {
    let _ = log.borrow_mut().append(ic_logging::EntryInput {
        created_at_secs: clock_secs(), level, source, topic: None, message,
    });
});
```

Blob uses the source `blob` and fixed messages. It emits an info entry for first
registration, final release, and each change between drained and active admission.
A command refused for capacity or an elapsed deadline emits one warning per returned
refusal. Reads, historical successes and authorization denials are silent. The
recorder receives no principals, object roots or request payloads. A trap rolls
back heap entries in the same IC execution. After upgrade, rebind to the host’s
new heap log. Metrics and logging can be enabled independently.

See the [repository](https://github.com/dragginzgame/ic-blob-storage) for the
design, limits and qualification. Licensed under the Toko Studios Network Licence
1.1; see the packaged `LICENSE.md`.
