//! Offline upstream-manifest conversion; no authority, allocation or dispatch.
mod installation;
use super::{
    Failure,
    artifacts::{FailureRecord, Run},
    local_body::LocalBody,
    read,
    reference_inputs::ReferenceFiles,
};
#[cfg(test)]
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
    fmt::Write,
    num::NonZeroU64,
    path::{Path, PathBuf},
};

pub(super) const MANIFEST_BYTES: u64 = 256 * 1024;
pub(super) const BINDING_BYTES: u64 = 12 * 1024;

/// Exact bounded metadata and an unopened body selected by the offline caller.
pub(super) struct InputFiles {
    pub binding: Vec<u8>,
    pub manifest: Vec<u8>,
    pub installation: Vec<u8>,
    pub body: PathBuf,
}

/// Shared one-file validation and streaming snapshot for single and batch input tools.
pub(super) struct PreparedInput {
    files: InputFiles,
    binding: Binding,
    pub permission: UploadAdmissionRequest,
    request: UploadManifestRequest,
    pub resources: ic_blob_storage::dto::configuration::ServiceResourceInput,
    maximum: NonZeroU64,
    admission: Vec<u8>,
    preparation: Vec<u8>,
    references: ReferenceFiles,
    browser: Vec<u8>,
}

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
    preparation: PreparationHints,
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

/// Exact caller-selected SDK hints; omitted arguments stay omitted on restart.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PreparationHints {
    #[serde(
        default,
        deserialize_with = "hint",
        skip_serializing_if = "Option::is_none"
    )]
    content_type: Option<String>,
    #[serde(
        default,
        deserialize_with = "hint",
        skip_serializing_if = "Option::is_none"
    )]
    filename: Option<String>,
}

fn hint<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Option<String>, D::Error> {
    let value = String::deserialize(decoder)?;
    if value.len() > 4096 {
        return Err(serde::de::Error::custom(
            "preparation hint exceeds byte bound",
        ));
    }
    Ok(Some(value))
}

use super::parsing::positive;
use super::parsing::principal;
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
    let mut flags = super::parsing::flags(&args[1..])?;
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
    PreparedInput::load(
        InputFiles {
            binding: read(binding_path, BINDING_BYTES)?,
            manifest: read(manifest_path, MANIFEST_BYTES)?,
            installation: read(installation_path, super::candidate_candid::MAX_BYTES as u64)?,
            body: body_path.to_owned(),
        },
        maximum,
    )?
    .publish(directory, None)
}

impl PreparedInput {
    /// Cached original inputs for the maintained browser; it must snapshot and
    /// reverify the selected body's digest/SDK root before certificate intent.
    pub fn transfer_input(&self, body_sha256: &str) -> Result<Value, Failure> {
        Ok(
            json!({"binding":serde_json::from_slice::<Value>(&self.browser)
            .map_err(|_| Failure::Binding)?,
            "body":self.files.body,"body_sha256":body_sha256,
            "manifest_json":std::str::from_utf8(&self.files.manifest).map_err(|_| Failure::Binding)?,
            "preparation":self.binding.preparation,
            "bytes":self.permission.upload.bytes.to_string()}),
        )
    }
    /// Original maintained request packets for durable indexed setup.
    pub fn setup_requests(&self) -> (&[u8], &[u8]) {
        (&self.admission, &self.preparation)
    }
    pub fn declaration(
        &self,
    ) -> &ic_blob_storage::dto::upload::manifest::UploadManifestDeclaration {
        &self.request.declaration
    }
    pub fn check_requests(
        &self,
        permission: &[u8],
        manifest: &[u8],
        browser: &[u8],
        installation: &[u8],
    ) -> Result<(), Failure> {
        if permission != self.admission
            || manifest != self.preparation
            || browser != self.browser
            || installation != self.files.installation
        {
            return Err(Failure::Binding);
        }
        Ok(())
    }

    pub fn metadata_demand(&self) -> (u64, u64) {
        (
            self.request.declaration.headers.len() as u64,
            self.request
                .declaration
                .headers
                .iter()
                .map(|h| (h.name.len() + h.value.len() + 3) as u64)
                .sum(),
        )
    }

    pub fn verify_body(&self, expected: &str) -> Result<(), Failure> {
        let hashes = LocalBody::open(&self.files.body, self.permission.upload.bytes)?.verify(
            ProviderRootHash::try_from(self.permission.upload.root.as_slice()).expect("fixed root"),
            &self.request.declaration,
            self.maximum,
            &mut std::io::sink(),
        )?;
        if hashes.content_digest.to_string() != format!("sha256:{expected}") {
            return Err(Failure::Content);
        }
        Ok(())
    }

    pub fn project(&self) -> &str {
        &self.binding.project
    }
    pub fn bucket(&self) -> &str {
        &self.binding.bucket
    }
    pub fn chunks(&self) -> usize {
        self.request.declaration.chunks.len()
    }

    pub fn load(files: InputFiles, maximum: NonZeroU64) -> Result<Self, Failure> {
        let binding_bytes = &files.binding;
        let binding: Binding =
            serde_json::from_slice(binding_bytes).map_err(|_| Failure::Arguments)?;
        let permission = binding.permission()?;
        validate_namespaces(&binding, permission)?;
        let installation =
            installation::Installation::decode(&files.installation, &binding, permission, maximum)?;
        let manifest_bytes = &files.manifest;
        let limits = installation.limits;
        let declaration = decode_prepared_manifest(
            binding.root.parse().map_err(|_| Failure::Arguments)?,
            permission.upload.bytes,
            manifest_bytes,
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
        Ok(Self {
            resources: installation.resources,
            files,
            binding,
            permission,
            request,
            maximum,
            admission,
            preparation,
            references,
            browser,
        })
    }

    pub fn check_body(&self) -> Result<(), Failure> {
        LocalBody::open(&self.files.body, self.permission.upload.bytes).map(drop)
    }

    pub fn publish(
        self,
        directory: &Path,
        expected_digest: Option<&str>,
    ) -> Result<Value, Failure> {
        let Self {
            files,
            binding,
            request,
            maximum,
            admission,
            preparation,
            references,
            browser,
            ..
        } = self;
        let source = LocalBody::open(&files.body, request.permission.upload.bytes)?;
        // All bounded decoding and canonical conversion precedes claiming the output.
        let run = Run::create(directory)?;
        run.bytes("binding.json", &files.binding)?;
        run.bytes("manifest.json", &files.manifest)?;
        run.bytes("installation.candid", &files.installation)?;
        let hashes = snapshot(source, &request, maximum, &run)?;
        if expected_digest.is_some_and(|expected| {
            hashes.content_digest.to_string() != format!("sha256:{expected}")
        }) {
            run.json(
                "failure.json",
                &FailureRecord {
                    error: Failure::Content.code(),
                },
            )?;
            return Err(Failure::Content);
        }
        run.bytes("permission.candid", &admission)?;
        run.bytes("manifest.candid", &preparation)?;
        references.save(&run)?;
        run.bytes("certificate-binding.json", &browser)?;
        let result = json!({
            "schema": 1, "observation": "local_upload_inputs",
            "installation_file": "installation.candid",
            "installation_sha256": digest(&files.installation),
            "installation_binding_checked": true,
            "installed_state_observed": false, "namespace_provisioned": false,
            "binding_sha256": digest(&files.binding), "manifest_sha256": digest(&files.manifest),
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
pub(super) fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .fold(String::with_capacity(64), |mut result, byte| {
            write!(result, "{byte:02x}").expect("String writes cannot fail");
            result
        })
}

#[cfg(test)]
pub(in crate::native) mod tests;
