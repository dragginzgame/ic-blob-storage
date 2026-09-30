//! Bounded extraction of application bytes after Canic authenticates managed init.
use crate::dto::ManagedInstallationInput;
use candid::{DecoderConfig, Reserved, de::IDLDeserialize};
use thiserror::Error;

/// Maximum managed carrier size inspected before copying platform arguments.
pub const CARRIER_BYTES: usize = 256 * 1024;

/// Extract required application bytes from the current two-argument managed init.
/// Invoke only after Canic's synchronous initialization has validated the protected
/// envelope. Skipping that envelope does not authenticate it. The host must decode
/// the returned bytes under its own typed/semantic bounds and supply compiled release.
/// Carrier limit is 256 KiB, application limit 16 KiB; there is no default or fallback.
/// # Errors
/// Rejects missing, malformed, oversized or excessive-work Candid input.
pub fn application_arguments(bytes: &[u8]) -> Result<Vec<u8>, InitializationArgumentFailure> {
    if bytes.len() > CARRIER_BYTES {
        return Err(InitializationArgumentFailure::CarrierBound);
    }
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(2_000_000)
        .set_skipping_quota(200_000)
        .set_max_type_len(256);
    let mut decoder = IDLDeserialize::new_with_config(bytes, &config)
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    decoder
        .get_value::<Reserved>()
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    let application = decoder
        .get_value::<Option<Vec<u8>>>()
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    if !decoder.is_done() {
        return Err(InitializationArgumentFailure::Encoding);
    }
    decoder
        .done()
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    let application = application.ok_or(InitializationArgumentFailure::Missing)?;
    if application.len() > 16 * 1024 {
        return Err(InitializationArgumentFailure::ApplicationBound);
    }
    Ok(application)
}

/// Decode one explicit installation input after authenticated carrier extraction.
/// Semantic policy validation remains with the shared installation owner.
/// # Errors
/// Rejects invalid carrier, missing fields, extra values, trailing bytes or work bounds.
pub fn installation_arguments(
    bytes: &[u8],
) -> Result<ManagedInstallationInput, InitializationArgumentFailure> {
    let application = application_arguments(bytes)?;
    let mut config = DecoderConfig::new();
    config
        .set_decoding_quota(500_000)
        .set_skipping_quota(20_000)
        .set_max_type_len(128);
    let mut decoder = IDLDeserialize::new_with_config(&application, &config)
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    let input = decoder
        .get_value::<ManagedInstallationInput>()
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    if !decoder.is_done() {
        return Err(InitializationArgumentFailure::Encoding);
    }
    decoder
        .done()
        .map_err(|_| InitializationArgumentFailure::Encoding)?;
    Ok(input)
}

/// Managed argument extraction rejection, with no mutation or installation defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Error)]
pub enum InitializationArgumentFailure {
    /// The complete managed carrier exceeds its buffering envelope.
    #[error("managed carrier exceeds byte bound")]
    CarrierBound,
    /// The application payload exceeds the maintained configuration envelope.
    #[error("application arguments exceed byte bound")]
    ApplicationBound,
    /// The application payload is required.
    #[error("application arguments missing")]
    Missing,
    /// Candid encoding or bounded decoding work is invalid.
    #[error("invalid managed argument encoding")]
    Encoding,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_exact_application_bytes_and_rejects_missing_malformed_and_oversized_input() {
        let input = vec![255; 16 * 1024];
        let encoded = candid::encode_args(("protected envelope", Some(input.clone()))).unwrap();
        assert_eq!(application_arguments(&encoded), Ok(input));
        assert_eq!(
            application_arguments(&candid::encode_args((7u8, Some(vec![1u8]), 9u8)).unwrap()),
            Err(InitializationArgumentFailure::Encoding)
        );
        assert_eq!(
            application_arguments(&candid::encode_args((7u8, None::<Vec<u8>>)).unwrap()),
            Err(InitializationArgumentFailure::Missing)
        );
        assert_eq!(
            application_arguments(
                &candid::encode_args((7u8, Some(vec![0u8; 16 * 1024 + 1]))).unwrap()
            ),
            Err(InitializationArgumentFailure::ApplicationBound)
        );
        assert_eq!(
            application_arguments(&vec![0; 256 * 1024 + 1]),
            Err(InitializationArgumentFailure::CarrierBound)
        );
        assert_eq!(
            application_arguments(b"invalid"),
            Err(InitializationArgumentFailure::Encoding)
        );
        let mut trailing = encoded;
        trailing.push(0);
        assert_eq!(
            application_arguments(&trailing),
            Err(InitializationArgumentFailure::Encoding)
        );
    }
}
