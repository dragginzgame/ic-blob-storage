//! Canonical service declarations and callable surface shared by host adapters.
//! Hosts own authentication, lifecycle and endpoint registration. Blob-owned
//! methods declare their codec; the Caffeine certificate remains Candid.

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
    /// Blob-owned exact CBOR value.
    Cbor,
    /// Bounded, method- and direction-bound IC Wire contract.
    IcWire,
    /// Provider-defined Caffeine certificate contract.
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
macro_rules! method_spec {
    (query, tenant_inspection, $($rest:tt)*) => {
        MethodSpec { name: crate::protocol::TENANT_INSPECTION_METHOD, mode: MethodMode::Query, codec: MethodCodec::IcWire }
    };
    (update, tenant_update, $($rest:tt)*) => {
        MethodSpec { name: crate::protocol::TENANT_UPDATE_METHOD, mode: MethodMode::Update, codec: MethodCodec::IcWire }
    };
    (query, $category:ident, fn $name:ident $($rest:tt)*) => {
        MethodSpec { name: stringify!($name), mode: MethodMode::Query, codec: MethodCodec::Cbor }
    };
    (update, $category:ident, $($rest:tt)*) => { method_spec!(@update $($rest)*) };
    (@update async fn $name:ident $($rest:tt)*) => {
        MethodSpec { name: stringify!($name), mode: MethodMode::Update, codec: MethodCodec::Cbor }
    };
    (certificate, $category:ident, $name:literal, $($rest:tt)*) => {
        MethodSpec { name: $name, mode: MethodMode::Update, codec: MethodCodec::Candid }
    };
}
macro_rules! method_specs {
    ($emit:ident; $([$($entry:tt)*])*) => { &[$(method_spec!($($entry)*)),*] };
}
/// Complete callable service surface; linking this value exports no endpoint.
pub const SERVICE_METHODS: &[MethodSpec] = crate::__blob_service_catalog!(
    dispatch = method_specs,
    emit = unused,
    workflow = unused,
    context = unused,
    contracts = crate,
    clock = unused,
    caller = unused,
    trap = unused,
);

/// Mount the canonical methods through a host's per-method bounded endpoint emitter.
/// The provider certificate entry uses its separate Candid emitter. The context
/// must capture the actual caller/canister; clock, caller and trap paths must be
/// trusted platform adapters. Linking contracts exports no lifecycle or endpoints.
#[macro_export]
macro_rules! mount_service_endpoints {
    (emit = $emit:ident, workflow = $workflow:ident, context = $context:ident, contracts = $contracts:ident,
     clock = $clock:path, caller = $caller:path, trap = $trap:path $(,)?) => {
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
        pub(super) use $contracts::upload::binding::UploadContext;
        use $crate::__emit_blob_service;
        $crate::__blob_service_catalog!(
            dispatch = __emit_blob_service,
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
#[doc(hidden)]
#[macro_export]
macro_rules! __emit_blob_service {
    ($emit:ident; $([$($entry:tt)*])*) => { $($emit! { $($entry)* })* };
}
#[doc(hidden)]
#[macro_export]
macro_rules! __blob_service_catalog {
    (dispatch = $dispatch:ident, emit = $emit:ident, workflow = $workflow:ident, context = $context:ident,
     contracts = $contracts:ident, clock = $clock:path, caller = $caller:path, trap = $trap:path $(,)?) => {
        $dispatch! { $emit;
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
            [update, request,
            async fn blob_attest_upload(
                input: $contracts::dto::upload::completion::UploadAttestationRequest,
            ) -> Result<
                $contracts::dto::upload::completion::UploadAttestationMutation,
                $contracts::dto::upload::completion::UploadAttestationFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::attest(call_context, &input)
            }]
            [query, request,
            fn blob_upload_attestation(
                input: UploadAdmissionRequest,
            ) -> Result<
                $contracts::dto::upload::completion::UploadAttestationResponse,
                $contracts::dto::upload::completion::UploadAttestationFailure,
            > {
                $workflow::attestation($context(), input)
            }]
            [query, request,
            fn blob_verification_manifest(
                input: UploadAdmissionRequest,
            ) -> Result<UploadManifestResponse, UploadManifestFailure> {
                $workflow::verification_manifest($context(), input)
            }]
            [query, request,
            fn blob_verification_plan(
                input: UploadAdmissionRequest,
            ) -> Result<
                $contracts::dto::upload::completion::UploadVerificationPlan,
                $contracts::dto::upload::completion::UploadAttestationFailure,
            > {
                $workflow::verification_plan($context(), input)
            }]
        }
    };
}
