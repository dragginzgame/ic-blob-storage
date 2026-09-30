//! Published Canic managed lifecycle with shared real service owners and fixture inputs.
use blob_canic_probe::{CompositionSnapshot, ProbeFailure};
use candid::{CandidType, Deserialize, Principal};
use canic::{
    dto::{
        component_registry::ComponentRuntimeDirectoryPreparationRequest, role::OperationReceipt,
    },
    testing::{
        ManagedComponentGroupFixture, ManagedComponentGroupQualificationInput,
        ManagedRoleQualificationArtifact, install_managed_component_group,
    },
};
use ic_blob_storage::dto::tenant::{
    TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
};
use ic_blob_storage_canic::ManagedCallFailure;
use ic_testkit::pic::{CandidCallExt, CanisterInstallExt};
use std::time::Duration;
mod declaration;
mod installation;

struct Fixture {
    group: ManagedComponentGroupFixture,
    app: Principal,
    directory: ComponentRuntimeDirectoryPreparationRequest,
}
#[derive(CandidType)]
enum Command {
    ConfigureRuntime(Box<ComponentRuntimeDirectoryPreparationRequest>),
}
#[derive(CandidType, Deserialize)]
enum CommandResponse {
    OperationAccepted(OperationReceipt),
}
impl Fixture {
    fn new() -> Self {
        let mut artifact =
            ManagedRoleQualificationArtifact::new("storage".parse().unwrap(), wasm());
        artifact.application_init_args =
            Some(candid::encode_one(blob_canic_probe::configuration::input()).unwrap());
        let group = install_managed_component_group(ManagedComponentGroupQualificationInput::new(
            include_str!("../../../../canisters/test/canic_probe/canic.toml"),
            "storage",
            &"01".repeat(32),
            vec![
                Principal::from_slice(&[2, 1]),
                Principal::from_slice(&[3, 1]),
            ],
            vec![artifact],
        ))
        .unwrap();
        let app = group.unique_role(&"storage".parse().unwrap()).unwrap();
        let status = group.runtime_status(app).unwrap();
        let directory = ComponentRuntimeDirectoryPreparationRequest {
            authority: status.runtime.authority.unwrap(),
            direct_children: vec![],
            operation_id: status.operation_id,
        };
        Self {
            group,
            app,
            directory,
        }
    }
    fn app(&self) -> Principal {
        self.app
    }
    fn root(&self) -> Principal {
        self.group.root()
    }
    fn pic(&self) -> &ic_testkit::pocket_ic::PocketIc {
        self.group.pic()
    }
    fn arguments(&self) -> Vec<u8> {
        self.pic()
            .query_candid_as::<Result<Vec<u8>, ProbeFailure>, _>(
                self.app(),
                Principal::from_slice(&[2, 1]),
                "probe_init_arguments",
                (),
            )
            .unwrap()
            .unwrap()
    }
    fn reset_empty_fixture_to_prepared(&self) {
        // Only this fresh controlled fixture has no enrolled tenants or provider effects.
        let args = self.arguments();
        self.pic()
            .wait_out_install_code_rate_limit(Duration::from_secs(5));
        self.pic()
            .reinstall_canister(self.app(), wasm(), args, Some(self.root()))
            .unwrap();
    }
    fn configure_and_wait_until_active(&self, ticks: usize) {
        let response: Result<CommandResponse, canic::Error> = self
            .pic()
            .update_candid_as(
                self.app(),
                self.root(),
                canic::protocol::CANIC_COMMAND,
                (Command::ConfigureRuntime(Box::new(self.directory.clone())),),
            )
            .unwrap();
        let CommandResponse::OperationAccepted(receipt) = response.unwrap();
        assert_eq!(receipt.operation_id, self.directory.operation_id);
        for _ in 0..ticks {
            let status = self.group.runtime_status(self.app()).unwrap();
            if status.runtime.phase == canic::dto::component_registry::ComponentRuntimePhase::Active
                && status.fleet_activation.phase
                    == canic::dto::fleet_activation::FleetActivationPhase::Active
            {
                return;
            }
            self.pic().advance_time(Duration::from_secs(1));
            self.pic().tick();
        }
        panic!("bounded managed activation did not complete");
    }
    fn upgrade_same_release(&self, cooldown: Duration) {
        self.group
            .upgrade_same_release(self.app(), cooldown)
            .unwrap();
    }
}

fn wasm() -> Vec<u8> {
    std::fs::read(std::env::var_os("BLOB_CANIC_PROBE_WASM").expect("explicit Canic fixture path"))
        .unwrap()
}
fn fleet_admission(f: &Fixture, operator: Principal, tenant: Principal, outsider: Principal) {
    for actor in [operator, tenant] {
        assert_eq!(
            f.pic()
                .query_candid_as::<Result<Principal, ManagedCallFailure>, _>(
                    f.app(),
                    actor,
                    "probe_fleet_caller",
                    ()
                )
                .unwrap(),
            Ok(actor)
        );
    }
    assert_eq!(
        f.pic()
            .query_candid_as::<Result<Principal, ManagedCallFailure>, _>(
                f.app(),
                outsider,
                "probe_fleet_caller",
                ()
            )
            .unwrap(),
        Err(ManagedCallFailure::Admission)
    );
}
fn replacement_upgrade_refuses(f: &Fixture) {
    f.pic()
        .wait_out_install_code_rate_limit(Duration::from_secs(5));
    let before = f.pic().get_stable_memory(f.app());
    let error = f
        .pic()
        .upgrade_canister(
            f.app(),
            wasm(),
            candid::encode_one(7u8).unwrap(),
            Some(f.root()),
        )
        .unwrap_err();
    assert_eq!(
        error.reject_code,
        ic_testkit::pocket_ic::RejectCode::CanisterError
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}

fn denied_callers(f: &Fixture, input: TenantUpdateRequest, outsider: Principal) {
    let mut denied = vec![outsider, f.root()];
    denied.extend(
        f.pic()
            .canister_status(f.app(), Some(f.root()))
            .unwrap()
            .settings
            .controllers,
    );
    denied.sort_unstable();
    denied.dedup();
    for actor in denied {
        let before = f.pic().get_stable_memory(f.app());
        let result: Result<TenantEnrollmentResponse, ProbeFailure> = f
            .pic()
            .update_candid_as(f.app(), actor, "probe_enroll", (input,))
            .unwrap();
        assert_eq!(result, Err(ProbeFailure::Tenant(TenantFailure::Denied)));
        assert_eq!(f.pic().get_stable_memory(f.app()), before);
    }
}

#[test]
fn synchronous_installation_precedes_activation_and_all_owner_restore_preserves_authority() {
    let operator = Principal::from_slice(&[2, 1]);
    let tenant = Principal::from_slice(&[3, 1]);
    let outsider = Principal::from_slice(&[7, 1]);
    let f = Fixture::new();
    f.reset_empty_fixture_to_prepared();
    let snapshot = || {
        f.pic()
            .query_candid::<CompositionSnapshot, _>(f.app(), "probe_snapshot", ())
            .unwrap()
    };
    let installed = snapshot();
    assert_eq!(installed.configuration.service, f.app());
    assert_eq!(installed.configuration.namespace, u128::MAX);
    assert_eq!(installed.project, "fixture/β?&=");
    assert_eq!(installed.verifier, Principal::from_slice(&[5, 1]));
    assert_eq!(installed.release, "01".repeat(32));
    assert_eq!(installed.fences, [false; 4]);
    assert_eq!(installed.deferred_runs, 0);
    assert_eq!(installed.neighbor, b"neighbor");
    let scope = TenantScope {
        service: f.app(),
        namespace: u128::MAX,
        tenant,
    };
    let input = TenantUpdateRequest {
        scope,
        expected: None,
        active: true,
    };
    let enroll = |actor| {
        f.pic()
            .update_candid_as::<Result<TenantEnrollmentResponse, ProbeFailure>, _>(
                f.app(),
                actor,
                "probe_enroll",
                (input,),
            )
            .unwrap()
    };
    let before = f.pic().get_stable_memory(f.app());
    assert_eq!(
        enroll(operator),
        Err(ProbeFailure::Managed(ManagedCallFailure::Inactive))
    );
    let forwarded: Result<TenantEnrollmentResponse, ProbeFailure> = f
        .pic()
        .update_candid_as(f.app(), operator, "probe_forward_enroll", (f.app(), input))
        .unwrap();
    assert_eq!(
        forwarded,
        Err(ProbeFailure::Managed(ManagedCallFailure::Inactive))
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    f.configure_and_wait_until_active(30);
    for _ in 0..30 {
        if snapshot().deferred_runs > 0 {
            break;
        }
        f.pic().advance_time(Duration::from_secs(1));
        f.pic().tick();
    }
    assert!(snapshot().deferred_runs > 0);
    denied_callers(&f, input, outsider);
    let enrollment = enroll(operator).unwrap();
    fleet_admission(&f, operator, tenant, outsider);
    replacement_upgrade_refuses(&f);
    f.upgrade_same_release(Duration::from_secs(5));
    let restored = snapshot();
    assert_eq!(restored.configuration, installed.configuration);
    assert_eq!(restored.project, installed.project);
    assert_eq!(restored.verifier, installed.verifier);
    assert_eq!(restored.release, installed.release);
    assert_eq!(restored.neighbor, installed.neighbor);
    assert_eq!(restored.fences, [true; 4]);
    let before = f.pic().get_stable_memory(f.app());
    let retained: Result<TenantEnrollmentResponse, ProbeFailure> = f
        .pic()
        .query_candid_as(f.app(), tenant, "probe_tenant", (scope,))
        .unwrap();
    assert_eq!(
        retained.unwrap(),
        TenantEnrollmentResponse {
            fenced: true,
            ..enrollment
        }
    );
    assert_eq!(
        enroll(operator),
        Err(ProbeFailure::Tenant(TenantFailure::Fenced))
    );
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
}
