//! Exact unresolved outbox histories survive either same-release restoration order.
use super::{Application, capture, wasm};
use blob_test_protocol::consumer::{
    AssetView, Failure, Fault, Recovery, Release, Revocation, Run, Use,
};
use candid::CandidType;
use ic_blob_storage::dto::{
    operator::LocalServiceStatus,
    reference::{
        ReferenceChange, ReferenceFailure, ReferenceMutationResponse, ReferenceReceiptLookup,
        status::ReferenceStatusResponse,
    },
};
use ic_testkit::pic::{CandidCallExt, CanisterInstallExt};
use std::time::Duration;

#[derive(Clone, Copy)]
enum PendingEffect {
    Retain,
    Release,
}
#[derive(Clone, Copy)]
enum RestoreOrder {
    ConsumerFirst,
    ServiceFirst,
}
#[derive(Clone, Debug, Eq, PartialEq, CandidType)]
struct ObservedHistory {
    assets: [AssetView; 2],
    receipts: [ReferenceReceiptLookup; 2],
    references: [ReferenceStatusResponse; 2],
    accounting: LocalServiceStatus,
    consumer_fenced: bool,
}
impl ObservedHistory {
    fn read(app: &Application) -> Self {
        let fenced: Result<bool, Failure> = app
            .f
            .pic()
            .query_candid_as(app.tenant, app.operator, "fenced", ())
            .unwrap();
        Self {
            assets: [app.asset(1), app.asset(2)],
            // These passive service queries explicitly select the tenant identity;
            // the fenced application itself cannot record recovery acknowledgments.
            receipts: [app.receipt(2, false), app.receipt(2, true)],
            references: [app.reference_state(1), app.reference_state(2)],
            accounting: app.status(),
            consumer_fenced: fenced.unwrap(),
        }
    }
    fn with_fences(&self, consumer: bool, service: bool) -> Self {
        let mut expected = self.clone();
        expected.consumer_fenced = consumer;
        expected.accounting.uploads.fenced = service;
        expected.accounting.funding.fenced = service;
        expected.accounting.gateways.fenced = service;
        expected.accounting.reads.fenced = service;
        for reference in &mut expected.references {
            reference.fenced = service;
        }
        expected
    }
}

fn interrupt(app: &Application, effect: PendingEffect) -> ObservedHistory {
    assert!(app.mutate("register", &app.fresh).unwrap().published);
    match effect {
        PendingEffect::Retain => {
            app.trap(
                "register",
                &Run {
                    fault: Fault::AfterRetain,
                    ..app.reuse.clone()
                },
            );
            app.mutate("cancel", &2u128).unwrap();
        }
        PendingEffect::Release => {
            assert!(app.mutate("register", &app.reuse).unwrap().published);
            app.mutate("cancel", &2u128).unwrap();
            app.trap(
                "release",
                &Release {
                    asset: 2,
                    fault: Fault::AfterRelease,
                },
            );
        }
    }
    let history = ObservedHistory::read(app);
    assert_eq!(history.assets[0].registration, app.fresh.registration);
    let reuse = &history.assets[1];
    assert_eq!(reuse.registration, app.reuse.registration);
    assert!(reuse.cancelled && !reuse.published && reuse.retain_started);
    assert_eq!(reuse.release_result, None);
    assert!(
        matches!(history.receipts[0], ReferenceReceiptLookup::Found(r)
        if r.request == app.command(2, false) && r.result == Ok(ReferenceChange::Changed))
    );
    match effect {
        PendingEffect::Retain => {
            assert_eq!(reuse.retain_result, None);
            assert!(!reuse.release_started && !reuse.published_once);
            assert_eq!(history.receipts[1], ReferenceReceiptLookup::Absent);
            assert!(history.references[1].live);
        }
        PendingEffect::Release => {
            assert_eq!(reuse.retain_result, Some(Ok(ReferenceChange::Changed)));
            assert!(reuse.release_started && reuse.published_once);
            assert!(
                matches!(history.receipts[1], ReferenceReceiptLookup::Found(r)
                if r.request == app.command(2, true) && r.result == Ok(ReferenceChange::Changed))
            );
            assert!(!history.references[1].live);
        }
    }
    let uploads = history.accounting.uploads;
    assert_eq!(
        [
            uploads.reserved_bytes,
            uploads.logical_bytes,
            uploads.physical_bytes,
            uploads.liability_bytes
        ],
        [0, 10, 10, 10]
    );
    assert!(history.assets[0].published && history.references[0].live);
    capture(&app.report, "unresolved-history.candid", &history);
    history
}

fn upgrade_consumer(app: &Application) {
    app.f
        .pic()
        .wait_out_install_code_rate_limit(Duration::from_secs(5));
    app.f
        .pic()
        .upgrade_canister(
            app.tenant,
            wasm(),
            candid::encode_args(()).unwrap(),
            Some(app.f.root()),
        )
        .unwrap();
}

fn refused_mutations(app: &Application, label: &str) {
    let commands = [
        (
            "prepare",
            candid::encode_one(&app.reuse.registration).unwrap(),
        ),
        ("admit", candid::encode_one(&app.fresh).unwrap()),
        ("register", candid::encode_one(&app.reuse).unwrap()),
        ("cancel", candid::encode_one(1u128).unwrap()),
        (
            "release",
            candid::encode_one(Release {
                asset: 2,
                fault: Fault::None,
            })
            .unwrap(),
        ),
        (
            "recover",
            candid::encode_one(Recovery {
                asset: 2,
                release: false,
            })
            .unwrap(),
        ),
        (
            "recover",
            candid::encode_one(Recovery {
                asset: 2,
                release: true,
            })
            .unwrap(),
        ),
        (
            "use_asset",
            candid::encode_one(Use {
                asset: 1,
                attach: true,
            })
            .unwrap(),
        ),
        (
            "revoke",
            candid::encode_one(Revocation {
                asset: 1,
                fault: Fault::None,
                max_reply_bytes: 4096,
            })
            .unwrap(),
        ),
        ("recover_revocation", candid::encode_one(1u128).unwrap()),
    ];
    let mut results = Vec::new();
    for (method, argument) in commands {
        let result: Result<AssetView, Failure> = candid::decode_one(
            &app.f
                .pic()
                .update_call(app.tenant, app.operator, method, argument)
                .unwrap(),
        )
        .unwrap();
        assert_eq!(result, Err(Failure::Fenced));
        results.push((method.to_owned(), result));
    }
    capture(&app.report, &format!("{label}-refusals.candid"), &results);
    let resumed: Result<(), Failure> = app
        .f
        .pic()
        .update_candid_as(app.tenant, app.operator, "resume", ())
        .unwrap();
    assert_eq!(resumed, Err(Failure::Fenced));
}

fn denied_controller(app: &Application) {
    let asset: Result<AssetView, Failure> = app
        .f
        .pic()
        .query_candid_as(app.tenant, app.f.root(), "asset", (2u128,))
        .unwrap();
    assert_eq!(asset, Err(Failure::Denied));
    let recovery: Result<AssetView, Failure> = app
        .f
        .pic()
        .update_candid_as(
            app.tenant,
            app.f.root(),
            "recover",
            (Recovery {
                asset: 2,
                release: true,
            },),
        )
        .unwrap();
    assert_eq!(recovery, Err(Failure::Denied));
}

fn check_phase(
    app: &Application,
    original: &ObservedHistory,
    consumer: bool,
    service: bool,
    label: &str,
) {
    let consumer_bytes = app.f.pic().get_stable_memory(app.tenant);
    let service_bytes = app.f.pic().get_stable_memory(app.f.app());
    let expected = original.with_fences(consumer, service);
    assert_eq!(ObservedHistory::read(app), expected);
    if consumer {
        refused_mutations(app, label);
    }
    if service {
        for command in [app.command(2, false), app.command(2, true)] {
            let result: Result<ReferenceMutationResponse, ReferenceFailure> = app
                .f
                .pic()
                .update_candid_as(app.f.app(), app.tenant, "blob_apply_reference", (command,))
                .unwrap();
            assert_eq!(result, Err(ReferenceFailure::Fenced));
        }
    }
    denied_controller(app);
    assert_eq!([app.asset(1), app.asset(2)], original.assets);
    assert_eq!(app.f.pic().get_stable_memory(app.tenant), consumer_bytes);
    assert_eq!(app.f.pic().get_stable_memory(app.f.app()), service_bytes);
    capture(&app.report, &format!("{label}-history.candid"), &expected);
}

fn journey(effect: PendingEffect, order: RestoreOrder, label: &str) {
    let app = Application::new(label);
    capture(&app.report, "restore-order.candid", &label.to_owned());
    let original = interrupt(&app, effect);
    match order {
        RestoreOrder::ConsumerFirst => {
            upgrade_consumer(&app);
            check_phase(&app, &original, true, false, "consumer-only");
            app.f.upgrade_same_release(Duration::from_secs(5));
        }
        RestoreOrder::ServiceFirst => {
            app.f.upgrade_same_release(Duration::from_secs(5));
            check_phase(&app, &original, false, true, "service-only");
            upgrade_consumer(&app);
        }
    }
    check_phase(&app, &original, true, true, "both-fenced");
    upgrade_consumer(&app);
    app.f.upgrade_same_release(Duration::from_secs(5));
    check_phase(&app, &original, true, true, "repeated-restore");
}

#[test]
fn unresolved_retain_survives_application_first_restore() {
    journey(
        PendingEffect::Retain,
        RestoreOrder::ConsumerFirst,
        "retain-consumer-first",
    );
}
#[test]
fn unresolved_retain_survives_service_first_restore() {
    journey(
        PendingEffect::Retain,
        RestoreOrder::ServiceFirst,
        "retain-service-first",
    );
}
#[test]
fn unresolved_release_survives_application_first_restore() {
    journey(
        PendingEffect::Release,
        RestoreOrder::ConsumerFirst,
        "release-consumer-first",
    );
}
#[test]
fn unresolved_release_survives_service_first_restore() {
    journey(
        PendingEffect::Release,
        RestoreOrder::ServiceFirst,
        "release-service-first",
    );
}
