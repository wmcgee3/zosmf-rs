pub use crate::utils::RecordRange;

use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use z_osmf_macros::Endpoint;

use crate::convert::{TryFromResponse, TryIntoTarget};

use super::JobIdentifier;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReadJobFile<T> {
    data: T,
}

impl ReadJobFile<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for ReadJobFile<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(ReadJobFile {
            data: value.text().await?.into(),
        })
    }
}

impl ReadJobFile<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for ReadJobFile<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::Error> {
        Ok(ReadJobFile {
            data: value.bytes().await?,
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum JobFileID {
    JCL,
    ID(i32),
}

impl std::fmt::Display for JobFileID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobFileID::JCL => write!(f, "JCL"),
            JobFileID::ID(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restjobs/jobs/{subsystem}{identifier}/files/{id}/records")]
pub struct ReadJobFileBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_subsystem)]
    subsystem: Box<str>,
    #[endpoint(path)]
    identifier: JobIdentifier,
    #[endpoint(path)]
    id: JobFileID,
    #[endpoint(optional, header = "X-IBM-Record-Range")]
    record_range: Option<RecordRange>,
    #[endpoint(optional, skip_setter, query = "mode")]
    data_type: Option<DataType>,
    #[endpoint(optional, query = "fileEncoding")]
    encoding: Option<Box<str>>,
    #[endpoint(optional, query = "search")]
    search: Option<Box<str>>,
    #[endpoint(optional, query = "research")]
    search_regex: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_search_case_sensitive)]
    search_case_sensitive: bool,
    #[endpoint(optional, query = "maxreturnsize")]
    search_max_return: Option<i32>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<U> ReadJobFileBuilder<ReadJobFile<U>>
where
    ReadJobFile<U>: TryFromResponse,
{
    pub fn binary(self) -> ReadJobFileBuilder<ReadJobFile<Bytes>> {
        ReadJobFileBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            id: self.id,
            record_range: self.record_range,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            search: self.search,
            search_regex: self.search_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            target_type: PhantomData,
        }
    }

    pub fn record(self) -> ReadJobFileBuilder<ReadJobFile<Bytes>> {
        ReadJobFileBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            id: self.id,
            record_range: self.record_range,
            data_type: Some(DataType::Record),
            encoding: self.encoding,
            search: self.search,
            search_regex: self.search_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> ReadJobFileBuilder<ReadJobFile<Box<str>>> {
        ReadJobFileBuilder {
            base_url: self.base_url,
            client: self.client,
            subsystem: self.subsystem,
            identifier: self.identifier,
            id: self.id,
            record_range: self.record_range,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            search: self.search,
            search_regex: self.search_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            target_type: PhantomData,
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
enum DataType {
    Binary,
    Record,
    Text,
}

fn build_search_case_sensitive<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &ReadJobFileBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    if builder.search_case_sensitive {
        request_builder = request_builder.query(&["insensitive", "false"]);
    }

    request_builder
}

fn set_subsystem<T>(mut builder: ReadJobFileBuilder<T>, value: Box<str>) -> ReadJobFileBuilder<T>
where
    T: TryFromResponse,
{
    builder.subsystem = format!("-{}/", value).into();

    builder
}
