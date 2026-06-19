mod response;
use chrono::{DateTime, FixedOffset};
use response::Root;

use crate::{utils::http_get_bz, VtClient, VtResult};

impl VtClient<'_> {
    pub async fn url_feed(&self, time: DateTime<FixedOffset>) -> VtResult<Vec<Root>> {
        //! Get a URL feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //! use chrono::DateTime;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhhmm
        //! vt.url_feed(DateTime::now());
        //! ```
        let url = format!("{}/feeds/urls/{}", self.endpoint, time.format("%Y%m%d%H%M"));
        http_get_bz(&self.api_key, &self.user_agent, &url).await
    }

    pub async fn url_feed_hourly(&self, time: DateTime<FixedOffset>) -> VtResult<Vec<Root>> {
        //! Hourly public_api.file feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //! use chrono::DateTime;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhh
        //! vt.url_feed(DateTime::now());
        //! ```
        let url = format!("{}/feeds/urls/hourly/{}", self.endpoint, time.format("%Y%m%d%H%M"));
        http_get_bz(&self.api_key, &self.user_agent, &url).await
    }
}
