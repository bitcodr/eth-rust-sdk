
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    Vault,
    r#"[function deposit(uint256 amount)
        function withdraw(uint256 amount)
        function balanceOf(address user) view returns (uint256)]"#
);

pub struct VaultClient<M> {
    contract: Vault<M>,
}

impl<M: Middleware> VaultClient<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let contract = Vault::new(address, client);
        Self { contract }
    }

    pub async fn deposit(&self, amount: U256) -> Result<TxHash, ContractError<M>> {
        let tx = self.contract.deposit(amount);
        let pending_tx = tx.send().await?;
        Ok(*pending_tx)
    }

    pub async fn withdraw(&self, amount: U256) -> Result<TxHash, ContractError<M>> {
        let tx = self.contract.withdraw(amount);
        let pending_tx = tx.send().await?;
        Ok(*pending_tx)
    }

    pub async fn balance_of(&self, user: Address) -> Result<U256, ContractError<M>> {
        self.contract.balance_of(user).call().await
    }
}
