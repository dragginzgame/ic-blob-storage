//! Local ingress update payload/rollback evidence, not gateway certificate acceptance.
use super::*;
mod ingress;
use ic_blob_storage::{
    model::identity::ProviderRootHash,
    workflow::uploads::certificate::CAFFEINE_UPLOAD_CERTIFICATE_METHOD as METHOD,
};
#[derive(candid::CandidType, candid::Deserialize)]
struct ProviderReply {
    method: String,
    blob_hash: String,
}

fn root(permission: Permission) -> String {
    ProviderRootHash::try_from(permission.request.root.as_slice())
        .unwrap()
        .to_string()
}
fn configure(f: &Fixture, input: ExposureInput) {
    f.harness
        .pic
        .update_candid_as::<(), _>(
            f.service,
            f.operator,
            "configure_certificate_fixture",
            (input,),
        )
        .unwrap();
}
fn refused(f: &Fixture, actor: Principal, root: &str) {
    let before = f.harness.pic.get_stable_memory(f.service);
    let rejection = f
        .harness
        .pic
        .update_call(f.service, actor, METHOD, candid::encode_one(root).unwrap())
        .unwrap_err();
    assert_eq!(rejection.reject_code, RejectCode::CanisterError);
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}

#[test]
fn certificate_ingress_requires_local_permission_and_independent_host_evidence() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(u128::MAX, 8);
    f.admit(f.tenant, permission).unwrap();
    let yes = input(&f, permission);
    let root = root(permission);
    refused(&f, f.uploader, &root); // No configured fixture evidence by default.
    let rejection = f
        .harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            "configure_certificate_fixture",
            candid::encode_one(yes).unwrap(),
        )
        .unwrap_err();
    assert_eq!(rejection.reject_code, RejectCode::CanisterError);
    configure(&f, yes);
    refused(&f, f.uploader, &root); // Still unprepared.
    f.prepare(&declaration).unwrap();
    for scenario in [
        ExposureScenario::Unknown,
        ExposureScenario::StaleSubstitute,
        ExposureScenario::ForeignSubstitute,
    ] {
        configure(&f, ExposureInput { scenario, ..yes });
        refused(&f, f.uploader, &root);
    }
    configure(&f, yes);
    for actor in [
        f.tenant,
        f.operator,
        f.controller,
        f.other,
        Principal::anonymous(),
    ] {
        refused(&f, actor, &root);
    }
    for root in [
        String::new(),
        "sha256:00".into(),
        format!("sha256:{}", "ff".repeat(32)),
        "x".repeat(4096),
    ] {
        refused(&f, f.uploader, &root);
    }
    // The provider contract is an update, never a query that could roll back intent.
    assert!(
        f.harness
            .pic
            .query_call(
                f.service,
                f.uploader,
                METHOD,
                candid::encode_one(&root).unwrap()
            )
            .is_err()
    );
    let (other, other_declaration) = f.permission(7, 9);
    f.admit(f.tenant, other).unwrap();
    f.prepare(&other_declaration).unwrap();
    refused(&f, f.uploader, &self::root(other)); // Configured evidence belongs to the first root.

    // Independent wire shape mirrors official Mixin.mo; no Result wrapper is accepted.
    let reply: ProviderReply = f
        .harness
        .pic
        .update_candid_as(f.service, f.uploader, METHOD, (&root,))
        .unwrap();
    assert_eq!(reply.method, "upload");
    assert_eq!(reply.blob_hash, root);
    assert_eq!(
        inspect(&f, f.uploader, yes).unwrap().state,
        UploadState::ExposurePossible
    );
    assert_eq!(
        inspect(&f, f.uploader, input(&f, other)).unwrap().state,
        UploadState::Reserved
    );
    refused(&f, f.uploader, &root);
}

#[test]
fn certificate_response_traps_rollback_and_lost_committed_reply_cannot_reissue() {
    let f = Fixture::new();
    f.enroll(None, true).unwrap();
    let (permission, declaration) = f.permission(1, 1);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&declaration).unwrap();
    let yes = input(&f, permission);
    let root = root(permission);
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
        configure(&f, fault);
        refused(&f, f.uploader, &root);
        assert_eq!(
            inspect(&f, f.uploader, yes).unwrap().state,
            UploadState::Reserved
        );
        assert_eq!(f.status(), totals);
    }
    configure(&f, yes);
    // Ignore a successful committed response, as if the uploader lost its reply.
    f.harness
        .pic
        .update_call(
            f.service,
            f.uploader,
            METHOD,
            candid::encode_one(&root).unwrap(),
        )
        .unwrap();
    assert_eq!(
        inspect(&f, f.uploader, yes).unwrap().state,
        UploadState::ExposurePossible
    );
    refused(&f, f.uploader, &root);
    f.revoke(permission).unwrap();
    assert_eq!(f.status(), totals);
    let retained = inspect(&f, f.tenant, yes).unwrap();
    assert!(retained.revoked);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(inspect(&f, f.uploader, yes), Ok(retained));
    refused(&f, f.uploader, &root); // Fixture configuration is absent after upgrade.
    configure(&f, yes);
    refused(&f, f.uploader, &root); // Even replaced substitute facts cannot unfreeze the owner.
    assert_eq!(inspect(&f, f.uploader, yes), Ok(retained));
}
