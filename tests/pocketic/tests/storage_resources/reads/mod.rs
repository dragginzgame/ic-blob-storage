//! The host owns measurement windows and authority; shared arithmetic owns the aggregate.
use super::*;

fn observe(f: &Fixture, actor: Principal) -> Result<RestorationResources, Failure> {
    f.harness
        .pic
        .query_candid_as(f.service, actor, "restoration_resources", ())
        .unwrap()
}

#[test]
fn restoration_reads_are_attributed_only_to_the_operator_reopen_window() {
    let f = Fixture::new();
    let fresh = observe(&f, f.operator).unwrap();
    assert!(!fresh.restored);
    assert!(fresh.reads.is_empty());

    f.enroll(None, true).unwrap();
    let (permission, input) = f.permission(1, 7);
    f.admit(f.tenant, permission).unwrap();
    f.prepare(&input).unwrap();
    f.expose(permission.request).unwrap();
    let before = f.status();
    assert_eq!(
        candid::encode_one(observe(&f, f.operator).unwrap()).unwrap(),
        candid::encode_one(&fresh).unwrap()
    );

    // Installation instruction debt must cool before testing restore semantics.
    install_code::settle_install_code_debt(&f.harness.pic);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let restored = observe(&f, f.operator).unwrap();
    assert!(restored.restored);
    assert!(
        restored
            .reads
            .iter()
            .any(|r| r.calls > 0 && r.bytes > 0 && r.instructions > 0)
    );
    for row in &restored.reads {
        assert_ne!(row.memory, "");
        assert!(row.instructions <= restored.stores_instructions);
        if row.calls == 0 {
            assert_eq!((row.bytes, row.instructions), (0, 0));
        }
    }
    assert_eq!(
        f.status(),
        Status {
            fenced: true,
            ..before
        }
    );
    let frozen = candid::encode_one(&restored).unwrap();
    for actor in [f.controller, f.tenant, Principal::anonymous()] {
        assert!(matches!(observe(&f, actor), Err(Failure::Denied)));
    }
    f.lookup(f.tenant, permission.request).unwrap();
    assert_eq!(
        candid::encode_one(observe(&f, f.operator).unwrap()).unwrap(),
        frozen
    );

    install_code::settle_install_code_debt(&f.harness.pic);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    let again = observe(&f, f.operator).unwrap();
    let attribution = |r: RestorationResources| {
        r.reads
            .into_iter()
            .map(|r| (r.memory, r.calls, r.bytes))
            .collect::<Vec<_>>()
    };
    assert_eq!(attribution(again), attribution(restored));
}
