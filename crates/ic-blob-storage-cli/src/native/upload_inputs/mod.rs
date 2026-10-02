//! Offline upstream-manifest conversion; no authority, allocation or dispatch.
mod installation;
use super::{
    Failure,
    artifacts::{FailureRecord, Run},
    local_body::LocalBody,
    read,
    reference_inputs::ReferenceFiles,
};
use candid::Principal;
use ic_blob_storage::{
    dto::{
        reference::ReferenceUpload,
        upload::{admission::UploadAdmissionRequest, manifest::UploadManifestRequest},
    },
    model::identity::{ProviderRootHash, caffeine::CaffeineContentHashes},
    model::service::read::download::CaffeineDownloadScope,
    ops::{
        caffeine::preparation::{PreparedManifestLimits, decode_prepared_manifest},
        service::uploads::{
            admission::reply::validate_request, manifests::reply::validate_declaration,
        },
    },
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::{Display, Write},
    num::NonZeroU64,
    path::Path,
    str::FromStr,
};

const MANIFEST_BYTES: u64 = 256 * 1024;

/// Existing browser client's passive original binding; no store or authority.
#[derive(Serialize)]
struct BrowserCertificateBinding {
    key: String,
    service: String,
    tenant: String,
    uploader: String,
    operation: String,
    root: String,
    project: String,
    bucket: String,
    permission: Vec<u8>,
}
impl BrowserCertificateBinding {
    fn new(
        request: UploadAdmissionRequest,
        permission: Vec<u8>,
        project: &str,
        bucket: &str,
    ) -> Self {
        let service = request.upload.service.to_text();
        let tenant = request.upload.tenant.to_text();
        let operation = request.upload.upload.to_string();
        Self {
            key: format!("{service}:{tenant}:{operation}"),
            service,
            tenant,
            uploader: request.uploader.to_text(),
            operation,
            root: ProviderRootHash::try_from(request.upload.root.as_slice())
                .expect("fixed provider root")
                .to_string(),
            permission,
            project: project.into(),
            bucket: bucket.into(),
        }
    }
}

/// Caller-selected identities remain unallocated and unauthenticated local data.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    schema: u8,
    project: String,
    bucket: String,
    service: String,
    namespace: String,
    tenant: String,
    uploader: String,
    upload: String,
    object: String,
    incarnation: String,
    first_reference: String,
    root: String,
    bytes: String,
    expires_at_ns: String,
}

fn positive<T: FromStr + Display>(value: &str) -> Result<T, Failure> {
    let parsed = value.parse::<T>().map_err(|_| Failure::Arguments)?;
    if parsed.to_string() != value || value == "0" {
        return Err(Failure::Arguments);
    }
    Ok(parsed)
}
fn principal(value: &str) -> Result<Principal, Failure> {
    let result = Principal::from_text(value).map_err(|_| Failure::Arguments)?;
    if result.to_text() != value {
        return Err(Failure::Arguments);
    }
    Ok(result)
}
impl Binding {
    fn permission(&self) -> Result<UploadAdmissionRequest, Failure> {
        if self.schema != 1 {
            return Err(Failure::Arguments);
        }
        let root: ProviderRootHash = self.root.parse().map_err(|_| Failure::Arguments)?;
        let request = UploadAdmissionRequest {
            upload: ReferenceUpload {
                service: principal(&self.service)?,
                namespace: positive(&self.namespace)?,
                tenant: principal(&self.tenant)?,
                upload: positive(&self.upload)?,
                object: positive(&self.object)?,
                incarnation: positive(&self.incarnation)?,
                first_reference: positive(&self.first_reference)?,
                root: *root.as_bytes(),
                bytes: positive(&self.bytes)?,
            },
            uploader: principal(&self.uploader)?,
            expires_at_ns: positive(&self.expires_at_ns)?,
        };
        validate_request(request).map_err(|_| Failure::Arguments)?;
        Ok(request)
    }
}

fn validate_namespaces(
    binding: &Binding,
    permission: UploadAdmissionRequest,
) -> Result<(), Failure> {
    for namespace in [&binding.project, &binding.bucket] {
        if namespace.starts_with('\u{feff}') || namespace.ends_with('\u{feff}') {
            return Err(Failure::Arguments);
        }
        CaffeineDownloadScope::new(
            permission.upload.service,
            permission
                .upload
                .namespace
                .try_into()
                .map_err(|_| Failure::Arguments)?,
            namespace,
        )
        .map_err(|_| Failure::Arguments)?;
    }
    // Fetch Headers requires ByteString project values, in addition to the
    // shared UTF-8 namespace bounds. Bucket travels in the SDK's JSON/query.
    if binding
        .project
        .chars()
        .any(|character| u32::from(character) > 255)
    {
        return Err(Failure::Arguments);
    }
    Ok(())
}

pub(super) fn run(args: &[String]) -> Result<Value, Failure> {
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        if pair.len() != 2 || flags.insert(pair[0].as_str(), pair[1].as_str()).is_some() {
            return Err(Failure::Arguments);
        }
    }
    let mut take = |name| flags.remove(name).ok_or(Failure::Arguments);
    let binding_path = Path::new(take("--binding")?);
    let installation_path = Path::new(take("--installation")?);
    let manifest_path = Path::new(take("--manifest")?);
    let maximum: NonZeroU64 = positive(take("--max-bytes")?)?;
    let directory = Path::new(take("--run-dir")?);
    let body_path = Path::new(take("--body")?);
    if !flags.is_empty() || maximum.get() > 1024 * 1024 * 1024 {
        return Err(Failure::Arguments);
    }
    let binding_bytes = read(binding_path, 4096)?;
    let binding: Binding =
        serde_json::from_slice(&binding_bytes).map_err(|_| Failure::Arguments)?;
    let permission = binding.permission()?;
    validate_namespaces(&binding, permission)?;
    let installation =
        installation::Installation::load(installation_path, &binding, permission, maximum)?;
    let manifest_bytes = read(manifest_path, MANIFEST_BYTES)?;
    let limits = installation.limits;
    let declaration = decode_prepared_manifest(
        binding.root.parse().map_err(|_| Failure::Arguments)?,
        permission.upload.bytes,
        &manifest_bytes,
        PreparedManifestLimits {
            max_json_bytes: usize::try_from(MANIFEST_BYTES)
                .expect("256 KiB fits usize")
                .try_into()
                .expect("positive JSON bound"),
            manifest: limits,
        },
    )
    .map_err(|_| Failure::PreparedManifest)?;
    validate_declaration(permission, &declaration, limits)
        .map_err(|_| Failure::PreparedManifest)?;
    let request = UploadManifestRequest {
        permission,
        declaration,
    };
    let admission = candid::encode_one(permission).map_err(|_| Failure::Arguments)?;
    let references = ReferenceFiles::new(permission.upload, permission.upload.first_reference)?;
    let preparation = candid::encode_one(&request).map_err(|_| Failure::Arguments)?;
    if admission.len() > 4096 || preparation.len() > 65536 {
        return Err(Failure::ReplyLimit);
    }
    let browser = serde_json::to_vec(&BrowserCertificateBinding::new(
        permission,
        admission.clone(),
        &binding.project,
        &binding.bucket,
    ))
    .map_err(|_| Failure::Arguments)?;
    let source = LocalBody::open(body_path, permission.upload.bytes)?;
    // All bounded decoding and canonical conversion precedes claiming the output.
    let run = Run::create(directory)?;
    run.bytes("binding.json", &binding_bytes)?;
    run.bytes("manifest.json", &manifest_bytes)?;
    run.bytes("installation.candid", &installation.bytes)?;
    let hashes = snapshot(source, &request, maximum, &run)?;
    run.bytes("permission.candid", &admission)?;
    run.bytes("manifest.candid", &preparation)?;
    references.save(&run)?;
    run.bytes("certificate-binding.json", &browser)?;
    let result = json!({
        "schema": 1, "observation": "local_upload_inputs",
        "installation_file": "installation.candid",
        "installation_sha256": digest(&installation.bytes),
        "installation_binding_checked": true,
        "installed_state_observed": false, "namespace_provisioned": false,
        "binding_sha256": digest(&binding_bytes), "manifest_sha256": digest(&manifest_bytes),
        "permission_sha256": digest(&admission), "preparation_sha256": digest(&preparation),
        "root": binding.root, "bytes": binding.bytes,
        "chunks": request.declaration.chunks.len(),
        "permission_file": "permission.candid", "preparation_file": "manifest.candid",
        "first_reference": references.summary(),
        "certificate_binding_file": "certificate-binding.json",
        "project": binding.project, "bucket": binding.bucket,
        "certificate_binding_sha256": digest(&browser),
        "body_file": "body.bin", "content_digest": hashes.content_digest.to_string(),
        "authenticated": false, "identities_allocated": false, "body_verified": true,
        "service_dispatched": false, "provider_dispatched": false,
    });
    run.json("summary.json", &result)?;
    Ok(result)
}

/// Publish verified bytes before any usable request or browser binding output.
fn snapshot(
    source: LocalBody,
    request: &UploadManifestRequest,
    maximum: NonZeroU64,
    run: &Run,
) -> Result<CaffeineContentHashes, Failure> {
    let mut body = run.open_body()?;
    let root = ProviderRootHash::try_from(request.permission.upload.root.as_slice())
        .expect("fixed provider root");
    let hashes = match source.verify(root, &request.declaration, maximum, &mut body) {
        Ok(hashes) => hashes,
        Err(error) => {
            run.json(
                "failure.json",
                &FailureRecord {
                    error: error.code(),
                },
            )?;
            return Err(error);
        }
    };
    body.sync_all().map_err(|_| Failure::File)?;
    drop(body);
    run.publish_body()?;
    Ok(hashes)
}
fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut result, byte| {
            write!(result, "{byte:02x}").expect("String writes cannot fail");
            result
        })
}

#[cfg(test)]
mod tests;
