pub mod create;
pub mod delete;
pub mod list;
pub mod list_members;
pub mod read;
pub mod write;

use std::sync::Arc;

use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use crate::error::Error;

use self::create::{CreateDataset, CreateDatasetBuilder};
use self::delete::{DeleteDataset, DeleteDatasetBuilder};
use self::list::{DatasetName, ListDatasets, ListDatasetsBuilder};
use self::list_members::{ListMembers, ListMembersBuilder, MemberName};
use self::read::{ReadDataset, ReadDatasetBuilder};
use self::write::{WriteDataset, WriteDatasetBuilder};

/// # DatasetsClient
///
/// A sub-client for organizing the dataset functionality of the z/OSMF Rest APIs.
///
/// This client is intended to be accessed via the `datasets` attribute of the [ZOsmf](crate::ZOsmf) struct:
/// ```
/// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
/// # use z_osmf::datasets::DatasetsClient;
/// let _: DatasetsClient = zosmf.datasets();
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct DatasetsClient {
    base_url: Arc<str>,
    client: reqwest::Client,
}

impl DatasetsClient {
    pub(super) fn new(base_url: Arc<str>, client: reqwest::Client) -> Self {
        DatasetsClient { base_url, client }
    }

    /// # Examples
    ///
    /// Creating a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_dataset = zosmf
    ///     .datasets()
    ///     .create("JIAHJ.REST.TEST.NEWDS")
    ///     .volume("zmf046")
    ///     .device_type("3390")
    ///     .organization("PS")
    ///     .space_allocation_unit("TRK")
    ///     .primary_space(10)
    ///     .secondary_space(5)
    ///     .average_block_size(500)
    ///     .record_format("FB")
    ///     .block_size(400)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a partitioned dataset (PDS):
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_pds = zosmf
    ///     .datasets()
    ///     .create("JIAHJ.REST.TEST.NEWDS02")
    ///     .volume("zmf046")
    ///     .device_type("3390")
    ///     .organization("PO")
    ///     .space_allocation_unit("TRK")
    ///     .primary_space(10)
    ///     .secondary_space(5)
    ///     .directory_blocks(10)
    ///     .average_block_size(500)
    ///     .record_format("FB")
    ///     .block_size(400)
    ///     .record_length(80)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Creating a library / partitioned dataset extended (PDS-E):
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let create_pdse = zosmf
    ///     .datasets()
    ///     .create("JIAHJ.REST.TEST.NEWDS02")
    ///     .volume("zmf046")
    ///     .device_type("3390")
    ///     .organization("PO")
    ///     .space_allocation_unit("TRK")
    ///     .primary_space(10)
    ///     .secondary_space(5)
    ///     .directory_blocks(10)
    ///     .average_block_size(500)
    ///     .record_format("FB")
    ///     .block_size(400)
    ///     .record_length(80)
    ///     .dataset_type("LIBRARY")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn create(&self, dataset_name: &str) -> CreateDatasetBuilder<CreateDataset> {
        CreateDatasetBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Deleting a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_dataset = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.DATASET")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Deleting an uncataloged sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_uncataloged = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.DATASET2")
    ///     .volume("ZMF046")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Deleting a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_member = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.PDS")
    ///     .member("MEMBER01")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Deleting an uncataloged PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let delete_uncataloged_member = zosmf
    ///     .datasets()
    ///     .delete("JIAHJ.REST.TEST.PDS.UNCAT")
    ///     .member("MEMBER01")
    ///     .volume("ZMF046")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, dataset_name: &str) -> DeleteDatasetBuilder<DeleteDataset> {
        DeleteDatasetBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Listing datasets:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_datasets = zosmf
    ///     .datasets()
    ///     .list("IBMUSER.CONFIG.*")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Listing the base attributes of uncataloged datasets:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_datasets_base = zosmf
    ///     .datasets()
    ///     .list("**")
    ///     .volume("PEVTS2")
    ///     .attributes_base()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list(&self, name_pattern: &str) -> ListDatasetsBuilder<ListDatasets<DatasetName>> {
        ListDatasetsBuilder::new(self.base_url.clone(), self.client.clone(), name_pattern)
    }

    /// # Examples
    ///
    /// Listing PDS members:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_members = zosmf
    ///     .datasets()
    ///     .list_members("SYS1.PROCLIB")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Listing the base attributes of PDS members:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let list_members_base = zosmf
    ///     .datasets()
    ///     .list_members("SYS1.PROCLIB")
    ///     .attributes_base()
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_members(&self, dataset_name: &str) -> ListMembersBuilder<ListMembers<MemberName>> {
        ListMembersBuilder::new(
            self.base_url.clone(),
            self.client.clone(),
            dataset_name.to_string(),
        )
    }

    /// # Examples
    ///
    /// Reading a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let read_member = zosmf
    ///     .datasets()
    ///     .read("SYS1.PARMLIB")
    ///     .member("SMFPRM00")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Reading a sequential dataset:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// let read_dataset = zosmf
    ///     .datasets()
    ///     .read("JIAHJ.REST.SRVMP")
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read(&self, dataset_name: &str) -> ReadDatasetBuilder<ReadDataset<Box<str>>> {
        ReadDatasetBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }

    /// # Examples
    ///
    /// Writing to a PDS member:
    /// ```
    /// # async fn example(zosmf: z_osmf::ZOsmf) -> anyhow::Result<()> {
    /// # let string_data = "".to_string();
    /// let write_dataset = zosmf
    ///     .datasets()
    ///     .write("SYS1.PARMLIB")
    ///     .member("SMFPRM00")
    ///     .if_match("B5C6454F783590AA8EC15BD88E29EA63")
    ///     .text(string_data)
    ///     .build()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write(&self, dataset_name: &str) -> WriteDatasetBuilder<WriteDataset> {
        WriteDatasetBuilder::new(self.base_url.clone(), self.client.clone(), dataset_name)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum DataType {
    Binary,
    Record,
    #[default]
    Text,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataType::Binary => "binary",
                DataType::Record => "record",
                DataType::Text => "text",
            }
        )
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum MigratedRecall {
    Error,
    NoWait,
    Wait,
}

impl From<MigratedRecall> for HeaderValue {
    fn from(val: MigratedRecall) -> HeaderValue {
        match val {
            MigratedRecall::Error => "error",
            MigratedRecall::NoWait => "nowait",
            MigratedRecall::Wait => "wait",
        }
        .try_into()
        .unwrap()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum ObtainEnq {
    Exclusive,
    SharedReadWrite,
}

impl From<ObtainEnq> for HeaderValue {
    fn from(val: ObtainEnq) -> HeaderValue {
        match val {
            ObtainEnq::Exclusive => "EXCLU",
            ObtainEnq::SharedReadWrite => "SHRW",
        }
        .try_into()
        .unwrap()
    }
}

pub(crate) fn get_session_ref(response: &reqwest::Response) -> Result<Option<Box<str>>, Error> {
    Ok(response
        .headers()
        .get("X-IBM-Session-Ref")
        .map(|v| v.to_str())
        .transpose()?
        .map(|v| v.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_data_type() {
        assert_eq!(format!("{}", DataType::Binary), "binary");

        assert_eq!(format!("{}", DataType::Record), "record");

        assert_eq!(format!("{}", DataType::Text), "text");
    }

    #[test]
    fn display_migrated_recall() {
        let header_value: HeaderValue = MigratedRecall::Error.into();
        assert_eq!(header_value, HeaderValue::from_static("error"));

        let header_value: HeaderValue = MigratedRecall::NoWait.into();
        assert_eq!(header_value, HeaderValue::from_static("nowait"));

        let header_value: HeaderValue = MigratedRecall::Wait.into();
        assert_eq!(header_value, HeaderValue::from_static("wait"));
    }

    #[test]
    fn display_obtain_enq() {
        let header_value: HeaderValue = ObtainEnq::Exclusive.into();
        assert_eq!(header_value, HeaderValue::from_static("EXCLU"));

        let header_value: HeaderValue = ObtainEnq::SharedReadWrite.into();
        assert_eq!(header_value, HeaderValue::from_static("SHRW"));
    }

    #[test]
    fn test_get_session_ref() {
        let response = reqwest::Response::from(
            http::Response::builder()
                .header("X-IBM-Session-Ref", "ABCD1234")
                .body("")
                .unwrap(),
        );
        assert_eq!(get_session_ref(&response).unwrap(), Some("ABCD1234".into()));

        let response = reqwest::Response::from(http::Response::new(""));
        assert_eq!(get_session_ref(&response).unwrap(), None);
    }
}
