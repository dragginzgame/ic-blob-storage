use super::*;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_host_fs::testing::FixtureDirectory;

fn saved(kind: Kind) -> (FixtureDirectory, Input, UploadAdmissionRequest) {
    let temp = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let permission = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::self_authenticating([1]),
            tenant: Principal::self_authenticating([2]),
            namespace: u128::MAX,
            upload: u128::MAX,
            object: u128::MAX,
            incarnation: u128::MAX,
            first_reference: u128::MAX,
            root: [1; 32],
            bytes: 10,
        },
        uploader: Principal::self_authenticating([3]),
        expires_at_ns: u64::MAX,
    };
    let input = Input {
        kind,
        service: permission.upload.service,
        namespace: permission.upload.namespace,
        request: temp.path().join("permission.wire"),
        directory: Some(temp.path().join("run")),
    };
    std::fs::write(
        &input.request,
        ic_blob_storage_contracts::document::encode(permission).unwrap(),
    )
    .unwrap();
    (temp, input, permission)
}
#[test]
fn exact_roles_scope_and_invalid_ids_refuse_before_artifact_claim() {
    let (_temp, mut input, p) = saved(Kind::Admit);
    assert!(request::load(&input, p.upload.tenant).is_ok());
    std::fs::write(
        &input.request,
        ic_blob_storage_contracts::document::encode((p, ic_platform::candid::contract::Reserved))
            .unwrap(),
    )
    .unwrap();
    assert!(matches!(
        request::load(&input, p.upload.tenant),
        Err(Failure::Arguments)
    ));
    std::fs::write(
        &input.request,
        ic_blob_storage_contracts::document::encode(p).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        request::load(&input, p.uploader),
        Err(Failure::Binding)
    ));
    input.kind = Kind::Manifest;
    assert!(request::load(&input, p.uploader).is_ok());
    input.namespace -= 1;
    assert!(matches!(
        request::load(&input, p.uploader),
        Err(Failure::Binding)
    ));
    input.namespace = p.upload.namespace;
    let mut invalid = p;
    invalid.upload.first_reference = 0;
    std::fs::write(
        &input.request,
        ic_blob_storage_contracts::document::encode(invalid).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        request::load(&input, p.upload.tenant),
        Err(Failure::InvalidReply)
    ));
    assert!(!input.directory.unwrap().exists());
}
#[test]
fn original_permission_echo_and_typed_refusal_are_required_for_acknowledgment() {
    let (_temp, input, p) = saved(Kind::Admit);
    let saved = request::load(&input, p.upload.tenant).unwrap();
    let mut reply = UploadAdmissionMutation {
        admission: UploadAdmissionResponse {
            permission: p,
            state: UploadState::Reserved,
            revoked: false,
        },
        replayed: false,
    };
    let encode = |r: &UploadAdmissionMutation| {
        ic_blob_storage_contracts::wire::encode_response("blob_admit_upload", Ok::<_, A>(r))
            .unwrap()
    };
    assert_eq!(
        observation(&saved, &encode(&reply)).unwrap().json()["replayed"],
        false
    );
    reply.admission.permission.expires_at_ns -= 1;
    assert!(matches!(
        observation(&saved, &encode(&reply)),
        Err(Failure::Binding)
    ));
    assert!(matches!(
        observation(
            &saved,
            &ic_blob_storage_contracts::wire::encode_response(
                "blob_admit_upload",
                Err::<UploadAdmissionMutation, _>(A::Fenced)
            )
            .unwrap()
        ),
        Err(Failure::UploadAdmissionRefused(A::Fenced))
    ));
}

#[test]
fn mutations_require_a_journal_while_inspections_do_not() {
    let (_temp, mut input, permission) = saved(Kind::Admit);
    input.directory = None;
    for kind in [Kind::Admit, Kind::Revoke] {
        input.kind = kind;
        assert!(matches!(
            request::load(&input, permission.upload.tenant),
            Err(Failure::Arguments)
        ));
    }
    for kind in [Kind::Permission, Kind::Manifest] {
        input.kind = kind;
        let saved = request::load(&input, permission.upload.tenant).unwrap();
        assert_eq!(saved.payload.kind(), kind);
        assert!(saved.payload.directory().is_none());
    }
}

#[test]
fn prepared_requests_preserve_external_loader_bytes_roles_and_claims_without_files() {
    use crate::native::upload_inputs::{InputFiles, PreparedInput, tests as fixture};
    let temp = FixtureDirectory::new(&std::env::temp_dir(), "blob-test").unwrap();
    let selected = PreparedInput::load(InputFiles {
        binding: serde_json::to_vec(&fixture::binding()).unwrap(),
        manifest: serde_json::to_vec(&fixture::manifest()).unwrap(),
        installation: crate::native::upload_inputs::installation::Installation::decode(
            ic_blob_storage_contracts::wire::encode_request(
                "blob_install",
                fixture::installation(&fixture::binding()),
            )
            .unwrap(),
            10.try_into().unwrap(),
        )
        .unwrap(),
        body: temp.path().join("body.bin").into(),
    })
    .unwrap();
    let (permission, manifest) = selected.setup_requests();
    for kind in [
        Kind::Admit,
        Kind::Prepare,
        Kind::Revoke,
        Kind::Permission,
        Kind::Manifest,
    ] {
        let actor = if kind == Kind::Prepare {
            selected.permission.uploader
        } else {
            selected.permission.upload.tenant
        };
        let claim = kind.mutation().then(|| temp.path().join("claim"));
        let prepared = Request::prepared(
            kind,
            &selected,
            claim.clone(),
            selected.permission.upload.service,
            selected.permission.upload.namespace,
            actor,
        )
        .unwrap();
        let input = Input {
            kind,
            service: selected.permission.upload.service,
            namespace: selected.permission.upload.namespace,
            request: temp.path().join("request.wire"),
            directory: claim,
        };
        std::fs::write(
            &input.request,
            if kind == Kind::Prepare {
                manifest
            } else {
                permission
            },
        )
        .unwrap();
        let loaded = request::load(&input, actor).unwrap();
        assert_eq!(prepared.argument, loaded.argument);
        assert_eq!(prepared.payload.directory(), loaded.payload.directory());
        std::fs::remove_file(&input.request).unwrap();
        assert!(
            Request::prepared(
                kind,
                &selected,
                input.directory,
                input.service,
                input.namespace,
                actor
            )
            .is_ok()
        );
        assert!(matches!(
            Request::prepared(
                kind,
                &selected,
                None,
                input.service,
                input.namespace,
                Principal::anonymous()
            ),
            Err(Failure::Binding | Failure::Arguments)
        ));
        if kind.mutation() {
            assert!(matches!(
                Request::prepared(kind, &selected, None, input.service, input.namespace, actor),
                Err(Failure::Arguments)
            ));
        }
    }
    assert!(!temp.path().join("claim").exists());
}
