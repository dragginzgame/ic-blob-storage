use super::*;
mod certificate;
use blob_test_protocol::storage::exposure::{
    ExposureBlocker as B, ExposureInput, ExposureOutcome, ExposureScenario,
};
use ic_blob_storage::dto::upload::{
    UploadState,
    admission::{UploadAdmissionFailure as A, UploadAdmissionResponse},
    exposure::UploadExposureFailure as E,
};
fn input(f: &Fixture, permission: Permission) -> ExposureInput {
    assert_eq!(permission.uploader, f.uploader);
    ExposureInput {
        permission: admission_input(permission),
        scenario: ExposureScenario::QualifiedSubstitute,
        trap_write: false,
        trap_after: false,
    }
}
fn expose(f: &Fixture, actor: Principal, input: ExposureInput) -> Result<ExposureOutcome, E> {
    f.harness
        .pic
        .update_candid_as(f.service, actor, "expose", (input,))
        .unwrap()
}
fn preview(f: &Fixture, input: ExposureInput) -> Result<Vec<B>, E> {
    f.harness
        .pic
        .query_candid_as(f.service, f.uploader, "exposure_preview", (input,))
        .unwrap()
}
fn inspect(
    f: &Fixture,
    actor: Principal,
    input: ExposureInput,
) -> Result<UploadAdmissionResponse, E> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "exposure_status", (input.permission,))
        .unwrap()
}
#[test]
fn exposure_missing_host_facts_and_stale_previews_cannot_mutate_or_bypass_authority() {
    let f = Fixture::new();
    let enrolled = f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(u128::MAX, 8);
    f.admit(f.tenant, permission).unwrap();
    let yes = input(&f, permission);
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Unprepared));
    f.prepare(&declaration).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    let missing = ExposureInput {
        scenario: ExposureScenario::Unknown,
        ..yes
    };
    let expected = vec![
        B::PrechargeLimits,
        B::ProviderNamespace,
        B::ReplayCharging,
        B::Recovery,
        B::Durability,
    ];
    assert_eq!(preview(&f, missing), Ok(expected.clone()));
    assert_eq!(
        expose(&f, f.uploader, missing),
        Ok(ExposureOutcome::Blocked(expected))
    );
    assert_eq!(
        expose(
            &f,
            f.uploader,
            ExposureInput {
                scenario: ExposureScenario::StaleSubstitute,
                ..yes
            }
        ),
        Ok(ExposureOutcome::Blocked(vec![B::StaleObservation]))
    );
    assert_eq!(
        expose(
            &f,
            f.uploader,
            ExposureInput {
                scenario: ExposureScenario::ForeignSubstitute,
                ..yes
            }
        ),
        Err(E::EvidenceBinding)
    );
    for actor in [f.tenant, f.controller, f.operator, f.other] {
        assert_eq!(expose(&f, actor, yes), Err(E::Permission(A::Denied)));
    }
    let mut changed = yes;
    changed.permission.expires_at_ns -= 1;
    assert_eq!(
        expose(&f, f.uploader, changed),
        Err(E::Permission(A::Conflict))
    );
    assert_eq!(
        inspect(&f, f.tenant, changed),
        Err(E::Permission(A::Conflict))
    );
    assert_eq!(preview(&f, yes), Ok(vec![]));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.enroll(Some(enrolled), false).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Permission(A::Inactive)));
    assert_eq!(
        inspect(&f, f.uploader, yes).unwrap().state,
        UploadState::Reserved
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
#[test]
fn exposure_write_and_response_traps_roll_back_but_lost_committed_acknowledgment_never_allows_reissue()
 {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&declaration).unwrap();
    let yes = input(&f, permission);
    let before = f.harness.pic.get_stable_memory(f.service);
    let totals = f.status();
    for fault in [
        ExposureInput {
            trap_write: true,
            ..yes
        },
        ExposureInput {
            trap_after: true,
            ..yes
        },
    ] {
        let rejection = f
            .harness
            .pic
            .update_call(
                f.service,
                f.uploader,
                "expose",
                candid::encode_one(fault).unwrap(),
            )
            .unwrap_err();
        assert_eq!(rejection.reject_code, RejectCode::CanisterError);
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        assert_eq!(f.status(), totals);
        assert_eq!(
            inspect(&f, f.uploader, yes).unwrap().state,
            UploadState::Reserved
        );
    }
    // Discard the application's acknowledgment after the IC update committed.
    f.harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            "expose",
            candid::encode_one(yes).unwrap(),
        )
        .unwrap();
    let original = inspect(&f, f.uploader, yes).unwrap();
    assert_eq!(original.state, UploadState::ExposurePossible);
    assert_eq!(original.permission, yes.permission);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Phase));
    for actor in [f.operator, f.controller, f.other] {
        assert_eq!(inspect(&f, actor, yes), Err(E::Permission(A::Denied)));
    }
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.revoke(permission).unwrap();
    assert_eq!(f.status(), totals);
    let withdrawn = inspect(&f, f.tenant, yes).unwrap();
    assert!(withdrawn.revoked);
    assert_eq!(withdrawn.state, UploadState::ExposurePossible);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, yes), Ok(withdrawn));
    assert_eq!(expose(&f, f.uploader, yes), Err(E::Permission(A::Fenced)));
}
