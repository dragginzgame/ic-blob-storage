use std::io::{self, Read};

use ic_blob_storage::local::{Error as BlobError, LocalModel, ReferenceStatus};
use ic_platform_types::Principal;

use super::{Catalog, CatalogError, Publication, commit_verified, fixture, remove, verify_read};

struct FailedEnd;

impl Read for FailedEnd {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::new(
            io::ErrorKind::ConnectionReset,
            "response failed after body",
        ))
    }
}

#[test]
fn complete_body_without_successful_eof_is_not_a_verified_read() {
    let body = fixture::body();
    let request = fixture::request(1, Principal::from_slice(&[2, 1]));
    let source = body.as_slice().chain(FailedEnd);
    assert_eq!(
        verify_read(source, &request).unwrap_err().kind(),
        io::ErrorKind::ConnectionReset
    );
}

#[test]
fn read_must_not_truncate_excess_or_accept_short_content() {
    let body = fixture::body();
    let request = fixture::request(1, Principal::from_slice(&[2, 1]));
    assert_eq!(
        verify_read(&body[..body.len() - 1], &request)
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidData
    );
    let excess = body.as_slice().chain(&b"extra"[..]);
    assert_eq!(
        verify_read(excess, &request).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
}

#[test]
fn cancellation_after_verification_is_rechecked_at_publication() {
    let application = Principal::from_slice(&[1, 1]);
    let uploader = Principal::from_slice(&[2, 1]);
    let body = fixture::body();
    let original = fixture::request(1, uploader);
    for registered in [false, true] {
        let mut blob = LocalModel::new(application, fixture::limits()).unwrap();
        let mut catalog = Catalog::new();
        blob.authorize(
            application,
            original,
            &fixture::chunks(),
            &fixture::HEADERS,
            10,
        )
        .unwrap();
        if registered {
            blob.register(uploader, original.id, 11).unwrap();
        }
        verify_read(body.as_slice(), &original).unwrap();
        // Represents cancellation between a completed read and catalog commit.
        blob.release(application, original.first_reference).unwrap();
        let after_cancel = blob.usage(application).unwrap();
        assert_eq!(
            commit_verified(&mut catalog, &blob, application, "scene", &original),
            Err(CatalogError::ReferenceNotLive)
        );
        blob.authorize(application, original, &[], &[], 200)
            .unwrap();
        assert_eq!(
            commit_verified(&mut catalog, &blob, application, "scene", &original),
            Err(CatalogError::ReferenceNotLive)
        );
        assert!(catalog.is_empty());
        assert_eq!(blob.usage(application).unwrap(), after_cancel);
        assert_eq!(after_cancel.reserved_bytes, 0);
        assert_eq!(after_cancel.live_bytes, 0);
        assert_eq!(
            after_cancel.unresolved_bytes,
            if registered { fixture::BYTES as u64 } else { 0 }
        );
    }
}

#[test]
fn catalog_refusals_leave_published_references_and_targets_unchanged() {
    let application = Principal::from_slice(&[1, 1]);
    let uploader = Principal::from_slice(&[2, 1]);
    let original = fixture::request(1, uploader);
    let mut blob = LocalModel::new(application, fixture::limits()).unwrap();
    let mut catalog = Catalog::new();
    blob.authorize(
        application,
        original,
        &fixture::chunks(),
        &fixture::HEADERS,
        10,
    )
    .unwrap();
    blob.register(uploader, original.id, 11).unwrap();
    verify_read(fixture::body().as_slice(), &original).unwrap();
    assert_eq!(
        commit_verified(&mut catalog, &blob, application, "scene", &original),
        Ok(Publication::Inserted)
    );
    assert_eq!(
        commit_verified(&mut catalog, &blob, application, "scene", &original),
        Ok(Publication::Replayed)
    );
    let before = blob.usage(application).unwrap();
    let catalog_before = catalog.clone();
    assert_eq!(
        commit_verified(&mut catalog, &blob, application, "alias", &original),
        Err(CatalogError::ReferenceAlreadyPublished)
    );
    let changed = super::UploadRequest {
        root: [0; 32],
        ..original
    };
    assert_eq!(
        commit_verified(&mut catalog, &blob, application, "scene", &changed),
        Err(CatalogError::RequestChanged)
    );
    assert_eq!(
        commit_verified(&mut catalog, &blob, uploader, "scene", &original),
        Err(CatalogError::Blob(BlobError::Denied))
    );
    assert_eq!(
        remove(
            &mut catalog,
            &mut blob,
            uploader,
            "scene",
            original.first_reference
        ),
        Err(CatalogError::Blob(BlobError::Denied))
    );
    assert_eq!(
        remove(
            &mut catalog,
            &mut blob,
            application,
            "missing",
            original.first_reference
        ),
        Err(CatalogError::StaleRemoval)
    );
    assert_eq!(catalog, catalog_before);
    assert_eq!(blob.usage(application).unwrap(), before);
    assert_eq!(
        blob.inspect_reference(application, original.first_reference),
        Ok(ReferenceStatus::Live)
    );
}
