mod response;
use chrono::{DateTime, FixedOffset};
use response::Root;

use crate::{utils::http_get_bz, VtClient, VtResult};

impl VtClient<'_> {
    pub async fn file_feed(&self, time: DateTime<FixedOffset>) -> VtResult<Vec<Root>> {
        //! Get a file feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //! use chrono::DateTime;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhhmm
        //! vt.file_feed(DateTime::now());
        //! ```
        let url = format!("{}/feeds/files/{}", self.endpoint, time.format("%Y%m%d%H%M"));
        http_get_bz(&self.api_key, &self.user_agent, &url).await
    }

    pub async fn file_feed_behaviours(&self, time: DateTime<FixedOffset>) -> VtResult<Vec<Root>> {
        //! Get a file sandbox behaviour feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //! use chrono::DateTime;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhhmm
        //! vt.file_feed(DateTime::now());
        //! ```
        let url = format!("{}/feeds/file-behaviours/{}", self.endpoint, time.format("%Y%m%d%H%M"));
        http_get_bz(&self.api_key, &self.user_agent, &url).await
    }

    pub async fn file_feed_behaviours_hourly(&self, time: DateTime<FixedOffset>) -> VtResult<Vec<Root>> {
        //! Hourly file feed sandbox behaviour feed batch.
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //! use chrono::DateTime;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhh
        //! vt.file_feed(DateTime::now());
        //! ```
        let url = format!("{}/feeds/file-behaviours/hourly/{}", self.endpoint, time.format("%Y%m%d%H"));
        http_get_bz(&self.api_key, &self.user_agent, &url).await
    }

    pub async fn file_feed_hourly(&self, time: DateTime<FixedOffset>) -> VtResult<Vec<Root>> {
        //! Hourly file feed packages.
        //!
        //! ## Example Usage
        //! ```rust
        //! use async_vt3::VtClient;
        //! use chrono::DateTime;
        //!
        //! let vt = VtClient::new("Your API Key");
        //!
        //! // A string in format YYYYMMDDhh
        //! vt.file_feed(DateTime::now());
        //! ```
        let url = format!("{}/feeds/files/hourly/{}", self.endpoint, time.format("%Y%m%d%H"));
        http_get_bz(&self.api_key, &self.user_agent, &url).await
    }
}
