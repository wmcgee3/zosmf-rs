#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

//! # z_osmf
//!
//! The VERY work in progress Rust z/OSMF<sup>TM</sup> [^1] Client.
//!
//! ---
//!
//! [^1]: z/OSMF<sup>TM</sup>, z/OS<sup>TM</sup>, and the lowercase letter z<sup>TM</sup> (probably) are trademarks owned by International Business Machines Corporation ("IBM").
//! This crate is not approved, endorsed, acknowledged, or even tolerated by IBM.
//! (Please don't sue me, Big Blue)

pub use bytes::Bytes;

#[cfg(feature = "datasets")]
pub mod datasets;
pub mod error;
#[cfg(feature = "files")]
pub mod files;
#[cfg(feature = "jobs")]
pub mod jobs;

mod convert;
mod utils;

use self::error::Error;

/// # ZOsmf
///
/// Client for interacting with z/OSMF.
///
/// ```
/// # async fn example() -> anyhow::Result<()> {
/// # use z_osmf::ZOsmf;
/// let client_builder = reqwest::ClientBuilder::new();
/// let base_url = "https://zosmf.mainframe.my-company.com";
/// let username = "USERNAME";
///
/// let zosmf = ZOsmf::new(client_builder, base_url)?;
/// zosmf.login(username, "PASSWORD").await?;
///
/// let my_datasets = zosmf.list_datasets(username).build().await?;
///
/// for dataset in my_datasets.items().iter() {
///     println!("{:?}", dataset);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct ZOsmf {
    base_url: Box<str>,
    client: reqwest::Client,
}

impl ZOsmf {
    /// Create a new z/OSMF client.
    ///
    /// # Example
    /// ```
    /// # async fn example() -> anyhow::Result<()> {
    /// # use z_osmf::ZOsmf;
    /// let client_builder = reqwest::ClientBuilder::new();
    /// let base_url = "https://zosmf.mainframe.my-company.com";
    ///
    /// let zosmf = ZOsmf::new(client_builder, base_url)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new<B>(client_builder: reqwest::ClientBuilder, base_url: B) -> Result<Self, Error>
    where
        B: std::fmt::Display,
    {
        let base_url = format!("{}", base_url).trim_end_matches('/').into();
        let client = client_builder.cookie_store(true).build()?;

        Ok(ZOsmf { base_url, client })
    }

    /// Authenticate with z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.login("USERNAME", "PASSWORD").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn login<U, P>(&self, username: U, password: P) -> Result<(), Error>
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        self.client
            .post(format!("{}/zosmf/services/authenticate", self.base_url))
            .basic_auth(username, Some(password))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Logout of z/OSMF.
    ///
    /// <p style="background:rgba(255,181,77,0.16);padding:0.75em;">
    /// <strong>Warning:</strong> Logging out before an action has completed,
    /// like immediately after submitting a job, can cause the action to fail.
    /// </p>
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// zosmf.logout().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn logout(&self) -> Result<(), Error> {
        self.client
            .delete(format!("{}/zosmf/services/authenticate", self.base_url))
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn get_zosmf() -> ZOsmf {
        ZOsmf::new(reqwest::Client::builder(), "https://test.com").unwrap()
    }

    pub(crate) trait GetJson {
        fn json(&self) -> Option<serde_json::Value>;
    }

    impl GetJson for reqwest::Request {
        fn json(&self) -> Option<serde_json::Value> {
            Some(
                serde_json::from_slice(self.body()?.as_bytes()?)
                    .expect("failed to deserialize JSON"),
            )
        }
    }
}
