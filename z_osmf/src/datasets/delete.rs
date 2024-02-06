use std::marker::PhantomData;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use z_osmf_macros::{Endpoint, Getters};

use crate::convert::TryFromResponse;
use crate::error::Error;
use crate::utils::get_transaction_id;

#[derive(Clone, Debug, Deserialize, Getters, Serialize)]
pub struct DatasetDelete {
    transaction_id: Box<str>,
}

impl TryFromResponse for DatasetDelete {
    async fn try_from_response(value: reqwest::Response) -> Result<Self, Error> {
        let transaction_id = get_transaction_id(&value)?;

        Ok(DatasetDelete { transaction_id })
    }
}

#[derive(Clone, Debug, Endpoint)]
#[endpoint(method = delete, path = "/zosmf/restfiles/ds/{volume}{dataset_name}{member}")]
pub struct DatasetDeleteBuilder<T>
where
    T: TryFromResponse,
{
    base_url: Arc<str>,
    client: reqwest::Client,

    #[endpoint(path)]
    dataset_name: Box<str>,
    #[endpoint(optional, path, setter_fn = set_volume)]
    volume: Box<str>,
    #[endpoint(optional, path, setter_fn = set_member)]
    member: Box<str>,
    #[endpoint(optional, header = "X-IBM-Dsname-Encoding")]
    dsname_encoding: Option<Box<str>>,

    #[endpoint(optional, skip_setter, skip_builder)]
    target_type: PhantomData<T>,
}

fn set_member<T>(mut builder: DatasetDeleteBuilder<T>, value: Box<str>) -> DatasetDeleteBuilder<T>
where
    T: TryFromResponse,
{
    builder.member = format!("({})", value).into();

    builder
}

fn set_volume<T>(mut builder: DatasetDeleteBuilder<T>, value: Box<str>) -> DatasetDeleteBuilder<T>
where
    T: TryFromResponse,
{
    builder.volume = format!("-({})/", value).into();

    builder
}

#[cfg(test)]
mod tests {
    use crate::tests::*;

    #[test]
    fn example_1() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .client
            .delete("https://test.com/zosmf/restfiles/ds/JIAHJ.REST.TEST.DATASET")
            .build()
            .unwrap();

        let delete_dataset = zosmf
            .delete_dataset("JIAHJ.REST.TEST.DATASET")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_dataset)
        );
    }

    #[test]
    fn example_2() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .client
            .delete("https://test.com/zosmf/restfiles/ds/-(ZMF046)/JIAHJ.REST.TEST.DATASET2")
            .build()
            .unwrap();

        let delete_uncataloged = zosmf
            .delete_dataset("JIAHJ.REST.TEST.DATASET2")
            .volume("ZMF046")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_uncataloged)
        );
    }

    #[test]
    fn example_3() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .client
            .delete("https://test.com/zosmf/restfiles/ds/JIAHJ.REST.TEST.PDS(MEMBER01)")
            .build()
            .unwrap();

        let delete_member = zosmf
            .delete_dataset("JIAHJ.REST.TEST.PDS")
            .member("MEMBER01")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_member)
        );
    }

    #[test]
    fn example_4() {
        let zosmf = get_zosmf();

        let manual_request = zosmf
            .client
            .delete(
                "https://test.com/zosmf/restfiles/ds/-(ZMF046)/JIAHJ.REST.TEST.PDS.UNCAT(MEMBER01)",
            )
            .build()
            .unwrap();

        let delete_uncataloged_member = zosmf
            .delete_dataset("JIAHJ.REST.TEST.PDS.UNCAT")
            .member("MEMBER01")
            .volume("ZMF046")
            .get_request()
            .unwrap();

        assert_eq!(
            format!("{:?}", manual_request),
            format!("{:?}", delete_uncataloged_member)
        );
    }
}
