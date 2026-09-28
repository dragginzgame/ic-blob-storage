use super::*;
use crate::{
    model::service::read::session::{
        ReadChunkTarget, ReadSessionError, ReadSessionLimits, ReadSessionUsage,
    },
    ops::service::reads::{ReadSessionMemories, StableReadSessions},
    workflow::reads::{
        ReadAuthorityError,
        sessions::{ReadSessionWorkflowError, begin, complete},
    },
};
pub(super) fn store() -> StableReadSessions<VectorMemory> {
    StableReadSessions::install(
        ReadSessionMemories {
            journal: VectorMemory::default(),
            sessions: VectorMemory::default(),
            tenants: VectorMemory::default(),
        },
        config(),
        ReadSessionLimits::new(
            1.try_into().unwrap(),
            1.try_into().unwrap(),
            64.try_into().unwrap(),
            64.try_into().unwrap(),
            64.try_into().unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn read_sessions_hold_capacity_across_revocation_and_stale_callbacks_cannot_free_new_reads() {
    let (mut registry, uploads, input) = read_authority::setup();
    let scope =
        crate::model::gateway::registry::GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap();
    let chunk = ReadChunkTarget {
        target: read_authority::target(input),
        index: 0,
    };
    let mut sessions = store();
    assert_eq!(
        begin(
            &registry,
            &uploads,
            &mut sessions,
            context(4),
            scope,
            ReadChunkTarget { index: 1, ..chunk }
        ),
        Err(ReadSessionWorkflowError::Session(ReadSessionError::Chunk))
    );
    assert_eq!(sessions.inspect(context(2)).unwrap().last_sequence, 0);
    let first = begin(&registry, &uploads, &mut sessions, context(4), scope, chunk).unwrap();
    registry.remove(context(2), scope, p(6)).unwrap();
    registry.add(context(2), scope, p(6)).unwrap();
    assert_eq!(
        sessions.inspect(context(2)).unwrap().usage,
        ReadSessionUsage {
            sessions: 1,
            reserved_bytes: 64
        }
    );
    assert_eq!(
        begin(&registry, &uploads, &mut sessions, context(4), scope, chunk),
        Err(ReadSessionWorkflowError::Session(
            ReadSessionError::Capacity
        ))
    );
    assert_eq!(
        complete(&registry, &uploads, &mut sessions, context(4), &first),
        Err(ReadSessionWorkflowError::Authority(
            ReadAuthorityError::Stale
        ))
    );
    assert_eq!(sessions.inspect(context(2)).unwrap().usage.sessions, 0);
    let next = begin(&registry, &uploads, &mut sessions, context(4), scope, chunk).unwrap();
    assert_eq!(
        complete(&registry, &uploads, &mut sessions, context(4), &first),
        Err(ReadSessionWorkflowError::Session(ReadSessionError::Stale))
    );
    assert_eq!(sessions.inspect(context(2)).unwrap().usage.sessions, 1);
    complete(&registry, &uploads, &mut sessions, context(4), &next).unwrap();
    assert_eq!(sessions.inspect(context(2)).unwrap().usage.sessions, 0);
}

#[test]
fn read_session_completion_keeps_capacity_when_either_authority_owner_is_restored() {
    use crate::ops::service::gateways::StableGatewayRegistry;
    let gm = VectorMemory::default();
    let um = memory();
    let mut registry = StableGatewayRegistry::install(gm.clone(), config()).unwrap();
    let scope =
        crate::model::gateway::registry::GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap();
    registry.add(context(2), scope, p(6)).unwrap();
    let mut uploads = StableUploads::install(clone_memory(&um), config()).unwrap();
    let input = lifecycle::exposed(&mut uploads);
    uploads.confirm_upload(input.request).unwrap();
    let mut sessions = store();
    let ticket = begin(
        &registry,
        &uploads,
        &mut sessions,
        context(4),
        scope,
        ReadChunkTarget {
            target: read_authority::target(input),
            index: 0,
        },
    )
    .unwrap();
    let before = sessions.inspect(context(2)).unwrap();
    let restored_registry = StableGatewayRegistry::open(gm, config()).unwrap();
    let restored_uploads = StableUploads::open(clone_memory(&um), config()).unwrap();
    assert_eq!(
        complete(
            &restored_registry,
            &uploads,
            &mut sessions,
            context(4),
            &ticket
        ),
        Err(ReadSessionWorkflowError::Authority(
            ReadAuthorityError::Registry(crate::ops::service::gateways::GatewayStoreError::Fenced)
        ))
    );
    assert_eq!(
        complete(
            &registry,
            &restored_uploads,
            &mut sessions,
            context(4),
            &ticket
        ),
        Err(ReadSessionWorkflowError::Authority(
            ReadAuthorityError::Uploads(UploadStoreError::Fenced)
        ))
    );
    assert_eq!(sessions.inspect(context(2)).unwrap(), before);
    complete(&registry, &uploads, &mut sessions, context(4), &ticket).unwrap();
}
