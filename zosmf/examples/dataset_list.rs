#[path = "_setup/mod.rs"]
mod _setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let zosmf = _setup::get_zosmf().await?;

    let list_datasets = zosmf.datasets.list("IBMUSER.CONFIG.*").build().await?;

    println!("{:#?}", list_datasets);

    let list_datasets_base = zosmf
        .datasets
        .list("**")
        .volume("PEVTS2")
        .attributes_base()
        .build()
        .await?;

    println!("{:#?}", list_datasets_base);

    Ok(())
}
