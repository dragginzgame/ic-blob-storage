use super::*;
fn cancel(f: &Fixture, id: u128) -> Result<ManifestIntentView, C> {
    f.harness
        .pic
        .update_candid_as(f.uploader, f.operator, "cancel_manifest", (id,))
        .unwrap()
}
#[test]
fn uploader_cancellation_keeps_unsent_and_uncertain_history_through_recovery_and_upgrade() {
    for reconcile in [false, true] {
        let f = fixture();
        let (first, input) = intent(&f, 1, 2);
        f.admit(f.tenant, first).unwrap();
        save(&f, &input).unwrap();
        let before = f.harness.pic.get_stable_memory(f.service);
        let denied: Result<ManifestIntentView, C> = f
            .harness
            .pic
            .update_candid_as(f.uploader, f.other, "cancel_manifest", (1u128,))
            .unwrap();
        assert_eq!(denied, Err(C::Denied));
        assert!(!view(&f, 1).unwrap().cancelled);
        let unsent = cancel(&f, 1).unwrap();
        assert!(unsent.cancelled && !unsent.started);
        assert_eq!(unsent.result, None);
        assert_eq!(cancel(&f, 1), Ok(unsent.clone()));
        assert_eq!(save(&f, &input), Ok(unsent.clone()));
        assert_eq!(dispatch(&f, run(1)), Err(C::State));
        assert_eq!(recover(&f, 1), Err(C::State));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        let (second, pending_input) = intent(&f, 2, 3);
        f.admit(f.tenant, second).unwrap();
        save(&f, &pending_input).unwrap();
        assert_eq!(
            dispatch(
                &f,
                ManifestDispatch {
                    max_reply_bytes: 1,
                    ..run(2)
                }
            ),
            Err(C::Transport)
        );
        let pending = cancel(&f, 2).unwrap();
        assert!(pending.cancelled && pending.started);
        assert_eq!(pending.result, None);
        let before = f.harness.pic.get_stable_memory(f.service);
        let retained = if reconcile {
            let recovered = recover(&f, 2).unwrap();
            assert!(recovered.cancelled);
            assert_eq!(
                recovered.result,
                Some(Ok(pending_input.request.declaration.clone()))
            );
            recovered
        } else {
            pending
        };
        assert_eq!(dispatch(&f, run(2)), Err(C::State));
        let (_, third) = intent(&f, 3, 4);
        assert_eq!(save(&f, &third), Err(C::Capacity));
        assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
        f.harness
            .pic
            .upgrade_canister(
                f.uploader,
                wasm(),
                candid::encode_args(()).unwrap(),
                Some(f.controller),
            )
            .unwrap();
        assert_eq!(view(&f, 1), Ok(unsent));
        assert_eq!(view(&f, 2), Ok(retained));
        assert_eq!(cancel(&f, 2), Err(C::Fenced));
        assert_eq!(dispatch(&f, run(1)), Err(C::Fenced));
        assert_eq!(recover(&f, 2), Err(C::Fenced));
    }
}
