//! State access, sample construction and local mutations for the test fixture.

pub(crate) mod content;
pub(crate) mod obligations;
pub(crate) mod sync;
pub(crate) mod uploads;

use std::{
    cell::RefCell,
    num::{NonZeroU128, NonZeroUsize},
};

use candid::Principal;
use ic_blob_storage::model::{
    catalog::{BlobCatalog, CatalogLimits, CatalogReferenceKey, ConfirmedObject},
    gateway::{
        GatewayListLimits,
        membership::GatewayMembership,
        registry::{GatewayRegistry, GatewayScope},
    },
    identity::ProviderRootHash,
    lifecycle::{
        ReferenceId,
        binding::{ObjectBinding, ObjectIdentity, ReferenceKey},
        requests::{
            ReferenceOperation, ReferenceRequest, ReferenceRequestId, ReferenceRequestOutcome,
        },
    },
};

thread_local! {
    static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
}

pub(crate) struct State {
    pub catalog: BlobCatalog,
    pub uploads: uploads::Uploads,
    pub registry: GatewayRegistry,
    pub operator: Principal,
    gateway: Principal,
}

pub(crate) fn bound(value: usize) -> NonZeroUsize {
    NonZeroUsize::new(value).expect("positive fixture bound")
}

fn number(value: u128) -> NonZeroU128 {
    NonZeroU128::new(value).expect("positive fixture identity")
}

fn root(value: u8) -> ProviderRootHash {
    ProviderRootHash::try_from([value; 32].as_slice()).expect("fixture root")
}

pub(crate) fn reference(
    value: u8,
    service: Principal,
    tenant: Principal,
) -> Option<CatalogReferenceKey> {
    let object = ObjectBinding::new(
        service,
        tenant,
        ObjectIdentity {
            namespace: number(1),
            object: NonZeroU128::new(u128::from(value))?,
            incarnation: number(1),
        },
    )
    .ok()?;
    Some(CatalogReferenceKey {
        root: root(value),
        reference: ReferenceKey::new(object, ReferenceId::new(number(1))),
    })
}

pub(crate) fn initialize(
    service: Principal,
    first: Principal,
    second: Principal,
    gateway: Principal,
    operator: Principal,
) {
    let mut catalog = BlobCatalog::new(
        service,
        CatalogLimits {
            max_objects: bound(2),
            max_tenant_objects: bound(1),
            max_physical_bytes: number(300),
            max_liability_bytes: number(300),
            max_tenant_logical_bytes: number(200),
            max_references_per_object: bound(1),
            max_receipts_per_object: bound(2),
        },
    )
    .expect("fixture catalog");
    for (value, tenant, bytes) in [(1, first, 100), (2, second, 200)] {
        let key = reference(value, service, tenant).expect("fixture binding");
        catalog
            .insert_confirmed(ConfirmedObject {
                root: key.root,
                bytes,
                first: key.reference,
            })
            .expect("supplied confirmed-object facts, not provider evidence");
    }
    let mut membership = GatewayMembership::new(GatewayListLimits {
        max_entries: bound(1),
        max_unique: bound(1),
    });
    membership.add(gateway).expect("fixture gateway");
    let registry = GatewayRegistry::new(
        GatewayScope::new(service, number(1), operator).expect("fixture scope"),
        membership,
    );
    STATE.with_borrow_mut(|state| {
        *state = Some(State {
            catalog,
            uploads: uploads::initialize(service, first, second),
            registry,
            operator,
            gateway,
        });
    });
}

pub(crate) fn read<T>(f: impl FnOnce(&State) -> T) -> T {
    STATE.with_borrow(|state| f(state.as_ref().expect("initialized fixture")))
}

pub(crate) fn binding(value: u8) -> Option<ObjectBinding> {
    read(|state| {
        state
            .catalog
            .get(root(value))
            .map(|journal| journal.lifecycle().binding())
    })
}

pub(crate) fn release(value: u8, actor: Principal) -> bool {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().expect("initialized fixture");
        let Some(journal) = state.catalog.get(root(value)) else {
            return false;
        };
        let reference =
            ReferenceKey::new(journal.lifecycle().binding(), ReferenceId::new(number(1)));
        matches!(
            state.catalog.apply_reference(
                root(value),
                actor,
                ReferenceRequest {
                    id: ReferenceRequestId::new(number(1)),
                    operation: ReferenceOperation::Release(reference),
                }
            ),
            Ok(ReferenceRequestOutcome::Recorded { result: Ok(_) }
                | ReferenceRequestOutcome::Replayed { result: Ok(_) })
        )
    })
}

pub(crate) fn revoke_gateway() {
    STATE.with_borrow_mut(|state| {
        let state = state.as_mut().expect("initialized fixture");
        state.registry.remove(state.gateway);
    });
}
