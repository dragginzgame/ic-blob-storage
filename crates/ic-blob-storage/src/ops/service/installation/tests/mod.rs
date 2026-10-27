use super::*;
use crate::model::billing::journal::FundingIntent;
use crate::ops::service::configuration::tests::candidate;
use crate::ops::service::stores::grants;
use crate::test_support::memory::FixtureMemory;
use crate::workflow::operator::inspect;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::tenant::TenantUpdate;
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_storage::adapter::Storable;

const RELEASE: &str = "test-release";

#[test]
fn complete_requests_preserve_authority_and_match_configuration_and_store_keys() {
    let requests = super::requests("installation-test").unwrap();
    let keys: std::collections::BTreeSet<_> =
        requests.iter().map(|r| r.stable_key().as_str()).collect();
    let mut expected: std::collections::BTreeSet<_> = grants::requests("installation-test")
        .unwrap()
        .iter()
        .map(|r| r.stable_key().as_str().to_owned())
        .collect();
    expected.insert(INSTALLATION_MEMORY_KEY.to_owned());
    assert_eq!(
        keys.into_iter()
            .map(str::to_owned)
            .collect::<std::collections::BTreeSet<_>>(),
        expected
    );
    assert_eq!(requests.len(), expected.len());
    assert!(
        requests
            .iter()
            .all(|r| r.authority() == "installation-test")
    );
    assert!(super::requests("").is_err());
}

#[test]
fn compiled_library_version_binds_installation_and_reopen() {
    let memory = memory();
    let validated = ValidatedServiceInstallation::new(
        candidate().service,
        ServiceInstallationCandidate {
            release: crate::LIBRARY_VERSION,
            ..input()
        },
    )
    .unwrap();
    let installed = ServiceInstallation::install(memories(&memory), validated).unwrap();
    let context = UploadContext {
        service: candidate().service,
        actor: candidate().operator,
    };
    let view = crate::workflow::installation::inspect(&installed, context).unwrap();
    assert_eq!(view.release, crate::LIBRARY_VERSION);
    assert_eq!(crate::LIBRARY_VERSION, env!("CARGO_PKG_VERSION"));
    drop(installed);
    let reopened = ServiceInstallation::open(
        memories(&memory),
        candidate().service,
        crate::LIBRARY_VERSION,
    )
    .unwrap();
    assert_eq!(
        crate::workflow::installation::inspect(&reopened, context)
            .unwrap()
            .release,
        crate::LIBRARY_VERSION
    );
}
#[test]
fn installation_record_uses_current_wire_bytes_and_rejects_oversized_encoding() {
    let memory = memory();
    let installed = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    let expected = installed.configuration.encode_record().unwrap();
    assert_eq!(installed.configuration.to_bytes().as_ref(), expected);
    assert_eq!(installed.configuration.clone().into_bytes(), expected);
    let mut oversized = installed.configuration;
    oversized.project = "x".repeat(16_384);
    assert!(std::panic::catch_unwind(|| oversized.into_bytes()).is_err());
}

fn input() -> ServiceInstallationCandidate<'static> {
    ServiceInstallationCandidate {
        platform_installation_version: 0,
        configuration: candidate(),
        project: "project/β?&=",
        release: RELEASE,
    }
}
fn validated() -> ValidatedServiceInstallation {
    ValidatedServiceInstallation::new(candidate().service, input()).unwrap()
}
fn memory() -> [FixtureMemory; 15] {
    std::array::from_fn(|_| FixtureMemory::default())
}
fn memories(
    memory: &[FixtureMemory; 15],
) -> ServiceInstallationMemories<ic_memory::RuntimeMemory<ic_memory::VectorMemory>> {
    let requests = grants::requests("installation-test").unwrap();
    ServiceInstallationMemories {
        configuration: memory[0].lease(),
        stores: grants::open::<_, std::convert::Infallible>(|key| {
            let index = requests
                .iter()
                .position(|r| r.stable_key().as_str() == key)
                .unwrap();
            Ok(memory[index + 1].lease())
        })
        .unwrap(),
    }
}
fn bytes(memory: &[FixtureMemory; 15]) -> [Vec<u8>; 15] {
    memory.each_ref().map(|m| m.borrow().clone())
}

#[test]
fn observed_version_gaps_fence_without_changing_journals_or_the_installation_anchor() {
    for (previous, current, gap) in [
        (7, 7, false),
        (7, 8, false),
        (7, 9, true),
        (9, 7, true),
        (u64::MAX, u64::MAX, false),
        (u64::MAX, 0, true),
        (0, u64::MAX, true),
    ] {
        let memory = memory();
        let validated = ValidatedServiceInstallation::new(
            candidate().service,
            ServiceInstallationCandidate {
                platform_installation_version: 7,
                ..input()
            },
        )
        .unwrap();
        let mut installation = ServiceInstallation::install(memories(&memory), validated).unwrap();
        let before = bytes(&memory);
        installation.last_platform_version = Some(previous);
        let request = installation.prepare_active_continuity(current);
        assert_eq!(request.map(|r| r.installation_version), gap.then_some(7));
        assert_eq!(installation.last_platform_version, Some(current));
        assert_eq!(installation.stores().uploads.is_fenced(), gap);
        assert_eq!(installation.stores().funding.is_fenced(), gap);
        assert_eq!(installation.platform_installation_version(), 7);
        assert_eq!(bytes(&memory), before);
        installation.observe_platform_version(current);
        assert_eq!(installation.stores().uploads.is_fenced(), gap);
        assert_eq!(bytes(&memory), before);
    }
}

#[test]
fn already_fenced_restoration_never_requests_automatic_continuity_recovery() {
    let memory = memory();
    let installed = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    drop(installed);
    let mut restored =
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE).unwrap();
    let before = bytes(&memory);
    restored.last_platform_version = Some(7);
    for current in [9, 10, 0, u64::MAX] {
        assert!(restored.prepare_active_continuity(current).is_none());
        assert_eq!(restored.last_platform_version, Some(current));
        assert!(restored.stores().uploads.is_fenced());
        assert!(restored.stores().funding.is_fenced());
        assert_eq!(bytes(&memory), before);
    }
}

#[test]
fn configuration_inspection_binds_operator_service_and_preserves_restore_fences() {
    use crate::workflow::installation::inspect;
    use ic_blob_storage_contracts::dto::configuration::HostFailure;
    let memory = memory();
    let installed = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    let context = UploadContext {
        service: candidate().service,
        actor: candidate().operator,
    };
    let view = inspect(&installed, context).unwrap();
    assert_eq!(view.configuration, candidate());
    assert_eq!(view.project, input().project);
    assert_eq!(view.release, RELEASE);
    assert!(!view.fenced);
    let before = bytes(&memory);
    for actor in [
        candidate().payment_account,
        Principal::from_slice(&[88, 1]),
        Principal::anonymous(),
    ] {
        assert_eq!(
            inspect(&installed, UploadContext { actor, ..context }),
            Err(HostFailure::Denied)
        );
    }
    assert_eq!(
        inspect(
            &installed,
            UploadContext {
                service: Principal::from_slice(&[77, 1]),
                ..context
            }
        ),
        Err(HostFailure::Denied)
    );
    drop(installed);
    let restored =
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE).unwrap();
    assert_eq!(
        inspect(&restored, context),
        Ok(
            ic_blob_storage_contracts::dto::configuration::HostConfigurationView {
                fenced: true,
                ..view
            }
        )
    );
    assert_eq!(bytes(&memory), before);
}

#[test]
fn whole_candidate_rejects_invalid_bindings_project_and_release() {
    let original = input();
    assert!(matches!(
        ValidatedServiceInstallation::new(Principal::from_slice(&[88, 1]), original),
        Err(ServiceInstallationError::Configuration(
            ServiceConfigurationAdapterError::Input(
                ic_blob_storage_contracts::configuration::ConfigurationInputError::ServiceBinding
            )
        ))
    ));
    for project in ["", " padded", "control\n"] {
        assert!(matches!(
            ValidatedServiceInstallation::new(
                candidate().service,
                ServiceInstallationCandidate {
                    project,
                    ..original
                }
            ),
            Err(ServiceInstallationError::Project(
                DownloadScopeError::Project
            ))
        ));
    }
    for release in ["", " padded", "control\n", &"r".repeat(129)] {
        assert!(matches!(
            ValidatedServiceInstallation::new(
                candidate().service,
                ServiceInstallationCandidate {
                    release,
                    ..original
                }
            ),
            Err(ServiceInstallationError::ReleaseIdentity)
        ));
    }
}

#[test]
fn allocated_grant_anywhere_rejects_before_installation_or_owner_writes() {
    for occupied in 0..15 {
        let memory = memory();
        assert_eq!(ic_memory::Memory::grow(&memory[occupied].lease(), 1), 0);
        memory[occupied]
            .lease()
            .write(0, b"retained installation obligation");
        let before = bytes(&memory);
        let result = ServiceInstallation::install(memories(&memory), validated());
        if occupied == 0 {
            assert!(matches!(
                result,
                Err(ServiceInstallationError::AlreadyAllocated)
            ));
        } else {
            assert!(matches!(
                result,
                Err(ServiceInstallationError::Stores(
                    ServiceStoreError::AlreadyAllocated
                ))
            ));
        }
        assert_eq!(bytes(&memory), before);
    }
}

#[test]
fn missing_and_foreign_configuration_refuse_without_initialization() {
    let memory = memory();
    assert!(matches!(
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
        Err(ServiceInstallationError::Missing)
    ));
    assert!(memory.iter().all(|m| m.lease().size() == 0));
    let configuration = memory[0].lease();
    configuration.grow(1).unwrap();
    configuration.write(0, b"retained foreign obligation");
    drop(configuration);
    let before = bytes(&memory);
    assert!(matches!(
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
        Err(ServiceInstallationError::InvalidRecord)
    ));
    assert_eq!(bytes(&memory), before);
}

#[test]
fn corrupt_configuration_cell_refuses_without_repairing_other_owners() {
    for invalid_codec in [false, true] {
        let memory = memory();
        drop(ServiceInstallation::install(memories(&memory), validated()).unwrap());
        if invalid_codec {
            drop(StableCell::new(
                memory[0].lease(),
                b"not a configuration record".to_vec(),
            ));
        } else {
            let cell = memory[0].lease();
            // The collection's external layout byte is deliberately unsupported.
            cell.write(3, &[u8::MAX]);
        }
        let before = bytes(&memory);
        if invalid_codec {
            assert!(
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    ServiceInstallation::open(memories(&memory), candidate().service, RELEASE)
                }))
                .is_err()
            );
        } else {
            assert!(matches!(
                ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
                Err(ServiceInstallationError::InvalidRecord)
            ));
        }
        assert_eq!(bytes(&memory), before);
    }
}

#[test]
fn wrong_host_release_format_project_and_missing_owner_never_repair() {
    let memory = memory();
    drop(ServiceInstallation::install(memories(&memory), validated()).unwrap());
    let before = bytes(&memory);
    for (service, release, expected) in [
        (
            Principal::from_slice(&[88, 1]),
            RELEASE,
            InstallationBindingError::Service,
        ),
        (
            candidate().service,
            "different-release",
            InstallationBindingError::Release,
        ),
    ] {
        assert!(matches!(
            ServiceInstallation::open(memories(&memory), service, release),
            Err(ServiceInstallationError::Binding(error)) if error == expected
        ));
        assert_eq!(bytes(&memory), before);
    }
    let mut missing = memory.clone();
    missing[14] = FixtureMemory::default();
    let missing_before = bytes(&missing);
    assert!(matches!(
        ServiceInstallation::open(memories(&missing), candidate().service, RELEASE),
        Err(ServiceInstallationError::Stores(ServiceStoreError::Missing))
    ));
    assert_eq!(bytes(&missing), missing_before);
    let mut records: StableCell<ConfigurationRecord, _> =
        StableCell::load(memory[0].lease()).unwrap();
    let mut record = records.get().clone();
    // Both prior allocation and verifier-bearing installation formats must refuse
    // even when the host release string is unchanged. Never mutate old bytes.
    for retired in [
        "ic-blob-storage/installation:platform-anchor-funding-credit-index-renewal",
        "ic-blob-storage/installation:project-upload-grants-platform-anchor-funding-credit-index-renewal",
    ] {
        record.format = retired.to_owned();
        records.set(record.clone());
        drop(records);
        let before = bytes(&memory);
        assert!(matches!(
            ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
            Err(ServiceInstallationError::Binding(
                InstallationBindingError::Format
            ))
        ));
        assert_eq!(bytes(&memory), before);
        records = StableCell::load(memory[0].lease()).unwrap();
    }
    record.format = ConfigurationRecord::FORMAT.to_owned();
    record.project = " invalid-project".to_owned();
    records.set(record);
    drop(records);
    let before = bytes(&memory);
    assert!(matches!(
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
        Err(ServiceInstallationError::Project(
            DownloadScopeError::Project
        ))
    ));
    assert_eq!(bytes(&memory), before);
}

#[test]
fn populated_installation_preserves_full_configuration_scope_and_owner_fences() {
    let memory = memory();
    let original = input();
    let mut installation = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    let context = UploadContext {
        service: candidate().service,
        actor: candidate().operator,
    };
    let scope = OperatorScope {
        service: candidate().service,
        namespace: candidate().namespace,
        cashier: candidate().billing.cashier,
        payment_account: candidate().payment_account,
    };
    let enrollment = installation
        .stores_mut()
        .uploads
        .update_tenant(
            context,
            TenantUpdate {
                tenant: candidate().payment_account,
                expected: None,
                active: true,
            },
        )
        .unwrap();
    installation
        .stores_mut()
        .funding
        .prepare(
            context,
            FundingIntent {
                service: candidate().service,
                namespace: candidate().namespace.try_into().unwrap(),
                cashier: candidate().billing.cashier,
                account: candidate().payment_account,
                operation: u128::MAX.try_into().unwrap(),
                offered: 10.try_into().unwrap(),
                target_balance: Some(u128::MAX.try_into().unwrap()),
            },
        )
        .unwrap();
    let status = inspect(installation.stores().into(), context, scope).unwrap();
    drop(installation);
    let before = bytes(&memory);
    let restored =
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE).unwrap();
    assert_eq!(restored.configuration(), original.configuration);
    assert_eq!(restored.download_scope().project(), original.project);
    assert_eq!(restored.download_scope().owner(), candidate().service);
    assert_eq!(
        restored.download_scope().namespace().get(),
        candidate().namespace
    );
    assert_eq!(restored.release(), RELEASE);
    assert_eq!(
        restored
            .stores()
            .uploads
            .tenant(context, candidate().payment_account),
        Ok(Some(enrollment))
    );
    let mut expected = status;
    expected.uploads.fenced = true;
    expected.funding.fenced = true;
    expected.gateways.fenced = true;

    assert_eq!(
        inspect(restored.stores().into(), context, scope),
        Ok(expected)
    );
    assert_eq!(bytes(&memory), before);
}

#[test]
fn shared_funding_access_releases_borrows_and_observes_version_gaps() {
    use crate::ops::service::funding::access::FundingJournalAccess;
    use crate::workflow::service::{ServiceExecutionLimits, ServiceHost, ServiceWorkflow};
    use std::cell::{Cell, RefCell};
    thread_local! {
        static OWNER: RefCell<Option<ServiceInstallation<ic_memory::VectorMemory>>> = const { RefCell::new(None) };
        static VERSION: Cell<u64> = const { Cell::new(0) };
    }
    struct Host;
    impl ServiceHost for Host {
        type Memory = ic_memory::VectorMemory;
        fn with_installation<R>(
            operation: impl FnOnce(&mut ServiceInstallation<Self::Memory>) -> R,
        ) -> R {
            OWNER.with_borrow_mut(|owner| operation(owner.as_mut().unwrap()))
        }
        fn platform_version() -> u64 {
            VERSION.get()
        }
        fn time() -> u64 {
            0
        }
        fn limits() -> ServiceExecutionLimits {
            panic!("journal access needs no dispatch limits")
        }
    }
    let memory = memory();
    OWNER.with_borrow_mut(|owner| {
        *owner = Some(ServiceInstallation::install(memories(&memory), validated()).unwrap());
    });
    let before = bytes(&memory);
    let access = ServiceWorkflow::<Host>::ACCESS;
    assert!(!access.with_funding_journal(|journal| journal.is_fenced()));
    assert_preparation_borrow_boundaries(&access);
    assert_eq!(bytes(&memory), before);
    // A separate callback borrow observes a management-version gap before access.
    VERSION.set(2);
    assert!(access.with_funding_journal(|journal| journal.is_fenced()));
    Host::with_installation(|owner| {
        assert!(owner.stores().uploads.is_fenced());
    });
    assert_eq!(bytes(&memory), before);
    OWNER.with_borrow_mut(|owner| *owner = None);
}

fn assert_preparation_borrow_boundaries(
    access: &impl crate::ops::service::funding::access::FundingJournalAccess,
) {
    use crate::model::billing::journal::{FundingIntent, FundingJournalScope};
    use crate::ops::service::funding::FundingJournalError;
    use crate::policy::billing::admission::evidence::FundingAdmissionEvidence;
    use crate::policy::billing::admission::journal::FundingHostEvidence;
    use crate::workflow::funding::{FundingPreparationResult, prepare};
    use std::cell::Cell;
    let (intent, context) = access.with_funding_journal(|journal| {
        let config = journal.inspection_configuration();
        let bindings = config.bindings();
        (
            FundingIntent {
                service: bindings.service,
                cashier: config.billing().cashier(),
                account: bindings.payment_account,
                namespace: bindings.namespace,
                operation: 1.try_into().unwrap(),
                offered: 1.try_into().unwrap(),
                target_balance: None,
            },
            UploadContext {
                service: bindings.service,
                actor: bindings.operator,
            },
        )
    });
    let denied = UploadContext {
        actor: ic_platform::Principal::anonymous(),
        ..context
    };
    assert_eq!(
        prepare(access, denied, intent, || panic!(
            "unauthorized evidence callback"
        )),
        Err(FundingJournalError::NotOperator)
    );
    let mut wrong_scope = intent;
    wrong_scope.namespace = (intent.namespace.get() - 1).try_into().unwrap();
    assert_eq!(
        prepare(access, context, wrong_scope, || panic!(
            "foreign-scope evidence callback"
        )),
        Err(FundingJournalError::Binding)
    );
    let observed = Cell::new(false);
    let result = prepare(access, context, intent, || {
        observed.set(true);
        // Evidence acquisition can borrow the same host: no owner borrow leaked.
        assert_eq!(
            access
                .with_funding_journal(|journal| journal.lookup(context, intent))
                .unwrap(),
            None
        );
        FundingHostEvidence {
            scope: FundingJournalScope {
                service: intent.service,
                cashier: intent.cashier,
                account: intent.account,
                namespace: intent.namespace,
            },
            provider_qualified: false,
            admission: FundingAdmissionEvidence {
                available_cycles: None,
                recovery: None,
                activity: None,
            },
        }
    })
    .unwrap();
    assert!(observed.get());
    assert!(matches!(result, FundingPreparationResult::Blocked(_)));
}
