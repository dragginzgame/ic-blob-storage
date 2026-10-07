//! Headless test extraction, not a production uploader or gateway verifier.
use super::*;
use ic_agent::{
    agent::{Envelope, signed_update_inspect},
    lookup_value, to_request_id,
};

const MAX_CERTIFICATE_BYTES: usize = 128 * 1024;

#[derive(Debug, Eq, PartialEq)]
pub(super) enum ProofError {
    Size,
    Intent,
    Encoding,
    Signature,
    Status,
    Reply,
}

/// Bind the signed envelope, exact request ID and expected uploader/service/root
/// before interpreting any certificate. Mainnet trust must never come from status.
pub(super) fn extract(
    agent: &Agent,
    intent: &SignedUpdate,
    expected: &ExpectedUpload,
    bytes: &[u8],
) -> Result<Vec<u8>, ProofError> {
    if bytes.len() > MAX_CERTIFICATE_BYTES {
        return Err(ProofError::Size);
    }
    let arg = candid::encode_one(&expected.root).map_err(|_| ProofError::Intent)?;
    if intent.sender != expected.uploader
        || intent.canister_id != expected.service
        || intent.effective_canister_id != expected.service
        || intent.method_name != METHOD
        || intent.arg != arg
    {
        return Err(ProofError::Intent);
    }
    signed_update_inspect(
        expected.uploader,
        expected.service,
        METHOD,
        &arg,
        intent.ingress_expiry,
        intent.signed_update.clone(),
    )
    .map_err(|_| ProofError::Intent)?;
    let envelope: Envelope<'_> =
        serde_cbor::from_slice(&intent.signed_update).map_err(|_| ProofError::Intent)?;
    if to_request_id(&envelope.content).map_err(|_| ProofError::Intent)? != intent.request_id {
        return Err(ProofError::Intent);
    }
    let cert: ic_agent::Certificate =
        serde_cbor::from_slice(bytes).map_err(|_| ProofError::Encoding)?;
    agent
        .verify(&cert, expected.service)
        .map_err(|_| ProofError::Signature)?;
    let path = [
        b"request_status".as_slice(),
        intent.request_id.as_slice(),
        b"status",
    ];
    if lookup_value(&cert, path).map_err(|_| ProofError::Status)? != b"replied" {
        return Err(ProofError::Status);
    }
    let path = [
        b"request_status".as_slice(),
        intent.request_id.as_slice(),
        b"reply",
    ];
    let reply = lookup_value(&cert, path).map_err(|_| ProofError::Reply)?;
    let decoded: ProviderReply = candid::decode_one(reply).map_err(|_| ProofError::Reply)?;
    if decoded.method != "upload" || decoded.blob_hash != expected.root {
        return Err(ProofError::Reply);
    }
    Ok(reply.to_vec())
}

/// One raw v4 call, preserving the provider-facing certificate bytes. No automatic
/// resubmit, redirects, provider request or interpretation of HTTP success as issuance.
pub(super) async fn submit(url: &str, intent: &SignedUpdate) -> Vec<u8> {
    let client = reqwest::Client::builder()
        .no_proxy()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(20))
        .build()
        .unwrap();
    let mut response = client
        .post(format!(
            "{url}api/v4/canister/{}/call",
            intent.effective_canister_id
        ))
        .header("Content-Type", "application/cbor")
        .body(intent.signed_update.clone())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.unwrap() {
        assert!(body.len() + chunk.len() <= MAX_CERTIFICATE_BYTES * 2);
        body.extend_from_slice(&chunk);
    }
    match serde_cbor::from_slice::<ic_agent::TransportCallResponse>(&body).unwrap() {
        ic_agent::TransportCallResponse::Replied { certificate } => certificate,
        response => panic!("local v4 call did not produce a certificate: {response:?}"),
    }
}
