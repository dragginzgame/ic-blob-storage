//! Experimental standalone host. All domain and admission rules live in the library.
//! Exports only owned Wire methods; no Caffeine certificate, callback or network call.

#[cfg_attr(
    not(target_arch = "wasm32"),
    expect(dead_code, reason = "The installed IC entrypoints call these codecs.")
)]
mod codec;
// Platform Runtime exposes IC system calls and transport only on Wasm.
#[cfg(target_arch = "wasm32")]
mod host;
