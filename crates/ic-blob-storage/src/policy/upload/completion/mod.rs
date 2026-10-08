//! Actual caller authorization; no controller, tenant or gateway membership override.
use ic_blob_storage_contracts::upload::binding::UploadContext;
use ic_blob_storage_contracts::upload::completion::CompletionAuthority;
/// Assess a host-installed role against actual execution context.
#[must_use]
pub fn may_attest(authority: CompletionAuthority, context: UploadContext) -> bool {
    context.service == authority.service() && context.actor == authority.verifier()
}
