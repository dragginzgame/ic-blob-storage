//! Explicit Canic composition. Linking exports no endpoints or lifecycle hooks.
//! The artifact owns declarations, lifecycle participants and endpoint selection;
//! all storage and tenant transitions remain in the independent service core.
pub mod arguments;
pub mod dto;
pub mod lifecycle;
pub mod memory;
use candid::{CandidType, Deserialize};

/// Managed transport rejection, before any blob handler or effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq, CandidType, Deserialize)]
pub enum ManagedCallFailure {
    /// The Component is not active under Canic's maintained activation authority.
    Inactive,
    /// The selected Fleet-admission projection does not admit the transport caller.
    Admission,
}

/// Explicitly emit named Canic transport guards in the owning artifact.
/// Canic remains that artifact's direct dependency, as required by its role
/// validator. Linking this library emits nothing. These functions export no IC
/// endpoints; the artifact must invoke them before shared handlers.
#[macro_export]
macro_rules! declare_contexts {
    (context = $context:ident, fleet_context = $fleet_context:ident $(,)?) => {
        /// Require correlated active runtime/activation and obtain actual caller bindings.
        /// This supplies no tenant, operator, verifier or provider authority.
        /// # Errors
        /// Refuses absent, Prepared, inconsistent or inactive managed authority.
        pub fn $context() -> Result<
            $crate::__macro_support::ic_blob_storage::model::service::upload::UploadContext,
            $crate::ManagedCallFailure,
        > {
            use ::canic::{
                api::canister::deployment::ComponentRuntimeApi,
                dto::{
                    component_registry::ComponentRuntimePhase,
                    fleet_activation::FleetActivationPhase,
                },
            };
            let runtime =
                ComponentRuntimeApi::status().map_err(|_| $crate::ManagedCallFailure::Inactive)?;
            if runtime.phase != ComponentRuntimePhase::Active {
                return Err($crate::ManagedCallFailure::Inactive);
            }
            let operation = ComponentRuntimeApi::operation_status(runtime.operation_id)
                .map_err(|_| $crate::ManagedCallFailure::Inactive)?;
            if operation.fleet_activation.phase != FleetActivationPhase::Active {
                return Err($crate::ManagedCallFailure::Inactive);
            }
            Ok(
                $crate::__macro_support::ic_blob_storage::model::service::upload::UploadContext {
                    service: ::ic_cdk::api::canister_self(),
                    actor: ::ic_cdk::api::msg_caller(),
                },
            )
        }
        /// Require activation and the selected compiled Fleet-admission projection.
        /// Shared service handlers must still enforce tenant/operator/verifier rules.
        /// # Errors
        /// Refuses inactive authority or absent, invalid, stale or fenced admission.
        pub fn $fleet_context() -> Result<
            $crate::__macro_support::ic_blob_storage::model::service::upload::UploadContext,
            $crate::ManagedCallFailure,
        > {
            let context = $context()?;
            ::canic::fleet_admission::require_caller()
                .map_err(|_| $crate::ManagedCallFailure::Admission)?;
            Ok(context)
        }
    };
}

/// Emit explicit installation/restoration operations in the owning Canic artifact.
/// Invoke these only from synchronous lifecycle participants, after Canic has
/// authenticated installation and checked its compiled release binding. The
/// artifact must publish only complete owners and trap failures for IC rollback.
/// Linking this library emits no lifecycle entrypoint or installation defaults.
#[macro_export]
macro_rules! declare_installation {
    (install = $install:ident, restore = $restore:ident $(,)?) => {
        /// Read the bounded managed input and install under validated Canic release authority.
        /// # Errors
        /// Refuses absent/inconsistent authority, invalid input or unavailable service grants.
        pub fn $install()
        -> Result<$crate::lifecycle::ManagedInstallation, $crate::lifecycle::LifecycleFailure> {
            let runtime = ::canic::api::canister::deployment::ComponentRuntimeApi::status()
                .map_err(|_| $crate::lifecycle::LifecycleFailure::ManagedAuthority)?;
            let operation =
                ::canic::api::canister::deployment::ComponentRuntimeApi::operation_status(
                    runtime.operation_id,
                )
                .map_err(|_| $crate::lifecycle::LifecycleFailure::ManagedAuthority)?;
            $crate::lifecycle::install(
                &operation
                    .fleet_activation
                    .identity
                    .release_build_id
                    .to_string(),
            )
        }
        /// Restore all service owners under Canic's validated same-release authority.
        /// # Errors
        /// Refuses replacement arguments, inconsistent authority or invalid retained state.
        pub fn $restore()
        -> Result<$crate::lifecycle::ManagedInstallation, $crate::lifecycle::LifecycleFailure> {
            let runtime = ::canic::api::canister::deployment::ComponentRuntimeApi::status()
                .map_err(|_| $crate::lifecycle::LifecycleFailure::ManagedAuthority)?;
            let operation =
                ::canic::api::canister::deployment::ComponentRuntimeApi::operation_status(
                    runtime.operation_id,
                )
                .map_err(|_| $crate::lifecycle::LifecycleFailure::ManagedAuthority)?;
            $crate::lifecycle::restore(
                &operation
                    .fleet_activation
                    .identity
                    .release_build_id
                    .to_string(),
            )
        }
    };
}

/// Support used only by explicitly invoked declaration macros.
#[doc(hidden)]
pub mod __macro_support {
    pub use ic_blob_storage;
    pub use ic_blob_storage::ic_memory;
}

/// Explicitly contribute shared installation/store requests before Canic bootstrap.
/// The artifact must separately grant an application range to the same authority.
/// Invoking this once registers metadata only; it opens no memory or endpoints.
#[macro_export]
macro_rules! declare_memories {
    (authority = $authority:expr $(,)?) => {
        const _: () = {
            fn register()
            -> Result<(), $crate::__macro_support::ic_memory::StaticMemoryDeclarationError> {
                for request in $crate::memory::requests($authority)? {
                    $crate::__macro_support::ic_memory::register_memory_request(request)?;
                }
                Ok(())
            }
            #[$crate::__macro_support::ic_memory::__reexports::ctor::ctor(
                                unsafe, anonymous,
                                crate_path = $crate::__macro_support::ic_memory::__reexports::ctor
                            )]
            fn defer() {
                $crate::__macro_support::ic_memory::defer_static_memory_registration(register);
            }
        };
    };
}
