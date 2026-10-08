//! Explicit immutable resource ceilings, never reservations or accounting.
use std::num::NonZeroU128;
use std::num::NonZeroUsize;
/// Explicit lifetime and outstanding-obligation bounds. No defaults are inferred.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CatalogLimits {
    /// Lifetime object/root slots, including settled objects and their history.
    pub max_objects: NonZeroUsize,
    /// Lifetime object/root slots per tenant across namespaces, including zero bytes.
    pub max_tenant_objects: NonZeroUsize,
    /// Physical bytes across every tenant, including logically released objects.
    pub max_physical_bytes: NonZeroU128,
    /// Bytes with outstanding billing obligations, including physically deleted objects.
    pub max_liability_bytes: NonZeroU128,
    /// Logical bytes per tenant, counted once per live object across namespaces.
    pub max_tenant_logical_bytes: NonZeroU128,
    /// Lifetime reference slots per object, including released identities.
    pub max_references_per_object: NonZeroUsize,
    /// Lifetime request receipts per object; active-reference release slots are reserved.
    pub max_receipts_per_object: NonZeroUsize,
}

/// Additional concurrent upload bounds, independent of lifetime catalog bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UploadLimits {
    /// Maximum reserved or possibly exposed operations across tenants.
    pub max_active: NonZeroUsize,
    /// Maximum reserved or possibly exposed operations per tenant.
    pub max_tenant_active: NonZeroUsize,
}

/// Explicit processing and membership limits; no deployment defaults are chosen.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GatewayListLimits {
    /// Maximum input length, including duplicate principals.
    pub max_entries: NonZeroUsize,
    /// Maximum number of distinct principals retained after normalization.
    pub max_unique: NonZeroUsize,
}
