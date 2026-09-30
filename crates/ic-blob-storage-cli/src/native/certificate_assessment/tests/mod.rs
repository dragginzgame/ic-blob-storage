use super::super::arguments::Command;
use super::*;
use ic_blob_storage::dto::{
    reference::ReferenceUpload, upload::certificate::UploadCertificateAssessmentResponse,
};

fn permission() -> UploadAdmissionRequest {
    UploadAdmissionRequest {
        upload: ReferenceUpload {
            service: Principal::from_slice(&[1, 1]),
            tenant: Principal::from_slice(&[2, 1]),
            namespace: u128::MAX,
            upload: u128::MAX - 1,
            object: u128::MAX - 2,
            incarnation: u128::MAX - 3,
            first_reference: u128::MAX - 4,
            root: [6; 32],
            bytes: u64::MAX,
        },
        uploader: Principal::from_slice(&[3, 1]),
        expires_at_ns: u64::MAX,
    }
}
fn arguments(p: UploadAdmissionRequest, path: &std::path::Path) -> Vec<String> {
    [
        "certificate-assessment",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "unused.pem",
        "--actor",
        &p.uploader.to_text(),
        "--service",
        &p.upload.service.to_text(),
        "--namespace",
        &p.upload.namespace.to_string(),
        "--permission",
        path.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn saved_permission_scope_and_uploader_are_checked_before_identity_or_transport() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("permission.candid");
    let p = permission();
    let bytes = candid::encode_one(p).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let mut args = arguments(p, &path);
    let options = Options::parse(&args).unwrap();
    let Command::CertificateAssessment(mut input) = options.command else {
        panic!("wrong command")
    };
    assert_eq!(open(&input, p.uploader).unwrap().0, p);
    assert_eq!(open(&input, p.upload.tenant), Err(Failure::Denied));
    input.namespace = 1;
    assert_eq!(open(&input, p.uploader), Err(Failure::Binding));
    assert_eq!(
        super::super::execute(&{
            let index = args.iter().position(|s| s == "--namespace").unwrap();
            args[index + 1] = "1".into();
            args.clone()
        }),
        Err(Failure::Binding)
    );
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::write(&path, vec![0; 4097]).unwrap();
    assert_eq!(open(&input, p.uploader), Err(Failure::File));
}

#[test]
fn json_preserves_full_width_identity_and_never_turns_a_snapshot_into_authority() {
    let p = permission();
    let options = Options::parse(&arguments(p, std::path::Path::new("unused"))).unwrap();
    for blockers in [
        vec![],
        vec![
            B::StaleObservation,
            B::PrechargeLimits,
            B::ProviderNamespace,
            B::ReplayCharging,
            B::Recovery,
            B::Durability,
        ],
    ] {
        let response = UploadCertificateAssessmentResponse {
            permission: p,
            assessed_at_ns: u64::MAX,
            blockers,
        };
        let bytes = candid::encode_one(Ok::<_, E>(&response)).unwrap();
        let value = output(p, &bytes, &options).unwrap();
        assert_eq!(value["upload"]["object"], p.upload.object.to_string());
        assert_eq!(value["upload"]["bytes"], u64::MAX.to_string());
        assert_eq!(value["assessed_at_ns"], u64::MAX.to_string());
        assert_eq!(value["issuance_authorized"], false);
        assert_eq!(value["retry_authorized"], false);
        if response.blockers.is_empty() {
            assert_eq!(value["blockers"], json!([]));
        } else {
            assert_eq!(
                value["blockers"],
                json!([
                    "stale_observation",
                    "precharge_limits",
                    "provider_namespace",
                    "replay_charging",
                    "recovery",
                    "durability"
                ])
            );
        }
    }
    let denied: Result<UploadCertificateAssessmentResponse, _> = Err(E::Revoked);
    assert_eq!(
        output(p, &candid::encode_one(denied).unwrap(), &options),
        Err(Failure::AssessmentRefused(E::Revoked))
    );
}
