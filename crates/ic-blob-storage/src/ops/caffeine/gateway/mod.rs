//! Bounded decoding and application of Cashier gateway-list replies.
//!
//! Selected schema: `storage_gateway_list_v1 : () -> (vec principal) query` in
//! `docs/evidence/caffeine-cashier.did`, SHA-256
//! `232b08e4514048d4de48d6d1bf4387f577bfb64c7e2e2ded699a5e52d475d76f`.
//! No transport, caller authentication, persistence or automatic sync occurs here.

use std::num::NonZeroUsize;

use candid::{Principal, de::DecoderConfig, decode_one_with_config};
use thiserror::Error;

use crate::model::gateway::registry::{
    GatewayRegistry, GatewayScope, GatewaySyncError, GatewaySyncToken,
};

/// Explicit Candid resource bounds, separate from registry membership limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewayReplyLimits {
    /// Bound supplied bytes before decoding; transport buffering needs its own bound.
    pub max_bytes: NonZeroUsize,
    /// Maximum Candid decoding work under the decoder's cost model.
    pub decoding_quota: NonZeroUsize,
    /// Maximum work spent skipping additional fields or arguments.
    pub skipping_quota: NonZeroUsize,
    /// Maximum entries in the Candid type table.
    pub max_type_entries: NonZeroUsize,
}

/// Decode and apply a reply to the exact pending sync, without partial mutation.
///
/// The caller must establish the response's Cashier and supply trusted transport
/// scope; the payload cannot assert its own authority. Scope and token checks
/// precede decoding. Byte/work/type bounds constrain decoding; registry raw and
/// distinct entry limits validate the decoded list before replacement. They do
/// not bound transport buffering or prevent decoding allocation in advance.
///
/// On failure the pending attempt remains available for explicit cancellation or
/// a valid response. Success consumes it. Neither result authorizes paid effects.
/// # Errors
/// Rejects wrong/stale correlation, oversized, malformed or over-budget Candid,
/// and invalid membership candidates, preserving all registry state on failure.
pub fn apply_gateway_sync_reply(
    registry: &mut GatewayRegistry,
    token: GatewaySyncToken,
    response_scope: GatewayScope,
    bytes: &[u8],
    limits: GatewayReplyLimits,
) -> Result<(), GatewayReplyError> {
    registry.check_sync(token, response_scope)?;
    if bytes.len() > limits.max_bytes.get() {
        return Err(GatewayReplyError::ReplyTooLarge);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(limits.decoding_quota.get())
        .set_skipping_quota(limits.skipping_quota.get())
        .set_max_type_len(limits.max_type_entries.get())
        .set_full_error_message(false);
    let principals: Vec<Principal> =
        decode_one_with_config(bytes, &config).map_err(|_| GatewayReplyError::InvalidReply)?;
    registry.apply_sync(token, response_scope, &principals)?;
    Ok(())
}

/// A gateway-list reply rejected without changing membership or pending state.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum GatewayReplyError {
    /// Byte limit exceeded before decoding.
    #[error("gateway reply exceeds byte limit")]
    ReplyTooLarge,
    /// Malformed, incompatible or above configured decoder work/type bounds.
    #[error("invalid or over-budget gateway reply")]
    InvalidReply,
    /// Registry correlation or membership validation failed.
    #[error(transparent)]
    Sync(#[from] GatewaySyncError),
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU128;

    use super::*;
    use crate::model::gateway::{
        GatewayListError, GatewayListLimits, membership::GatewayMembership,
    };

    fn n(value: usize) -> NonZeroUsize {
        NonZeroUsize::new(value).expect("positive bound")
    }

    fn p(value: u8) -> Principal {
        Principal::from_slice(&[value, 1])
    }

    fn registry() -> GatewayRegistry {
        let scope =
            GatewayScope::new(p(1), NonZeroU128::new(1).expect("namespace"), p(2)).expect("scope");
        let mut membership = GatewayMembership::new(GatewayListLimits {
            max_entries: n(3),
            max_unique: n(2),
        });
        membership.add(p(3)).expect("initial gateway");
        GatewayRegistry::new(scope, membership)
    }

    fn limits() -> GatewayReplyLimits {
        GatewayReplyLimits {
            max_bytes: n(4096),
            decoding_quota: n(100_000),
            skipping_quota: n(1000),
            max_type_entries: n(32),
        }
    }

    #[test]
    fn independent_candid_fixture_applies_the_observed_gateway_list() {
        // didc 0.5.4 encoded the retained public query observation. This proves
        // codec compatibility, not the source/freshness of any future response.
        let hex = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/caffeine-gateway/observed.hex"
        ));
        let (pairs, remainder) = hex.trim().as_bytes().as_chunks::<2>();
        assert!(remainder.is_empty(), "complete fixture hex bytes");
        let bytes: Vec<u8> = pairs
            .iter()
            .map(|pair| {
                u8::from_str_radix(std::str::from_utf8(pair).expect("ASCII hex"), 16)
                    .expect("fixture hex byte")
            })
            .collect();
        let mut registry = registry();
        let scope = registry.scope();
        let token = registry.begin_sync().expect("sync");
        apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits())
            .expect("independent gateway-list fixture");
        assert_eq!(
            registry.gateways().principals(),
            &[Principal::from_text(
                "jf2g2-sl4zh-zhvyx-v3zcb-hmumz-2at2q-wuamm-4qiml-43msi-r3man-eqe"
            )
            .expect("retained observed gateway")]
        );
    }

    #[test]
    fn malformed_and_over_budget_replies_preserve_the_exact_pending_attempt() {
        let mut registry = registry();
        let scope = registry.scope();
        let token = registry.begin_sync().expect("sync");
        let before = registry.clone();
        let good = candid::encode_one(vec![p(5), p(4), p(5)]).expect("gateway list");
        let extra = candid::encode_args((vec![p(5)], vec![vec!["extra".repeat(20)]]))
            .expect("extra argument");
        let mut trailing = good.clone();
        trailing.push(0);
        for bytes in [
            Vec::new(),
            candid::encode_args(()).expect("missing argument"),
            candid::encode_one(vec![42_u64]).expect("wrong element type"),
            good[..good.len() - 1].to_vec(),
            trailing,
        ] {
            assert_eq!(
                apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits()),
                Err(GatewayReplyError::InvalidReply)
            );
            assert_eq!(registry, before);
        }
        for (bytes, bounded, error) in [
            (
                &good,
                GatewayReplyLimits {
                    max_bytes: n(good.len() - 1),
                    ..limits()
                },
                GatewayReplyError::ReplyTooLarge,
            ),
            (
                &good,
                GatewayReplyLimits {
                    decoding_quota: n(1),
                    ..limits()
                },
                GatewayReplyError::InvalidReply,
            ),
            (
                &extra,
                GatewayReplyLimits {
                    skipping_quota: n(1),
                    ..limits()
                },
                GatewayReplyError::InvalidReply,
            ),
            (
                &extra,
                GatewayReplyLimits {
                    max_type_entries: n(1),
                    ..limits()
                },
                GatewayReplyError::InvalidReply,
            ),
        ] {
            assert_eq!(
                apply_gateway_sync_reply(&mut registry, token, scope, bytes, bounded),
                Err(error)
            );
            assert_eq!(registry, before);
        }
        apply_gateway_sync_reply(
            &mut registry,
            token,
            scope,
            &good,
            GatewayReplyLimits {
                max_bytes: n(good.len()),
                ..limits()
            },
        )
        .expect("valid reply at byte bound after rejection");
        assert_eq!(registry.gateways().principals(), &[p(5), p(4)]);
        let completed = registry.clone();
        assert_eq!(
            apply_gateway_sync_reply(&mut registry, token, scope, &good, limits()),
            Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync))
        );
        assert_eq!(registry, completed);

        let token = registry.begin_sync().expect("next sync");
        apply_gateway_sync_reply(&mut registry, token, scope, &extra, limits())
            .expect("extra data within skip budget");
        assert_eq!(registry.gateways().principals(), &[p(5)]);
    }

    #[test]
    fn decoded_invalid_membership_never_partially_replaces_the_registry() {
        let mut registry = registry();
        let scope = registry.scope();
        let token = registry.begin_sync().expect("sync");
        let before = registry.clone();
        for (principals, error) in [
            (vec![], GatewayListError::Empty),
            (
                vec![p(4); 4],
                GatewayListError::TooManyEntries {
                    actual: 4,
                    maximum: 3,
                },
            ),
            (
                vec![p(4), p(5), p(6)],
                GatewayListError::TooManyUnique { maximum: 2 },
            ),
            (
                vec![p(4), Principal::anonymous()],
                GatewayListError::InvalidPrincipal {
                    index: 1,
                    principal: Principal::anonymous(),
                },
            ),
            (
                vec![p(4), Principal::management_canister()],
                GatewayListError::InvalidPrincipal {
                    index: 1,
                    principal: Principal::management_canister(),
                },
            ),
        ] {
            let bytes = candid::encode_one(principals).expect("gateway list");
            assert_eq!(
                apply_gateway_sync_reply(&mut registry, token, scope, &bytes, limits()),
                Err(GatewayReplyError::Sync(GatewaySyncError::InvalidList(
                    error
                )))
            );
            assert_eq!(registry, before);
        }
        registry
            .cancel_sync(token)
            .expect("caller can abandon rejected reply");
        registry.begin_sync().expect("explicit new attempt");
    }

    #[test]
    fn correlation_rejects_before_parsing_or_byte_limit_checks() {
        let mut registry = registry();
        let scope = registry.scope();
        let token = registry.begin_sync().expect("sync");
        let wrong = GatewayScope::new(p(1), scope.namespace(), p(9)).expect("other cashier");
        let before = registry.clone();
        for bytes in [vec![], vec![0; 4097]] {
            assert_eq!(
                apply_gateway_sync_reply(&mut registry, token, wrong, &bytes, limits()),
                Err(GatewayReplyError::Sync(GatewaySyncError::WrongScope))
            );
            assert_eq!(registry, before);
        }
        registry.remove(p(3));
        let revoked = registry.clone();
        assert_eq!(
            apply_gateway_sync_reply(&mut registry, token, scope, &[], limits()),
            Err(GatewayReplyError::Sync(GatewaySyncError::StaleSync))
        );
        assert_eq!(registry, revoked);
    }
}
