use super::*;
use crate::{
    dto::configuration::{
        ServiceBillingInput, ServiceFundingInput, ServiceReadInput, ServiceResourceInput,
    },
    model::service::configuration::{ServiceLimitError, ServicePrincipal},
};

fn p(n: u8) -> Principal {
    Principal::from_slice(&[n, 1])
}
pub(crate) fn candidate() -> ServiceConfigurationInput {
    ServiceConfigurationInput {
        service: p(1),
        operator: p(2),
        payment_account: p(3),
        namespace: u128::MAX,
        resources: ServiceResourceInput {
            max_tenants: 2,
            max_object_bytes: 10,
            max_headers: 8,
            max_header_bytes: 1024,
            max_chunks: 4,
            max_tenant_chunks: 2,
            max_objects: 4,
            max_tenant_objects: 2,
            max_physical_bytes: u128::MAX,
            max_liability_bytes: u128::MAX,
            max_tenant_logical_bytes: u128::MAX,
            max_references_per_object: 2,
            max_receipts_per_object: 3,
            max_active: 4,
            max_tenant_active: 2,
        },
        billing: ServiceBillingInput {
            cashier: p(4),
            reserve: u128::MAX,
            minimum_balance: u128::MAX,
            target_balance: u128::MAX,
            max_gateway_entries: 8,
            max_gateway_unique: 4,
        },
        funding: ServiceFundingInput {
            allocated: u128::MAX,
            reserve: 100,
            max_attempts: 4,
        },
        reads: ServiceReadInput {
            sessions: 2,
            tenant_sessions: 1,
            reply_bytes: 2048,
            bytes: 4096,
            tenant_bytes: 2048,
        },
    }
}
fn decoded(
    input: &ServiceConfigurationInput,
) -> Result<ServiceStoreConfiguration, ConfigurationInputError> {
    decode_candidate(
        p(1),
        &candid::encode_one(input).unwrap(),
        NonZeroUsize::new(4096).unwrap(),
    )
}

#[test]
fn candid_candidate_preserves_full_width_values_and_independent_roles() {
    let input = candidate();
    let stores = decoded(&input).unwrap();
    let config = stores.service();
    assert_eq!(
        config.bindings(),
        ServiceBindings {
            service: p(1),
            operator: p(2),
            payment_account: p(3),
            namespace: NonZeroU128::new(u128::MAX).unwrap()
        }
    );
    assert_eq!(config.billing().cashier(), p(4));
    assert_eq!(
        config.billing().funding_limits(),
        FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).unwrap()
    );
    assert_eq!(config.limits().catalog.max_physical_bytes.get(), u128::MAX);
    assert_eq!(config.limits().catalog.max_liability_bytes.get(), u128::MAX);
    assert_eq!(
        config.limits().catalog.max_tenant_logical_bytes.get(),
        u128::MAX
    );
    assert_eq!(config.manifest_limits().max_content_bytes.get(), 10);
    assert_eq!(config.limits().manifests.max_chunks.get(), 4);
    assert_eq!(config.limits().uploads.max_tenant_active.get(), 2);
    assert_eq!(stores, validate_candidate(p(1), input).unwrap());
    assert_eq!(
        stores.funding().reconstruct(&[]).unwrap().available(),
        u128::MAX
    );
    assert_eq!(stores.reads().max_reply_bytes(), 2048);
}

#[test]
fn host_identity_and_special_principals_fail_without_substitution() {
    let input = candidate();
    assert_eq!(
        validate_candidate(p(9), input),
        Err(ConfigurationInputError::ServiceBinding)
    );
    for principal in [Principal::anonymous(), Principal::management_canister()] {
        let bad = ServiceConfigurationInput {
            operator: principal,
            ..input
        };
        assert_eq!(
            decoded(&bad),
            Err(ServiceConfigurationError::InvalidPrincipal {
                field: ServicePrincipal::Operator
            }
            .into())
        );
        let bad = ServiceConfigurationInput {
            payment_account: principal,
            ..input
        };
        assert_eq!(
            decoded(&bad),
            Err(ServiceConfigurationError::InvalidPrincipal {
                field: ServicePrincipal::PaymentAccount
            }
            .into())
        );
        let bad = ServiceConfigurationInput {
            service: principal,
            ..input
        };
        assert_eq!(
            validate_candidate(principal, bad),
            Err(ServiceConfigurationError::InvalidPrincipal {
                field: ServicePrincipal::Service
            }
            .into())
        );
        let mut bad = input;
        bad.billing.cashier = principal;
        assert_eq!(
            decoded(&bad),
            Err(ConfigurationInputError::Billing(
                BillingConfigurationError::InvalidCashier { principal }
            ))
        );
    }
}

#[test]
fn zero_fields_and_inconsistent_resource_relationships_are_rejected() {
    type Change = fn(&mut ServiceResourceInput);
    let mut input = candidate();
    input.namespace = 0;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::ZeroScalar(
            ConfigurationScalar::Namespace
        ))
    );
    let zero_counts: &[(Change, ServiceCount)] = &[
        (|r| r.max_tenants = 0, ServiceCount::Tenants),
        (|r| r.max_headers = 0, ServiceCount::Headers),
        (|r| r.max_header_bytes = 0, ServiceCount::HeaderBytes),
        (|r| r.max_chunks = 0, ServiceCount::ManifestChunksRetained),
        (
            |r| r.max_tenant_chunks = 0,
            ServiceCount::TenantManifestChunks,
        ),
        (|r| r.max_objects = 0, ServiceCount::Objects),
        (|r| r.max_tenant_objects = 0, ServiceCount::TenantObjects),
        (
            |r| r.max_references_per_object = 0,
            ServiceCount::References,
        ),
        (|r| r.max_receipts_per_object = 0, ServiceCount::Receipts),
        (|r| r.max_active = 0, ServiceCount::Uploads),
        (|r| r.max_tenant_active = 0, ServiceCount::TenantUploads),
    ];
    for (change, field) in zero_counts {
        let mut input = candidate();
        change(&mut input.resources);
        assert_eq!(
            decoded(&input),
            Err(ConfigurationInputError::ZeroCount(*field))
        );
    }
    let zero_scalars: &[(Change, ConfigurationScalar)] = &[
        (|r| r.max_object_bytes = 0, ConfigurationScalar::ObjectBytes),
        (
            |r| r.max_physical_bytes = 0,
            ConfigurationScalar::PhysicalBytes,
        ),
        (
            |r| r.max_liability_bytes = 0,
            ConfigurationScalar::LiabilityBytes,
        ),
        (
            |r| r.max_tenant_logical_bytes = 0,
            ConfigurationScalar::TenantLogicalBytes,
        ),
    ];
    for (change, field) in zero_scalars {
        let mut input = candidate();
        change(&mut input.resources);
        assert_eq!(
            decoded(&input),
            Err(ConfigurationInputError::ZeroScalar(*field))
        );
    }
    let mut input = candidate();
    input.resources.max_receipts_per_object = 2;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Configuration(
            ServiceLimitError::InsufficientReferenceReceipts.into()
        ))
    );
    let mut input = candidate();
    input.resources.max_tenant_active = 3;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Configuration(
            ServiceLimitError::TenantUploadsExceedObjects.into()
        ))
    );
}

#[test]
fn byte_and_decoder_limits_reject_before_any_candidate_is_accepted() {
    let bytes = candid::encode_one(candidate()).unwrap();
    let exact = NonZeroUsize::new(bytes.len()).unwrap();
    assert!(decode_candidate(p(1), &bytes, exact).is_ok());
    assert_eq!(
        decode_candidate(p(1), &bytes, NonZeroUsize::new(bytes.len() - 1).unwrap()),
        Err(ConfigurationInputError::Limit)
    );
    assert_eq!(
        decode_candidate(p(1), &vec![0; bytes.len() + 1], exact),
        Err(ConfigurationInputError::Limit)
    );
    for malformed in [vec![], b"DIDL".to_vec(), candid::encode_one(1_u8).unwrap()] {
        assert_eq!(
            decode_candidate(p(1), &malformed, exact),
            Err(ConfigurationInputError::Encoding)
        );
    }
}

#[test]
fn billing_thresholds_and_gateway_envelope_reuse_model_failures() {
    let mut input = candidate();
    input.billing.target_balance = 1;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Funding(
            FundingLimitsError::MinimumExceedsTarget
        ))
    );
    let mut input = candidate();
    input.billing.max_gateway_unique = input.billing.max_gateway_entries + 1;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Configuration(
            ServiceLimitError::GatewayUniqueExceedsEntries.into()
        ))
    );
    input.billing.max_gateway_entries = 0;
    assert!(matches!(
        decoded(&input),
        Err(ConfigurationInputError::Billing(
            BillingConfigurationError::ZeroGatewayLimit { .. }
        ))
    ));
}

#[test]
fn attachment_allocation_is_separate_and_requires_positive_retained_capacity() {
    let mut input = candidate();
    input.funding.max_attempts = 0;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::FundingCapacity)
    );
    input = candidate();
    input.funding.reserve = 0;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::ZeroScalar(
            ConfigurationScalar::FundingReserve
        ))
    );
    input = candidate();
    input.funding.allocated = input.funding.reserve - 1;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Allocation(
            FundingAllocationError::ReserveExceedsAllocation
        ))
    );
    input.funding.allocated = input.funding.reserve;
    let config = decoded(&input).unwrap();
    // Funding may be configured as entirely reserved, independent of provider thresholds.
    assert_eq!(config.funding().reconstruct(&[]).unwrap().transferable(), 0);
    assert_eq!(
        config.service().billing().funding_limits(),
        FundingLimits::new(u128::MAX, u128::MAX, u128::MAX).unwrap()
    );
}

#[test]
fn read_limits_and_stable_codec_envelopes_are_checked_at_candidate_boundary() {
    let invalid_reads: &[fn(&mut ServiceReadInput)] = &[
        |r| r.sessions = 0,
        |r| r.tenant_sessions = 0,
        |r| r.reply_bytes = 0,
        |r| r.bytes = 0,
        |r| r.tenant_bytes = 0,
        |r| r.sessions = 1025,
        |r| r.tenant_sessions = r.sessions + 1,
        |r| r.reply_bytes = 2049,
        |r| r.bytes = r.tenant_bytes - 1,
    ];
    for change in invalid_reads {
        let mut input = candidate();
        change(&mut input.reads);
        assert_eq!(
            decoded(&input),
            Err(ConfigurationInputError::Stores(ServiceStoreError::Reads(
                ReadSessionError::Limits
            )))
        );
    }
    let mut input = candidate();
    input.resources.max_header_bytes = 65_536;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Stores(ServiceStoreError::Uploads(
            super::super::uploads::UploadStoreError::UnsupportedEnvelope
        )))
    );
    input = candidate();
    input.billing.max_gateway_entries = 1025;
    input.billing.max_gateway_unique = 1025;
    assert_eq!(
        decoded(&input),
        Err(ConfigurationInputError::Stores(
            ServiceStoreError::Gateways(
                super::super::gateways::GatewayStoreError::UnsupportedEnvelope
            )
        ))
    );
}
