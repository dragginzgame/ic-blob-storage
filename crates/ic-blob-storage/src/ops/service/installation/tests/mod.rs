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
fn installation_record_keeps_candid_bytes_and_rejects_oversized_encoding() {
    let memory = memory();
    let installed = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    let expected = ic_platform::candid::contract::encode_one(&installed.configuration).unwrap();
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
fn memory() -> [FixtureMemory; 17] {
    std::array::from_fn(|_| FixtureMemory::default())
}
fn memories(
    memory: &[FixtureMemory; 17],
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
fn bytes(memory: &[FixtureMemory; 17]) -> [Vec<u8>; 17] {
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
    for occupied in 0..17 {
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
fn missing_configuration_and_wrong_row_count_refuse_without_initialization() {
    let memory = memory();
    assert!(matches!(
        ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
        Err(ServiceInstallationError::Missing)
    ));
    assert!(memory.iter().all(|m| m.lease().size() == 0));
    drop(StableBTreeMap::<u8, ConfigurationRecord, _>::new(
        memory[0].lease(),
    ));
    for count in [0, 2] {
        let mut records = StableBTreeMap::<u8, ConfigurationRecord, _>::load(memory[0].lease());
        for key in 0..count {
            records.insert(key, configuration::record(&input()));
        }
        drop(records);
        let before = bytes(&memory);
        assert!(matches!(
            ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
            Err(ServiceInstallationError::RecordCount)
        ));
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
    missing[16] = FixtureMemory::default();
    let missing_before = bytes(&missing);
    assert!(matches!(
        ServiceInstallation::open(memories(&missing), candidate().service, RELEASE),
        Err(ServiceInstallationError::Stores(ServiceStoreError::Missing))
    ));
    assert_eq!(bytes(&missing), missing_before);
    let mut records: StableBTreeMap<u8, ConfigurationRecord, _> =
        StableBTreeMap::load(memory[0].lease());
    let mut record = records.get(&0).unwrap();
    // Both prior allocation and verifier-bearing installation formats must refuse
    // even when the host release string is unchanged. Never mutate old bytes.
    for retired in [
        "ic-blob-storage/installation:platform-anchor-funding-credit-index-renewal",
        "ic-blob-storage/installation:project-upload-grants-platform-anchor-funding-credit-index-renewal",
    ] {
        record.format = retired.to_owned();
        records.insert(0, record.clone());
        drop(records);
        let before = bytes(&memory);
        assert!(matches!(
            ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
            Err(ServiceInstallationError::Binding(
                InstallationBindingError::Format
            ))
        ));
        assert_eq!(bytes(&memory), before);
        records = StableBTreeMap::load(memory[0].lease());
    }
    record.format = ConfigurationRecord::FORMAT.to_owned();
    record.project = " invalid-project".to_owned();
    records.insert(0, record);
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
fn populated_installation_preserves_full_configuration_scope_verifier_and_owner_fences() {
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
    expected.reads.fenced = true;
    assert_eq!(
        inspect(restored.stores().into(), context, scope),
        Ok(expected)
    );
    assert_eq!(bytes(&memory), before);
}
