//! One lifetime command in the existing fixture store, never a production operator journal.
use super::ConsumerRecord;
use blob_test_protocol::consumer::Failure;
use candid::{CandidType, Principal};
use ic_blob_storage::dto::tenant::{TenantEnrollment, TenantScope, TenantUpdateRequest};
use serde::Deserialize;

#[derive(Clone, CandidType, Deserialize)]
pub(crate) struct TenantCommandRecord {
    service: Principal,
    namespace: u128,
    tenant: Principal,
    expected: Option<(u64, bool)>,
    active: bool,
}
impl TenantCommandRecord {
    fn request(&self) -> TenantUpdateRequest {
        TenantUpdateRequest {
            scope: TenantScope {
                service: self.service,
                namespace: self.namespace,
                tenant: self.tenant,
            },
            expected: self
                .expected
                .map(|(generation, active)| TenantEnrollment { generation, active }),
            active: self.active,
        }
    }
}
impl ConsumerRecord {
    pub(crate) fn validate_tenant_command(&self) -> Result<(), Failure> {
        if let Some(record) = &self.tenant_command {
            self.check_tenant_scope(record.request().scope)?;
            if record
                .expected
                .is_some_and(|(generation, _)| generation == 0)
            {
                return Err(Failure::Invalid);
            }
        }
        Ok(())
    }
    pub(crate) fn check_tenant_scope(&self, scope: TenantScope) -> Result<(), Failure> {
        if scope.service != self.service
            || scope.namespace == 0
            || [Principal::anonymous(), Principal::management_canister()].contains(&scope.tenant)
        {
            return Err(Failure::Invalid);
        }
        Ok(())
    }
    pub(crate) fn start_tenant_command(
        &mut self,
        input: TenantUpdateRequest,
    ) -> Result<(), Failure> {
        self.check_tenant_scope(input.scope)?;
        if let Some(record) = &self.tenant_command {
            return Err(if record.request() == input {
                Failure::Pending
            } else {
                Failure::Conflict
            });
        }
        self.tenant_command = Some(TenantCommandRecord {
            service: input.scope.service,
            namespace: input.scope.namespace,
            tenant: input.scope.tenant,
            expected: input.expected.map(|v| (v.generation, v.active)),
            active: input.active,
        });
        Ok(())
    }
    pub(crate) fn tenant_command(&self) -> Option<TenantUpdateRequest> {
        self.tenant_command
            .as_ref()
            .map(TenantCommandRecord::request)
    }
}
