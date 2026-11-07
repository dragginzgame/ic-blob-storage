//! Controlled balance observations, not a production account or Cashier API.
use ic_platform::candid::contract::CandidType;
use ic_platform::principal::ExternalPrincipal;
use serde::Deserialize;

/// Independent local receiver shape for the maintained balance request encoder.
/// This fixture endpoint is a substitute, not a production Cashier implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, serde::Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct BalanceSourceRequest {
    /// Account selected by the sender's request, checked by the controlled source.
    pub account: ExternalPrincipal,
}

/// Independent receiver shape for a local Cashier relationship-query experiment.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize, serde::Serialize)]
#[candid_path("ic_platform::candid::contract")]
pub struct RelationshipSourceRequest {
    /// Usage owner selected by the encoded request, not the expected payer.
    pub canister: ExternalPrincipal,
}

/// Driver-installed raw response, independent of the production decoder schema.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct BalanceSourceConfig {
    /// Account the incoming request must explicitly name.
    pub account: ExternalPrincipal,
    /// Independent Candid fixture bytes; up to 4,097 for the oversize case.
    pub bytes: Vec<u8>,
    /// Reject instead of returning the bytes.
    pub reject: bool,
    /// Hold across actual IC rounds until explicitly resumed or bounded exhaustion.
    pub hold: bool,
}

/// Driver-only source scheduling evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, serde::Serialize)]
pub struct BalanceSourceView {
    /// Restored source cannot answer or resume.
    pub fenced: bool,
    /// Lifetime received calls, bounded to 64.
    pub requests: u64,
    /// Exact held account request, if any.
    pub pending: Option<ExternalPrincipal>,
}
