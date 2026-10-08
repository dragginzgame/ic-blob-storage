use super::*;
use ic_blob_storage_contracts::dto::reference::ReferenceUpload;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestDeclaration;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestHeader;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHashLimits;
use ic_blob_storage_contracts::identity::caffeine::CaffeineHeader;
use ic_blob_storage_contracts::identity::caffeine::manifest::builder::CaffeineManifestBuilder;

fn fixture() -> (
    tempfile::TempDir,
    UploadAdmissionRequest,
    UploadManifestResponse,
) {
    let dir = tempfile::tempdir().unwrap();
    let headers = [CaffeineHeader {
        name: "Content-Length",
        value: "3",
    }];
    let mut builder = CaffeineManifestBuilder::new(
        3,
        &headers,
        CaffeineHashLimits {
            max_content_bytes: 3.try_into().unwrap(),
            max_append_bytes: 3.try_into().unwrap(),
            max_headers: 1.try_into().unwrap(),
            max_header_bytes: 64.try_into().unwrap(),
        },
        1.try_into().unwrap(),
    )
    .unwrap();
    builder.append(0, b"abc").unwrap();
    let prepared = builder.finish().unwrap();
    let actor = Principal::self_authenticating([1]);
    let permission = UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::self_authenticating([2]),
            tenant: actor,
            namespace: u128::MAX,
            upload: u128::MAX - 1,
            object: u128::MAX - 2,
            incarnation: u128::MAX - 3,
            first_reference: u128::MAX - 4,
            root: *prepared.hashes().provider_root.as_bytes(),
            bytes: 3,
        },
        uploader: actor,
        expires_at_ns: u64::MAX,
    };
    let response = UploadManifestResponse {
        permission,
        manifest: UploadManifestInspection::Prepared(UploadManifestDeclaration {
            chunks: prepared
                .manifest()
                .chunks()
                .iter()
                .map(|c| *c.as_bytes())
                .collect(),
            headers: vec![UploadManifestHeader {
                name: "Content-Length".into(),
                value: "3".into(),
            }],
        }),
    };
    std::fs::write(
        dir.path().join("permission"),
        candid::encode_one(permission).unwrap(),
    )
    .unwrap();
    std::fs::write(dir.path().join("body"), b"abc").unwrap();
    (dir, permission, response)
}
fn open(dir: &Path, p: UploadAdmissionRequest) -> Result<Verification, Failure> {
    Verification::open(
        p.upload.service,
        p.upload.namespace,
        p.uploader,
        &dir.join("permission"),
        &dir.join("body"),
        3.try_into().unwrap(),
    )
}
fn reply(value: UploadManifestResponse) -> Vec<u8> {
    candid::encode_one(Ok::<_, UploadManifestFailure>(value)).unwrap()
}
#[test]
fn verified_local_bytes_preserve_exact_identity_without_completion_authority() {
    let (dir, p, response) = fixture();
    let report = open(dir.path(), p)
        .unwrap()
        .finish(&reply(response), p.uploader, "local", "http://127.0.0.1")
        .unwrap();
    assert_eq!(report["upload"]["object"], p.upload.object.to_string());
    assert_eq!(
        report["upload"]["first_reference"],
        p.upload.first_reference.to_string()
    );
    assert_eq!(report["provider_completion"], "not_established");
    assert_eq!(report["retry_authorized"], false);
}
#[test]
fn changed_manifest_unprepared_and_corrupt_or_changing_files_refuse_evidence() {
    let (dir, p, response) = fixture();
    let mut changed = response.clone();
    changed.permission.expires_at_ns -= 1;
    assert_eq!(
        open(dir.path(), p)
            .unwrap()
            .finish(&reply(changed), p.uploader, "local", "x"),
        Err(Failure::Binding)
    );
    let mut missing = response.clone();
    missing.manifest = UploadManifestInspection::Unprepared;
    assert_eq!(
        open(dir.path(), p)
            .unwrap()
            .finish(&reply(missing), p.uploader, "local", "x"),
        Err(Failure::Unprepared)
    );
    for body in [b"abd".as_slice(), b"ab", b"abcd"] {
        std::fs::write(dir.path().join("body"), b"abc").unwrap();
        let verification = open(dir.path(), p).unwrap();
        // Mutate the same inode after the pre-query size check, testing the streaming bound/EOF.
        std::fs::write(dir.path().join("body"), body).unwrap();
        assert_eq!(
            verification.finish(&reply(response.clone()), p.uploader, "local", "x"),
            Err(Failure::Content)
        );
    }
}
#[test]
fn permission_scope_actor_and_resource_bounds_reject_before_query() {
    let (dir, p, _) = fixture();
    let mut wrong = p;
    wrong.upload.namespace = 1;
    assert!(matches!(open(dir.path(), wrong), Err(Failure::Binding)));
    wrong = p;
    wrong.uploader = Principal::self_authenticating([9]);
    assert!(matches!(open(dir.path(), wrong), Err(Failure::Denied)));
    assert!(matches!(
        Verification::open(
            p.upload.service,
            p.upload.namespace,
            p.uploader,
            &dir.path().join("permission"),
            &dir.path().join("body"),
            2.try_into().unwrap()
        ),
        Err(Failure::Arguments)
    ));
    let mutations: [fn(&mut UploadAdmissionRequest); 9] = [
        |p| p.upload.namespace = 0,
        |p| p.upload.upload = 0,
        |p| p.upload.object = 0,
        |p| p.upload.incarnation = 0,
        |p| p.upload.first_reference = 0,
        |p| p.upload.bytes = 0,
        |p| p.upload.tenant = Principal::anonymous(),
        |p| p.uploader = Principal::management_canister(),
        |p| p.upload.service = Principal::anonymous(),
    ];
    for mutate in mutations {
        let mut invalid = p;
        mutate(&mut invalid);
        std::fs::write(
            dir.path().join("permission"),
            candid::encode_one(invalid).unwrap(),
        )
        .unwrap();
        assert_eq!(
            Verification::open(
                invalid.upload.service,
                invalid.upload.namespace,
                p.upload.tenant,
                &dir.path().join("permission"),
                &dir.path().join("missing-body"),
                3.try_into().unwrap(),
            )
            .map(|_| ()),
            Err(Failure::Arguments),
            "{invalid:?}"
        );
    }
    std::fs::write(dir.path().join("permission"), vec![0; 4097]).unwrap();
    assert!(matches!(open(dir.path(), p), Err(Failure::File)));
}
