use super::*;
use crate::{
    model::{
        gateway::registry::GatewayScope, identity::caffeine::manifest::CaffeineManifestError,
        service::read::session::ReadChunkTarget,
    },
    ops::service::{
        gateways::StableGatewayRegistry,
        reads::{
            StableReadSessions,
            access::ReadSessionAccess,
            transport::{ReadChunkRequest, ReadChunkResponse, ReadChunkTransport},
        },
        uploads::read::verification::ReadVerificationError,
    },
    workflow::reads::{
        ReadAuthorityError,
        chunk::{ReadChunkError, VerifiedReadChunk, read_chunk},
        sessions::ReadSessionWorkflowError,
    },
};
use std::{
    cell::{Cell, RefCell},
    future::{Future, poll_fn},
    pin::pin,
    task::{Context, Poll, Waker},
};
struct Owners {
    registry: StableGatewayRegistry<VectorMemory>,
    uploads: StableUploads<VectorMemory>,
    sessions: StableReadSessions<VectorMemory>,
}
struct Host(RefCell<Owners>);
impl ReadSessionAccess for Host {
    type Memory = VectorMemory;
    fn with_read_sessions<R>(
        &self,
        operation: impl FnOnce(
            &StableGatewayRegistry<VectorMemory>,
            &StableUploads<VectorMemory>,
            &mut StableReadSessions<VectorMemory>,
        ) -> R,
    ) -> R {
        let mut state = self.0.borrow_mut();
        let Owners {
            registry,
            uploads,
            sessions,
        } = &mut *state;
        operation(registry, uploads, sessions)
    }
}
fn setup() -> (Host, GatewayScope, ReadChunkTarget) {
    let (registry, uploads, permission) = read_authority::setup();
    (
        Host(RefCell::new(Owners {
            registry,
            uploads,
            sessions: read_sessions::store(),
        })),
        GatewayScope::new(p(1), NonZeroU128::MIN, p(3)).unwrap(),
        ReadChunkTarget {
            target: read_authority::target(permission),
            index: 0,
        },
    )
}
struct Transport<F> {
    calls: Cell<u32>,
    ready: Cell<bool>,
    respond: F,
}
impl<F: Fn(ReadChunkRequest) -> Result<ReadChunkResponse, u8>> ReadChunkTransport for Transport<F> {
    type Error = u8;
    async fn read_chunk(&self, request: ReadChunkRequest) -> Result<ReadChunkResponse, u8> {
        self.calls.set(self.calls.get() + 1);
        poll_fn(|_| {
            if self.ready.get() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
        (self.respond)(request)
    }
}
fn transport<F>(respond: F) -> Transport<F> {
    Transport {
        calls: Cell::new(0),
        ready: Cell::new(true),
        respond,
    }
}
fn response(request: ReadChunkRequest) -> ReadChunkResponse {
    assert_eq!(request.max_reply_bytes.get(), 64);
    ReadChunkResponse {
        source: request.chunk.target.gateway,
        root: request.chunk.target.root,
        index: request.chunk.index,
        bytes: vec![1; 10],
    }
}
fn run<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    match future
        .as_mut()
        .poll(&mut Context::from_waker(Waker::noop()))
    {
        Poll::Ready(result) => result,
        Poll::Pending => panic!("expected synchronous substitute completion"),
    }
}
#[test]
fn verified_reads_bind_response_and_settle_all_returned_failures_without_touching_uploads() {
    let (host, scope, chunk) = setup();
    let before = host.0.borrow().uploads.usage().unwrap();
    for case in 0..8 {
        let source = transport(|request| {
            let mut reply = response(request);
            match case {
                0 => reply.source = p(7),
                1 => reply.root = ProviderRootHash::try_from([8; 32].as_slice()).unwrap(),
                2 => reply.index = 1,
                3 => reply.bytes = vec![1; 65],
                4 => reply.bytes[0] ^= 1,
                5 => {
                    reply.bytes.pop();
                }
                6 => return Err(7),
                _ => {}
            }
            Ok(reply)
        });
        let result = run(read_chunk(&host, &source, context(4), scope, chunk));
        let expected = match case {
            0..=2 => Err(ReadChunkError::Binding),
            3 => Err(ReadChunkError::ReplyTooLarge),
            4 => Err(ReadChunkError::Verification(
                ReadVerificationError::Content(CaffeineManifestError::ChunkHashMismatch),
            )),
            5 => Err(ReadChunkError::Verification(
                ReadVerificationError::Content(CaffeineManifestError::ChunkLengthMismatch {
                    expected: 10,
                    actual: 9,
                }),
            )),
            6 => Err(ReadChunkError::Transport(7)),
            _ => Ok(VerifiedReadChunk {
                index: 0,
                offset: 0,
                bytes: vec![1; 10],
            }),
        };
        assert_eq!(result, expected);
        assert_eq!(source.calls.get(), 1);
        let state = host.0.borrow();
        assert_eq!(
            state.sessions.inspect(context(2)).unwrap().usage.sessions,
            0
        );
        assert_eq!(state.uploads.usage().unwrap(), before);
    }
}
#[test]
fn verified_reads_admit_on_poll_recheck_before_hashing_and_keep_abandoned_slots() {
    let (host, scope, chunk) = setup();
    let source = transport(|request| {
        let mut reply = response(request);
        reply.bytes[0] ^= 1;
        Ok(reply)
    });
    source.ready.set(false);
    let mut future = pin!(read_chunk(&host, &source, context(4), scope, chunk));
    assert_eq!(
        host.0
            .borrow()
            .sessions
            .inspect(context(2))
            .unwrap()
            .last_sequence,
        0
    );
    let mut cx = Context::from_waker(Waker::noop());
    assert!(future.as_mut().poll(&mut cx).is_pending());
    assert_eq!(
        host.0
            .borrow()
            .sessions
            .inspect(context(2))
            .unwrap()
            .usage
            .sessions,
        1
    );
    // The host borrow was released across the await. Even restored membership
    // does not authorize the original callback, and corrupt bytes are not hashed.
    host.0
        .borrow_mut()
        .registry
        .remove(context(2), scope, p(6))
        .unwrap();
    host.0
        .borrow_mut()
        .registry
        .add(context(2), scope, p(6))
        .unwrap();
    source.ready.set(true);
    assert_eq!(
        future.as_mut().poll(&mut cx),
        Poll::Ready(Err(ReadChunkError::Session(
            ReadSessionWorkflowError::Authority(ReadAuthorityError::Stale)
        )))
    );
    assert_eq!(
        host.0
            .borrow()
            .sessions
            .inspect(context(2))
            .unwrap()
            .usage
            .sessions,
        0
    );
    source.ready.set(false);
    {
        let mut abandoned = pin!(read_chunk(&host, &source, context(4), scope, chunk));
        assert!(abandoned.as_mut().poll(&mut cx).is_pending());
    }
    let calls = source.calls.get();
    assert_eq!(
        run(read_chunk(&host, &source, context(4), scope, chunk)),
        Err(ReadChunkError::Session(ReadSessionWorkflowError::Session(
            crate::model::service::read::session::ReadSessionError::Capacity
        )))
    );
    assert_eq!(source.calls.get(), calls);
    assert_eq!(
        host.0
            .borrow()
            .sessions
            .inspect(context(2))
            .unwrap()
            .usage
            .sessions,
        1
    );
}
