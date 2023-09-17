use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use zosmf_macros::{Endpoint, Getters};

use crate::data_type::*;
use crate::datasets::utils::*;
use crate::utils::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetRead<T> {
    data: T,
    etag: Option<String>,
    session_ref: Option<String>,
    transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetReadBuilder<T> {
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: String,
    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: String,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member: String,
    #[endpoint(optional, query = "search", builder_fn = "build_search")]
    search_pattern: Option<String>,
    #[endpoint(optional, skip_builder)]
    search_is_regex: bool,
    #[endpoint(optional, skip_builder)]
    search_case_sensitive: bool,
    #[endpoint(optional, skip_builder)]
    search_max_return: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = "build_data_type")]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<String>,
    #[endpoint(optional, builder_fn = "build_return_etag")]
    return_etag: bool,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(optional, header = "X-IBM-Obtain-ENQ")]
    obtain_enq: Option<ObtainEnq>,
    #[endpoint(optional, header = "X-IBM-Session-Ref")]
    session_ref: Option<String>,
    #[endpoint(optional, builder_fn = "build_release_enq")]
    release_enq: bool,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<String>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type_marker: PhantomData<T>,
}

impl<T> DatasetReadBuilder<T> {
    pub fn data_type_binary(self) -> DatasetReadBuilder<Binary> {
        DatasetReadBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn data_type_record(self) -> DatasetReadBuilder<Record> {
        DatasetReadBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Record),
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn data_type_text(self) -> DatasetReadBuilder<Text> {
        DatasetReadBuilder {
            base_url: self.base_url,
            client: self.client,
            search_pattern: self.search_pattern,
            search_is_regex: self.search_is_regex,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member: self.member,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            return_etag: self.return_etag,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }
}

impl<'a> DatasetReadBuilder<Text> {
    pub async fn build(self) -> anyhow::Result<DatasetRead<String>> {
        let response = self.get_response().await?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let data = response.text().await?;

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

impl<B> DatasetReadBuilder<B>
where
    B: BytesDataType,
{
    pub async fn build(self) -> anyhow::Result<DatasetRead<Bytes>> {
        let response = self.get_response().await?;
        let (etag, session_ref, transaction_id) = get_headers(&response)?;
        let data = response.bytes().await?;

        Ok(DatasetRead {
            data,
            etag,
            session_ref,
            transaction_id,
        })
    }
}

fn set_member<T>(
    mut dataset_read_builder: DatasetReadBuilder<T>,
    value: String,
) -> DatasetReadBuilder<T> {
    dataset_read_builder.member = format!("({})", value);

    dataset_read_builder
}

fn set_volume<T>(
    mut dataset_read_builder: DatasetReadBuilder<T>,
    value: String,
) -> DatasetReadBuilder<T> {
    dataset_read_builder.volume = format!("-({})/", value);

    dataset_read_builder
}

fn build_search<T>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder {
    let DatasetReadBuilder {
        search_pattern,
        search_is_regex,
        search_case_sensitive,
        search_max_return,
        ..
    } = &dataset_read_builder;

    if let Some(search) = search_pattern {
        request_builder = request_builder.query(&[(
            if *search_is_regex {
                "research"
            } else {
                "search"
            },
            search,
        )]);
        if *search_case_sensitive {
            request_builder = request_builder.query(&[("insensitive", "false")]);
        }
        if let Some(max) = search_max_return {
            request_builder = request_builder.query(&[("maxreturnsize", max)]);
        }
    }

    request_builder
}

fn build_data_type<T>(
    request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder {
    let DatasetReadBuilder {
        data_type,
        encoding,
        ..
    } = &dataset_read_builder;

    let key = "X-IBM-Data-Type";

    match (data_type, encoding) {
        (Some(data_type), Some(encoding)) => {
            request_builder.header(key, format!("{};fileEncoding={}", data_type, encoding))
        }
        (Some(data_type), None) => request_builder.header(key, format!("{}", data_type)),
        (None, Some(encoding)) => {
            request_builder.header(key, format!("text;fileEncoding={}", encoding))
        }
        (None, None) => request_builder,
    }
}

fn build_release_enq<T>(
    mut request_builder: reqwest::RequestBuilder,
    builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder {
    if builder.release_enq {
        request_builder = request_builder.header("X-IBM-Release-ENQ", "true");
    }

    request_builder
}

fn build_return_etag<T>(
    mut request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &DatasetReadBuilder<T>,
) -> reqwest::RequestBuilder {
    if dataset_read_builder.return_etag {
        request_builder = request_builder.header("X-IBM-Return-Etag", "true");
    }

    request_builder
}

fn get_headers(
    response: &reqwest::Response,
) -> anyhow::Result<(Option<String>, Option<String>, String)> {
    Ok((
        get_etag(response)?,
        get_session_ref(response)?,
        get_transaction_id(response)?,
    ))
}
