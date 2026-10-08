//! Service models and records; no implicit installation or lifecycle authority.

pub mod installation;
pub mod read;
pub mod recovery;
pub mod tenant;
pub mod upload;

#[cfg(test)]
mod configuration_tests;
