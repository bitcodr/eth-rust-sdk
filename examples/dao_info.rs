
use ethers::prelude::*;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;
use eth_rust_sdk::dao::DaoClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let rpc_url = env::var("RPC_URL")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let provider = Provider::<Http>::try_from(rpc_url)?.interval(std::time::Duration::from_millis(10));
    let wallet: LocalWallet = private_key.parse()?;
    let client = Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(1u64)));

    let dao_address: Address = "0xYourDaoContractAddressHere".parse()?;
    let dao = DaoClient::new(dao_address, client);

    let (id, description, votes) = dao.get_proposal(U256::from(0)).await?;
    println!("Proposal {}: {} ({} votes)", id, description, votes);

    Ok(())
}
