use std::marker::PhantomData;

use reqwest::header::HeaderValue;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use zosmf_macros::{Endpoint, Getter};

use crate::datasets::utils::MigratedRecall;

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberList<T> {
    items: T,
    json_version: i32,
    more_rows: Option<bool>,
    returned_rows: i32,
    total_rows: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BaseMembers {
    FixedOrVariable(Vec<MemberFixedOrVariable>),
    Undefined(Vec<MemberUndefined>),
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberFixedOrVariable {
    #[serde(rename = "member")]
    name: String,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberUndefined {
    #[serde(rename = "member")]
    name: String,
}

#[derive(Clone, Debug, Deserialize, Getter, Serialize)]
pub struct MemberName {
    #[serde(rename = "member")]
    name: String,
}

#[derive(Endpoint)]
#[endpoint(method = get, path = "/zosmf/restfiles/ds/{dataset_name}/member")]
pub struct MemberListBuilder<'a, T> {
    base_url: &'a str,
    client: &'a Client,

    #[endpoint(path)]
    dataset_name: String,

    #[endpoint(optional, query = "start")]
    start: Option<String>,
    #[endpoint(optional, query = "pattern")]
    pattern: Option<String>,
    #[endpoint(optional, header = "X-IBM-Max-Items")]
    max_items: Option<i32>,
    #[endpoint(optional, header = "X-IBM-Attributes", skip_setter)]
    attributes: Option<Attrs>,
    #[endpoint(optional, header = "X-IBM-Migrated-Recall")]
    migrated_recall: Option<MigratedRecall>,
    #[endpoint(optional, skip_setter, skip_builder)]
    attrs: PhantomData<T>,
}

impl<'a, T> MemberListBuilder<'a, T>
where
    T: for<'de> Deserialize<'de>,
{
    pub fn attributes_base(self) -> MemberListBuilder<'a, BaseMembers> {
        MemberListBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Base),
            migrated_recall: self.migrated_recall,
            attrs: PhantomData,
        }
    }

    pub fn attributes_member(self) -> MemberListBuilder<'a, Vec<MemberName>> {
        MemberListBuilder {
            base_url: self.base_url,
            client: self.client,
            dataset_name: self.dataset_name,
            start: self.start,
            pattern: self.pattern,
            max_items: self.max_items,
            attributes: Some(Attrs::Member),
            migrated_recall: self.migrated_recall,
            attrs: PhantomData,
        }
    }

    pub async fn build(self) -> anyhow::Result<MemberList<T>> {
        let response = self.get_response().await?;

        let ResponseJson {
            items,
            returned_rows,
            more_rows,
            total_rows,
            json_version,
        } = response.json().await?;

        Ok(MemberList {
            items,
            json_version,
            more_rows,
            returned_rows,
            total_rows,
        })
    }
}

#[derive(Clone, Copy, Debug)]
enum Attrs {
    Base,
    Member,
}

impl From<Attrs> for HeaderValue {
    fn from(val: Attrs) -> HeaderValue {
        match val {
            Attrs::Base => "base",
            Attrs::Member => "member",
        }
        .try_into()
        .unwrap()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson<T> {
    items: T,
    returned_rows: i32,
    #[serde(default)]
    more_rows: Option<bool>,
    #[serde(default)]
    total_rows: Option<i32>,
    #[serde(rename = "JSONversion")]
    json_version: i32,
}
