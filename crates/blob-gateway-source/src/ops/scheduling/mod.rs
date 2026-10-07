//! Bounded local scheduling only; random bytes are discarded.
use ic_cdk::call::Call;

pub(super) async fn wait(ready: impl Fn() -> bool) -> bool {
    // Self-calls can drain within one round. Management callbacks let the test
    // driver observe an actual pending call before releasing its captured reply.
    for _ in 0..128 {
        if ready() {
            return true;
        }
        if Call::unbounded_wait(candid::Principal::management_canister(), "raw_rand")
            .await
            .is_err()
        {
            break;
        }
    }
    false
}
