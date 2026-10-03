//! Local byte evidence against an authenticated original declaration, never completion authority.
use super::{
    Failure, local_body::LocalBody, read, references::upload_json,
    upload_setup::manifest_reply_limits,
};
use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use ic_blob_storage::{
    dto::upload::{admission::UploadAdmissionRequest, manifest::UploadManifestInspection},
    ops::service::uploads::manifests::reply::{self, UploadManifestReplyError},
};
use serde_json::{Value, json};
use std::{num::NonZeroU64, path::Path};

pub(super) struct Verification {
    pub permission: UploadAdmissionRequest,
    body: LocalBody,
    max_bytes: NonZeroU64,
}
impl Verification {
    pub fn open(
        service: Principal,
        namespace: u128,
        actor: Principal,
        permission: &Path,
        body: &Path,
        max_bytes: NonZeroU64,
    ) -> Result<Self, Failure> {
        let encoded = read(permission, 4096)?;
        let mut config = DecoderConfig::new();
        config
            .set_decoding_quota(100_000)
            .set_skipping_quota(0)
            .set_max_type_len(32)
            .set_max_header_len(4096)
            .set_full_error_message(false);
        let permission: UploadAdmissionRequest =
            decode_one_with_config(&encoded, &config).map_err(|_| Failure::Arguments)?;
        let upload = permission.upload;
        if upload.service != service || upload.namespace != namespace {
            return Err(Failure::Binding);
        }
        if actor != upload.tenant && actor != permission.uploader {
            return Err(Failure::Denied);
        }
        if [
            upload.namespace,
            upload.upload,
            upload.object,
            upload.incarnation,
            upload.first_reference,
        ]
        .contains(&0)
            || upload.bytes == 0
            || upload.bytes > max_bytes.get()
            || [upload.tenant, permission.uploader]
                .iter()
                .any(|p| *p == Principal::anonymous() || *p == Principal::management_canister())
        {
            return Err(Failure::Arguments);
        }
        let body = LocalBody::open(body, upload.bytes)?;
        // Keep this open handle across the query; a path replacement cannot select another file.
        Ok(Self {
            permission,
            body,
            max_bytes,
        })
    }

    pub fn finish(
        self,
        response: &[u8],
        actor: Principal,
        network: &str,
        url: &str,
    ) -> Result<Value, Failure> {
        let declaration = reply::inspection(
            self.permission,
            response,
            manifest_reply_limits(self.max_bytes),
        )
        .map_err(|e| match e {
            UploadManifestReplyError::Limit => Failure::ReplyLimit,
            UploadManifestReplyError::Binding => Failure::Binding,
            UploadManifestReplyError::Invalid => Failure::InvalidReply,
            UploadManifestReplyError::Remote(_) => Failure::ManifestRefused,
        })?;
        let UploadManifestInspection::Prepared(declaration) = declaration.manifest else {
            return Err(Failure::Unprepared);
        };
        let upload = self.permission.upload;
        let root = upload.root.as_slice().try_into().expect("fixed root");
        let hashes = self
            .body
            .verify(root, &declaration, self.max_bytes, &mut std::io::sink())?;
        // Verification proves the computed root equals this original upload root.
        Ok(
            json!({"schema":1,"observation":"local_upload_bytes","actor":actor.to_text(),
            "network":network,"url":url,"manifest_authentication":"query_signatures",
            "upload":upload_json(upload),
            "uploader":self.permission.uploader.to_text(),"expires_at_ns":self.permission.expires_at_ns.to_string(),
            "content_digest":hashes.content_digest.to_string(),"provider_completion":"not_established",
            "provider_availability":"not_observed","retry_authorized":false}),
        )
    }
}

#[cfg(test)]
mod tests;
