//! Explicit bounded stable storage and platform calls for the application substitute.
pub(crate) mod manifests;
use crate::model::ConsumerRecord;
use blob_test_protocol::consumer::{AssetView, Failure, Fault};
use candid::{CandidType, Deserialize, Principal, de::DecoderConfig};
use ic_blob_storage::{
    dto::{
        download::DownloadRequest,
        reference::{ReferenceCommand, ReferenceReceiptLookup, ReferenceReceiptResponse},
    },
    ic_memory::{
        GenericRangePolicy, MemoryManagerAuthorityRecord, MemoryManagerConfig,
        MemoryManagerIdRange, MemoryManagerRangeMode, MemoryRequest, MemoryRuntime, RuntimeMemory,
        SchemaMetadata, SealedDeclarationSnapshot, StaticMemoryRangeDeclaration,
        ic_stable_structures::{DefaultMemoryImpl, Memory},
    },
    model::service::read::download::CaffeineDownloadScope,
    ops::service::{
        reads::download::{client::ReplicatedDownloadClient, reply::DownloadReplyLimits},
        references::client::ReplicatedReferenceClient,
    },
};
use std::cell::{Cell, RefCell};
const KEY: &str = "fixture.consumer.v1";
const MAX: usize = 16_384;
struct Host {
    _runtime: MemoryRuntime<DefaultMemoryImpl>,
    memory: RuntimeMemory<DefaultMemoryImpl>,
    record: ConsumerRecord,
}
thread_local! {
    static HOST: RefCell<Option<Host>> = const { RefCell::new(None) };
    static WAITING: Cell<bool> = const { Cell::new(false) };
    static READY: Cell<bool> = const { Cell::new(false) };
}
fn config() -> DecoderConfig {
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(1_000_000)
        .set_skipping_quota(1024)
        .set_max_type_len(128)
        .set_max_header_len(MAX)
        .set_full_error_message(false);
    config
}
#[expect(
    clippy::needless_pass_by_value,
    reason = "CDK supplies owned bounded ingress"
)]
pub(crate) fn decode<T: CandidType + for<'de> Deserialize<'de>>(bytes: Vec<u8>) -> T {
    assert!(bytes.len() <= 4096, "consumer ingress bound");
    candid::decode_one_with_config(&bytes, &config()).expect("consumer ingress")
}
pub(crate) fn initialize(initial: Option<(Principal, Principal)>) {
    let grant = StaticMemoryRangeDeclaration::new(
        MemoryManagerAuthorityRecord::new(
            MemoryManagerIdRange::new(120, 120).unwrap(),
            "fixture",
            MemoryManagerRangeMode::Allowed,
            None,
        )
        .unwrap(),
    )
    .unwrap();
    let request = MemoryRequest::new("fixture", KEY, SchemaMetadata::default()).unwrap();
    let declarations = SealedDeclarationSnapshot::new(&[], &[grant], &[request]).unwrap();
    let mut runtime = MemoryRuntime::new_with_config(
        DefaultMemoryImpl::default(),
        MemoryManagerConfig::new(16).unwrap(),
    )
    .unwrap();
    runtime
        .bootstrap(&declarations, &GenericRangePolicy)
        .unwrap();
    let memory = runtime.open_memory_by_key(KEY).unwrap();
    let record = if let Some((operator, service)) = initial {
        assert_eq!(memory.size(), 0, "initialization cannot reset history");
        assert_ne!(memory.grow(1), -1, "fixture memory allocation");
        ConsumerRecord::new(operator, service, ic_cdk::api::canister_self())
    } else {
        assert!(memory.size() > 0, "missing consumer history");
        let mut header = [0; 9];
        memory.read(0, &mut header);
        assert_eq!(&header[..5], b"CONS\x01", "consumer envelope");
        let length = u32::from_le_bytes(header[5..].try_into().unwrap()) as usize;
        assert!((1..=MAX).contains(&length), "consumer envelope length");
        let mut bytes = vec![0; length];
        memory.read(9, &mut bytes);
        let mut record: ConsumerRecord =
            candid::decode_one_with_config(&bytes, &config()).expect("consumer record");
        assert_eq!(
            record.tenant,
            ic_cdk::api::canister_self(),
            "consumer identity"
        );
        record.validate().expect("consumer invariants");
        record.fence();
        record
    };
    save(&memory, &record);
    HOST.with_borrow_mut(|host| {
        assert!(host.is_none());
        *host = Some(Host {
            _runtime: runtime,
            memory,
            record,
        });
    });
}
fn save(memory: &RuntimeMemory<DefaultMemoryImpl>, record: &ConsumerRecord) {
    record.validate().expect("consumer invariants");
    let bytes = candid::encode_one(record).unwrap();
    assert!(bytes.len() <= MAX, "consumer record bound");
    memory.write(9, &bytes);
    memory.write(0, b"CONS\x01");
    memory.write(5, &u32::try_from(bytes.len()).unwrap().to_le_bytes());
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
        save(&host.memory, &next);
        host.record = next;
        Ok(result)
    })
}
pub(crate) fn view(actor: Principal, id: u128) -> Result<AssetView, Failure> {
    read(actor, |r| r.view(id))
}
pub(crate) fn fault(selected: Fault, at: Fault) {
    if selected == at {
        ic_cdk::trap("consumer transaction interruption");
    }
}
fn client(command: ReferenceCommand) -> ReplicatedReferenceClient {
    ReplicatedReferenceClient::new(
        ic_cdk::api::canister_self(),
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
        if ic_cdk::call::Call::unbounded_wait(Principal::management_canister(), "raw_rand")
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
    permission: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> ic_blob_storage::ops::service::uploads::admission::client::ReplicatedUploadAdmissionClient {
    ic_blob_storage::ops::service::uploads::admission::client::ReplicatedUploadAdmissionClient::new(
        ic_cdk::api::canister_self(),
        permission.upload.service,
        30.try_into().unwrap(),
    )
    .expect("validated consumer configuration")
}
pub(crate) async fn admission_status(
    permission: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
) -> Result<ic_blob_storage::dto::upload::admission::UploadAdmissionResponse, Failure> {
    admission_client(permission)
        .inspect(permission, 4096.try_into().unwrap())
        .await
        .map_err(|_| Failure::Transport)
}
pub(crate) async fn admit(
    permission: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
    max: u32,
) -> Result<
    Result<
        ic_blob_storage::dto::upload::admission::UploadAdmissionResponse,
        ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
    >,
    Failure,
> {
    use ic_blob_storage::ops::service::uploads::admission::{
        client::UploadAdmissionClientError, reply::UploadAdmissionReplyError,
    };
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
    permission: ic_blob_storage::dto::upload::admission::UploadAdmissionRequest,
    max: u32,
) -> Result<
    Result<
        ic_blob_storage::dto::upload::admission::UploadAdmissionResponse,
        ic_blob_storage::dto::upload::admission::UploadAdmissionFailure,
    >,
    Failure,
> {
    use ic_blob_storage::ops::service::uploads::admission::{
        client::UploadAdmissionClientError, reply::UploadAdmissionReplyError,
    };
    let max = (max as usize).try_into().map_err(|_| Failure::Invalid)?;
    match admission_client(permission).revoke(permission, max).await {
        Ok(response) => Ok(Ok(response.admission)),
        Err(UploadAdmissionClientError::Reply(UploadAdmissionReplyError::Remote(error))) => {
            Ok(Err(error))
        }
        Err(_) => Err(Failure::Transport),
    }
}
