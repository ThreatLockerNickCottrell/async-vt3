/// VirusTotal API v3
/// Clean & Simple interface to access the VirusTotal v3 Public & Enterprise REST api's.
/// ### Available `feature` flags
/// - hunting
/// - feeds
/// - enterprise
///
/// ```rust
/// use async_vt3::VtClient;
///
/// let vt_client = VtClient::new("YOUR API KEY");
/// ```
///
pub mod public_api;

#[cfg(feature = "enterprise")]
mod enterprise;

#[cfg(feature = "feeds")]
mod feeds;

#[cfg(feature = "hunting")]
mod hunting;
#[cfg(feature = "hunting")]
pub use self::hunting::{livehunt::SubmitLivehuntRuleset, retrohunt::SubmitRetrohuntJob};

static DEFAULT_USER_AGENT: &str = "rust-client/vt3+https://github.com/marirs/vt3-rs";

pub mod error;
mod utils;

mod api_key;
pub use crate::api_key::ApiKey;

use error::VtError;
pub type VtResult<T> = Result<T, VtError>;

const API_ENDPOINT: &'static str = "https://www.virustotal.com/api/v3";
#[derive(Clone)]
pub struct VtClient<'a> {
    api_key: ApiKey,
    endpoint: &'a str,
    user_agent: &'a str,
}

impl<'a> VtClient<'a> {
    pub const fn new(api_key: ApiKey) -> Self {
        //! Creates a new VirusTotal API Client to access the VirusTotal REST API v3
        //!
        //! ## Example usage
        //! ```rust
        //! use async_vt3::VtClient;
        //!
        //! let vt_client = VtClient::new("YOUR API KEY");
        //! ```
        VtClient {
            api_key: api_key,
            endpoint: API_ENDPOINT,
            user_agent: DEFAULT_USER_AGENT,
        }
    }

    /// Sets a new user-agent that from the default
    pub fn user_agent<'b: 'a>(self, user_agent: &'a str) -> VtClient {
        Self { user_agent, ..self }
    }
}

#[cfg(test)]
mod tests {
    use crate::ApiKey;

use super::VtClient;

    #[test]
    fn test_vtclient() {
        let vt_client = VtClient::new(ApiKey::default());
        assert_eq!(vt_client.api_key, "000000000000000000000000000000000000000000000000000000000000000");
        assert_eq!(vt_client.endpoint, "https://www.virustotal.com/api/v3")
    }
}
