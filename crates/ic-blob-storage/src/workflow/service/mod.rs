//! Complete service dispatch over an explicitly supplied installation owner.
//!
//! Hosts retain endpoint registration, lifecycle invocation, memory bootstrap and
//! actual platform observations. This module owns borrowing discipline and the
//! continuity observations surrounding shared workflows; no borrow crosses await.
use crate::ops::caffeine::decoding::CaffeineReplyLimits;
use crate::ops::service::{
    account::AccountInspectionAccess,
    gateways::{StableGatewayRegistry, access::GatewayRegistryAccess},
    installation::ServiceInstallation,
    recovery::{ActiveContinuityRequest, ContinuityInstallationAccess, RecoveryInstallationAccess},
    stores::ServiceStores,
};
use ic_blob_storage_contracts::{
    download::scope::CaffeineDownloadScope, upload::completion::CompletionAuthority,
};
use std::{
    marker::PhantomData,
    num::{NonZeroU32, NonZeroUsize},
};

/// Host-selected execution bounds; sharing dispatch does not select host policy.
pub struct ServiceExecutionLimits {
    /// Timeout for read-only provider observations, in seconds.
    pub timeout: NonZeroU32,
    /// Gateway observation decoder limits.
    pub gateways: CaffeineReplyLimits,
    /// Account observation decoder limits.
    pub account: CaffeineReplyLimits,
    /// Maximum upload-history entries examined per call.
    pub history_scan: NonZeroUsize,
    /// Maximum upload-history entries returned per call.
    pub history_results: NonZeroUsize,
    /// Maximum funding-history entries returned per call.
    pub funding_results: NonZeroUsize,
}

/// Explicit access to a host's sole installed service, without lifecycle ownership.
pub trait ServiceHost {
    /// Backing of the host's retained memory runtime.
    type Memory: ic_memory::Memory;
    /// Invoke exactly once and release the borrow before returning. Propagate traps.
    /// Every invocation must access the same installation; never initialize or reset it.
    fn with_installation<R>(
        operation: impl FnOnce(&mut ServiceInstallation<Self::Memory>) -> R,
    ) -> R;
    /// Actual platform version, including management changes and callback execution.
    fn platform_version() -> u64;
    /// Actual platform time in nanoseconds.
    fn time() -> u64;
    /// Explicit policy selected by this host.
    fn limits() -> ServiceExecutionLimits;
}

/// Shared typed dispatch; a host may bind this as its endpoint catalog's workflow.
/// Linking or naming this type registers no endpoint, memory, timer or lifecycle.
pub struct ServiceWorkflow<H>(PhantomData<fn() -> H>);
impl<H: ServiceHost> ServiceWorkflow<H> {
    const ACCESS: Self = Self(PhantomData);
    fn observe_version() {
        H::with_installation(|installation| {
            installation.observe_platform_version(H::platform_version());
        });
    }
    fn with_installation<R>(operation: impl FnOnce(&ServiceInstallation<H::Memory>) -> R) -> R {
        Self::observe_version();
        H::with_installation(|installation| operation(installation))
    }
    fn with_certificate<R>(operation: impl FnOnce(&mut ServiceInstallation<H::Memory>) -> R) -> R {
        Self::observe_version();
        H::with_installation(operation)
    }
    fn read<R>(operation: impl FnOnce(&ServiceStores<H::Memory>) -> R) -> R {
        Self::with_installation(|installation| operation(installation.stores()))
    }
    fn mutate<R>(operation: impl FnOnce(&mut ServiceStores<H::Memory>) -> R) -> R {
        Self::with_certificate(|installation| operation(installation.stores_mut()))
    }
    fn with_completion<R>(
        operation: impl FnOnce(&mut ServiceStores<H::Memory>, CompletionAuthority) -> R,
    ) -> R {
        Self::with_certificate(|installation| {
            let authority = installation.completion_authority();
            operation(installation.stores_mut(), authority)
        })
    }
    fn with_download<R>(
        operation: impl FnOnce(
            &crate::ops::service::uploads::StableUploads<H::Memory>,
            &CaffeineDownloadScope,
        ) -> R,
    ) -> R {
        Self::with_installation(|installation| {
            operation(
                &installation.stores().uploads,
                installation.download_scope(),
            )
        })
    }
    fn with_verification<R>(
        operation: impl FnOnce(
            &crate::ops::service::uploads::StableUploads<H::Memory>,
            CompletionAuthority,
            &CaffeineDownloadScope,
        ) -> R,
    ) -> R {
        Self::with_installation(|installation| {
            operation(
                &installation.stores().uploads,
                installation.completion_authority(),
                installation.download_scope(),
            )
        })
    }
    /// Delegate `before_update` with shared continuity and host-selected bounds.
    pub async fn before_update() {
        crate::workflow::installation::recovery::before_update(&Self::ACCESS).await;
    }
    /// Delegate `resume_current_instance` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub async fn resume_current_instance(
        context: UploadContext,
    ) -> Result<(), ic_blob_storage_contracts::dto::recovery::CurrentInstanceRecoveryFailure> {
        crate::workflow::installation::recovery::resume_host_instance(&Self::ACCESS, context).await
    }
    /// Delegate `certificate_assessment` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn certificate_assessment(
        context: UploadContext,
        root: &str,
        now: u64,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::certificate::UploadCertificateAssessmentResponse,
        ic_blob_storage_contracts::dto::upload::exposure::UploadExposureFailure,
    > {
        Self::with_installation(|installation| {
            crate::workflow::uploads::certificate::inspect_installation(
                installation,
                context,
                root,
                now,
            )
        })
    }

    /// Delegate `certificate` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn certificate(
        context: UploadContext,
        root: &str,
        now: u64,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::certificate::CaffeineUploadCertificateResponse,
        crate::workflow::uploads::certificate::UploadCertificateFailure,
    > {
        Self::with_certificate(|installation| {
            crate::workflow::uploads::certificate::issue_installation(
                installation,
                context,
                root,
                now,
            )
        })
    }
    /// Delegate `reference_capacity` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn reference_capacity(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityResponse,
        ic_blob_storage_contracts::dto::reference::capacity::ReferenceCapacityFailure,
    > {
        Self::read(|stores| {
            crate::workflow::references::capacity::inspect(&stores.uploads, context, input)
        })
    }
    /// Delegate `upload_capacity` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn upload_capacity(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::tenant::TenantScope,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityResponse,
        ic_blob_storage_contracts::dto::upload::capacity::UploadCapacityFailure,
    > {
        Self::read(|stores| {
            crate::workflow::uploads::capacity::inspect(&stores.uploads, context, input)
        })
    }
    /// Delegate `revoke_gateway` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn revoke_gateway(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::gateway::GatewayRevocationRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::gateway::GatewayRevocationResponse,
        ic_blob_storage_contracts::dto::gateway::GatewayRevocationFailure,
    > {
        Self::mutate(|stores| {
            crate::workflow::gateways::revocation::revoke(&mut stores.gateways, context, input)
        })
    }
    /// Delegate `history` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn history(
        context: UploadContext,
        input: UploadHistoryRequest,
    ) -> Result<UploadHistoryPage, UploadHistoryFailure> {
        use crate::model::catalog::admission::read::UploadPageLimits;
        Self::read(|stores| {
            crate::workflow::uploads::history::inspect(
                &stores.uploads,
                context,
                input,
                UploadPageLimits {
                    max_scan: H::limits().history_scan,
                    max_results: H::limits().history_results,
                },
            )
        })
    }
    /// Delegate `funding_history` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn funding_history(
        context: UploadContext,
        input: FundingHistoryRequest,
    ) -> Result<FundingHistoryPage, FundingHistoryFailure> {
        Self::read(|stores| {
            crate::workflow::funding::history::inspect(
                &stores.funding,
                context,
                input,
                H::limits().funding_results,
            )
        })
    }
    /// Delegate `funding_outcome` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn funding_outcome(
        context: UploadContext,
        input: FundingOutcomeRequest,
    ) -> Result<Option<FundingOutcomeResponse>, FundingOutcomeFailure> {
        Self::read(|stores| {
            crate::workflow::funding::outcome::inspect(&stores.funding, context, input)
        })
    }
    /// Delegate `funding_assessment` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn funding_assessment(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationResponse,
        ic_blob_storage_contracts::dto::funding::assessment::FundingPreparationFailure,
    > {
        Self::read(|stores| {
            crate::workflow::funding::assessment::inspect(&stores.funding, context, input)
        })
    }
    /// Delegate `local_status` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn local_status(
        context: UploadContext,
        input: OperatorScope,
    ) -> Result<LocalServiceStatus, LocalStatusFailure> {
        Self::read(|stores| {
            crate::workflow::operator::inspect(OperatorStores::from(stores), context, input)
        })
    }
    /// Delegate `configuration` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn configuration(
        actor: ic_platform::Principal,
    ) -> Result<HostConfigurationView, HostFailure> {
        Self::with_installation(|installation| {
            crate::workflow::installation::inspect(
                installation,
                UploadContext {
                    service: ic_cdk::api::canister_self(),
                    actor,
                },
            )
        })
    }
    /// Delegate `update_tenant` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn update_tenant(
        context: UploadContext,
        input: TenantUpdateRequest,
    ) -> Result<TenantEnrollmentResponse, TenantFailure> {
        Self::mutate(|stores| crate::workflow::tenants::update(&mut stores.uploads, context, input))
    }
    /// Delegate `tenant` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn tenant(
        context: UploadContext,
        input: TenantScope,
    ) -> Result<TenantEnrollmentResponse, TenantFailure> {
        Self::read(|stores| crate::workflow::tenants::inspect(&stores.uploads, context, input))
    }
    /// Delegate `admit` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn admit(
        context: UploadContext,
        input: UploadAdmissionRequest,
        now: u64,
    ) -> Result<UploadAdmissionMutation, UploadAdmissionFailure> {
        Self::mutate(|stores| {
            crate::workflow::uploads::admission::admit(&mut stores.uploads, context, input, now)
        })
    }
    /// Delegate `admission` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn admission(
        context: UploadContext,
        input: UploadAdmissionRequest,
    ) -> Result<UploadAdmissionResponse, UploadAdmissionFailure> {
        Self::read(|stores| {
            crate::workflow::uploads::admission::inspect(&stores.uploads, context, input)
        })
    }
    /// Delegate `upload_status` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn upload_status(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::reference::ReferenceUpload,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::UploadStatusResponse,
        ic_blob_storage_contracts::dto::upload::UploadStatusFailure,
    > {
        Self::read(|stores| crate::workflow::uploads::inspect(&stores.uploads, context, input))
    }
    /// Delegate `revoke` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn revoke(
        context: UploadContext,
        input: UploadAdmissionRequest,
    ) -> Result<UploadRevocationResponse, UploadAdmissionFailure> {
        Self::mutate(|stores| {
            crate::workflow::uploads::admission::revoke(&mut stores.uploads, context, input)
        })
    }
    /// Delegate `prepare` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn prepare(
        context: UploadContext,
        input: &UploadManifestRequest,
        now: u64,
    ) -> Result<UploadManifestMutation, UploadManifestFailure> {
        Self::mutate(|stores| {
            crate::workflow::uploads::manifests::prepare(&mut stores.uploads, context, input, now)
        })
    }
    /// Delegate `manifest` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn manifest(
        context: UploadContext,
        input: UploadAdmissionRequest,
    ) -> Result<UploadManifestResponse, UploadManifestFailure> {
        Self::read(|stores| {
            crate::workflow::uploads::manifests::inspect(&stores.uploads, context, input)
        })
    }
    /// Delegate `reference` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn reference(
        context: UploadContext,
        input: ReferenceCommand,
    ) -> Result<ReferenceMutationResponse, ReferenceFailure> {
        Self::mutate(|stores| {
            crate::workflow::references::apply(&mut stores.uploads, context, input)
        })
    }
    /// Delegate `receipt` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn receipt(
        context: UploadContext,
        input: ReferenceCommand,
    ) -> Result<ReferenceReceiptLookup, ReferenceFailure> {
        Self::read(|stores| crate::workflow::references::receipt(&stores.uploads, context, input))
    }

    /// Delegate `sync_gateways` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub async fn sync_gateways(
        context: UploadContext,
        input: OperatorScope,
    ) -> Result<
        ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncResponse,
        ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure,
    > {
        let outcome = crate::workflow::gateways::sync::refresh(
            &Self::ACCESS,
            context,
            input,
            H::limits().timeout,
            H::limits().gateways,
        )
        .await;
        Self::observe_version();
        outcome
    }
    /// Delegate `cancel_gateway_sync` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn cancel_gateway_sync(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncCancellation,
    ) -> Result<(), ic_blob_storage_contracts::dto::gateway::sync::GatewaySyncFailure> {
        Self::mutate(|stores| {
            crate::workflow::gateways::sync::cancel(&mut stores.gateways, context, input)
        })
    }

    /// Delegate `discover` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn discover(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryResponse,
        ic_blob_storage_contracts::dto::upload::discovery::UploadDiscoveryFailure,
    > {
        Self::read(|stores| {
            crate::workflow::uploads::discovery::inspect(&stores.uploads, context, input)
        })
    }

    /// Delegate `download` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn download(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::download::DownloadRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::download::DownloadResponse,
        ic_blob_storage_contracts::dto::download::DownloadFailure,
    > {
        Self::with_download(|uploads, scope| {
            crate::workflow::reads::download::handle(uploads, context, scope, input)
        })
    }

    /// Delegate `reference_status` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn reference_status(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::reference::status::ReferenceStatusRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::reference::status::ReferenceStatusResponse,
        ReferenceFailure,
    > {
        Self::read(|stores| {
            crate::workflow::references::status::inspect(&stores.uploads, context, input)
        })
    }

    /// Delegate `inspect_account` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub async fn inspect_account(
        context: UploadContext,
        input: ic_blob_storage_contracts::dto::account::AccountInspectionRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::account::AccountInspectionResponse,
        ic_blob_storage_contracts::dto::account::AccountInspectionFailure,
    > {
        use crate::ops::caffeine::query::transport::replicated::account::ReplicatedAccountQuery;
        use ic_blob_storage_contracts::dto::account::AccountInspectionFailure;
        let transport = ReplicatedAccountQuery::new(
            input.scope.service,
            input.scope.cashier,
            input.scope.payment_account,
            H::limits().timeout,
        )
        .map_err(|_| AccountInspectionFailure::Invalid)?;
        let outcome = crate::workflow::account::inspect(
            &Self::ACCESS,
            &transport,
            context,
            input,
            H::limits().account,
        )
        .await;
        Self::observe_version();
        outcome
    }

    /// Delegate `attest` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn attest(
        context: UploadContext,
        input: &ic_blob_storage_contracts::dto::upload::completion::UploadAttestationRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::completion::UploadAttestationMutation,
        ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure,
    > {
        Self::with_completion(|stores, authority| {
            crate::workflow::uploads::completion::attest(
                &mut stores.uploads,
                authority,
                context,
                input,
                H::time(),
            )
        })
    }
    /// Delegate `attestation` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn attestation(
        context: UploadContext,
        input: UploadAdmissionRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::completion::UploadAttestationResponse,
        ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure,
    > {
        Self::with_completion(|stores, authority| {
            crate::workflow::uploads::completion::inspect(
                &stores.uploads,
                authority,
                context,
                input,
            )
        })
    }

    /// Delegate `verification_manifest` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn verification_manifest(
        context: UploadContext,
        input: UploadAdmissionRequest,
    ) -> Result<UploadManifestResponse, UploadManifestFailure> {
        Self::with_completion(|stores, authority| {
            crate::workflow::uploads::completion::manifest(
                &stores.uploads,
                authority,
                context,
                input,
            )
        })
    }

    /// Delegate `verification_plan` with shared continuity and host-selected bounds.
    /// # Errors
    /// Returns the operation's typed authority, binding, fence or resource refusal.
    pub fn verification_plan(
        context: UploadContext,
        input: UploadAdmissionRequest,
    ) -> Result<
        ic_blob_storage_contracts::dto::upload::completion::UploadVerificationPlan,
        ic_blob_storage_contracts::dto::upload::completion::UploadAttestationFailure,
    > {
        Self::with_verification(|uploads, authority, scope| {
            crate::workflow::uploads::completion::verification_plan(
                uploads, authority, scope, context, input,
            )
        })
    }
}
impl<H: ServiceHost> RecoveryInstallationAccess for ServiceWorkflow<H> {
    type Memory = H::Memory;
    fn with_installation<R>(
        &self,
        operation: impl FnOnce(&mut ServiceInstallation<H::Memory>) -> R,
    ) -> R {
        Self::with_certificate(operation)
    }
}
impl<H: ServiceHost> ContinuityInstallationAccess for ServiceWorkflow<H> {
    fn observe_version(&self) {
        Self::observe_version();
    }
    fn prepare_active_continuity(&self) -> Option<ActiveContinuityRequest> {
        H::with_installation(|installation| {
            installation.prepare_active_continuity(H::platform_version())
        })
    }
}
impl<H: ServiceHost> AccountInspectionAccess for ServiceWorkflow<H> {
    type Memory = H::Memory;
    fn with_account_stores<R>(
        &self,
        operation: impl FnOnce(OperatorStores<'_, H::Memory>) -> R,
    ) -> R {
        Self::read(|stores| operation(OperatorStores::from(stores)))
    }
}
impl<H: ServiceHost> GatewayRegistryAccess for ServiceWorkflow<H> {
    type Memory = H::Memory;
    fn with_gateway_registry<R>(
        &self,
        operation: impl FnOnce(&mut StableGatewayRegistry<H::Memory>) -> R,
    ) -> R {
        Self::mutate(|stores| operation(&mut stores.gateways))
    }
}
use crate::ops::service::operator::OperatorStores;
use ic_blob_storage_contracts::dto::configuration::HostConfigurationView;
use ic_blob_storage_contracts::dto::configuration::HostFailure;
use ic_blob_storage_contracts::dto::funding::FundingHistoryFailure;
use ic_blob_storage_contracts::dto::funding::FundingHistoryPage;
use ic_blob_storage_contracts::dto::funding::FundingHistoryRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeFailure;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeRequest;
use ic_blob_storage_contracts::dto::funding::outcome::FundingOutcomeResponse;
use ic_blob_storage_contracts::dto::operator::LocalServiceStatus;
use ic_blob_storage_contracts::dto::operator::LocalStatusFailure;
use ic_blob_storage_contracts::dto::operator::OperatorScope;
use ic_blob_storage_contracts::dto::reference::ReferenceCommand;
use ic_blob_storage_contracts::dto::reference::ReferenceFailure;
use ic_blob_storage_contracts::dto::reference::ReferenceMutationResponse;
use ic_blob_storage_contracts::dto::reference::ReferenceReceiptLookup;
use ic_blob_storage_contracts::dto::tenant::TenantEnrollmentResponse;
use ic_blob_storage_contracts::dto::tenant::TenantFailure;
use ic_blob_storage_contracts::dto::tenant::TenantScope;
use ic_blob_storage_contracts::dto::tenant::TenantUpdateRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionFailure;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionMutation;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionRequest;
use ic_blob_storage_contracts::dto::upload::admission::UploadAdmissionResponse;
use ic_blob_storage_contracts::dto::upload::admission::UploadRevocationResponse;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryFailure;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryPage;
use ic_blob_storage_contracts::dto::upload::history::UploadHistoryRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestFailure;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestMutation;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestRequest;
use ic_blob_storage_contracts::dto::upload::manifest::UploadManifestResponse;
use ic_blob_storage_contracts::upload::binding::UploadContext;
