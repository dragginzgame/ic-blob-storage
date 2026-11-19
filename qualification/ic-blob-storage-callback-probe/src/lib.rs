//! Local provider candidate: callbacks and a certified-ingress reply, with no provider calls.
//! Never deploy this qualification artifact on a live funded owner.

#[cfg_attr(
    not(target_arch = "wasm32"),
    expect(dead_code, reason = "The installed IC entrypoints call these codecs.")
)]
mod external;
// Platform Runtime exposes IC system calls and transport only on Wasm.
#[cfg(target_arch = "wasm32")]
mod host;
