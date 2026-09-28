//! Host-supplied authenticated query transport; no IC update fallback.
pub mod replicated;
use super::CashierQueryRequest;
use candid::Principal;
use std::{future::Future, num::NonZeroUsize};

/// A transport-authenticated source and its bounded raw response.
/// Not an ingress DTO or proof object: implementing hosts establish authenticity
/// and correlate these bytes to the exact supplied request, without cached replies.
pub struct CashierQueryResponse {
    /// Actual authenticated response source, never a payload assertion.
    pub cashier: Principal,
    /// Encoded reply within the supplied byte limit.
    pub bytes: Vec<u8>,
}
/// Trusted host integration for maintained read-only queries.
/// Send the supplied canonical target, method and arguments once per invocation,
/// authenticate the response and bound buffering before returning bytes. Never
/// silently fall back to an update or attach a payment. A replicated read-only
/// implementation requires separate qualification. Local substitutes must be
/// labelled explicitly and establish no deployed-provider guarantees.
pub trait CashierQueryTransport {
    /// Host-owned transport failure; no automatic retry or cancellation follows.
    type Error;
    /// Execute exactly this query with the supplied response byte budget.
    fn query(
        &self,
        request: &CashierQueryRequest,
        max_bytes: NonZeroUsize,
    ) -> impl Future<Output = Result<CashierQueryResponse, Self::Error>>;
}
