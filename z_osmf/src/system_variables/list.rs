use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::{ClientCore, Result};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SystemId {
    #[default]
    Local,
    Named {
        sysplex: String,
        system: String,
    },
}

impl std::fmt::Display for SystemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemId::Local => write!(f, "local"),
            SystemId::Named { sysplex, system } => write!(f, "{}.{}", sysplex, system),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Getters, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemVariable {
    name: Box<str>,
    value: Box<str>,
    description: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SystemVariableList {
    inner: Box<[SystemVariable]>,
}

impl TryFromResponse for SystemVariableList {
    async fn try_from_response(value: reqwest::Response) -> Result<Self> {
        let ResponseJson { variables } = value.json().await?;

        Ok(SystemVariableList { inner: variables })
    }
}

impl std::ops::Deref for SystemVariableList {
    type Target = [SystemVariable];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = get, path = "/zosmf/variables/rest/1.0/systems/{system_id}")]
pub struct SystemVariableListBuilder<T>
where
    T: TryFromResponse,
{
    core: Arc<ClientCore>,

    #[endpoint(path, builder_fn = build_system_id)]
    system_id: Option<SystemId>,
    #[endpoint(skip_setter, builder_fn = build_names)]
    names: Option<Vec<String>>,

    target_type: PhantomData<T>,
}

impl<T> SystemVariableListBuilder<T>
where
    T: TryFromResponse,
{
    pub fn name<V>(self, value: V) -> Self
    where
        V: ToString,
    {
        let mut new = self;
        match new.names {
            Some(ref mut names) => names.push(value.to_string()),
            None => new.names = Some(vec![value.to_string()]),
        }

        new
    }

    pub fn names<V>(self, value: &[V]) -> Self
    where
        V: ToString,
    {
        let mut new = self;
        match new.names {
            Some(ref mut names) => names.extend(value.iter().map(|v| v.to_string())),
            None => new.names = Some(value.iter().map(|v| v.to_string()).collect()),
        }

        new
    }
}

#[derive(Deserialize)]
struct ResponseJson {
    #[serde(rename = "system-variable-list")]
    variables: Box<[SystemVariable]>,
}

fn build_names<T>(
    request_builder: reqwest::RequestBuilder,
    builder: &SystemVariableListBuilder<T>,
) -> reqwest::RequestBuilder
where
    T: TryFromResponse,
{
    let query: Box<[_]> = builder
        .names
        .iter()
        .map(|name| ("var-name", name))
        .collect();

    request_builder.query(&query)
}

fn build_system_id<T>(builder: &SystemVariableListBuilder<T>) -> &SystemId
where
    T: TryFromResponse,
{
    builder.system_id.as_ref().unwrap_or(&SystemId::Local)
}
