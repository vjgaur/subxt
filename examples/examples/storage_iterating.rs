use subxt::{OnlineClient, SubstrateConfig};

#[subxt::subxt(runtime_metadata_path = "../artifacts/substrate.scale")]
pub mod cord {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<SubstrateConfig>::new().await?;

    // Build a storage query to iterate over account information.
    let storage_query = cord::storage().system().account_root();

    // Get back an iterator of results (here, we are fetching 10 items at
    // a time from the node, but we always iterate over oen at a time).
    let mut results = api
        .storage()
        .at_latest()
        .await?
        .iter(storage_query, 10)
        .await?;

    while let Some((key, value)) = results.next().await? {
        println!("Key: 0x{}", hex::encode(&key));
        println!("Value: {:?}", value);
    }

    Ok(())
}
