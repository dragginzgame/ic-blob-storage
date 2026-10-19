use super::*;
use ic_platform::Principal;

fn permission() -> UploadAdmissionRequest {
    UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::self_authenticating([1]),
            tenant: Principal::self_authenticating([2]),
            namespace: u128::MAX,
            upload: u128::MAX - 1,
            object: u128::MAX - 2,
            incarnation: u128::MAX - 3,
            first_reference: u128::MAX - 4,
            root: [7; 32],
            bytes: 3,
        },
        uploader: Principal::self_authenticating([3]),
        // Cleanup inputs remain representable after upload permission expiry.
        expires_at_ns: 1,
    }
}
fn args(base: &Path, permission: UploadAdmissionRequest, action: &str) -> Vec<String> {
    std::fs::write(
        base.join("permission.cbor"),
        ic_blob_storage_contracts::document::encode(permission).unwrap(),
    )
    .unwrap();
    vec![
        "reference-inputs".into(),
        "--permission".into(),
        base.join("permission.cbor").display().to_string(),
        "--action".into(),
        action.into(),
        "--reference".into(),
        u128::MAX.to_string(),
        "--operation".into(),
        u128::MAX.to_string(),
        "--run-dir".into(),
        base.join("output").display().to_string(),
    ]
}
#[test]
fn exact_full_width_retain_and_expired_permission_release_need_no_network() {
    for (name, action) in [
        ("retain", ReferenceAction::Retain),
        ("release", ReferenceAction::Release),
    ] {
        let base = tempfile::tempdir().unwrap();
        let permission = permission();
        let args = args(base.path(), permission, name);
        let result = crate::native::execute(&args).unwrap();
        let output = base.path().join("output");
        let bytes = std::fs::read(output.join("reference.cbor")).unwrap();
        let command: ReferenceCommand = ic_blob_storage_contracts::document::decode(
            &bytes,
            ic_blob_storage_contracts::wire::MAX_MESSAGE_BYTES,
        )
        .unwrap();
        assert_eq!(
            command,
            ReferenceCommand {
                upload: permission.upload,
                reference: u128::MAX,
                operation: u128::MAX,
                action
            }
        );
        let status: ReferenceStatusRequest = ic_blob_storage_contracts::document::decode(
            &std::fs::read(output.join("reference-status.cbor")).unwrap(),
            ic_blob_storage_contracts::wire::MAX_MESSAGE_BYTES,
        )
        .unwrap();
        assert_eq!(
            status,
            ReferenceStatusRequest {
                upload: permission.upload,
                reference: u128::MAX
            }
        );
        let download: DownloadRequest = ic_blob_storage_contracts::document::decode(
            &std::fs::read(output.join("download.cbor")).unwrap(),
            ic_blob_storage_contracts::wire::MAX_MESSAGE_BYTES,
        )
        .unwrap();
        assert_eq!(download.object, permission.upload.object);
        assert_eq!(download.incarnation, permission.upload.incarnation);
        assert_eq!(download.reference, u128::MAX);
        assert_eq!(download.tenant, permission.upload.tenant);
        assert_eq!(download.root, permission.upload.root);
        assert_eq!(
            result["request_sha256"],
            ContentDigest::compute(&bytes).to_string()
        );
        assert_eq!(result["identities_allocated"], false);
        assert_eq!(result["service_dispatched"], false);
        assert_eq!(result["retry_authorized"], false);
        assert_eq!(
            std::fs::read(output.join("permission.cbor")).unwrap(),
            std::fs::read(base.path().join("permission.cbor")).unwrap()
        );
        assert_eq!(crate::native::execute(&args), Err(Failure::ExistingRun));
        assert_eq!(std::fs::read(output.join("reference.cbor")).unwrap(), bytes);
    }
}
#[test]
fn ambiguous_identities_and_options_refuse_before_claim() {
    for (index, value) in [
        (4, "other"),
        (6, "0"),
        (6, "01"),
        (6, "+1"),
        (8, "0"),
        (8, "340282366920938463463374607431768211456"),
    ] {
        let base = tempfile::tempdir().unwrap();
        let mut args = args(base.path(), permission(), "release");
        args[index] = value.into();
        assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
    for extra in [
        vec!["--action".into(), "retain".into()],
        vec!["--unknown".into(), "x".into()],
        vec!["--incomplete".into()],
    ] {
        let base = tempfile::tempdir().unwrap();
        let mut args = args(base.path(), permission(), "release");
        args.extend(extra);
        assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
}
#[test]
fn malformed_or_invalid_permission_refuses_and_partial_runs_are_never_resumed() {
    let base = tempfile::tempdir().unwrap();
    let args = args(base.path(), permission(), "release");
    for bytes in [
        b"invalid".to_vec(),
        ic_blob_storage_contracts::document::encode((
            permission(),
            ic_platform::candid::contract::Reserved,
        ))
        .unwrap(),
        ic_blob_storage_contracts::document::encode((permission(), ())).unwrap(),
        ic_blob_storage_contracts::document::encode(ReferenceCommand {
            upload: permission().upload,
            reference: 1,
            operation: 1,
            action: ReferenceAction::Release,
        })
        .unwrap(),
    ] {
        std::fs::write(base.path().join("permission.cbor"), bytes).unwrap();
        assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
        assert!(!base.path().join("output").exists());
    }
    let mut invalid = permission();
    invalid.upload.tenant = Principal::anonymous();
    std::fs::write(
        base.path().join("permission.cbor"),
        ic_blob_storage_contracts::document::encode(invalid).unwrap(),
    )
    .unwrap();
    assert_eq!(crate::native::execute(&args), Err(Failure::Arguments));
    std::fs::write(base.path().join("permission.cbor"), vec![0; 4097]).unwrap();
    assert_eq!(crate::native::execute(&args), Err(Failure::File));
    std::fs::write(
        base.path().join("permission.cbor"),
        ic_blob_storage_contracts::document::encode(permission()).unwrap(),
    )
    .unwrap();
    std::fs::create_dir(base.path().join("output")).unwrap();
    assert_eq!(crate::native::execute(&args), Err(Failure::ExistingRun));
    assert!(
        std::fs::read_dir(base.path().join("output"))
            .unwrap()
            .next()
            .is_none()
    );
}
