use sp_keyring::AccountKeyring;
use subxt::{OnlineClient, config::CordConfig};

// Generate an interface that we can use from the node's metadata.
#[subxt::subxt(runtime_metadata_path = "../artifacts/cord_new.scale")]
pub mod cord {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new API client, configured to talk to Polkadot nodes.
    let api = OnlineClient::<CordConfig>::new().await?;

    // Build a storage query to access account information.
    let account = AccountKeyring::Alice.to_account_id().into();
    let storage_query = cord::storage().system().account(&account);

    // Use that query to `fetch` a result. This returns an `Option<_>`, which will be
    // `None` if no value exists at the given address. You can also use `fetch_default`
    // where applicable, which will return the default value if none exists.
    let result = api
        .storage()
        .at_latest()
        .await?
        .fetch(&storage_query)
        .await?;

    println!("Alice has free balance: {}", result.unwrap().data.free);
    Ok(())
}
