use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Bytes;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::{get_etag, get_transaction_id};
use crate::ClientCore;

use super::DataType;

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, PartialEq, Serialize)]
pub struct Read<T> {
    #[getter(skip)]
    data: T,
    etag: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl Read<Box<str>> {
    pub fn data(&self) -> &str {
        &self.data
    }
}

impl TryFromResponse for Read<Box<str>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = value.text().await?.into();

        Ok(Read {
            data,
            etag,
            transaction_id,
        })
    }
}

impl Read<Bytes> {
    pub fn data(&self) -> &Bytes {
        &self.data
    }
}

impl TryFromResponse for Read<Bytes> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = value.bytes().await?;

        Ok(Read {
            data,
            etag,
            transaction_id,
        })
    }
}

impl Read<Option<Box<str>>> {
    pub fn data(&self) -> Option<&str> {
        self.data.as_deref()
    }
}

impl TryFromResponse for Read<Option<Box<str>>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.text().await?.into())
        };

        Ok(Read {
            data,
            etag,
            transaction_id,
        })
    }
}

impl Read<Option<Bytes>> {
    pub fn data(&self) -> Option<&Bytes> {
        self.data.as_ref()
    }
}

impl TryFromResponse for Read<Option<Bytes>> {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let (etag, transaction_id) = get_headers(&value)?;

        let data = if value.status() == StatusCode::NOT_MODIFIED {
            None
        } else {
            Some(value.bytes().await?)
        };

        Ok(Read {
            data,
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/fs{path}")]
pub struct ReadBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path)]
    path: Box<str>,
    #[endpoint(optional, query = "search")]
    search: Option<Box<str>>,
    #[endpoint(optional, query = "research")]
    regex_search: Option<Box<str>>,
    #[endpoint(optional, builder_fn = build_search_case_sensitive)]
    search_case_sensitive: bool,
    #[endpoint(optional, query = "maxreturnsize")]
    search_max_return: Option<i32>,
    #[endpoint(optional, skip_setter, builder_fn = build_data_type)]
    data_type: Option<DataType>,
    #[endpoint(optional, skip_builder)]
    encoding: Option<Box<str>>,
    #[endpoint(optional, header = "If-None-Match", skip_setter)]
    etag: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

impl<U> ReadBuilder<Read<U>>
where
    Read<U>: TryFromResponse,
    Read<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> ReadBuilder<Read<Bytes>> {
        ReadBuilder {
            core: self.core,
            path: self.path,
            search: self.search,
            regex_search: self.regex_search,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> ReadBuilder<Read<Box<str>>> {
        ReadBuilder {
            core: self.core,
            path: self.path,
            search: self.search,
            regex_search: self.regex_search,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn if_none_match<E>(self, etag: E) -> ReadBuilder<Read<Option<U>>>
    where
        E: Into<Box<str>>,
    {
        ReadBuilder {
            core: self.core,
            path: self.path,
            search: self.search,
            regex_search: self.regex_search,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: self.data_type,
            encoding: self.encoding,
            etag: Some(etag.into()),
            target_type: PhantomData,
        }
    }
}

impl<U> ReadBuilder<Read<Option<U>>>
where
    Read<Option<U>>: TryFromResponse,
{
    pub fn binary(self) -> ReadBuilder<Read<Option<Bytes>>> {
        ReadBuilder {
            core: self.core,
            path: self.path,
            search: self.search,
            regex_search: self.regex_search,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Binary),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }

    pub fn text(self) -> ReadBuilder<Read<Option<Box<str>>>> {
        ReadBuilder {
            core: self.core,
            path: self.path,
            search: self.search,
            regex_search: self.regex_search,
            search_case_sensitive: self.search_case_sensitive,
            search_max_return: self.search_max_return,
            data_type: Some(DataType::Text),
            encoding: self.encoding,
            etag: self.etag,
            target_type: PhantomData,
        }
    }
}

fn build_data_type<T>(
    request_builder: reqwest::RequestBuilder,
    dataset_read_builder: &ReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let ReadBuilder {
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

fn build_search_case_sensitive<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &ReadBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    match builder.search_case_sensitive {
        true => request_builder.query(&[("insensitive", "false")]),
        false => request_builder,
    }
}

fn get_headers(response: &reqwest::Response) -> Result<(Option<Box<str>>, Box<str>), Error> {
    Ok((get_etag(response)?, get_transaction_id(response)?))
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn data_type() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .header("X-IBM-Data-Type", "binary")
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .read("/u/jiahj/testFile.txt")
            .binary()
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
    }

    #[test]
    fn etag() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .header("X-IBM-Data-Type", "text;fileEncoding=IBM-1047")
            .header("If-None-Match", "abcd1234")
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .read("/u/jiahj/testFile.txt")
            .binary()
            .text()
            .if_none_match("abcd1234")
            .binary()
            .text()
            .encoding("IBM-1047")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
    }

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/etc/inetd.conf")
            .build()
            .unwrap();

        let read_file = zosmf.files().read("/etc/inetd.conf").get_request().unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", read_file))
    }

    #[test]
    fn encoding() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/u/jiahj/testFile.txt")
            .header("X-IBM-Data-Type", "text;fileEncoding=ISO8859-1")
            .build()
            .unwrap();

        let request = zosmf
            .files()
            .read("/u/jiahj/testFile.txt")
            .encoding("ISO8859-1")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", request));
    }

    #[test]
    fn regex() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/etc/inetd.conf")
            .query(&[("research", ".*")])
            .build()
            .unwrap();

        let read_file = zosmf
            .files()
            .read("/etc/inetd.conf")
            .regex_search(".*")
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", read_file))
    }

    #[test]
    fn search() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .core
            .client
            .get("https://test.com/zosmf/restfiles/fs/etc/inetd.conf")
            .query(&[
                ("search", "something"),
                ("insensitive", "false"),
                ("maxreturnsize", "10"),
            ])
            .build()
            .unwrap();

        let read_file = zosmf
            .files()
            .read("/etc/inetd.conf")
            .search("something")
            .search_case_sensitive(true)
            .search_max_return(10)
            .get_request()
            .unwrap();

        assert_eq!(format!("{:?}", manual_request), format!("{:?}", read_file))
    }
}
