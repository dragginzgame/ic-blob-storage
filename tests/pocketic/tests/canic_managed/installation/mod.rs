//! Exact managed input, release authority and failed-install transaction evidence.
use super::{Fixture, wasm};
use blob_canic_probe::CompositionSnapshot;
use canic::dto::abi::v1::CanisterInitPayload;
use ic_blob_storage::dto::{
    configuration::HostFailure,
    tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
};
use ic_blob_storage_canic::arguments::{InitializationArgumentFailure, installation_arguments};
use ic_testkit::pic::{CandidCallExt, CanisterInstallExt};
use std::time::Duration;

#[derive(candid::CandidType)]
struct MissingPolicy {
    project: String,
}
pub(super) fn enroll(f: &Fixture) -> (TenantScope, TenantEnrollmentResponse) {
    let scope = TenantScope {
        service: f.app(),
        namespace: u128::MAX,
        tenant: candid::Principal::from_slice(&[3, 1]),
    };
    let reply: Result<TenantEnrollmentResponse, TenantFailure> = f
        .pic()
        .update_candid_as(
            f.app(),
            candid::Principal::from_slice(&[2, 1]),
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope,
                expected: None,
                active: true,
            },),
        )
        .unwrap();
    (scope, reply.unwrap())
}

fn invalid_carriers(original: &[u8]) -> Vec<Vec<u8>> {
    let (payload, application): (CanisterInitPayload, Option<Vec<u8>>) =
        candid::decode_args(original).unwrap();
    let mut output = vec![
        candid::encode_args((payload.clone(), None::<Vec<u8>>)).unwrap(),
        candid::encode_args((payload.clone(), Some(vec![0u8; 16 * 1024 + 1]))).unwrap(),
        candid::encode_args((payload.clone(), Some(b"malformed".to_vec()))).unwrap(),
        candid::encode_args((payload.clone(), Some(vec![0u8; 256 * 1024]))).unwrap(),
        candid::encode_args((
            payload.clone(),
            Some(
                candid::encode_one(MissingPolicy {
                    project: "explicit but incomplete".to_owned(),
                })
                .unwrap(),
            ),
        ))
        .unwrap(),
        candid::encode_args((payload.clone(), application.clone(), 7u8)).unwrap(),
    ];
    let mut input = blob_canic_probe::configuration::input();
    input.resources.max_object_bytes = 0;
    output.push(
        candid::encode_args((payload.clone(), Some(candid::encode_one(input).unwrap()))).unwrap(),
    );
    let mut input = blob_canic_probe::configuration::input();
    input.completion_verifier = candid::Principal::anonymous();
    output.push(
        candid::encode_args((payload.clone(), Some(candid::encode_one(input).unwrap()))).unwrap(),
    );
    let mut input = blob_canic_probe::configuration::input();
    " ".clone_into(&mut input.project);
    output.push(
        candid::encode_args((payload.clone(), Some(candid::encode_one(input).unwrap()))).unwrap(),
    );
    let mut wrong_release = payload;
    wrong_release.release_build_id = "02".repeat(32).parse().unwrap();
    output.push(candid::encode_args((wrong_release, application)).unwrap());
    output
}

#[test]
fn explicit_managed_input_and_release_failures_roll_back_the_whole_installation() {
    let f = Fixture::new();
    let (scope, enrolled) = enroll(&f);
    let original = f.arguments();
    let expected = blob_canic_probe::configuration::input();
    assert_eq!(installation_arguments(&original), Ok(expected.clone()));
    let (payload, _): (CanisterInitPayload, Option<Vec<u8>>) =
        candid::decode_args(&original).unwrap();
    assert_eq!(
        installation_arguments(
            &candid::encode_args((
                payload.clone(),
                Some(candid::encode_args((&expected, 7u8)).unwrap())
            ))
            .unwrap()
        ),
        Err(InitializationArgumentFailure::Encoding),
    );
    let mut trailing = candid::encode_one(&expected).unwrap();
    trailing.push(0);
    assert_eq!(
        installation_arguments(&candid::encode_args((payload, Some(trailing))).unwrap()),
        Err(InitializationArgumentFailure::Encoding),
    );
    let before = f.pic().get_stable_memory(f.app());
    let installed: CompositionSnapshot =
        f.pic().query_candid(f.app(), "probe_snapshot", ()).unwrap();
    for carrier in invalid_carriers(&original) {
        f.pic()
            .wait_out_install_code_rate_limit(Duration::from_secs(5));
        let error = f
            .pic()
            .reinstall_canister(f.app(), wasm(), carrier, Some(f.root()))
            .unwrap_err();
        assert_eq!(
            error.reject_code,
            ic_testkit::pocket_ic::RejectCode::CanisterError
        );
        assert_eq!(f.pic().get_stable_memory(f.app()), before);
        let retained: CompositionSnapshot =
            f.pic().query_candid(f.app(), "probe_snapshot", ()).unwrap();
        // Deferred callback counts are transient; installation and all owners are exact.
        assert_eq!(retained.configuration, installed.configuration);
        assert_eq!(retained.release, installed.release);
        assert_eq!(retained.project, installed.project);
        assert_eq!(retained.verifier, installed.verifier);
        assert_eq!(retained.fences, installed.fences);
        assert_eq!(retained.neighbor, installed.neighbor);
        let tenant: Result<TenantEnrollmentResponse, TenantFailure> = f
            .pic()
            .query_candid_as(f.app(), scope.tenant, "blob_tenant", (scope,))
            .unwrap();
        assert_eq!(tenant.unwrap(), enrolled);
    }
    let outsider: Result<Vec<u8>, HostFailure> = f
        .pic()
        .query_candid_as(
            f.app(),
            candid::Principal::anonymous(),
            "probe_init_arguments",
            (),
        )
        .unwrap();
    assert_eq!(outsider, Err(HostFailure::Denied));
}
