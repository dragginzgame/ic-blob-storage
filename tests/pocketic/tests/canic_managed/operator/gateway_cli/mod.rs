//! Same native gateway controls through the public managed service fixture.
use super::Journey;
use crate::{
    account_native_cli::signer,
    canic_managed::cli::{live, local_subnet_key},
    gateway_native_cli::GatewayCli,
};
use std::time::Duration;

#[test]
fn managed_signed_gateway_decisions_preserve_uncertainty_and_occupied_restore_fences() {
    let j = Journey::with_operator(signer());
    let root = local_subnet_key(&j.f);
    let (mut gateway, url) = live(&j.f);
    let cli = GatewayCli::new(j.scope, j.gateway, url, &root, "managed");
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    cli.refusals();
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    let pending = cli.decisions(|mode| j.mode(mode));
    j.f.pic().stop_progress();
    j.f.upgrade_same_release(Duration::from_secs(5));
    let before = j.f.pic().get_stable_memory(j.f.app());
    let source = j.f.pic().get_stable_memory(j.scope.cashier);
    j.f.pic().auto_progress();
    cli.fenced(&pending);
    assert_eq!(j.f.pic().get_stable_memory(j.f.app()), before);
    assert_eq!(j.f.pic().get_stable_memory(j.scope.cashier), source);
    gateway.stop_live();
}
