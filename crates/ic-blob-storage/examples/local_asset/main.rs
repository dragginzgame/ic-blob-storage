//! Offline application walkthrough: publication is a catalog decision.
//!
//! Synthetic actors, a fixed SDK-vector declaration and in-memory reads stand in
//! for the application's authenticated session and provider transfer. This emits
//! no certificate, accesses no network and supplies no persistence/rollback proof.
//! Run with `make example-local`; see docs/design/ic-blob-storage-design.md.

mod catalog;
mod fixture;

use std::io::{self, Read};

use ic_blob_storage::caffeine::{Limits as ReadLimits, RootVerifier, VerificationError};
use ic_blob_storage::local::{
    Error as BlobError, LocalModel, ReferenceStatus, UploadRequest, UploadState,
};
use ic_blob_storage::references::RetentionState;
use ic_platform_types::Principal;

use catalog::{Catalog, Error as CatalogError, Publication, commit_verified, remove};

fn main() -> io::Result<()> {
    let application = Principal::from_slice(&[1, 1]);
    let alice = Principal::from_slice(&[2, 1]);
    let bob = Principal::from_slice(&[3, 1]);
    let mut blob = LocalModel::new(application, fixture::limits()).expect("fixed application");
    let chunks = fixture::chunks();
    let mut catalog = Catalog::new();
    println!("Local example only: no uploads, certificates or persistent state.");

    // The app already knows which reference to cancel if the client disappears.
    let cancelled = fixture::cancelled_request(alice);
    blob.authorize(application, cancelled, &[cancelled.root], &[], 10)
        .expect("reserve cancelled draft");
    blob.release(application, cancelled.first_reference)
        .expect("cancel before registration");
    assert_eq!(blob.usage(application).unwrap().reserved_bytes, 0);
    let retired = blob.inspect_object(application, cancelled.root).unwrap();
    assert_eq!(retired.original_upload.state, UploadState::Cancelled);
    assert_eq!(retired.retention, None);
    assert_eq!(
        blob.authorize(
            application,
            UploadRequest {
                id: 2.try_into().unwrap(),
                first_reference: 102.try_into().unwrap(),
                ..cancelled
            },
            &[cancelled.root],
            &[],
            11
        ),
        Err(BlobError::RootRetired)
    );
    println!("Cancelled draft: reservation released; root and IDs permanently retired.");

    // This is the complete, immutable synthetic response, not downloaded input
    // used to choose its own expected root. The declaration is fixed independently.
    let body = fixture::body();
    for (number, uploader, asset) in [(2, alice, "scene"), (3, bob, "preview")] {
        let request = fixture::request(number, uploader);
        let view = blob
            .authorize(application, request, &chunks, &fixture::HEADERS, 11)
            .expect("authorize application use");
        if number == 2 {
            assert_eq!(view.state, UploadState::Reserved);
            blob.register(uploader, request.root, 12)
                .expect("local registration");
        } else {
            // A second asset gets a distinct reference, never another upload right.
            assert_eq!(view.state, UploadState::Attached);
        }
        verify_read(body.as_slice(), &request)?;
        // Publication follows successful EOF AND integrity verification.
        assert_eq!(
            commit_verified(&mut catalog, &blob, application, asset, &request),
            Ok(Publication::Inserted)
        );
    }
    let shared = blob.usage(application).unwrap();
    assert_eq!(catalog.len(), 2);
    assert_eq!(shared.live_bytes, fixture::BYTES as u64);
    assert_eq!(shared.unresolved_bytes, fixture::BYTES as u64);
    // Catalog entries need only reference IDs; Blob owns their immutable targets.
    let original_scene = fixture::request(2, alice);
    for &id in catalog.values() {
        let reference = blob.inspect_reference(application, id).unwrap();
        assert_eq!(reference.root, original_scene.root);
        assert_eq!(reference.status, ReferenceStatus::Live);
    }
    println!(
        "Published scene and preview: two references, one {0}-byte object.",
        fixture::BYTES
    );

    // Simulate losing the catalog commit's reply. Retry the exact original intent,
    // not registration, and do not treat the existing asset as a cleanup failure.
    assert_eq!(
        commit_verified(&mut catalog, &blob, application, "scene", &original_scene),
        Ok(Publication::Replayed)
    );
    assert_eq!(blob.usage(application).unwrap(), shared);
    println!("Publication reply lost: exact retry preserved the original asset and reference.");

    // A failed read must not publish, and cleanup releases only this draft's use.
    let rejected = fixture::request(4, bob);
    blob.authorize(application, rejected, &chunks, &fixture::HEADERS, 13)
        .expect("reserve independent draft reference");
    let mut corrupt = body.clone();
    corrupt[fixture::BYTES - 1] ^= 1;
    match verify_read(corrupt.as_slice(), &rejected) {
        Ok(()) => {
            commit_verified(&mut catalog, &blob, application, "damaged", &rejected)
                .expect("publish verified draft");
        }
        Err(error) => {
            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            blob.release(application, rejected.first_reference)
                .expect("release rejected draft");
        }
    }
    assert!(!catalog.contains_key("damaged"));
    assert_eq!(
        blob.inspect_reference(application, rejected.first_reference)
            .map(|view| view.status),
        Ok(ReferenceStatus::Released)
    );
    println!("Rejected corrupt read: no catalog entry; draft reference released.");

    // Verification can succeed while the application catalog refuses the commit.
    // This reference was freshly allocated; it is not a retry of the published one.
    let collision = fixture::request(5, alice);
    blob.authorize(application, collision, &chunks, &fixture::HEADERS, 14)
        .expect("allocate fresh reference for another draft");
    verify_read(body.as_slice(), &collision)?;
    match commit_verified(&mut catalog, &blob, application, "scene", &collision) {
        Err(CatalogError::NameTaken) => {
            // Only this known fresh, unpublished draft may be compensated.
            blob.release(application, collision.first_reference)
                .expect("release uncommitted use");
        }
        other => panic!("expected this fresh draft to collide: {other:?}"),
    }
    assert_eq!(catalog["scene"], fixture::request(2, alice).first_reference);
    assert_eq!(
        blob.inspect_reference(application, collision.first_reference)
            .map(|view| view.status),
        Ok(ReferenceStatus::Released)
    );
    assert_eq!(
        blob.inspect_reference(application, catalog["scene"])
            .map(|view| view.status),
        Ok(ReferenceStatus::Live)
    );
    println!("Catalog collision: kept the existing asset and released only the new use.");

    // These local calls have no await. This is not a remote catalog transaction.
    remove(
        &mut catalog,
        &mut blob,
        application,
        "scene",
        original_scene.first_reference,
    )
    .expect("release scene");
    let shared_object = blob
        .inspect_object(application, original_scene.root)
        .unwrap();
    assert_eq!(shared_object.original_upload.request, original_scene);
    assert_eq!(shared_object.retention, Some(RetentionState::Open));
    assert_eq!(shared_object.live_references, 1);
    println!("Scene removed: object remains Open with one live reference.");
    assert_eq!(
        blob.inspect_reference(application, catalog["preview"])
            .map(|view| view.status),
        Ok(ReferenceStatus::Live)
    );
    assert_eq!(
        blob.usage(application).unwrap().live_bytes,
        fixture::BYTES as u64
    );
    // The preview still protects the object, allowing a fresh scene reference.
    let replacement = fixture::request(6, alice);
    assert_eq!(
        blob.authorize(application, replacement, &chunks, &fixture::HEADERS, 15)
            .unwrap()
            .state,
        UploadState::Attached
    );
    verify_read(body.as_slice(), &replacement)?;
    commit_verified(&mut catalog, &blob, application, "scene", &replacement)
        .expect("publish replacement");
    assert_eq!(
        remove(
            &mut catalog,
            &mut blob,
            application,
            "scene",
            original_scene.first_reference
        ),
        Err(CatalogError::StaleRemoval)
    );
    assert_eq!(catalog["scene"], replacement.first_reference);
    assert_eq!(
        commit_verified(&mut catalog, &blob, application, "scene", &original_scene),
        Err(CatalogError::ReferenceNotLive)
    );
    println!("Delayed removal/republication refused: replacement scene remains live.");
    remove(
        &mut catalog,
        &mut blob,
        application,
        "scene",
        replacement.first_reference,
    )
    .expect("release replacement");
    let preview = fixture::request(3, bob);
    remove(
        &mut catalog,
        &mut blob,
        application,
        "preview",
        preview.first_reference,
    )
    .expect("final release");
    remove(
        &mut catalog,
        &mut blob,
        application,
        "preview",
        preview.first_reference,
    )
    .expect("repeat final release");
    let final_usage = blob.usage(application).unwrap();
    assert!(catalog.is_empty());
    assert_eq!(final_usage.reserved_bytes, 0);
    assert_eq!(final_usage.live_bytes, 0);
    assert_eq!(final_usage.unresolved_bytes, fixture::BYTES as u64);
    let closing = blob
        .inspect_object(application, original_scene.root)
        .unwrap();
    assert_eq!(closing.original_upload, shared_object.original_upload);
    assert_eq!(closing.original_upload.state, UploadState::Registered);
    assert_eq!(closing.retention, Some(RetentionState::Closing));
    assert_eq!(closing.live_references, 0);
    println!(
        "Final release: live=0, unresolved={} declared bytes; no settlement claim.",
        final_usage.unresolved_bytes
    );
    Ok(())
}

// The application owns completion of the read. Do not stop at expected_bytes:
// trailing bytes or an error after the expected body still fail the response.
fn verify_read(mut source: impl Read, original: &UploadRequest) -> io::Result<()> {
    let manifest = fixture::limits().manifest;
    let mut verifier = RootVerifier::new(
        original.root,
        original.bytes,
        &fixture::HEADERS,
        ReadLimits {
            max_content_bytes: manifest.max_content_bytes,
            max_append_bytes: 65_536_usize.try_into().unwrap(),
            max_headers: manifest.max_headers,
            max_header_bytes: manifest.max_header_bytes,
        },
    )
    .map_err(integrity_error)?;
    let mut frame = [0; 65_536];
    loop {
        let count = match source.read(&mut frame) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if count == 0 {
            break;
        }
        verifier
            .append(verifier.received_bytes(), &frame[..count])
            .map_err(integrity_error)?;
    }
    verifier.finish().map_err(integrity_error)
}

fn integrity_error(error: VerificationError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, format!("{error:?}"))
}

#[cfg(test)]
mod tests;
