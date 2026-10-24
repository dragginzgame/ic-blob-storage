//! Host-only metadata from the same declarations that mount service endpoints.
//! Schemas describe serialization, never authentication or storage guarantees.
//! Rust allocation budgets are exported as evidence; JS heap usage is not bounded
//! by Wire's traversal counter. Provider Candid methods remain separately classified.
use crate::endpoints::{MethodCodec, MethodMode, MethodSpec, SERVICE_METHODS, WireContract};
use ic_wire::typescript::{GenerationError, Wire, native_schema};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

macro_rules! projection {
    ($mode:ident, $category:ident, fn $name:ident() -> $reply:ty $body:block) => {
        fn $name() -> Result<MethodSchema, GenerationError> { method::<(), $reply>(stringify!($name)) }
    };
    (provider, $($rest:tt)*) => {};
    (certificate, $($rest:tt)*) => {};
    (update, $category:ident, async $($rest:tt)*) => { projection!(update, $category, $($rest)*); };
    ($mode:ident, tenant_inspection, fn $name:ident($arg:ident: $request:ty) -> $reply:ty $body:block) => {
        fn $name() -> Result<MethodSchema, GenerationError> {
            let _ = std::marker::PhantomData::<fn($request) -> $reply>;
            method::<super::tenant::Scope<'_>, Result<super::tenant::Observation<'_>, crate::dto::tenant::TenantFailure>>(stringify!($name))
        }
    };
    ($mode:ident, tenant_update, fn $name:ident($arg:ident: $request:ty $(,)?) -> $reply:ty $body:block) => {
        fn $name() -> Result<MethodSchema, GenerationError> {
            let _ = std::marker::PhantomData::<fn($request) -> $reply>;
            method::<super::tenant::UpdateRequest<'_>, Result<super::tenant::Observation<'_>, crate::dto::tenant::TenantFailure>>(stringify!($name))
        }
    };
    ($mode:ident, $category:ident, fn $name:ident($arg:ident: $request:ty $(,)?) -> $reply:ty $body:block) => {
        fn $name() -> Result<MethodSchema, GenerationError> {
            method::<$request, $reply>(stringify!($name))
        }
    };
}
crate::__blob_service_types!(crate);

crate::__blob_service_catalog!(
    dispatch = [crate::__emit_blob_service],
    emit = projection,
    workflow = unused,
    context = unused,
    contracts = crate,
    clock = unused,
    caller = unused,
    trap = unused,
);

macro_rules! collect_projection {
    ($out:ident; provider, $($rest:tt)*) => {};
    ($out:ident; certificate, $($rest:tt)*) => {};
    ($out:ident; update, $category:ident, async $($rest:tt)*) => { collect_projection!($out; query, $category, $($rest)*); };
    ($out:ident; query, $category:ident, fn $name:ident $($rest:tt)*) => { $out.push($name()?); };
}
macro_rules! collect_projections {
    ($emit:ident; $([$constant:ident, $id:tt; $($entry:tt)*])*) => {{
        let mut methods = Vec::new();
        $(collect_projection!(methods; $($entry)*);)*
        methods
    }};
}
/// One catalog-owned method and its exact codec projections for native hosts.
/// Consumers need not parse names or rediscover bounds from serialized JSON.
#[derive(Clone, Debug)]
pub struct MethodSchema {
    /// Canonical name, dispatch direction and codec.
    pub spec: MethodSpec,
    /// Canonical method identity and admission budgets.
    pub contract: WireContract,
    /// Request schema from the actual request codec.
    pub request: Value,
    /// Response schema from the actual response codec.
    pub response: Value,
}
impl MethodSchema {
    fn into_json(self) -> Value {
        let request = self.contract.request;
        let response = self.contract.response;
        json!({
            "name": self.spec.name,
            "mode": match self.spec.mode { MethodMode::Query => "query", MethodMode::Update => "update" },
            "id": self.contract.id.to_string(),
            "request": self.request, "response": self.response,
            "requestBytes": request.bytes, "responseBytes": response.bytes,
            "requestAllocation": request.allocation, "responseAllocation": response.allocation,
            "requestNodes": request.nodes, "responseNodes": response.nodes,
            "requestDepth": request.depth, "responseDepth": response.depth,
        })
    }
}
fn method<Q: Wire, R: Wire>(name: &str) -> Result<MethodSchema, GenerationError> {
    let spec = *SERVICE_METHODS
        .iter()
        .find(|s| s.name == name)
        .ok_or(GenerationError::Schema)?;
    Ok(MethodSchema {
        spec,
        contract: spec.wire().map_err(|_| GenerationError::Schema)?,
        request: native_schema::<Q>()?,
        response: native_schema::<R>()?,
    })
}
/// Enumerate every mounted owned Wire method, excluding provider methods and lifecycle hooks.
/// Use [`SERVICE_METHODS`] for the complete Wire/Candid classification.
/// # Errors
/// Rejects unsupported schemas, duplicate identities/names and missing projections.
pub fn projections() -> Result<Vec<MethodSchema>, GenerationError> {
    let methods = crate::__blob_service_catalog!(
        dispatch = [collect_projections],
        emit = unused,
        workflow = unused,
        context = unused,
        contracts = crate,
        clock = unused,
        caller = unused,
        trap = unused,
    );
    validate(&methods)?;
    Ok(methods)
}
fn validate(methods: &[MethodSchema]) -> Result<(), GenerationError> {
    let mut names = BTreeSet::new();
    let mut ids = BTreeSet::new();
    for method in methods {
        if !names.insert(method.spec.name) || !ids.insert(method.contract.id) {
            return Err(GenerationError::Schema);
        }
    }
    let expected: BTreeSet<_> = SERVICE_METHODS
        .iter()
        .filter(|m| m.codec == MethodCodec::IcWire)
        .map(|m| m.name)
        .collect();
    if names != expected {
        return Err(GenerationError::Schema);
    }
    Ok(())
}
/// Serialize the canonical projections for the Wire registry and retained artifacts.
/// This preserves the existing JSON layout; native hosts can use [`projections`].
/// # Errors
/// Rejects unsupported or inconsistent method projections.
pub fn methods() -> Result<Vec<Value>, GenerationError> {
    Ok(projections()?
        .into_iter()
        .map(MethodSchema::into_json)
        .collect())
}
/// Generate TypeScript codecs. `runtime_module` must export the Blob scalar-aware
/// `createCodec` produced by [`runtime_adapter`], not the bare Wire runtime.
/// # Errors
/// Refuses incomplete/invalid contracts or unsupported metadata.
pub fn typescript(runtime_module: &str) -> Result<String, GenerationError> {
    ic_wire::typescript::generate_registry(&BTreeMap::new(), &methods()?, runtime_module)
}
/// Blob scalar validation adapter for the unmodified published Wire runtime.
/// Write [`ic_wire::typescript::RUNTIME`] at `wire_runtime_module`.
/// Rust allocation budgets do not imply JavaScript heap guarantees.
#[must_use]
pub fn runtime_adapter(wire_runtime_module: &str) -> String {
    format!(
        "import {{ createCodec as wireCodec, WireError }} from {};\n{}",
        json!(wire_runtime_module),
        include_str!("decimal.ts")
    )
}

#[cfg(test)]
mod tests;
