use super::*;
use crate::{
    model::service::read::download::CaffeineDownloadScope,
    ops::service::uploads::{
        completion::verification::reply, manifests::reply::UploadManifestReplyLimits,
    },
    workflow::uploads::completion::verification_plan,
};
#[test]
fn verification_plan_binds_installed_provider_and_only_exposed_unfenced_work() {
    let m = memory();
    let mut store = StableUploads::install(clone_memory(&m), config()).unwrap();
    let enrolled = enroll(&mut store);
    let manifest = manifest_boundary::input();
    let permission = manifest.permission;
    let scope = CaffeineDownloadScope::new(p(1), NonZeroU128::MIN, "installed project/β").unwrap();
    admit(&mut store, context(4), permission, 1).unwrap();
    prepare(&mut store, context(5), &manifest, 2).unwrap();
    assert_eq!(
        verification_plan(&store, authority(), &scope, context(9), permission),
        Err(UploadAttestationFailure::Phase)
    );
    let model = admission::parse_binding(p(1), permission).unwrap();
    store.expose(context(5), model.request, 3).unwrap();
    for caller in [2, 4, 5] {
        assert_eq!(
            verification_plan(&store, authority(), &scope, context(caller), permission),
            Err(UploadAttestationFailure::Denied)
        );
    }
    let wrong = CaffeineDownloadScope::new(p(2), NonZeroU128::MIN, scope.project()).unwrap();
    assert_eq!(
        verification_plan(&store, authority(), &wrong, context(9), permission),
        Err(UploadAttestationFailure::Permission(A::Binding))
    );
    store.revoke(context(4), model.request).unwrap();
    store
        .update_tenant(
            context(2),
            TenantUpdate {
                tenant: p(4),
                expected: Some(enrolled),
                active: false,
            },
        )
        .unwrap();
    let plan = verification_plan(&store, authority(), &scope, context(9), permission).unwrap();
    assert_eq!(plan.permission, permission);
    assert_eq!(plan.declaration, manifest.declaration);
    assert_eq!(plan.owner, p(1));
    assert_eq!(plan.project, scope.project());
    assert_eq!(plan.verifier, p(9));
    assert_eq!(plan.admitted_at_ns, 1);
    let restored = StableUploads::open(clone_memory(&m), config()).unwrap();
    assert_eq!(
        verification_plan(&restored, authority(), &scope, context(9), permission),
        Err(UploadAttestationFailure::Permission(A::Fenced))
    );
    let statement = UploadAttestationRequest {
        permission,
        content_digest: [7; 32],
        observed_at_ns: 4,
    };
    attest(&mut store, authority(), context(9), &statement, 5).unwrap();
    assert_eq!(
        verification_plan(&store, authority(), &scope, context(9), permission),
        Err(UploadAttestationFailure::Phase)
    );
}
#[test]
fn verification_plan_decoder_checks_scope_declaration_and_budgets() {
    use crate::ops::service::uploads::completion::reply::UploadAttestationReplyError as E;
    let manifest = manifest_boundary::input();
    let plan = UploadVerificationPlan {
        permission: manifest.permission,
        verifier: p(9),
        owner: p(1),
        project: "project".into(),
        admitted_at_ns: 1,
        declaration: manifest.declaration,
    };
    let limits = UploadManifestReplyLimits {
        max_reply_bytes: 65536.try_into().unwrap(),
        declaration: crate::model::identity::caffeine::manifest::CaffeineManifestLimits {
            max_content_bytes: 10.try_into().unwrap(),
            max_chunks: 1.try_into().unwrap(),
            max_headers: 8.try_into().unwrap(),
            max_header_bytes: 1024.try_into().unwrap(),
        },
    };
    let encoded = |p: &UploadVerificationPlan| {
        candid::encode_one(Ok::<_, UploadAttestationFailure>(p)).unwrap()
    };
    assert_eq!(
        reply::decode(authority(), plan.permission, &encoded(&plan), limits),
        Ok(plan.clone())
    );
    for change in 0..6 {
        let mut wrong = plan.clone();
        match change {
            0 => wrong.permission.expires_at_ns -= 1,
            1 => wrong.verifier = p(8),
            2 => wrong.owner = p(3),
            3 => wrong.project.clear(),
            4 => wrong.declaration.chunks[0][0] ^= 1,
            _ => wrong.declaration.chunks.push([0; 32]),
        }
        let expected = match change {
            0..=2 => E::Binding,
            5 => E::Limit,
            _ => E::Invalid,
        };
        assert_eq!(
            reply::decode(authority(), plan.permission, &encoded(&wrong), limits),
            Err(expected)
        );
    }
    assert_eq!(
        reply::decode(authority(), plan.permission, &vec![0; 65537], limits),
        Err(E::Limit)
    );
    assert_eq!(
        reply::decode(authority(), plan.permission, b"DIDL", limits),
        Err(E::Invalid)
    );
    let denied = candid::encode_one(Err::<UploadVerificationPlan, _>(
        UploadAttestationFailure::Denied,
    ))
    .unwrap();
    assert_eq!(
        reply::decode(authority(), plan.permission, &denied, limits),
        Err(E::Remote(UploadAttestationFailure::Denied))
    );
}
