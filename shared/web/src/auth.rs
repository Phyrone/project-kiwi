use std::time::Duration;

pub const WEBAUTHN_CHALLENGE_TIMEOUT_SECONDS: u64 = 5 * 60;
pub const WEBAUTHN_CHALLENGE_TIMEOUT: Duration =
    Duration::from_secs(WEBAUTHN_CHALLENGE_TIMEOUT_SECONDS);
pub const WEBAUTHN_CHALLENGE_PREFIX: &str = "webauthn/challenge";
