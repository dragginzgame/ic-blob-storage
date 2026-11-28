//! Blob Storage domain operations and local Caffeine content verification.
//!
//! The experimental local model composes grants, application references and
//! declared-byte accounting with the manifest check. Streaming verification
//! checks actual bytes against an independently trusted declaration.
//! The bounded durable service reuses that model with a host-granted Storage
//! lease. Hosts capture callers and compose memory/lifecycle hooks. The standalone
//! host exports experimental Wire methods, without provider authority or transfers.

#![forbid(unsafe_code)]

pub mod caffeine;
pub mod local;
#[cfg(feature = "metrics")]
pub mod metrics;
pub mod protocol;
pub mod references;

#[cfg(feature = "inspection")]
mod projection;
#[cfg(feature = "inspection")]
mod inspection {
    include!(concat!(env!("OUT_DIR"), "/inspection.rs"));
}

#[cfg(all(test, feature = "inspection"))]
mod inspection_tests {
    use super::{
        local::Error,
        protocol::{Command, Inspection, ManagementView, Observation, Outcome},
    };
    use ic_wire::typescript::{SchemaRegistry, Wire};

    fn check<Request: Wire, Reply: Wire>() {
        let mut registry = SchemaRegistry::new(Default::default()).unwrap();
        registry
            .register::<ic_wire::Message<Request>>("Request")
            .unwrap();
        registry
            .register::<ic_wire::Message<Reply>>("Reply")
            .unwrap();
        registry.finish().unwrap();
    }

    #[test]
    fn mounted_request_and_reply_types_register_without_overrides() {
        check::<Command, Result<Outcome, Error>>();
        check::<Inspection, Result<Observation, Error>>();
        check::<(), Result<ManagementView, Error>>();
    }
}
