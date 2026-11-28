//! Typed contracts shared by Blob mounts and callers.

use ic_blob_storage::{
    local::Error,
    protocol::{Command, Inspection, ManagementView, Observation, Outcome},
};
use ic_wire::{Direction, Encoding, Limits};
use serde::{Serialize, de::DeserializeOwned};

use crate::{RPC_LIMITS, frame};

/// A Blob request and reply carried under one method ID.
pub trait Contract {
    /// Request body.
    type Request: Serialize + DeserializeOwned;
    /// Reply body.
    type Response: Serialize + DeserializeOwned;

    /// Standalone export name and input to the Wire method ID.
    const NAME: &'static str;
    /// Routing identity for both frame directions.
    const ID: u64 = ic_wire::method_id(Self::NAME);
    /// Complete request frame admission.
    const REQUEST_LIMITS: Limits = RPC_LIMITS;
    /// Complete reply frame admission.
    const RESPONSE_LIMITS: Limits = RPC_LIMITS;

    /// Encode a request for this method.
    /// # Errors
    /// Propagates Wire serialization and admission errors.
    fn encode_request(value: &Self::Request) -> Result<Vec<u8>, ic_wire::Error> {
        frame::encode(
            value,
            Encoding::Frame {
                method: Self::ID,
                direction: Direction::Request,
            },
            Self::REQUEST_LIMITS,
        )
    }

    /// Decode this method's request.
    /// # Errors
    /// Propagates Wire framing and admission errors.
    fn decode_request(bytes: &[u8]) -> Result<Self::Request, ic_wire::Error> {
        frame::decode(
            bytes,
            Encoding::Frame {
                method: Self::ID,
                direction: Direction::Request,
            },
            Self::REQUEST_LIMITS,
        )
    }

    /// Encode a reply for this method.
    /// # Errors
    /// Propagates Wire serialization and admission errors.
    fn encode_response(value: &Self::Response) -> Result<Vec<u8>, ic_wire::Error> {
        frame::encode(
            value,
            Encoding::Frame {
                method: Self::ID,
                direction: Direction::Response,
            },
            Self::RESPONSE_LIMITS,
        )
    }

    /// Decode this method's reply.
    /// # Errors
    /// Propagates Wire framing and admission errors.
    fn decode_response(bytes: &[u8]) -> Result<Self::Response, ic_wire::Error> {
        frame::decode(
            bytes,
            Encoding::Frame {
                method: Self::ID,
                direction: Direction::Response,
            },
            Self::RESPONSE_LIMITS,
        )
    }
}

/// Captured-caller mutation.
pub struct Update;
impl Contract for Update {
    type Request = Command;
    type Response = Result<Outcome, Error>;
    const NAME: &'static str = "blob_update";
}

/// Caller-authorized query.
pub struct Query;
impl Contract for Query {
    type Request = Inspection;
    type Response = Result<Observation, Error>;
    const NAME: &'static str = "blob_query";
}

/// Replicated manager inspection.
pub struct Management;
impl Contract for Management {
    type Request = ();
    type Response = Result<ManagementView, Error>;
    const NAME: &'static str = "blob_management";
}
