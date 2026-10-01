//! Check the complete retained observation before signing. Local artifacts are trusted
//! verifier input, not portable service/provider signatures or proof against file tampering.
use super::{Failure, Input};
use crate::native::{
    arguments::Options,
    artifacts::{DownloadOutcomeRecord, HttpResponseRecord},
    attestation::Recovery,
    observe_upload::record::{
        DownloadRequestRecord, ObservationPlanRecord, ObservationSummaryRecord,
    },
    read,
};
use ic_blob_storage::{
    model::{
        identity::{ContentDigest, ProviderRootHash, caffeine::manifest::CaffeineManifestLimits},
        service::read::download::CaffeineDownloadScope,
    },
    ops::{
        caffeine::download::request_target,
        service::uploads::{
            completion::{reply::inspection_request, verification},
            manifests::reply::UploadManifestReplyLimits,
        },
    },
};
use serde::de::DeserializeOwned;
use std::{collections::BTreeMap, path::Path};

pub(super) struct Observation {
    pub recovery: Recovery,
    pub statement: Vec<u8>,
    pub hashes: BTreeMap<&'static str, String>,
}
struct Artifacts<'a> {
    path: &'a Path,
    hashes: BTreeMap<&'static str, String>,
}
struct Records {
    plan: ObservationPlanRecord<String>,
    summary: ObservationSummaryRecord<String>,
    request: DownloadRequestRecord<String>,
    outcome: DownloadOutcomeRecord<String>,
    response: HttpResponseRecord,
}
impl Artifacts<'_> {
    fn bytes(&mut self, name: &'static str, maximum: u64) -> Result<Vec<u8>, Failure> {
        let path = self.path.join(name);
        // Refuse aliases and partial/non-regular files. The directory must remain
        // controlled by the verifier; this is not a hostile-filesystem sandbox.
        if !std::fs::symlink_metadata(&path)
            .map_err(|_| Failure::Observation)?
            .is_file()
        {
            return Err(Failure::Observation);
        }
        let bytes = read(&path, maximum).map_err(|_| Failure::Observation)?;
        self.hashes
            .insert(name, ContentDigest::compute(&bytes).to_string());
        Ok(bytes)
    }
    fn json<T: DeserializeOwned>(&mut self, name: &'static str) -> Result<T, Failure> {
        serde_json::from_slice(&self.bytes(name, 16 * 1024)?).map_err(|_| Failure::Observation)
    }
}
fn require(value: bool) -> Result<(), Failure> {
    if value {
        Ok(())
    } else {
        Err(Failure::Observation)
    }
}
fn time(value: &str) -> Result<u64, Failure> {
    let parsed: u64 = value.parse().map_err(|_| Failure::Observation)?;
    require(parsed.to_string() == value)?;
    Ok(parsed)
}
impl Observation {
    pub fn open(options: &Options, input: &Input) -> Result<Self, Failure> {
        require(
            std::fs::symlink_metadata(&input.directory)
                .map_err(|_| Failure::Observation)?
                .is_dir(),
        )?;
        match std::fs::symlink_metadata(input.directory.join("failure.json")) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Ok(_) => return Err(Failure::Observation),
            Err(_) => return Err(Failure::File),
        }
        let mut artifacts = Artifacts {
            path: &input.directory,
            hashes: BTreeMap::new(),
        };
        let records = Records {
            plan: artifacts.json("plan.json")?,
            summary: artifacts.json("summary.json")?,
            request: artifacts.json("download-request.json")?,
            outcome: artifacts.json("download-outcome.json")?,
            response: artifacts.json("http-response.json")?,
        };
        let statement = artifacts.bytes("statement.candid", 4096)?;
        let recovery = Recovery::decode(
            input.service,
            input.namespace,
            options.actor,
            options.actor,
            &statement,
        )?;
        // The maintained observer always writes canonical Candid.
        require(
            candid::encode_one(recovery.statement).map_err(|_| Failure::Observation)? == statement,
        )?;
        let authority = recovery.authority;
        let permission = recovery.statement.permission;
        let encoded_permission = artifacts.bytes("permission.candid", 4096)?;
        require(
            inspection_request(authority, permission).map_err(|_| Failure::Observation)?
                == encoded_permission,
        )?;
        let maximum = time(&records.plan.max_content_bytes)?;
        require(maximum > 0 && maximum <= 1024 * 1024 * 1024)?;
        let service_response = artifacts.bytes("service-response.candid", 64 * 1024)?;
        let verified = verification::reply::decode(
            authority,
            permission,
            &service_response,
            UploadManifestReplyLimits {
                max_reply_bytes: (64 * 1024).try_into().unwrap(),
                declaration: CaffeineManifestLimits {
                    max_content_bytes: maximum.try_into().unwrap(),
                    max_chunks: 1024.try_into().unwrap(),
                    max_headers: 16.try_into().unwrap(),
                    max_header_bytes: 4096.try_into().unwrap(),
                },
            },
        )
        .map_err(|_| Failure::Observation)?;
        require(
            records.summary.statement_sha256 == ContentDigest::compute(&statement).to_string()
                && records.request.service_reply_sha256
                    == ContentDigest::compute(&service_response).to_string(),
        )?;
        records.validate(options, input, &recovery, &verified)?;
        Ok(Self {
            recovery,
            statement,
            hashes: artifacts.hashes,
        })
    }
}
impl Records {
    fn validate(
        &self,
        options: &Options,
        input: &Input,
        recovery: &Recovery,
        verified: &ic_blob_storage::dto::upload::completion::UploadVerificationPlan,
    ) -> Result<(), Failure> {
        let Self {
            plan,
            summary,
            request,
            outcome,
            response,
        } = self;
        let permission = recovery.statement.permission;
        require(plan.schema == 1 && summary.schema == 1)?;
        require(
            plan.network == options.network
                && summary.network == options.network
                && plan.service_url == options.url.as_str(),
        )?;
        require(
            plan.service == input.service.to_text()
                && summary.service == plan.service
                && plan.namespace == input.namespace.to_string()
                && summary.namespace == plan.namespace
                && plan.verifier == options.actor.to_text()
                && summary.verifier == plan.verifier,
        )?;
        require(
            plan.max_provider_requests == 1
                && plan.request_deadline_seconds == 30
                && plan.provider_charges == "unknown",
        )?;
        require(
            summary.observation == "provider_content"
                && summary.authentication == "query_signatures"
                && !summary.attestation_dispatched
                && !summary.retry_authorized
                && summary.future_retention == "not_established"
                && summary.billing_cessation == "not_established",
        )?;
        require(
            response.status == 200 && outcome.outcome == "verified" && outcome.error.is_none(),
        )?;
        let size = permission.upload.bytes.to_string();
        require(summary.bytes == size && request.bytes == size && outcome.received_bytes == size)?;
        let root =
            ProviderRootHash::try_from(permission.upload.root.as_slice()).expect("fixed root");
        require(
            summary.root == root.to_string()
                && summary.content_digest
                    == ContentDigest::try_from(recovery.statement.content_digest.as_slice())
                        .expect("fixed digest")
                        .to_string(),
        )?;
        require(summary.observed_at_ns == recovery.statement.observed_at_ns.to_string())?;
        let started = time(&plan.started_at_ns)?;
        let requested = time(&request.started_at_ns)?;
        require(
            started <= requested
                && requested <= recovery.statement.observed_at_ns
                && verified.admitted_at_ns <= recovery.statement.observed_at_ns,
        )?;
        require(
            summary.owner == verified.owner.to_text()
                && request.owner == summary.owner
                && summary.project == verified.project
                && request.project == verified.project
                && summary.gateway_origin == plan.gateway_origin
                && request.method == "GET",
        )?;
        let gateway = url::Url::parse(&plan.gateway_origin).map_err(|_| Failure::Observation)?;
        crate::native::arguments::validate_url(
            &gateway,
            options.network,
            options.network == "local",
        )
        .map_err(|_| Failure::Observation)?;
        let scope = CaffeineDownloadScope::new(
            verified.owner,
            recovery.authority.namespace(),
            &verified.project,
        )
        .map_err(|_| Failure::Observation)?;
        require(
            request.url
                == gateway
                    .join(&request_target(&scope, root))
                    .map_err(|_| Failure::Observation)?
                    .as_str(),
        )?;
        Ok(())
    }
}
