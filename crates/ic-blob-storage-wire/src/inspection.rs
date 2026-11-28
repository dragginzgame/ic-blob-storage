//! Wire schema projections of the shared Blob request and reply types.

use crate::projection::Project;
use ic_wire::typescript::{GenerationError, SchemaContext, Wire};
use std::marker::PhantomData;

/// Schema of the fixed Serde body of a Blob protocol type.
/// Use `ic_wire::Message<Schema<T>>` for a complete sparse frame root.
pub struct Schema<T>(PhantomData<T>);

impl<T: Project> Wire for Schema<T> {
    fn schema(context: &mut SchemaContext) -> Result<serde_json::Value, GenerationError> {
        T::Schema::schema(context)
    }
}

/// Fixed request schema for a typed Blob contract.
pub type RequestSchema<C> = Schema<<C as crate::Contract>::Request>;

/// Fixed response schema for a typed Blob contract.
pub type ResponseSchema<C> = Schema<<C as crate::Contract>::Response>;

#[cfg(test)]
mod tests {
    use super::{RequestSchema, ResponseSchema};
    use crate::{Contract, Management, Query, Update};
    use ic_wire::{
        Message,
        typescript::{Method, PayloadLimits, SchemaRegistry, Wire},
    };

    fn check<C: Contract>()
    where
        RequestSchema<C>: Wire,
        ResponseSchema<C>: Wire,
    {
        let mut registry = SchemaRegistry::new(Default::default()).unwrap();
        registry
            .register::<Message<RequestSchema<C>>>("Request")
            .unwrap();
        registry
            .register::<Message<ResponseSchema<C>>>("Reply")
            .unwrap();
        let graph = registry.finish().unwrap();
        let payload = |limits: ic_wire::Limits| PayloadLimits {
            // IC Wire's fixed header includes the method ID and direction.
            bytes: (limits.bytes - 11) as u64,
            nodes: limits.nodes as u64,
            depth: limits.depth as u64,
        };
        let source = graph
            .method_module(
                &[Method {
                    export: "codec",
                    id: C::ID,
                    request: "Request",
                    response: "Reply",
                    request_limits: payload(C::REQUEST_LIMITS),
                    response_limits: payload(C::RESPONSE_LIMITS),
                }],
                "./runtime.js",
                &Default::default(),
                |_| {},
            )
            .unwrap();
        assert!(source.contains("export const codec"));
    }

    #[test]
    fn all_blob_methods_generate_client_codecs() {
        check::<Update>();
        check::<Query>();
        check::<Management>();
    }
}
