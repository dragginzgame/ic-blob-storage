//! Bind local preparation to one complete candidate, without installed-state authority.
use super::{Binding, Failure, NonZeroU64, UploadAdmissionRequest};
use crate::native::candidate_candid;
use ic_blob_storage::{
    dto::configuration::{ServiceInstallationInput, ServiceResourceInput},
    model::identity::caffeine::manifest::CaffeineManifestLimits,
    ops::service::installation::{ServiceInstallationCandidate, ValidatedServiceInstallation},
};

pub(super) struct Installation {
    pub limits: CaffeineManifestLimits,
    pub resources: ServiceResourceInput,
}
impl Installation {
    pub fn decode(
        bytes: &[u8],
        binding: &Binding,
        permission: UploadAdmissionRequest,
        maximum: NonZeroU64,
    ) -> Result<Self, Failure> {
        let input: ServiceInstallationInput = candidate_candid::decode(bytes)?;
        ValidatedServiceInstallation::new(
            permission.upload.service,
            ServiceInstallationCandidate {
                configuration: input.configuration,
                project: &input.project,
                completion_verifier: input.completion_verifier,
                trusted_uploader: input.trusted_uploader,
                // Offline syntax/model validation only; the actual host supplies
                // its own compiled release and independently authenticates init.
                release: env!("CARGO_PKG_VERSION"),
                platform_installation_version: 0,
            },
        )
        .map_err(|_| Failure::Arguments)?;
        if input.configuration.namespace != permission.upload.namespace
            || input.project != binding.project
            || input.trusted_uploader != permission.uploader
        {
            return Err(Failure::Arguments);
        }
        let r = input.configuration.resources;
        // Retain the CLI's buffering/work ceilings while also enforcing the
        // validated candidate's per-manifest and lifetime leaf ceilings. This
        // does not observe remaining capacity, tenant authority or provisioning.
        let limits = CaffeineManifestLimits {
            max_content_bytes: NonZeroU64::new(maximum.get().min(r.max_object_bytes))
                .expect("validated positive object bound"),
            max_chunks: usize::try_from(1024.min(r.max_chunks).min(r.max_tenant_chunks))
                .expect("bounded portable chunk count")
                .try_into()
                .expect("validated positive chunk bound"),
            max_headers: usize::try_from(16.min(r.max_headers))
                .expect("bounded portable header count")
                .try_into()
                .expect("validated positive header bound"),
            max_header_bytes: usize::try_from(4096.min(r.max_header_bytes))
                .expect("bounded portable metadata bytes")
                .try_into()
                .expect("validated positive metadata bound"),
        };
        Ok(Self {
            limits,
            resources: r,
        })
    }
}
