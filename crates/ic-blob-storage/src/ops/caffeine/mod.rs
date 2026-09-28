//! Caffeine wire encoding/decoding; no transport, credentials or provider effects.
//!
//! Wire types have one private owner here. Decoded replies are observations,
//! not authority to create a confirmed object, settle a payment or retry it.
//! Callers must bind transport responses to their persisted operation and provider.

pub mod audit;
pub mod balance;
pub mod download;
pub mod funding;
pub mod gateway;
pub mod ledger;
pub mod query;
pub mod relationship;
pub mod upload;

mod wire;
