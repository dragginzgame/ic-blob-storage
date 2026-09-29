use super::*;
use ic_blob_storage::dto::{
    reference::ReferenceUpload,
    upload::{admission::UploadAdmissionRequest, completion::*},
};

fn statement() -> UploadAttestationRequest {
    UploadAttestationRequest {
        permission: UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: Principal::self_authenticating([1]),
                tenant: Principal::self_authenticating([2]),
                namespace: u128::MAX,
                upload: u128::MAX - 1,
                object: u128::MAX - 2,
                incarnation: u128::MAX - 3,
                first_reference: u128::MAX - 4,
                root: [8; 32],
                bytes: u64::MAX,
            },
            uploader: Principal::self_authenticating([3]),
            expires_at_ns: 10,
        },
        content_digest: [9; 32],
        observed_at_ns: u64::MAX - 1,
    }
}
fn verifier() -> Principal {
    Principal::self_authenticating([4])
}
fn open(path: &Path, statement: &UploadAttestationRequest) -> Result<Recovery, Failure> {
    Recovery::open(
        statement.permission.upload.service,
        statement.permission.upload.namespace,
        verifier(),
        verifier(),
        path,
    )
}
fn output(recovery: &Recovery, attestation: &UploadAttestationLookup, fenced: bool) -> Value {
    recovery
        .output(
            &candid::encode_one(Ok::<_, UploadAttestationFailure>(
                UploadAttestationResponse {
                    permission: recovery.statement.permission,
                    attestation: *attestation,
                    fenced,
                },
            ))
            .unwrap(),
            verifier(),
            "local",
            "http://127.0.0.1",
        )
        .unwrap()
}

#[test]
fn saved_statement_recovery_distinguishes_absence_match_and_conflict_without_retry() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let statement = statement();
    let original = candid::encode_one(statement).unwrap();
    std::fs::write(file.path(), &original).unwrap();
    let recovery = open(file.path(), &statement).unwrap();
    // Inspection depends only on retained exact intent, not the original content file.
    let receipt = UploadAttestationReceipt {
        request: statement,
        verifier: verifier(),
        accepted_at_ns: u64::MAX,
    };
    for fenced in [false, true] {
        let absent = output(&recovery, &UploadAttestationLookup::Absent, fenced);
        assert_eq!(absent["outcome"], "absent");
        assert!(absent["receipt"].is_null());
        assert_eq!(absent["retry_authorized"], false);
        let matched = output(&recovery, &UploadAttestationLookup::Found(receipt), fenced);
        assert_eq!(matched["outcome"], "matched");
        assert_eq!(matched["fenced"], fenced);
        assert_eq!(matched["upload"]["namespace"], u128::MAX.to_string());
        assert_eq!(matched["upload"]["object"], (u128::MAX - 2).to_string());
        assert_eq!(matched["receipt"]["accepted_at_ns"], u64::MAX.to_string());
        assert_eq!(matched["retry_authorized"], false);
        for time in [true, false] {
            let mut other = receipt;
            if time {
                other.request.observed_at_ns -= 1;
            } else {
                other.request.content_digest[0] ^= 1;
            }
            let conflict = output(&recovery, &UploadAttestationLookup::Found(other), fenced);
            assert_eq!(conflict["outcome"], "conflict");
            assert_eq!(conflict["retry_authorized"], false);
            assert_ne!(
                conflict["expected_statement"],
                json!({
                    "content_digest":conflict["receipt"]["content_digest"],
                    "observed_at_ns":conflict["receipt"]["observed_at_ns"],
                })
            );
        }
    }
    assert_eq!(std::fs::read(file.path()).unwrap(), original);
}

#[test]
fn invalid_saved_intent_scope_actor_and_input_limits_refuse_before_transport() {
    let file = tempfile::NamedTempFile::new().unwrap();
    let s = statement();
    std::fs::write(file.path(), candid::encode_one(s).unwrap()).unwrap();
    let mut other_scope = s;
    other_scope.permission.upload.namespace = 1;
    assert!(matches!(
        open(file.path(), &other_scope),
        Err(Failure::Binding)
    ));
    assert!(matches!(
        Recovery::open(
            s.permission.upload.service,
            u128::MAX,
            verifier(),
            Principal::self_authenticating([8]),
            file.path()
        ),
        Err(Failure::Denied)
    ));
    for actor in [
        s.permission.upload.tenant,
        s.permission.uploader,
        verifier(),
    ] {
        assert!(
            Recovery::open(
                s.permission.upload.service,
                u128::MAX,
                verifier(),
                actor,
                file.path()
            )
            .is_ok()
        );
    }
    let mut invalid = s;
    invalid.permission.upload.upload = 0;
    for bytes in [
        candid::encode_one(invalid).unwrap(),
        b"DIDL".to_vec(),
        candid::encode_args((s, 1_u8)).unwrap(),
    ] {
        std::fs::write(file.path(), bytes).unwrap();
        assert!(matches!(open(file.path(), &s), Err(Failure::Arguments)));
    }
    std::fs::write(file.path(), vec![0; 4097]).unwrap();
    assert!(matches!(open(file.path(), &s), Err(Failure::File)));
}

#[test]
fn command_requires_saved_statement_and_explicit_expected_verifier() {
    use super::super::arguments::{Command, Options};
    let s = statement();
    let args: Vec<String> = [
        "upload-attestation",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "verifier.pem",
        "--actor",
        &verifier().to_text(),
        "--service",
        &s.permission.upload.service.to_text(),
        "--namespace",
        &u128::MAX.to_string(),
        "--statement",
        "statement.candid",
        "--verifier",
        &verifier().to_text(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert!(
        matches!(Options::parse(&args).unwrap().command, Command::UploadAttestation { namespace, .. } if namespace == u128::MAX)
    );
    let mut missing = args.clone();
    missing.truncate(missing.len() - 2);
    assert!(matches!(Options::parse(&missing), Err(Failure::Arguments)));
    let mut unrelated = args;
    unrelated.extend(["--body".into(), "file".into()]);
    assert!(matches!(
        Options::parse(&unrelated),
        Err(Failure::Arguments)
    ));
}
