//! Adapter-owned conversion of generic Canic refusals into blob boundary replies.
pub(crate) mod account;
pub(crate) mod certificate;
pub(crate) mod fixture;
pub(crate) mod gateways;
use super::{
    INSTALLATION, PreparationForwardFailure,
    dto::{ProbeExposureFailure, TransportFailure},
};
use ic_blob_storage_canic::lifecycle::ManagedInstallation;

pub(crate) fn read<R>(operation: impl FnOnce(&ManagedInstallation) -> R) -> R {
    INSTALLATION.with_borrow(|owner| operation(owner.as_ref().expect("installed service")))
}
pub(crate) fn mutate<R>(operation: impl FnOnce(&mut ManagedInstallation) -> R) -> R {
    INSTALLATION.with_borrow_mut(|owner| operation(owner.as_mut().expect("installed service")))
}
use ic_blob_storage::dto::{
    account::AccountInspectionFailure,
    configuration::HostFailure,
    download::DownloadFailure,
    funding::{
        FundingHistoryFailure, assessment::FundingPreparationFailure,
        outcome::FundingOutcomeFailure,
    },
    gateway::{GatewayRevocationFailure, sync::GatewaySyncFailure},
    operator::LocalStatusFailure,
    reference::{ReferenceFailure, capacity::ReferenceCapacityFailure},
    tenant::TenantFailure,
    upload::{
        UploadStatusFailure, admission::UploadAdmissionFailure, capacity::UploadCapacityFailure,
        completion::UploadAttestationFailure, discovery::UploadDiscoveryFailure,
        exposure::UploadExposureFailure, history::UploadHistoryFailure,
        manifest::UploadManifestFailure,
    },
};

pub(crate) trait TransportDenial {
    fn denied() -> Self;
}
impl<E: TransportDenial> From<canic::access::AccessError> for TransportFailure<E> {
    fn from(_: canic::access::AccessError) -> Self {
        Self(E::denied())
    }
}
macro_rules! denial {
    ($error:ty, $value:expr) => {
        impl TransportDenial for $error {
            fn denied() -> Self {
                $value
            }
        }
    };
}
denial!(HostFailure, HostFailure::Denied);
denial!(TenantFailure, TenantFailure::Denied);
denial!(UploadAdmissionFailure, UploadAdmissionFailure::Denied);
denial!(
    UploadManifestFailure,
    UploadManifestFailure::Permission(UploadAdmissionFailure::Denied)
);
denial!(UploadCapacityFailure, UploadCapacityFailure::Denied);
denial!(UploadStatusFailure, UploadStatusFailure::Denied);
denial!(UploadHistoryFailure, UploadHistoryFailure::Denied);
denial!(UploadDiscoveryFailure, UploadDiscoveryFailure::Denied);
denial!(ReferenceFailure, ReferenceFailure::Denied);
denial!(ReferenceCapacityFailure, ReferenceCapacityFailure::Denied);
denial!(LocalStatusFailure, LocalStatusFailure::Denied);
denial!(FundingHistoryFailure, FundingHistoryFailure::Denied);
denial!(FundingOutcomeFailure, FundingOutcomeFailure::Denied);
denial!(FundingPreparationFailure, FundingPreparationFailure::Denied);
denial!(UploadAttestationFailure, UploadAttestationFailure::Denied);
denial!(DownloadFailure, DownloadFailure::Denied);
denial!(ProbeExposureFailure, ProbeExposureFailure::Denied);
denial!(AccountInspectionFailure, AccountInspectionFailure::Denied);
denial!(GatewayRevocationFailure, GatewayRevocationFailure::Denied);
denial!(GatewaySyncFailure, GatewaySyncFailure::Denied);
denial!(
    UploadExposureFailure,
    UploadExposureFailure::Permission(UploadAdmissionFailure::Denied)
);
impl From<canic::access::AccessError> for PreparationForwardFailure {
    fn from(_: canic::access::AccessError) -> Self {
        Self::Denied
    }
}
