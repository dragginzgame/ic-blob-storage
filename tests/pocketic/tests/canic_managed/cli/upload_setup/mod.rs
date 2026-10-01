//! The same native local upload setup through supported Canic public endpoints.
use super::{Fixture, live, local_subnet_key, manifest};
use crate::{
    account_native_cli::signer,
    upload_setup_cli::{Client, uploader},
};
use ic_blob_storage::dto::tenant::{
    TenantEnrollmentResponse, TenantFailure, TenantScope, TenantUpdateRequest,
};
use ic_testkit::pic::CandidCallExt;
use std::time::Duration;
#[test]
fn managed_signed_upload_setup_recovers_lost_and_pending_replies_then_cancels_and_fences() {
    let f = Fixture::new();
    let config = blob_canic_probe::configuration::input();
    f.configure_and_wait_until_active(100);
    f.pic()
        .update_candid_as::<Result<TenantEnrollmentResponse, TenantFailure>, _>(
            f.app(),
            config.operator,
            "blob_update_tenant",
            (TenantUpdateRequest {
                scope: TenantScope {
                    service: f.app(),
                    namespace: u128::MAX,
                    tenant: signer(),
                },
                expected: None,
                active: true,
            },),
        )
        .unwrap()
        .unwrap();
    let input = manifest(&f, signer(), uploader());
    let trusted = local_subnet_key(&f);
    let (mut replica, url) = live(&f);
    let client = Client::new(input, &[42; 10], &url, &trusted, "managed");
    client.before_restore();
    let scope = ic_blob_storage::dto::operator::OperatorScope {
        service: f.app(),
        namespace: config.namespace,
        cashier: config.billing.cashier,
        payment_account: config.payment_account,
    };
    crate::upload_setup_cli::cancelled_totals(f.pic(), scope, config.operator);
    replica.stop_progress();
    f.upgrade_same_release(Duration::from_secs(5));
    replica.auto_progress();
    let before = f.pic().get_stable_memory(f.app());
    client.restored();
    assert_eq!(f.pic().get_stable_memory(f.app()), before);
    crate::upload_setup_cli::cancelled_totals(f.pic(), scope, config.operator);
    replica.stop_live();
}
