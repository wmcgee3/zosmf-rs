use std::marker::PhantomData;

use anyhow::Context;
use bytes::Bytes;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use zosmf_macros::{Endpoint, Getters};

use crate::data_type::*;
use crate::datasets::utils::*;
use crate::utils::*;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetWrite {
    etag: String,
    transaction_id: String,
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member_name}")]
pub struct DatasetWriteBuilder<'a, D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(path)]
    dataset_name: String,
    #[endpoint(optional, path, setter_fn = "set_volume")]
    volume: String,
    #[endpoint(optional, path, setter_fn = "set_member")]
    member_name: String,
    #[endpoint(optional, header = "If-Match")]
    if_match: Option<String>,
    #[endpoint(optional, skip_setter, skip_builder)]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_setter, builder_fn = "build_data")]
    data: Option<D>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<String>,
    #[endpoint(optional, skip_builder)]
    crlf_newlines: bool,
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

impl<'a, D, T> DatasetWriteBuilder<'a, D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    pub fn data_type_binary(self, data: Bytes) -> DatasetWriteBuilder<'a, Bytes, Binary> {
        DatasetWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member_name: self.member_name,
            if_match: self.if_match,
            data_type: Some(DataType::Binary),
            data: Some(data),
            encoding: self.encoding,
            crlf_newlines: self.crlf_newlines,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn data_type_record(self, data: Bytes) -> DatasetWriteBuilder<'a, Bytes, Record> {
        DatasetWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member_name: self.member_name,
            if_match: self.if_match,
            data_type: Some(DataType::Record),
            data: Some(data),
            encoding: self.encoding,
            crlf_newlines: self.crlf_newlines,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub fn data_type_text(self, data: String) -> DatasetWriteBuilder<'a, String, Text> {
        DatasetWriteBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            volume: self.volume,
            member_name: self.member_name,
            if_match: self.if_match,
            data_type: Some(DataType::Text),
            data: Some(data),
            encoding: self.encoding,
            crlf_newlines: self.crlf_newlines,
            migrated_recall: self.migrated_recall,
            obtain_enq: self.obtain_enq,
            session_ref: self.session_ref,
            release_enq: self.release_enq,
            dsname_encoding: self.dsname_encoding,
            data_type_marker: PhantomData,
        }
    }

    pub async fn build(self) -> anyhow::Result<DatasetWrite> {
        let response = self.get_response().await?;

        let etag = get_etag(&response)?.context("missing etag")?;
        let transaction_id = get_transaction_id(&response)?;

        Ok(DatasetWrite {
            etag,
            transaction_id,
        })
    }
}

fn build_data<D, T>(
    mut request_builder: RequestBuilder,
    builder: &DatasetWriteBuilder<D, T>,
) -> RequestBuilder
where
    D: Into<reqwest::Body> + Clone,
{
    let key = "X-IBM-Data-Type";
    let DatasetWriteBuilder {
        data_type,
        data,
        encoding,
        crlf_newlines,
        ..
    } = builder;

    request_builder = match (data_type, encoding, crlf_newlines) {
        (data_type, encoding, crlf)
            if data_type.is_none() || *data_type == Some(DataType::Text) =>
        {
            request_builder.header(
                key,
                format!(
                    "text{}{}",
                    if let Some(encoding) = encoding {
                        format!(";fileEncoding={}", encoding)
                    } else {
                        "".to_string()
                    },
                    if *crlf { ";crlf=true" } else { "" }
                ),
            )
        }
        (Some(data_type), _, _) => request_builder.header(key, format!("{}", data_type)),
        _ => request_builder,
    };
    if let Some(value) = data {
        request_builder = request_builder.body(value.clone());
    }

    request_builder
}

fn build_release_enq<D, T>(
    mut request_builder: RequestBuilder,
    builder: &DatasetWriteBuilder<D, T>,
) -> RequestBuilder
where
    D: Into<reqwest::Body> + Clone,
{
    if builder.release_enq {
        request_builder = request_builder.header("X-IBM-Release-ENQ", "true");
    }

    request_builder
}

fn set_member<D, T>(
    mut builder: DatasetWriteBuilder<D, T>,
    value: String,
) -> DatasetWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    builder.member_name = format!("({})", value);

    builder
}

fn set_volume<D, T>(
    mut builder: DatasetWriteBuilder<D, T>,
    value: String,
) -> DatasetWriteBuilder<D, T>
where
    D: Into<reqwest::Body> + Clone,
{
    builder.volume = format!("-({})/", value);

    builder
}
