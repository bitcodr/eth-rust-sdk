
use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use eth_rust_sdk::nft::NftClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let provider = Provider::<Http>::try_from(rpc_url)?.interval(std::time::Duration::from_millis(10));
    let wallet: LocalWallet = private_key.parse()?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(1u64)));

    let nft_address: Address = "0xYourNftContractAddressHere".parse()?;
    let nft = NftClient::new(nft_address, client.clone());

    let token_id = U256::from(1);
    println!("Owner: {}", nft.owner_of(token_id).await?);
    println!("Token URI: {}", nft.token_uri(token_id).await?);
    Ok(())
}
