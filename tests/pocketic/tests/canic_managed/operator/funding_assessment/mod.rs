//! The same passive native query through the supported managed artifact and stopped Cashier.
use super::Journey;
use crate::{
    account_native_cli::signer,
    canic_managed::cli::{live, local_subnet_key},
    funding_assessment_cli::AssessmentCli,
};
use std::time::Duration;

#[test]
fn managed_signed_funding_assessment_preserves_occupied_and_restored_state() {
    let j = Journey::with_operator(signer());
    j.f.pic().stop_canister(j.scope.cashier, None).unwrap();
    let root = local_subnet_key(&j.f);
    let (mut gateway, url) = live(&j.f);
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    let cli = AssessmentCli::new(j.scope, &url, &root, "managed");
    cli.inspect(false);
    cli.refusals();
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    j.f.pic().stop_progress();
    j.f.upgrade_same_release(Duration::from_secs(5));
    let restored = j.f.pic().get_stable_memory(j.f.app());
    j.f.pic().auto_progress();
    cli.inspect(true);
    cli.refusals();
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), restored);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    gateway.stop_live();
}
