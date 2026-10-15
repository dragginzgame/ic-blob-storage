//! Reference-model implementations of the same read boundaries used by stable owners.
use super::UploadAdmissions;
use crate::model::service::upload::content::ContentLookup;
use crate::model::service::upload::planning::AdmissionCapacityLookup;
use crate::ops::service::references::capacity::{
    ReferenceCapacityObservation, ReferenceCapacitySource,
};
use crate::ops::service::uploads::UploadStoreError;
use crate::ops::service::uploads::capacity::{UploadCapacitySource, UploadCapacityView};
use crate::ops::service::uploads::discovery::{UploadDiscoveryObservation, UploadDiscoverySource};
use ic_blob_storage_contracts::upload::binding::UploadContext;

impl UploadDiscoverySource for UploadAdmissions {
    fn discovery_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<UploadDiscoveryObservation, UploadStoreError> {
        // The heap owner has no restore path; recreation is not operational recovery.
        Ok(UploadDiscoveryObservation {
            content: self.lookup_content(context, input)?,
            fenced: false,
        })
    }
}

impl UploadCapacitySource for UploadAdmissions {
    fn capacity_view(
        &self,
        context: UploadContext,
        scope: AdmissionCapacityLookup,
    ) -> Result<UploadCapacityView, UploadStoreError> {
        // The heap owner has no restore path. Hosts must not recreate it from an old snapshot.
        Ok(UploadCapacityView {
            headroom: self.admission_capacity(context, scope)?,
            fenced: false,
        })
    }
}

impl ReferenceCapacitySource for UploadAdmissions {
    fn reference_capacity_view(
        &self,
        context: UploadContext,
        input: ContentLookup,
    ) -> Result<ReferenceCapacityObservation, UploadStoreError> {
        // The heap owner has no restore path; recreation is not operational recovery.
        Ok(ReferenceCapacityObservation {
            headroom: self.reference_capacity(context, input)?,
            fenced: false,
        })
    }
}
