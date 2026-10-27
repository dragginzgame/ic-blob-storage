//! Explicit bounded stable storage and platform calls for the application substitute.
pub(crate) mod controls;
pub(crate) mod funding;
pub(crate) mod manifests;
pub(crate) mod tenants;
use crate::model::ConsumerRecord;
use blob_test_protocol::consumer::{AssetView, Failure, Fault};
use blob_test_protocol::fixture_storage::FixtureJournal;
use ic_blob_storage_client::admission::UploadAdmissionClientError;
use ic_blob_storage_client::download::ReplicatedDownloadClient;
use ic_blob_storage_client::references::ReplicatedReferenceClient;
use ic_blob_storage_contracts::download::reply::DownloadReplyLimits;
use ic_blob_storage_contracts::download::scope::CaffeineDownloadScope;
use ic_blob_storage_contracts::dto::download::DownloadRequest;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptLookup;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptResponse;
use ic_blob_storage_contracts::upload::admission::reply::UploadAdmissionReplyError;
use ic_memory::DefaultMemoryImpl;
use ic_platform::Principal;
use ic_wire::Limits;
use std::cell::{Cell, RefCell};
const KEY: &str = "fixture.consumer.wire.v1";
const MAX: usize = 16_384;
const LIMITS: Limits = Limits {
    bytes: MAX,
    allocation: MAX * 8,
    nodes: 262_144,
    depth: 32,
};
struct Host {
    journal: FixtureJournal<DefaultMemoryImpl>,
    record: ConsumerRecord,
}
thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
    static WAITING: Cell<bool> = const { Cell::new(false) };
    static READY: Cell<bool> = const { Cell::new(false) };
}
pub(crate) fn initialize(initial: Option<(Principal, Principal)>) {
    let mut journal = if initial.is_some() {
        FixtureJournal::create(DefaultMemoryImpl::default(), KEY, LIMITS)
    } else {
        FixtureJournal::open(DefaultMemoryImpl::default(), KEY, LIMITS)
    };
    let record = if let Some((operator, service)) = initial {
        ConsumerRecord::new(operator, service, ic_platform::cdk::api::canister_self())
    } else {
        let mut record: ConsumerRecord = journal.load();
        assert_eq!(
            record.tenant,
            ic_platform::cdk::api::canister_self(),
            "consumer identity"
        );
        record.validate().expect("consumer invariants");
        record.fence();
        record
    };
    record.validate().expect("consumer invariants");
    journal.save(&record);
    HOST.with_borrow_mut(|host| {
        assert!(host.is_none());
        *host = Some(Host { journal, record });
    });
}
pub(crate) fn read<R>(
    actor: Principal,
    f: impl FnOnce(&ConsumerRecord) -> Result<R, Failure>,
) -> Result<R, Failure> {
    HOST.with_borrow(|host| {
        let host = host.as_ref().unwrap();
        host.record.authorize(actor, false)?;
        f(&host.record)
    })
}
pub(crate) fn mutate<R>(
    actor: Principal,
    f: impl FnOnce(&mut ConsumerRecord) -> Result<R, Failure>,
) -> Result<R, Failure> {
    HOST.with_borrow_mut(|host| {
        let host = host.as_mut().unwrap();
        host.record.authorize(actor, true)?;
        let mut next = host.record.clone();
        let result = f(&mut next)?;
        next.validate().expect("consumer invariants");
        host.journal.save(&next);
        host.record = next;
        Ok(result)
    })
}
pub(crate) fn view(actor: Principal, id: u128) -> Result<AssetView, Failure> {
    read(actor, |r| r.view(id))
}
pub(crate) fn fault(selected: Fault, at: Fault) {
    if selected == at {
        ic_platform::cdk::api::trap("consumer transaction interruption");
    }
}
fn client(command: ReferenceCommand) -> ReplicatedReferenceClient {
    ReplicatedReferenceClient::new(
        ic_platform::cdk::api::canister_self(),
        command.upload.service,
        30.try_into().unwrap(),
    )
    .unwrap()
}
pub(crate) async fn apply(
    command: ReferenceCommand,
    max: u32,
) -> Result<ReferenceReceiptResponse, Failure> {
    let limit = (max as usize).try_into().map_err(|_| Failure::Invalid)?;
    client(command)
        .apply(command, limit)
        .await
        .map(|r| r.receipt)
        .map_err(|_| Failure::Transport)
}
pub(crate) async fn receipt(command: ReferenceCommand) -> Result<ReferenceReceiptLookup, Failure> {
    client(command)
        .receipt(command, 4096.try_into().unwrap())
        .await
        .map_err(|_| Failure::Transport)
}
pub(crate) async fn descriptor(
    source: blob_test_protocol::consumer::RegistrationSource,
) -> Result<(), Failure> {
    let (u, reference) = crate::model::binding(source);
    let scope = CaffeineDownloadScope::new(
        u.service,
        u.namespace.try_into().unwrap(),
        "fixture project/β?&=",
    )
    .unwrap();
    let client =
        ReplicatedDownloadClient::new(u.tenant, u.service, 30.try_into().unwrap()).unwrap();
    client
        .fetch(
            DownloadRequest {
                service: u.service,
                tenant: u.tenant,
                namespace: u.namespace,
                root: u.root,
                object: u.object,
                incarnation: u.incarnation,
                reference,
            },
            &scope,
            DownloadReplyLimits {
                max_reply_bytes: 4096.try_into().unwrap(),
                max_content_bytes: 10.try_into().unwrap(),
                max_headers: 8.try_into().unwrap(),
                max_header_bytes: 1024.try_into().unwrap(),
            },
        )
        .await
        .map(|_| ())
        .map_err(|_| Failure::Transport)
}
pub(crate) fn waiting(actor: Principal) -> Result<bool, Failure> {
    read(actor, |_| Ok(WAITING.get()))
}
pub(crate) fn resume(actor: Principal) -> Result<(), Failure> {
    read(actor, |r| r.authorize(actor, true))?;
    READY.set(true);
    Ok(())
}
pub(crate) async fn hold() -> Result<(), Failure> {
    if WAITING.replace(true) {
        return Err(Failure::Pending);
    }
    READY.set(false);
    for _ in 0..128 {
        if READY.get() {
            WAITING.set(false);
            return Ok(());
        }
        if ic_platform::cdk::call::Call::unbounded_wait(
            Principal::management_canister(),
            "raw_rand",
        )
        .take_raw_args(
            ic_platform::candid::contract::encode_args(()).expect("fixture scheduling arguments"),
        )
        .await
        .is_err()
        {
            break;
        }
    }
    WAITING.set(false);
    Err(Failure::Pending)
}

fn admission_client(
    permission: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest,
) -> ic_blob_storage_client::admission::ReplicatedUploadAdmissionClient {
    ic_blob_storage_client::admission::ReplicatedUploadAdmissionClient::new(
        ic_platform::cdk::api::canister_self(),
        permission.upload.service,
        30.try_into().unwrap(),
    )
    .expect("validated consumer configuration")
}
pub(crate) async fn admission_status(
    permission: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest,
) -> Result<ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse, Failure> {
    admission_client(permission)
        .inspect(permission, 4096.try_into().unwrap())
        .await
        .map_err(|_| Failure::Transport)
}
pub(crate) async fn admit(
    permission: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest,
    max: u32,
) -> Result<
    Result<
        ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse,
        ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
    >,
    Failure,
> {
    let max = (max as usize).try_into().map_err(|_| Failure::Invalid)?;
    match admission_client(permission).admit(permission, max).await {
        Ok(response) => Ok(Ok(response.admission)),
        Err(UploadAdmissionClientError::Reply(UploadAdmissionReplyError::Remote(error))) => {
            Ok(Err(error))
        }
        Err(_) => Err(Failure::Transport),
    }
}

pub(crate) async fn revoke(
    permission: ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest,
    max: u32,
) -> Result<
    Result<
        ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse,
        ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure,
    >,
    Failure,
> {
    let max = (max as usize).try_into().map_err(|_| Failure::Invalid)?;
    match admission_client(permission).revoke(permission, max).await {
        Ok(response) => Ok(Ok(response.admission)),
        Err(UploadAdmissionClientError::Reply(UploadAdmissionReplyError::Remote(error))) => {
            Ok(Err(error))
        }
        Err(_) => Err(Failure::Transport),
    }
}
