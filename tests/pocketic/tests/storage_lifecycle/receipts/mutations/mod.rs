use super::*;
fn mutate(
    f: &Fixture,
    input: &ReferenceClientInput,
) -> Result<ReferenceMutationResponse, ReferenceProbeFailure> {
    f.harness
        .pic
        .update_candid_as(f.tenant, f.operator, "fixture_mutate_reference", (input,))
        .unwrap()
}
#[test]
fn committed_mutation_with_unusable_reply_recovers_exact_receipt_and_cleanup_at_capacity() {
    let f = consumer();
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let retain = reference(permission.request, u128::MAX, 2, true);
    let selected = selection(&f, retain);
    // The service commits, but the client cannot use its reply. No automatic retry.
    assert_eq!(
        mutate(
            &f,
            &ReferenceClientInput {
                max_reply_bytes: 1,
                ..selected
            }
        ),
        Err(ReferenceProbeFailure::Limit)
    );
    assert_eq!(f.live(retain), Ok(true));
    let expected = changed(retain, false);
    assert_eq!(
        fetch(&f, &selected),
        Ok(ReferenceReceiptLookup::Found(expected.receipt))
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(mutate(&f, &selected), Ok(changed(retain, true)));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    let conflicting = ReferenceClientInput {
        request: ReferenceCommand {
            action: ReferenceAction::Release,
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        mutate(&f, &conflicting),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Conflict))
    );
    let absent = selection(&f, reference(permission.request, 4, 9, false));
    assert_eq!(
        mutate(&f, &absent),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Capacity))
    );
    assert_eq!(fetch(&f, &absent), Ok(ReferenceReceiptLookup::Absent));
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    assert_eq!(
        mutate(
            &f,
            &selection(&f, reference(permission.request, 4, 3, true))
        ),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Inactive))
    );
    for (operation, id) in [(2, 1), (3, 2)] {
        let release = reference(permission.request, operation, id, false);
        assert_eq!(
            mutate(&f, &selection(&f, release)),
            Ok(changed(release, false))
        );
    }
    assert_eq!(
        f.status().usage,
        JourneyUsage {
            logical: 0,
            physical: 10,
            liability: 10
        }
    );
    assert_eq!(mutate(&f, &selected), Ok(changed(retain, true)));
    assert_eq!(f.live(retain), Ok(false));
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            Fixture::installation(f.operator),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(
        mutate(&f, &selected),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Fenced))
    );
    assert_eq!(
        fetch(&f, &selected),
        Ok(ReferenceReceiptLookup::Found(expected.receipt))
    );
}
#[test]
fn replicated_mutation_preserves_recorded_failure_and_does_not_treat_it_as_release_success() {
    let f = consumer();
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let unknown = reference(permission.request, 1, 9, false);
    let selected = selection(&f, unknown);
    let result = mutate(&f, &selected).unwrap();
    assert!(!result.replayed);
    assert_eq!(
        result.receipt.result,
        Err(ReferenceTransitionFailure::UnknownReference)
    );
    assert_eq!(result.receipt.request, selected.request);
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        mutate(&f, &selected),
        Ok(ReferenceMutationResponse {
            replayed: true,
            ..result
        })
    );
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    let release = reference(permission.request, 2, 1, false);
    assert_eq!(
        mutate(&f, &selection(&f, release)),
        Ok(changed(release, false))
    );
    assert_eq!(
        fetch(&f, &selected),
        Ok(ReferenceReceiptLookup::Found(result.receipt))
    );
    assert_eq!(f.live(release), Ok(false));
}
#[test]
fn mutation_rejects_foreign_callers_invalid_intent_and_nonreplicated_execution_without_writes() {
    let f = consumer();
    let permission = f.exposed();
    f.fact(permission.request, ProviderFact::Uploaded).unwrap();
    let retain = reference(permission.request, 1, 2, true);
    let selected = selection(&f, retain);
    let before = f.harness.pic.get_stable_memory(f.service);
    for actor in [f.controller, f.operator, f.uploader, f.other] {
        let denied: Result<ReferenceMutationResponse, ReferenceFailure> = f
            .harness
            .pic
            .update_candid_as(
                f.service,
                actor,
                REFERENCE_APPLY_METHOD,
                (selected.request,),
            )
            .unwrap();
        assert_eq!(denied, Err(ReferenceFailure::Denied));
    }
    assert_eq!(
        mutate(
            &f,
            &ReferenceClientInput {
                tenant: f.other,
                ..selected
            }
        ),
        Err(ReferenceProbeFailure::Binding)
    );
    let invalid = ReferenceClientInput {
        request: ReferenceCommand {
            operation: 0,
            ..selected.request
        },
        ..selected
    };
    assert_eq!(mutate(&f, &invalid), Err(ReferenceProbeFailure::Invalid));
    let wrong_namespace = ReferenceClientInput {
        request: ReferenceCommand {
            upload: ReferenceUpload {
                namespace: 2,
                ..selected.request.upload
            },
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        mutate(&f, &wrong_namespace),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Binding))
    );
    let query: Result<ReferenceMutationResponse, ReferenceProbeFailure> = f
        .harness
        .pic
        .query_candid_as(
            f.tenant,
            f.operator,
            "fixture_nonreplicated_reference_mutation",
            (selected,),
        )
        .unwrap();
    assert_eq!(query, Err(ReferenceProbeFailure::Execution));
    assert_eq!(fetch(&f, &selected), Ok(ReferenceReceiptLookup::Absent));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
}
