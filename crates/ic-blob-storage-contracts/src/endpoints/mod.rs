//! Canonical service declarations and callable surface shared by host adapters.
//! Hosts own authentication, lifecycle and endpoint registration. Blob-owned
//! methods declare their codec; external Caffeine provider methods remain Candid.

/// Platform dispatch mode of a maintained service method.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MethodMode {
    /// Read-only platform query.
    Query,
    /// Replicated platform update.
    Update,
}
/// Codec required by this method's external contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MethodCodec {
    /// Bounded, method- and direction-bound IC Wire contract.
    IcWire,
    /// Provider-defined external Caffeine contract.
    Candid,
}
/// Passive callable metadata emitted from the same catalog as host endpoints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MethodSpec {
    /// Exact platform method name.
    pub name: &'static str,
    /// Query or update dispatch.
    pub mode: MethodMode,
    /// Required application codec.
    pub codec: MethodCodec,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __blob_method_spec {
    (provider, query, $category:ident, $name:literal, $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: $name, mode: $crate::endpoints::MethodMode::Query, codec: $crate::endpoints::MethodCodec::Candid }
    };
    (provider, update, $category:ident, $name:literal, $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: $name, mode: $crate::endpoints::MethodMode::Update, codec: $crate::endpoints::MethodCodec::Candid }
    };
    (query, tenant_inspection, $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: $crate::protocol::TENANT_INSPECTION_METHOD, mode: $crate::endpoints::MethodMode::Query, codec: $crate::endpoints::MethodCodec::IcWire }
    };
    (update, tenant_update, $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: $crate::protocol::TENANT_UPDATE_METHOD, mode: $crate::endpoints::MethodMode::Update, codec: $crate::endpoints::MethodCodec::IcWire }
    };
    (query, $category:ident, fn $name:ident $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: stringify!($name), mode: $crate::endpoints::MethodMode::Query, codec: $crate::endpoints::MethodCodec::IcWire }
    };
    (update, $category:ident, $($rest:tt)*) => { $crate::__blob_method_spec!(@update $($rest)*) };
    (@update async fn $name:ident $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: stringify!($name), mode: $crate::endpoints::MethodMode::Update, codec: $crate::endpoints::MethodCodec::IcWire }
    };
    (certificate, $category:ident, $name:literal, $($rest:tt)*) => {
        $crate::endpoints::MethodSpec { name: $name, mode: $crate::endpoints::MethodMode::Update, codec: $crate::endpoints::MethodCodec::Candid }
    };
}
macro_rules! method_specs {
    ($emit:ident; $([$($entry:tt)*])*) => { &[$( $crate::__blob_method_spec!($($entry)*)),*] };
}
/// Complete callable service surface; linking this value exports no endpoint.
pub const SERVICE_METHODS: &[MethodSpec] = crate::__blob_service_catalog!(
    dispatch = [method_specs],
    emit = unused,
    workflow = unused,
    context = unused,
    contracts = crate,
    clock = unused,
    caller = unused,
    trap = unused,
);

/// Mount the canonical methods through a host's per-method bounded endpoint emitter.
/// Provider entries use their separate bounded Candid emitters. The context
/// must capture the actual caller/canister; clock, caller and trap paths must be
/// trusted platform adapters. Linking contracts exports no lifecycle or endpoints.
/// Optional `describe = callback` follows `emit` and runs before endpoint emission.
/// It receives `(IcWire | Candid, rust_identifier, MethodSpec_expression)` for each
/// declaration. Use the expression in a const/static; `MethodSpec::wire` supplies
/// complete Wire policy. The callback owns artifact records and their format;
/// provider entries must remain external Candid. Existing emitters are unchanged.
#[macro_export]
macro_rules! mount_service_endpoints {
    (emit = $emit:ident, $(describe = $describe:ident,)? workflow = $workflow:ident, context = $context:ident, contracts = $contracts:ident,
     clock = $clock:path, caller = $caller:path, trap = $trap:path $(,)?) => {
        $(
            $crate::__blob_service_catalog!(
                dispatch = [$crate::__describe_blob_service], emit = $describe,
                workflow = $workflow, context = $context, contracts = $contracts,
                clock = $clock, caller = $caller, trap = $trap,
            );
        )?
        $crate::__blob_service_types!($contracts);
        pub(super) use $contracts::upload::binding::UploadContext;
        $crate::__blob_service_catalog!(
            dispatch = [$crate::__emit_blob_service],
            emit = $emit,
            workflow = $workflow,
            context = $context,
            contracts = $contracts,
            clock = $clock,
            caller = $caller,
            trap = $trap,
        );
    };
}
/// Complete Wire framing policy, usable by a host's static artifact declaration.
/// The provider's external Candid contract has no value of this type.
#[derive(Clone, Copy, Debug)]
pub struct WireContract {
    /// Stable method identity shared with the runtime codec.
    pub id: u64,
    /// Complete request admission limits, including the frame header.
    pub request: ic_wire::Limits,
    /// Complete response admission limits, including the frame header.
    pub response: ic_wire::Limits,
}
impl MethodSpec {
    /// Obtain the mounted Wire contract without enabling schema generation.
    /// # Errors
    /// External provider methods and unknown identities have no owned Wire contract.
    pub const fn wire(self) -> Result<WireContract, crate::wire::Error> {
        if matches!(self.codec, MethodCodec::Candid) {
            return Err(crate::wire::Error::Identity);
        }
        let id = match crate::wire::identity(self.name) {
            Ok(id) => id,
            Err(e) => return Err(e),
        };
        let request = match crate::wire::request_limits(self.name) {
            Ok(limits) => limits,
            Err(e) => return Err(e),
        };
        let response = match crate::wire::response_limits(self.name) {
            Ok(limits) => limits,
            Err(e) => return Err(e),
        };
        Ok(WireContract {
            id,
            request,
            response,
        })
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __describe_blob_service {
    ($emit:ident; $([$($entry:tt)*])*) => {
        $($crate::__describe_blob_method! { $emit; [$($entry)*]; $($entry)* })*
    };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __describe_blob_method {
    ($emit:ident; [$($original:tt)*]; provider, $mode:ident, $category:ident, $name:literal, $($rest:tt)*) => {
        $crate::__describe_blob_method! { $emit; [$($original)*]; @signature Candid; $($rest)* }
    };
    ($emit:ident; [$($original:tt)*]; certificate, $category:ident, $name:literal, $($rest:tt)*) => {
        $crate::__describe_blob_method! { $emit; [$($original)*]; @signature Candid; $($rest)* }
    };
    ($emit:ident; [$($original:tt)*]; $mode:ident, $category:ident, $($rest:tt)*) => {
        $crate::__describe_blob_method! { $emit; [$($original)*]; @signature IcWire; $($rest)* }
    };
    ($emit:ident; [$($original:tt)*]; @signature $codec:ident; async $($rest:tt)*) => {
        $crate::__describe_blob_method! { $emit; [$($original)*]; @signature $codec; $($rest)* }
    };
    ($emit:ident; [$($original:tt)*]; @signature $codec:ident; fn $name:ident $($rest:tt)*) => {
        $emit! { $codec, $name, $crate::__blob_method_spec!($($original)*) }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __emit_blob_service {
    ($emit:ident; $([$($entry:tt)*])*) => { $($emit! { $($entry)* })* };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __blob_service_catalog {
    (dispatch = [$($dispatch:tt)*], emit = $emit:ident, workflow = $workflow:ident, context = $context:ident,
     contracts = $contracts:ident, clock = $clock:path, caller = $caller:path, trap = $trap:path $(,)?) => {
        $($dispatch)*! { $emit;
            [update, request,
            async fn blob_resume_current_instance()
            -> Result<(), $contracts::dto::recovery::CurrentInstanceRecoveryFailure> {
                $workflow::resume_current_instance($context()).await
            }]
            [query, request,
            fn blob_configuration() -> Result<HostConfigurationView, HostFailure> {
                $workflow::configuration($caller())
            }]
            [query, request,
            fn blob_local_status(
                input: OperatorScope,
            ) -> Result<LocalServiceStatus, LocalStatusFailure> {
                $workflow::local_status($context(), input)
            }]
            [update, tenant_update,
            async fn blob_update_tenant(
                input: TenantUpdateRequest,
            ) -> Result<TenantEnrollmentResponse, TenantFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::update_tenant(call_context, input)
            }]
            [query, tenant_inspection,
            fn blob_tenant(input: TenantScope) -> Result<TenantEnrollmentResponse, TenantFailure> {
                $workflow::tenant($context(), input)
            }]
            [update, request,
            async fn blob_admit_upload(
                input: UploadAdmissionRequest,
            ) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::admit(call_context, input, $clock())
            }]
            [query, request,
            fn blob_upload_admission(
                input: UploadAdmissionRequest,
            ) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
                $workflow::admission($context(), input)
            }]
            [query, request,
            fn blob_upload_status(
                input: $contracts::dto::reference::ReferenceUpload,
            ) -> Result<
                $contracts::dto::upload::UploadStatusResponse,
                $contracts::dto::upload::UploadStatusFailure,
            > {
                $workflow::upload_status($context(), input)
            }]
            [update, request,
            async fn blob_revoke_upload(
                input: UploadAdmissionRequest,
            ) -> Result<UploadRevocationResponse, UploadAdmissionFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::revoke(call_context, input)
            }]
            [update, manifest,
            async fn blob_prepare_upload(
                input: UploadManifestRequest,
            ) -> Result<UploadManifestMutation, UploadManifestFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::prepare(call_context, &input, $clock())
            }]
            [query, request,
            fn blob_upload_certificate_assessment(
                root: String,
            ) -> Result<
                $contracts::dto::upload::certificate::UploadCertificateAssessmentResponse,
                $contracts::dto::upload::exposure::UploadExposureFailure,
            > {
                $workflow::certificate_assessment($context(), &root, $clock())
            }]
            [provider, query, roots, "_immutableObjectStorageBlobsAreLive",
            fn caffeine_blobs_are_live(roots: Vec<Vec<u8>>) -> Vec<bool> {
                $workflow::provider_liveness($context(), &roots)
                    .unwrap_or_else(|_| $trap("provider liveness refused"))
            }]
            [provider, query, empty, "_immutableObjectStorageBlobsToDelete",
            fn caffeine_blobs_to_delete() -> Vec<Vec<u8>> {
                $workflow::provider_deletions($context())
                    .unwrap_or_else(|_| $trap("provider deletion list refused"))
            }]
            [provider, update, roots, "_immutableObjectStorageConfirmBlobDeletion",
            async fn caffeine_confirm_blob_deletion(roots: Vec<Vec<u8>>) {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::acknowledge_provider_deletions(call_context, &roots)
                    .unwrap_or_else(|_| $trap("provider acknowledgment refused"))
            }]
            [certificate, request, "_immutableObjectStorageCreateCertificate",
            async fn caffeine_upload_certificate(
                root: String,
            ) -> $contracts::dto::upload::certificate::CaffeineUploadCertificateResponse {
                let call_context = $context();
                $workflow::before_update().await;
                // Never encode an error as a successful provider reply. Shared workflow
                // rechecks current authority/evidence and commits exposure synchronously.
                $workflow::certificate(call_context, &root, $clock())
                    .unwrap_or_else(|_| $trap("certificate issuance refused"))
            }]
            [query, request,
            fn blob_upload_manifest(
                input: UploadAdmissionRequest,
            ) -> Result<UploadManifestResponse, UploadManifestFailure> {
                $workflow::manifest($context(), input)
            }]
            [update, request,
            async fn blob_apply_reference(
                input: ReferenceCommand,
            ) -> Result<ReferenceMutationResponse, ReferenceFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::reference(call_context, input)
            }]
            [query, request,
            fn blob_reference_receipt(
                input: ReferenceCommand,
            ) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
                $workflow::receipt($context(), input)
            }]
            [query, request,
            fn blob_upload_history(
                input: UploadHistoryRequest,
            ) -> Result<UploadHistoryPage, UploadHistoryFailure> {
                $workflow::history($context(), input)
            }]
            [query, request,
            fn blob_funding_history(
                input: FundingHistoryRequest,
            ) -> Result<FundingHistoryPage, FundingHistoryFailure> {
                $workflow::funding_history($context(), input)
            }]
            [query, request,
            fn blob_funding_outcome(
                input: FundingOutcomeRequest,
            ) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
                $workflow::funding_outcome($context(), input)
            }]
            [query, request,
            fn blob_funding_preparation_assessment(
                input: FundingPreparationRequest,
            ) -> Result<FundingPreparationResponse, FundingPreparationFailure> {
                $workflow::funding_assessment($context(), input)
            }]
            [update, request,
            async fn blob_revoke_gateway(
                input: $contracts::dto::gateway::GatewayRevocationRequest,
            ) -> Result<
                $contracts::dto::gateway::GatewayRevocationResponse,
                $contracts::dto::gateway::GatewayRevocationFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::revoke_gateway(call_context, input)
            }]
            [update, request,
            async fn blob_sync_gateways(
                input: OperatorScope,
            ) -> Result<
                $contracts::dto::gateway::sync::GatewaySyncResponse,
                $contracts::dto::gateway::sync::GatewaySyncFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::sync_gateways(call_context, input).await
            }]
            [update, request,
            async fn blob_cancel_gateway_sync(
                input: $contracts::dto::gateway::sync::GatewaySyncCancellation,
            ) -> Result<(), $contracts::dto::gateway::sync::GatewaySyncFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::cancel_gateway_sync(call_context, input)
            }]
            [query, request,
            fn blob_upload_capacity(
                input: $contracts::dto::tenant::TenantScope,
            ) -> Result<
                $contracts::dto::upload::capacity::UploadCapacityResponse,
                $contracts::dto::upload::capacity::UploadCapacityFailure,
            > {
                $workflow::upload_capacity($context(), input)
            }]
            [query, request,
            fn blob_reference_capacity(
                input: $contracts::dto::reference::capacity::ReferenceCapacityRequest,
            ) -> Result<
                $contracts::dto::reference::capacity::ReferenceCapacityResponse,
                $contracts::dto::reference::capacity::ReferenceCapacityFailure,
            > {
                $workflow::reference_capacity($context(), input)
            }]
            [query, request,
            fn blob_lookup_content(
                input: $contracts::dto::upload::discovery::UploadDiscoveryRequest,
            ) -> Result<
                $contracts::dto::upload::discovery::UploadDiscoveryResponse,
                $contracts::dto::upload::discovery::UploadDiscoveryFailure,
            > {
                $workflow::discover($context(), input)
            }]
            [update, request,
            async fn blob_download_descriptor(
                input: $contracts::dto::download::DownloadRequest,
            ) -> Result<
                $contracts::dto::download::DownloadResponse,
                $contracts::dto::download::DownloadFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::download(call_context, input)
            }]
            [query, request,
            fn blob_reference_status(
                input: $contracts::dto::reference::status::ReferenceStatusRequest,
            ) -> Result<
                $contracts::dto::reference::status::ReferenceStatusResponse,
                $contracts::dto::reference::ReferenceFailure,
            > {
                $workflow::reference_status($context(), input)
            }]
            [update, request,
            async fn blob_inspect_account(
                input: $contracts::dto::account::AccountInspectionRequest,
            ) -> Result<
                $contracts::dto::account::AccountInspectionResponse,
                $contracts::dto::account::AccountInspectionFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::inspect_account(call_context, input).await
            }]
        }
    };
}

// Shared lexical names for the declaration catalog's endpoint and schema emitters.
#[doc(hidden)]
#[macro_export]
macro_rules! __blob_service_types {
    ($contracts:ident) => {
        pub(super) use $contracts::dto::funding::assessment::{
            FundingPreparationFailure, FundingPreparationRequest, FundingPreparationResponse,
        };
        pub(super) use $contracts::dto::funding::outcome::{
            FundingOutcomeFailure, FundingOutcomeRequest, FundingOutcomeResponse,
        };
        pub(super) use $contracts::dto::funding::{
            FundingHistoryFailure, FundingHistoryPage, FundingHistoryRequest,
        };
        pub(super) use $contracts::dto::operator::{
            LocalServiceStatus, LocalStatusFailure, OperatorScope,
        };
        pub(super) use $contracts::dto::upload::history::{
            UploadHistoryFailure, UploadHistoryPage, UploadHistoryRequest,
        };
        pub(super) use $contracts::dto::{
            configuration::{HostConfigurationView, HostFailure},
            reference::{
                ReferenceCommand, ReferenceFailure, ReferenceMutationResponse,
                ReferenceReceiptLookup,
            },
            tenant::{TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest},
            upload::{
                admission::{
                    UploadAdmissionFailure, UploadAdmissionMutation, UploadAdmissionRequest,
                    UploadAdmissionResponse, UploadRevocationResponse,
                },
                manifest::{
                    UploadManifestFailure, UploadManifestMutation, UploadManifestRequest,
                    UploadManifestResponse,
                },
            },
        };
    };
}
