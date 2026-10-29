//! Native transport policy; callers retain SDK and response-body handling.
use std::time::Duration;

pub(crate) fn client_builder(network: &str) -> reqwest::ClientBuilder {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .retry(reqwest::retry::never())
        .timeout(Duration::from_secs(30));
    // Local replica/provider requests must not leave loopback through a proxy.
    if network == "local" {
        client.no_proxy()
    } else {
        client
    }
}
