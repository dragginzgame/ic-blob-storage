//! Print the standalone canister's maintained Candid interface.
fn main() {
    println!("{}", ic_blob_storage_canister::provider_candid_interface());
}
