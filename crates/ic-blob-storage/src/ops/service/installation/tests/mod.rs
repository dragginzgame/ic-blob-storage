use super::*;
use crate::{
    dto::operator::OperatorScope,
    model::{
        billing::journal::FundingIntent,
        service::{tenant::TenantUpdate, upload::UploadContext},
    },
    ops::service::{configuration::tests::candidate, stores::grants},
    workflow::operator::inspect,
};
use ic_memory::ic_stable_structures::VectorMemory;

const RELEASE: &str = "test-release";
fn input() -> ServiceInstallationCandidate<'static> {
    ServiceInstallationCandidate {
        platform_installation_version: 0,
        configuration: candidate(),
        project: "project/β?&=",
        completion_verifier: Principal::from_slice(&[5, 1]),
        trusted_uploader: Principal::from_slice(&[6, 1]),
        release: RELEASE,
    }
}
fn validated() -> ValidatedServiceInstallation {
    ValidatedServiceInstallation::new(candidate().service, input()).unwrap()
}
fn memory() -> [VectorMemory; 17] {
    std::array::from_fn(|_| VectorMemory::default())
}
fn memories(memory: &[VectorMemory; 17]) -> ServiceInstallationMemories<VectorMemory> {
    let requests = grants::requests("installation-test").unwrap();
    ServiceInstallationMemories {
        configuration: memory[0].clone(),
        stores: grants::open::<_, std::convert::Infallible>(|key| {
            let index = requests
                .iter()
                .position(|r| r.stable_key().as_str() == key)
                .unwrap();
            Ok(memory[index + 1].clone())
        })
        .unwrap(),
    }
}
fn bytes(memory: &[VectorMemory; 17]) -> [Vec<u8>; 17] {
    memory.each_ref().map(|m| m.borrow().clone())
}

#[test]
fn configuration_inspection_binds_operator_service_and_preserves_restore_fences() {
    use crate::{dto::configuration::HostFailure, workflow::installation::inspect};
    let memory = memory();
    let installed = ServiceInstallation::install(memories(&memory), validated()).unwrap();
    let context = UploadContext {
        service: candidate().service,
        actor: candidate().operator,
    };
    let view = inspect(&installed, context).unwrap();
    assert_eq!(view.configuration, candidate());
    assert_eq!(view.project, input().project);
    assert_eq!(view.completion_verifier, input().completion_verifier);
    assert_eq!(view.trusted_uploader, input().trusted_uploader);
    assert_eq!(view.release, RELEASE);
    assert!(!view.fenced);
    let before = bytes(&memory);
    for actor in [
        candidate().payment_account,
        input().completion_verifier,
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
        Ok(crate::dto::configuration::HostConfigurationView {
            fenced: true,
            ..view
        })
    );
    assert_eq!(bytes(&memory), before);
}

#[test]
fn whole_candidate_rejects_invalid_bindings_project_verifier_and_release() {
    let original = input();
    assert!(matches!(
        ValidatedServiceInstallation::new(original.completion_verifier, original),
        Err(ServiceInstallationError::Configuration(
            ConfigurationInputError::ServiceBinding
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
    for completion_verifier in [Principal::anonymous(), Principal::management_canister()] {
        assert!(matches!(
            ValidatedServiceInstallation::new(
                candidate().service,
                ServiceInstallationCandidate {
                    completion_verifier,
                    ..original
                }
            ),
            Err(ServiceInstallationError::Verifier(
                InvalidCompletionAuthority
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
    for trusted_uploader in [Principal::anonymous(), Principal::management_canister()] {
        assert!(matches!(
            ValidatedServiceInstallation::new(
                candidate().service,
                ServiceInstallationCandidate {
                    trusted_uploader,
                    ..original
                }
            ),
            Err(ServiceInstallationError::Issuer(
                InvalidUploadIssuerAuthority
            ))
        ));
    }
}

#[test]
fn allocated_grant_anywhere_rejects_before_installation_or_owner_writes() {
    for occupied in 0..17 {
        let memory = memory();
        assert_eq!(memory[occupied].grow(1), 0);
        memory[occupied].write(0, b"retained installation obligation");
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
    assert!(memory.iter().all(|m| m.size() == 0));
    let mut records: BTreeMap<u8, ConfigurationRecord, _> = BTreeMap::new(memory[0].clone());
    for count in [0, 2] {
        for key in 0..count {
            records.insert(key, configuration::record(&input()));
        }
        let before = bytes(&memory);
        assert!(matches!(
            ServiceInstallation::open(memories(&memory), candidate().service, RELEASE),
            Err(ServiceInstallationError::RecordCount)
        ));
        assert_eq!(bytes(&memory), before);
    }
}

#[test]
fn wrong_host_release_invalid_retained_project_and_missing_owner_never_repair() {
    let memory = memory();
    drop(ServiceInstallation::install(memories(&memory), validated()).unwrap());
    let before = bytes(&memory);
    for (service, release, expected) in [
        (
            input().completion_verifier,
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
    missing[16] = VectorMemory::default();
    let missing_before = bytes(&missing);
    assert!(matches!(
        ServiceInstallation::open(memories(&missing), candidate().service, RELEASE),
        Err(ServiceInstallationError::Stores(ServiceStoreError::Missing))
    ));
    assert_eq!(bytes(&missing), missing_before);
    let mut records: BTreeMap<u8, ConfigurationRecord, _> = BTreeMap::load(memory[0].clone());
    let mut record = records.get(&0).unwrap();
    record.project = " invalid-project".to_owned();
    records.insert(0, record);
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
    assert_eq!(
        restored.completion_authority().verifier(),
        original.completion_verifier
    );
    assert_eq!(restored.release(), RELEASE);
    assert_eq!(
        restored.issuer_authority().uploader(),
        original.trusted_uploader
    );
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
