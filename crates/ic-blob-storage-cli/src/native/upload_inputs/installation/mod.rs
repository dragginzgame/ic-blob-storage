//! Bind local preparation to one complete candidate, without installed-state authority.
use super::{Binding, Failure, NonZeroU64, UploadAdmissionRequest};
use crate::native::exact_wire;
use ic_blob_storage_contracts::configuration::ServiceInstallationCandidate;
use ic_blob_storage_contracts::configuration::ValidatedInstallationInput;
use ic_blob_storage_contracts::dto::configuration::ServiceInstallationInput;
use ic_blob_storage_contracts::identity::caffeine::manifest::CaffeineManifestLimits;
use std::rc::Rc;

/// One immutable offline candidate and exact packet, shared within an invocation.
pub(in crate::native) struct Installation {
    pub input: ServiceInstallationInput,
    pub bytes: Vec<u8>,
    pub limits: CaffeineManifestLimits,
}
impl Installation {
    pub fn decode(bytes: Vec<u8>, maximum: NonZeroU64) -> Result<Rc<Self>, Failure> {
        let input: ServiceInstallationInput =
            exact_wire::decode(&bytes, exact_wire::INSTALLATION_BYTES)?;
        // Offline model validation only. Each file must independently bind the
        // proposed service below; this supplies no authenticated platform identity.
        ValidatedInstallationInput::new(
            input.configuration.service,
            ServiceInstallationCandidate {
                configuration: input.configuration,
                project: &input.project,
                completion_verifier: input.completion_verifier,
                // Offline syntax/model validation only; the actual host supplies
                // its own compiled release and independently authenticates init.
                release: env!("CARGO_PKG_VERSION"),
                platform_installation_version: 0,
            },
        )
        .map_err(|_| Failure::Arguments)?;
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
        Ok(Rc::new(Self {
            input,
            bytes,
            limits,
        }))
    }

    pub(super) fn bind(
        &self,
        binding: &Binding,
        permission: UploadAdmissionRequest,
    ) -> Result<(), Failure> {
        if self.input.configuration.service != permission.upload.service
            || self.input.configuration.namespace != permission.upload.namespace
            || self.input.project != binding.project
        {
            return Err(Failure::Arguments);
        }
        Ok(())
    }
}
