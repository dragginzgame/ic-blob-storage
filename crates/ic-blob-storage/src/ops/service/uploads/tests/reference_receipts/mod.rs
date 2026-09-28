use super::*;
use crate::ops::service::references::parse;
use crate::{dto::reference::*, workflow::references::receipt};

#[test]
#[expect(
    clippy::too_many_lines,
    reason = "One ordered independent-identity receipt journey through settlement"
)]
fn reference_receipt_boundary_preserves_independent_full_width_identities_and_failures() {
    let mut store = StableUploads::install(memory(), config()).unwrap();
    enroll(&mut store);
    let manifest = built();
    let mut input = permission(1);
    input.request.id = UploadRequestId::new(NonZeroU128::new(u128::MAX).unwrap());
    let object = ObjectBinding::new(
        p(1),
        p(4),
        ObjectIdentity {
            namespace: NonZeroU128::MIN,
            object: NonZeroU128::new(u128::MAX - 1).unwrap(),
            incarnation: NonZeroU128::new(u128::MAX - 2).unwrap(),
        },
    )
    .unwrap();
    input.request.object.first =
        ReferenceKey::new(object, ReferenceId::new(NonZeroU128::new(7).unwrap()));
    input.request.object.root = manifest.hashes().provider_root;
    let request = ReferenceCommand {
        upload: ReferenceUpload {
            service: p(1),
            tenant: p(4),
            namespace: 1,
            upload: u128::MAX,
            object: u128::MAX - 1,
            incarnation: u128::MAX - 2,
            first_reference: 7,
            root: *input.request.object.root.as_bytes(),
            bytes: 10,
        },
        reference: 8,
        operation: u128::MAX - 3,
        action: ReferenceAction::Release,
    };
    assert_eq!(
        receipt(&store, context(4), request),
        Err(ReferenceFailure::Unknown)
    );
    store.admit(context(4), input, 1).unwrap();
    assert_eq!(
        receipt(&store, context(4), request),
        Err(ReferenceFailure::Unconfirmed)
    );
    store
        .prepare_manifest(
            context(5),
            input.request,
            UploadManifest {
                chunks: manifest.manifest().chunks(),
                headers: &HEADERS,
            },
            2,
        )
        .unwrap();
    store.expose(context(5), input.request, 3).unwrap();
    store.confirm_upload(input.request).unwrap();
    assert_eq!(
        receipt(&store, context(4), request),
        Ok(ReferenceReceiptLookup::Absent)
    );
    let (upload, _) = parse(context(4), request).unwrap();
    assert_eq!(upload, input.request);
    crate::workflow::references::apply(&mut store, context(4), request).unwrap();
    let historical = ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
        request,
        result: Err(ReferenceTransitionFailure::UnknownReference),
    });
    assert_eq!(receipt(&store, context(4), request), Ok(historical));
    let release = ReferenceCommand {
        reference: 7,
        operation: 1,
        ..request
    };
    crate::workflow::references::apply(&mut store, context(4), release).unwrap();
    store.confirm_provider_deleted(upload).unwrap();
    store.confirm_billing_stopped(upload).unwrap();
    assert_eq!(receipt(&store, context(4), request), Ok(historical));
    assert_eq!(
        receipt(&store, context(2), request),
        Err(ReferenceFailure::Denied)
    );
    assert_eq!(
        receipt(
            &store,
            context(4),
            ReferenceCommand {
                action: ReferenceAction::Retain,
                ..request
            }
        ),
        Err(ReferenceFailure::Conflict)
    );
    for changed in [
        ReferenceUpload {
            upload: 5,
            ..request.upload
        },
        ReferenceUpload {
            first_reference: 1,
            ..request.upload
        },
        ReferenceUpload {
            root: [0; 32],
            ..request.upload
        },
    ] {
        let expected = if changed.upload == 5 {
            ReferenceFailure::Unknown
        } else {
            ReferenceFailure::Conflict
        };
        assert_eq!(
            receipt(
                &store,
                context(4),
                ReferenceCommand {
                    upload: changed,
                    ..request
                }
            ),
            Err(expected)
        );
    }
}
