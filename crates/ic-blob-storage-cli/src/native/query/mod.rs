//! One authenticated native query; callers own agents, budgets and evidence order.
use super::Failure;
use candid::Principal;
use ic_agent::Agent;
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
