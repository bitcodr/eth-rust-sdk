
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    Staking,
    r#"[function stake(uint256 amount)
        function unstake(uint256 amount)
        function balanceOf(address user) view returns (uint256)
        function earned(address user) view returns (uint256)]"#
);

pub struct StakingClient<M> {
    contract: Staking<M>,
}

impl<M: Middleware> StakingClient<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let contract = Staking::new(address, client);
        Self { contract }
    }

    pub async fn stake(&self, amount: U256) -> Result<TxHash, ContractError<M>> {
        let tx = self.contract.stake(amount);
        let pending_tx = tx.send().await?;
        Ok(*pending_tx)
    }

    pub async fn balance_of(&self, user: Address) -> Result<U256, ContractError<M>> {
        self.contract.balance_of(user).call().await
    }

    pub async fn earned(&self, user: Address) -> Result<U256, ContractError<M>> {
        self.contract.earned(user).call().await
    }
}
