//! Pure decisions over supplied domain values; no storage or platform access.

pub mod billing;
pub mod catalog;
pub mod diagnostics;
pub mod gateway;
pub mod installation;
#[cfg(any(test, feature = "test-support"))]
pub mod liveness;
pub mod recovery;
pub mod tenant;
pub mod upload;
