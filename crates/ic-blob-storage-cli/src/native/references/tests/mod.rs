use super::super::arguments::Command;
use super::*;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptResponse;
use ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse;

fn command() -> ReferenceCommand {
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
fn arguments(kind: &str, path: &std::path::Path) -> Vec<String> {
    let c = command();
    [
        kind,
        "--network",
        "ic",
        "--url",
        "https://icp-api.io",
        "--identity",
        "absent.pem",
        "--actor",
        &c.upload.tenant.to_text(),
        "--service",
        &c.upload.service.to_text(),
        "--namespace",
        &c.upload.namespace.to_string(),
        "--request",
        path.to_str().unwrap(),
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

#[test]
fn saved_exact_requests_bind_tenant_and_scope_before_identity_or_network() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("request.candid");
    let c = command();
    for (kind, encoded) in [
        ("reference-receipt", candid::encode_one(c).unwrap()),
        (
            "reference-status",
            candid::encode_one(ReferenceStatusRequest {
                upload: c.upload,
                reference: c.reference,
            })
            .unwrap(),
        ),
    ] {
        std::fs::write(&path, &encoded).unwrap();
        let args = arguments(kind, &path);
        let Command::Reference(mut input) = Options::parse(&args).unwrap().command else {
            panic!("wrong command")
        };
        let inspection = open(&input, c.upload.tenant).unwrap();
        assert_eq!(inspection.argument, encoded);
        assert!(matches!(
            open(&input, c.upload.service),
            Err(Failure::Denied)
        ));
        input.namespace = 1;
        assert!(matches!(
            open(&input, c.upload.tenant),
            Err(Failure::Binding)
        ));
        let mut foreign = args.clone();
        let index = foreign.iter().position(|s| s == "--namespace").unwrap();
        foreign[index + 1] = "1".into();
        assert_eq!(super::super::execute(&foreign), Err(Failure::Binding));
        let mut actor = args.clone();
        let index = actor.iter().position(|s| s == "--actor").unwrap();
        actor[index + 1] = c.upload.service.to_text();
        assert_eq!(super::super::execute(&actor), Err(Failure::Denied));
        assert_eq!(std::fs::read(&path).unwrap(), encoded);
        std::fs::write(&path, [0; 4097]).unwrap();
        assert!(matches!(open(&input, c.upload.tenant), Err(Failure::File)));
        std::fs::write(&path, b"DIDL").unwrap();
        assert!(matches!(
            open(&input, c.upload.tenant),
            Err(Failure::Arguments)
        ));
    }
    std::fs::write(
        &path,
        candid::encode_one(ReferenceCommand { reference: 0, ..c }).unwrap(),
    )
    .unwrap();
    assert_eq!(
        super::super::execute(&arguments("reference-receipt", &path)),
        Err(Failure::Arguments)
    );
}

#[test]
fn historical_success_failure_and_absence_never_observe_current_liveness_or_fence() {
    let c = command();
    let options = Options::parse(&arguments(
        "reference-receipt",
        std::path::Path::new("unused"),
    ))
    .unwrap();
    for result in [
        Ok(ReferenceChange::Changed),
        Ok(ReferenceChange::Unchanged),
        Err(ReferenceTransitionFailure::Released),
    ] {
        let lookup = ReferenceReceiptLookup::Found(ReferenceReceiptResponse { request: c, result });
        let bytes = candid::encode_one(Ok::<_, ReferenceFailure>(lookup)).unwrap();
        let value = output(&Request::Receipt(c), &bytes, &options).unwrap();
        assert_eq!(value["outcome"], "found");
        assert_eq!(value["upload"]["object"], c.upload.object.to_string());
        assert_eq!(value["upload"]["bytes"], u64::MAX.to_string());
        assert_eq!(value["reference"], c.reference.to_string());
        assert_eq!(value["operation"], c.operation.to_string());
        assert_eq!(value["reference_liveness"], "not_observed");
        assert_eq!(value["fence"], "not_observed");
        assert_eq!(value["retry_authorized"], false);
        assert_eq!(value["publication_authorized"], false);
        if result.is_err() {
            assert_eq!(
                value["result"],
                json!({"state":"failure","failure":"released"})
            );
        }
    }
    let absent =
        candid::encode_one(Ok::<_, ReferenceFailure>(ReferenceReceiptLookup::Absent)).unwrap();
    let value = output(&Request::Receipt(c), &absent, &options).unwrap();
    assert_eq!(value["outcome"], "absent");
    assert_eq!(value["result"], Value::Null);
    assert_eq!(value["retry_authorized"], false);
    let refused: Result<ReferenceReceiptLookup, _> = Err(ReferenceFailure::Conflict);
    assert_eq!(
        output(
            &Request::Receipt(c),
            &candid::encode_one(refused).unwrap(),
            &options
        ),
        Err(Failure::ReferenceRefused(ReferenceFailure::Conflict))
    );
}

#[test]
fn current_liveness_and_restore_fence_are_independent_bounded_observations() {
    let c = command();
    let request = ReferenceStatusRequest {
        upload: c.upload,
        reference: c.reference,
    };
    let options = Options::parse(&arguments(
        "reference-status",
        std::path::Path::new("unused"),
    ))
    .unwrap();
    for (live, fenced) in [(true, false), (true, true), (false, true)] {
        let bytes = candid::encode_one(Ok::<_, ReferenceFailure>(ReferenceStatusResponse {
            request,
            live,
            fenced,
        }))
        .unwrap();
        let value = output(&Request::Status(request), &bytes, &options).unwrap();
        assert_eq!(value["live"], live);
        assert_eq!(value["fenced"], fenced);
        assert_eq!(value["publication_authorized"], false);
        assert_eq!(value["retry_authorized"], false);
        assert!(value.get("operation").is_none());
    }
    let bytes = candid::encode_one(Ok::<_, ReferenceFailure>(ReferenceStatusResponse {
        request: ReferenceStatusRequest {
            reference: 1,
            ..request
        },
        live: true,
        fenced: false,
    }))
    .unwrap();
    assert_eq!(
        output(&Request::Status(request), &bytes, &options),
        Err(Failure::Binding)
    );
    assert_eq!(
        output(&Request::Status(request), &vec![0; 4097], &options),
        Err(Failure::ReplyLimit)
    );
    assert_eq!(
        output(&Request::Status(request), b"DIDL", &options),
        Err(Failure::InvalidReply)
    );
}
