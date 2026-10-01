use super::*;
use ic_blob_storage::dto::reference::{
    ReferenceChange, ReferenceCommand, ReferenceFailure, ReferenceReceiptResponse,
    ReferenceTransitionFailure, ReferenceUpload,
};

fn request() -> ReferenceCommand {
    ReferenceCommand {
        upload: ReferenceUpload {
            service: Principal::self_authenticating([1]),
            tenant: Principal::self_authenticating([2]),
            namespace: u128::MAX,
            upload: u128::MAX - 1,
            object: u128::MAX - 2,
            incarnation: u128::MAX - 3,
            first_reference: u128::MAX - 4,
            root: [7; 32],
            bytes: u64::MAX,
        },
        reference: u128::MAX - 5,
        operation: u128::MAX - 6,
        action: ReferenceAction::Retain,
    }
}
fn arguments(path: &std::path::Path, directory: &std::path::Path) -> Vec<String> {
    let request = request();
    [
        "submit-reference",
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "absent.pem",
        "--actor",
        &request.upload.tenant.to_text(),
        "--service",
        &request.upload.service.to_text(),
        "--namespace",
        &u128::MAX.to_string(),
        "--request",
        path.to_str().unwrap(),
        "--run-dir",
        directory.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn invalid_or_foreign_intent_refuses_before_signing_and_claiming() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("request.candid");
    let run = dir.path().join("dispatch");
    let args = arguments(&file, &run);
    for (input, failure) in [
        (
            ReferenceCommand {
                operation: 0,
                ..request()
            },
            Failure::Arguments,
        ),
        (
            ReferenceCommand {
                upload: ReferenceUpload {
                    namespace: 1,
                    ..request().upload
                },
                ..request()
            },
            Failure::Binding,
        ),
        (
            ReferenceCommand {
                upload: ReferenceUpload {
                    tenant: Principal::self_authenticating([3]),
                    ..request().upload
                },
                ..request()
            },
            Failure::Denied,
        ),
    ] {
        std::fs::write(&file, candid::encode_one(input).unwrap()).unwrap();
        assert_eq!(crate::native::execute(&args), Err(failure));
        assert!(!run.exists());
    }
    for flag in ["--request", "--run-dir", "--actor"] {
        let mut incomplete = args.clone();
        let index = incomplete.iter().position(|value| value == flag).unwrap();
        incomplete.drain(index..=index + 1);
        assert!(matches!(
            Options::parse(&incomplete),
            Err(Failure::Arguments)
        ));
    }
}

#[test]
fn stored_failure_pending_refusal_and_uncertainty_are_distinct_from_success() {
    for result in [
        Ok(ReferenceChange::Changed),
        Err(ReferenceTransitionFailure::UnknownReference),
    ] {
        let response = ReferenceMutationResponse {
            receipt: ReferenceReceiptResponse {
                request: request(),
                result,
            },
            replayed: false,
        };
        let value = outcome("id", &Ok(Some(response)));
        assert_eq!(value["outcome"], "recorded");
        assert_eq!(
            value["result"]["state"],
            if result.is_ok() { "success" } else { "failure" }
        );
        assert_eq!(value["reference_liveness"], "not_observed");
        assert_eq!(value["retry_authorized"], false);
        assert_eq!(value["billing_cessation"], "not_established");
    }
    for (result, expected) in [
        (Ok(None), "pending"),
        (
            Err(Failure::ReferenceRefused(ReferenceFailure::Fenced)),
            "refused",
        ),
        (Err(Failure::Binding), "uncertain"),
        (Err(Failure::Transport), "uncertain"),
    ] {
        let value = outcome("id", &result);
        assert_eq!(value["outcome"], expected);
        assert_eq!(value["result"], Value::Null);
        assert_eq!(value["retry_authorized"], false);
    }
}
