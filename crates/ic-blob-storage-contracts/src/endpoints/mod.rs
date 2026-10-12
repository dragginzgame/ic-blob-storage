//! Opt-in service declarations shared by standalone and framework hosts.
//!
//! This module owns method names, DTO signatures and immediate workflow delegation.
//! Hosts supply endpoint attributes, authenticated runtime context, clock and workflow
//! adapters. Linking contracts exports no IC methods or lifecycle participant.

/// Emit the maintained Blob service methods through a host-owned item callback.
///
/// The callback receives `query` or `update`, the `request`/`manifest` decoding
/// category and a function item. The provider certificate additionally supplies
/// its exact wire name. The host must attach its bounded decoding/authentication
/// attributes. Its context must capture the actual caller and canister; clock,
/// caller and trap paths must be trusted platform adapters. Workflow adapters
/// retain continuity checks, state borrowing and post-await observation.
/// Invoke inside the host endpoint module and pass its visible contract-crate
/// alias. Passive type imports are also available to the enclosing Candid scope.
/// This invocation does not install storage, allocate memory or export lifecycle.
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
        $emit! { update, request,
            async fn blob_resume_current_instance()
            -> Result<(), $contracts::dto::recovery::CurrentInstanceRecoveryFailure> {
                $workflow::resume_current_instance($context()).await
            }
        }
        $emit! { query, request,
            fn blob_configuration() -> Result<HostConfigurationView, HostFailure> {
                $workflow::configuration($caller())
            }
        }
        $emit! { query, request,
            fn blob_local_status(
                input: OperatorScope,
            ) -> Result<LocalServiceStatus, LocalStatusFailure> {
                $workflow::local_status($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_update_tenant(
                input: TenantUpdateRequest,
            ) -> Result<TenantEnrollmentResponse, TenantFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::update_tenant(call_context, input)
            }
        }
        $emit! { query, request,
            fn blob_tenant(input: TenantScope) -> Result<TenantEnrollmentResponse, TenantFailure> {
                $workflow::tenant($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_admit_upload(
                input: UploadAdmissionRequest,
            ) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::admit(call_context, input, $clock())
            }
        }
        $emit! { query, request,
            fn blob_upload_admission(
                input: UploadAdmissionRequest,
            ) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
                $workflow::admission($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_upload_status(
                input: $contracts::dto::reference::ReferenceUpload,
            ) -> Result<
                $contracts::dto::upload::UploadStatusResponse,
                $contracts::dto::upload::UploadStatusFailure,
            > {
                $workflow::upload_status($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_revoke_upload(
                input: UploadAdmissionRequest,
            ) -> Result<UploadRevocationResponse, UploadAdmissionFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::revoke(call_context, input)
            }
        }
        $emit! { update, manifest,
            async fn blob_prepare_upload(
                input: UploadManifestRequest,
            ) -> Result<UploadManifestMutation, UploadManifestFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::prepare(call_context, &input, $clock())
            }
        }
        $emit! { query, request,
            fn blob_upload_certificate_assessment(
                root: String,
            ) -> Result<
                $contracts::dto::upload::certificate::UploadCertificateAssessmentResponse,
                $contracts::dto::upload::exposure::UploadExposureFailure,
            > {
                $workflow::certificate_assessment($context(), &root, $clock())
            }
        }
        $emit! { certificate, request, "_immutableObjectStorageCreateCertificate",
            async fn caffeine_upload_certificate(
                root: String,
            ) -> $contracts::dto::upload::certificate::CaffeineUploadCertificateResponse {
                let call_context = $context();
                $workflow::before_update().await;
                // Never encode an error as a successful provider reply. Shared workflow
                // rechecks current authority/evidence and commits exposure synchronously.
                $workflow::certificate(call_context, &root, $clock())
                    .unwrap_or_else(|_| $trap("certificate issuance refused"))
            }
        }
        $emit! { query, request,
            fn blob_upload_manifest(
                input: UploadAdmissionRequest,
            ) -> Result<UploadManifestResponse, UploadManifestFailure> {
                $workflow::manifest($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_apply_reference(
                input: ReferenceCommand,
            ) -> Result<ReferenceMutationResponse, ReferenceFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::reference(call_context, input)
            }
        }
        $emit! { query, request,
            fn blob_reference_receipt(
                input: ReferenceCommand,
            ) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
                $workflow::receipt($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_upload_history(
                input: UploadHistoryRequest,
            ) -> Result<UploadHistoryPage, UploadHistoryFailure> {
                $workflow::history($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_funding_history(
                input: FundingHistoryRequest,
            ) -> Result<FundingHistoryPage, FundingHistoryFailure> {
                $workflow::funding_history($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_funding_outcome(
                input: FundingOutcomeRequest,
            ) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
                $workflow::funding_outcome($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_funding_preparation_assessment(
                input: FundingPreparationRequest,
            ) -> Result<FundingPreparationResponse, FundingPreparationFailure> {
                $workflow::funding_assessment($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_revoke_gateway(
                input: $contracts::dto::gateway::GatewayRevocationRequest,
            ) -> Result<
                $contracts::dto::gateway::GatewayRevocationResponse,
                $contracts::dto::gateway::GatewayRevocationFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::revoke_gateway(call_context, input)
            }
        }
        $emit! { update, request,
            async fn blob_sync_gateways(
                input: OperatorScope,
            ) -> Result<
                $contracts::dto::gateway::sync::GatewaySyncResponse,
                $contracts::dto::gateway::sync::GatewaySyncFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::sync_gateways(call_context, input).await
            }
        }
        $emit! { update, request,
            async fn blob_cancel_gateway_sync(
                input: $contracts::dto::gateway::sync::GatewaySyncCancellation,
            ) -> Result<(), $contracts::dto::gateway::sync::GatewaySyncFailure> {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::cancel_gateway_sync(call_context, input)
            }
        }
        $emit! { query, request,
            fn blob_upload_capacity(
                input: $contracts::dto::tenant::TenantScope,
            ) -> Result<
                $contracts::dto::upload::capacity::UploadCapacityResponse,
                $contracts::dto::upload::capacity::UploadCapacityFailure,
            > {
                $workflow::upload_capacity($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_reference_capacity(
                input: $contracts::dto::reference::capacity::ReferenceCapacityRequest,
            ) -> Result<
                $contracts::dto::reference::capacity::ReferenceCapacityResponse,
                $contracts::dto::reference::capacity::ReferenceCapacityFailure,
            > {
                $workflow::reference_capacity($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_lookup_content(
                input: $contracts::dto::upload::discovery::UploadDiscoveryRequest,
            ) -> Result<
                $contracts::dto::upload::discovery::UploadDiscoveryResponse,
                $contracts::dto::upload::discovery::UploadDiscoveryFailure,
            > {
                $workflow::discover($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_download_descriptor(
                input: $contracts::dto::download::DownloadRequest,
            ) -> Result<
                $contracts::dto::download::DownloadResponse,
                $contracts::dto::download::DownloadFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::download(call_context, input)
            }
        }
        $emit! { query, request,
            fn blob_reference_status(
                input: $contracts::dto::reference::status::ReferenceStatusRequest,
            ) -> Result<
                $contracts::dto::reference::status::ReferenceStatusResponse,
                $contracts::dto::reference::ReferenceFailure,
            > {
                $workflow::reference_status($context(), input)
            }
        }
        $emit! { update, request,
            async fn blob_inspect_account(
                input: $contracts::dto::account::AccountInspectionRequest,
            ) -> Result<
                $contracts::dto::account::AccountInspectionResponse,
                $contracts::dto::account::AccountInspectionFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::inspect_account(call_context, input).await
            }
        }
        $emit! { update, request,
            async fn blob_attest_upload(
                input: $contracts::dto::upload::completion::UploadAttestationRequest,
            ) -> Result<
                $contracts::dto::upload::completion::UploadAttestationMutation,
                $contracts::dto::upload::completion::UploadAttestationFailure,
            > {
                let call_context = $context();
                $workflow::before_update().await;
                $workflow::attest(call_context, &input)
            }
        }
        $emit! { query, request,
            fn blob_upload_attestation(
                input: UploadAdmissionRequest,
            ) -> Result<
                $contracts::dto::upload::completion::UploadAttestationResponse,
                $contracts::dto::upload::completion::UploadAttestationFailure,
            > {
                $workflow::attestation($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_verification_manifest(
                input: UploadAdmissionRequest,
            ) -> Result<UploadManifestResponse, UploadManifestFailure> {
                $workflow::verification_manifest($context(), input)
            }
        }
        $emit! { query, request,
            fn blob_verification_plan(
                input: UploadAdmissionRequest,
            ) -> Result<
                $contracts::dto::upload::completion::UploadVerificationPlan,
                $contracts::dto::upload::completion::UploadAttestationFailure,
            > {
                $workflow::verification_plan($context(), input)
            }
        }
    };
}
