#![forbid(unsafe_code)]

//! # z_osmf
//!
//! The (work in progress) Rust z/OSMF Client.

pub mod error;

#[cfg(feature = "datasets")]
pub mod datasets;
#[cfg(feature = "files")]
pub mod files;
#[cfg(feature = "jobs")]
pub mod jobs;

pub use crate::error::Error;

mod convert;
mod utils;

use std::sync::Arc;

#[cfg(feature = "datasets")]
use datasets::DatasetsClient;
#[cfg(feature = "files")]
use files::FilesClient;
#[cfg(feature = "jobs")]
use jobs::JobsClient;

/// # ZOsmf
///
/// Client for interacting with z/OSMF.
///
/// ```
/// # async fn example() -> Result<(), z_osmf::Error> {
/// # use z_osmf::ZOsmf;
/// let client_builder = reqwest::ClientBuilder::new();
/// let base_url = "https://zosmf.mainframe.my-company.com";
/// let username = "USERNAME";
///
/// let zosmf = ZOsmf::new(client_builder, base_url)?;
/// zosmf.login(username, "PASSWORD").await?;
///
/// let my_datasets = zosmf.datasets().list(username).build().await?;
///
/// for dataset in my_datasets.items().iter() {
///     println!("{:?}", dataset);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct ZOsmf {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl ZOsmf {
    /// Create a new z/OSMF client.
    ///
    /// # Example
    /// ```
    /// # async fn example() -> Result<(), z_osmf::Error> {
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
        let base_url: Arc<str> = format!("{}", base_url)
            .trim_end_matches('/')
            .to_string()
            .into();
        let client = client_builder.cookie_store(true).build()?;

        Ok(ZOsmf { base_url, client })
    }

    /// Authenticate with z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
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
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
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

    /// Create a [DatasetsClient] for working with datasets via z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
    /// let dataset_client = zosmf.datasets();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "datasets")]
    pub fn datasets(&self) -> DatasetsClient {
        DatasetsClient::new(self.base_url.clone(), self.client.clone())
    }

    /// Create a [FilesClient] for working with files via z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
    /// let files_client = zosmf.files();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "files")]
    pub fn files(&self) -> FilesClient {
        FilesClient::new(self.base_url.clone(), self.client.clone())
    }

    /// Create a [JobsClient] for working with jobs via z/OSMF.
    ///
    /// # Example
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> Result<(), z_osmf::Error> {
    /// let jobs_client = zosmf.jobs();
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(feature = "jobs")]
    pub fn jobs(&self) -> JobsClient {
        JobsClient::new(self.base_url.clone(), self.client.clone())
    }
}
