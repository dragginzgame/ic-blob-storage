//! One authenticated native query; callers own agents, budgets and evidence order.
use super::{Failure, artifacts::Run};
use candid::Principal;
use ic_agent::Agent;
use serde_json::{Value, json};
use std::future::Future;
use std::time::Duration;

/// Use the already configured signer/root without retry or a mutation fallback.
pub(super) async fn with_agent(
    agent: &Agent,
    service: Principal,
    method: &str,
    argument: Vec<u8>,
) -> Result<Vec<u8>, Failure> {
    tokio::time::timeout(
        Duration::from_secs(30),
        agent.query(&service, method).with_arg(argument).call(),
    )
    .await
    .map_err(|_| Failure::Timeout)?
    .map_err(|_| Failure::Transport)
}

/// Persist caller-owned evidence before transport, retaining raw replies before decoding.
pub(super) async fn recorded<Q, F>(
    run: &Run,
    label: &str,
    args: Vec<u8>,
    intent: &Value,
    query: Q,
) -> Result<Vec<u8>, Failure>
where
    Q: FnOnce(Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    run.bytes(&format!("{label}-args.cbor"), &args)?;
    run.json(&format!("{label}-intent.json"), intent)?;
    let bytes = query(args).await?;
    run.bytes(&format!("{label}-reply.cbor"), &bytes)?;
    Ok(bytes)
}

/// Keep numbered publication observations in their original evidence format.
pub(in crate::native) async fn observe<Q, F>(
    run: &Run,
    index: usize,
    method: &'static str,
    args: Vec<u8>,
    query: &mut Q,
) -> Result<Vec<u8>, Failure>
where
    Q: FnMut(&'static str, Vec<u8>) -> F,
    F: Future<Output = Result<Vec<u8>, Failure>>,
{
    let intent = json!({"method":method,"argument_sha256":super::upload_inputs::digest(&args),"updates":0,"automatic_retries":0});
    recorded(run, &format!("query-{index:04}"), args, &intent, |args| {
        query(method, args)
    })
    .await
}
