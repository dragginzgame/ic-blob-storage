//! Application-owned conditional catalog changes, not Blob service rules.
//!
//! These helpers run synchronously after the caller verifies the original read.
//! They do not store a Verified flag or supply a remote transaction/recovery log.

use std::collections::BTreeMap;

use ic_blob_storage::{
    local::{Error as BlobError, LocalModel, ReferenceStatus, UploadRequest},
    references::ReferenceId,
};
use ic_platform_types::Principal;

pub(super) type Catalog = BTreeMap<&'static str, ReferenceId>;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Publication {
    Inserted,
    Replayed,
}

#[derive(Debug, Eq, PartialEq)]
pub(super) enum Error {
    Blob(BlobError),
    RequestChanged,
    ReferenceNotLive,
    NameTaken,
    ReferenceAlreadyPublished,
    StaleRemoval,
}

// For a fresh insertion, the caller must first verify the original declaration
// through successful EOF. A retained matching entry establishes a prior commit,
// not current provider availability. Recheck liveness at this commit boundary.
pub(super) fn commit_verified(
    catalog: &mut Catalog,
    blob: &LocalModel,
    application: Principal,
    asset: &'static str,
    original: &UploadRequest,
) -> Result<Publication, Error> {
    let stored = blob
        .inspect_upload(application, original.id)
        .map_err(Error::Blob)?;
    if stored.request != *original {
        return Err(Error::RequestChanged);
    }
    let reference = blob
        .inspect_reference(application, original.first_reference)
        .map_err(Error::Blob)?;
    if reference.status != ReferenceStatus::Live {
        return Err(Error::ReferenceNotLive);
    }
    if let Some(&reference) = catalog.get(asset) {
        return if reference == original.first_reference {
            Ok(Publication::Replayed)
        } else {
            Err(Error::NameTaken)
        };
    }
    // Each asset needs its own use. Sharing a reference would let removing one
    // asset invalidate another even though Blob's reference owner is correct.
    if catalog
        .values()
        .any(|&reference| reference == original.first_reference)
    {
        return Err(Error::ReferenceAlreadyPublished);
    }
    catalog.insert(asset, original.first_reference);

    Ok(Publication::Inserted)
}

// Compare before releasing; a name alone is not a stable removal target.
pub(super) fn remove(
    catalog: &mut Catalog,
    blob: &mut LocalModel,
    application: Principal,
    asset: &'static str,
    expected: ReferenceId,
) -> Result<(), Error> {
    let reference = blob
        .inspect_reference(application, expected)
        .map_err(Error::Blob)?;

    match catalog.get(asset) {
        Some(&current) if current == expected => {
            blob.release(application, expected).map_err(Error::Blob)?;
            catalog.remove(asset);
            Ok(())
        }
        None if matches!(
            reference.status,
            ReferenceStatus::Released | ReferenceStatus::Retired
        ) =>
        {
            Ok(())
        }
        _ => Err(Error::StaleRemoval),
    }
}
