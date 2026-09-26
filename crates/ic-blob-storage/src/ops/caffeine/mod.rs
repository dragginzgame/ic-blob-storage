//! Caffeine response decoding; no requests, credentials or provider effects.
//!
//! Wire types have one private owner here. Decoded replies are observations,
//! not authority to create a confirmed object, settle a payment or retry it.
//! Callers must bind transport responses to their persisted operation and provider.

pub mod funding;
pub mod upload;
