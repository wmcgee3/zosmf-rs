use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::utils::{get_etag, get_transaction_id};

use super::RequestJson;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetMigrate {
    etag: Option<Box<str>>,
    transaction_id: Box<str>,
}

impl TryFromResponse for DatasetMigrate {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, crate::error::Error> {
        let etag = get_etag(&value)?;
        let transaction_id = get_transaction_id(&value)?;

        Ok(DatasetMigrate {
            etag,
            transaction_id,
        })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = put, path = "/zosmf/restfiles/ds/{volume}{name}{member}")]
pub struct DatasetMigrateBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(optional, path, setter_fn = set_volume)]
    volume: Box<str>,
    #[endpoint(path)]
    name: Box<str>,
    #[endpoint(optional, path, setter_fn = set_member)]
    member: Box<str>,
    #[endpoint(optional, builder_fn = build_body )]
    wait: bool,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

fn build_body<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &DatasetMigrateBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    request_builder.json(&RequestJson {
        request: "hmigrate",
        wait: builder.wait,
    })
}

fn set_member<T>(mut builder: DatasetMigrateBuilder<T>, value: Box<str>) -> DatasetMigrateBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}

fn set_volume<T>(mut builder: DatasetMigrateBuilder<T>, value: Box<str>) -> DatasetMigrateBuilder<T>
where
    T: TryFromResponse,
{
    builder.volume = format!("-({})/", value).into();

    builder
}
