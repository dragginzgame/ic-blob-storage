//! Explicit caller guards owned by this Canic artifact, before shared handlers.
ic_blob_storage_canic::declare_contexts!(context = context, fleet_context = fleet_context);
ic_blob_storage_canic::declare_installation!(
    authority = "blob-probe",
    install = install,
    restore = restore
);
