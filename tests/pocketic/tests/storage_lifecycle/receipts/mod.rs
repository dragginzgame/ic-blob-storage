use super::*;
mod cli;
mod mutations;
use blob_test_protocol::storage::reference::{ReferenceClientInput, ReferenceProbeFailure};
fn consumer() -> Fixture {
    let mut f = Fixture::new();
    let client = f.harness.pic.create_canister();
    f.harness.pic.install_canister(
        client,
        Fixture::wasm(),
        candid::encode_one(f.operator).unwrap(),
        None,
    );
    f.tenant = client;
    f
}
fn fetch(
    f: &Fixture,
    input: &ReferenceClientInput,
) -> Result<ReferenceReceiptLookup, ReferenceProbeFailure> {
    f.harness
        .pic
        .update_candid_as(
            f.tenant,
            f.operator,
            "fixture_fetch_reference_receipt",
            (input,),
        )
        .unwrap()
}
fn selection(f: &Fixture, request: ReferenceInput) -> ReferenceClientInput {
    ReferenceClientInput {
        request: receipt_request(request),
        tenant: f.tenant,
        max_reply_bytes: 4096,
    }
}
#[test]
fn replicated_receipt_preserves_historical_success_through_release_suspension_and_restore() {
    let f = consumer();
    let input = f.exposed();
    let retain = reference(input.request, 1, 2, true);
    let selected = selection(&f, retain);
    assert_eq!(
        fetch(&f, &selected),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Unconfirmed))
    );
    f.fact(input.request, ProviderFact::Uploaded).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(fetch(&f, &selected), Ok(ReferenceReceiptLookup::Absent));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.reference(retain).unwrap();
    let original = ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
        request: selected.request,
        result: Ok(ReferenceChange::Changed),
    });
    assert_eq!(fetch(&f, &selected), Ok(original));
    f.reference(reference(input.request, 2, 2, false)).unwrap();
    assert_eq!(f.live(retain), Ok(false));
    assert_eq!(fetch(&f, &selected), Ok(original));
    let changed = ReferenceClientInput {
        request: ReferenceCommand {
            action: ReferenceAction::Release,
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        fetch(&f, &changed),
        Err(ReferenceProbeFailure::Remote(ReferenceFailure::Conflict))
    );
    f.enroll(Some(f.tenant().unwrap()), false).unwrap();
    assert_eq!(fetch(&f, &selected), Ok(original));
    f.reference(reference(input.request, 3, 1, false)).unwrap();
    f.fact(input.request, ProviderFact::Deleted).unwrap();
    f.fact(input.request, ProviderFact::Settled).unwrap();
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(fetch(&f, &selected), Ok(original));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness
        .pic
        .upgrade_canister(
            f.service,
            Fixture::wasm(),
            candid::encode_one(f.operator).unwrap(),
            Some(f.controller),
        )
        .unwrap();
    assert_eq!(fetch(&f, &selected), Ok(original));
    assert_eq!(f.reference(retain), Err(ReferenceFailure::Fenced));
}
#[test]
fn replicated_receipt_returns_recorded_failure_and_enforces_actual_client_and_reply_bounds() {
    let f = consumer();
    let input = f.exposed();
    f.fact(input.request, ProviderFact::Uploaded).unwrap();
    let unknown = reference(input.request, 1, 2, false);
    assert_eq!(
        f.reference(unknown).unwrap().receipt.result,
        Err(ReferenceTransitionFailure::UnknownReference)
    );
    let selected = selection(&f, unknown);
    assert_eq!(
        fetch(&f, &selected),
        Ok(ReferenceReceiptLookup::Found(ReferenceReceiptResponse {
            request: selected.request,
            result: Err(ReferenceTransitionFailure::UnknownReference),
        }))
    );
    let before = f.harness.pic.get_stable_memory(f.service);
    assert_eq!(
        fetch(
            &f,
            &ReferenceClientInput {
                tenant: f.operator,
                ..selected
            }
        ),
        Err(ReferenceProbeFailure::Binding)
    );
    assert_eq!(
        fetch(
            &f,
            &ReferenceClientInput {
                max_reply_bytes: 1,
                ..selected
            }
        ),
        Err(ReferenceProbeFailure::Limit)
    );
    let wrong_tenant = ReferenceClientInput {
        request: ReferenceCommand {
            upload: ReferenceUpload {
                tenant: f.other,
                ..selected.request.upload
            },
            ..selected.request
        },
        ..selected
    };
    assert_eq!(
        fetch(&f, &wrong_tenant),
        Err(ReferenceProbeFailure::Binding)
    );
    let denied: Result<ReferenceReceiptLookup, ReferenceProbeFailure> = f
        .harness
        .pic
        .update_candid_as(
            f.tenant,
            f.other,
            "fixture_fetch_reference_receipt",
            (selected,),
        )
        .unwrap();
    assert_eq!(denied, Err(ReferenceProbeFailure::Denied));
    let query: Result<ReferenceReceiptLookup, ReferenceProbeFailure> = f
        .harness
        .pic
        .query_candid_as(
            f.tenant,
            f.operator,
            "fixture_nonreplicated_reference_receipt",
            (selected,),
        )
        .unwrap();
    assert_eq!(query, Err(ReferenceProbeFailure::Execution));
    assert_eq!(f.harness.pic.get_stable_memory(f.service), before);
    f.harness
        .pic
        .stop_canister(f.service, Some(f.controller))
        .unwrap();
    assert!(matches!(
        fetch(&f, &selected),
        Err(ReferenceProbeFailure::Rejected(_))
    ));
}
