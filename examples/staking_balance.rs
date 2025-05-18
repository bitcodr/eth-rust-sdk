
use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use eth_rust_sdk::staking::StakingClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let provider = Provider::<Http>::try_from(rpc_url)?.interval(std::time::Duration::from_millis(10));
    let wallet: LocalWallet = private_key.parse()?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(1u64)));

    let staking_address: Address = "0xYourStakingContractAddressHere".parse()?;
    let staking = StakingClient::new(staking_address, client.clone());

    let address = client.address();
    println!("Staked Balance: {}", staking.balance_of(address).await?);
    println!("Earned: {}", staking.earned(address).await?);

    Ok(())
}
