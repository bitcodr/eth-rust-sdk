
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    ERC20,
    r#"[function balanceOf(address account) view returns (uint256)
        function transfer(address recipient, uint256 amount) returns (bool)
        function approve(address spender, uint256 amount) returns (bool)]"#
);

pub struct Erc20Client<M> {
    contract: ERC20<M>,
}

impl<M: Middleware> Erc20Client<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let contract = ERC20::new(address, client);
        Self { contract }
    }

    pub async fn balance_of(&self, account: Address) -> Result<U256, ContractError<M>> {
        self.contract.balance_of(account).call().await
    }

    pub async fn transfer(&self, to: Address, amount: U256) -> Result<TxHash, ContractError<M>> {
        let tx = self.contract.transfer(to, amount);
        let pending_tx = tx.send().await?;
        Ok(*pending_tx)
    }

    pub async fn approve(&self, spender: Address, amount: U256) -> Result<TxHash, ContractError<M>> {
        let tx = self.contract.approve(spender, amount);
        let pending_tx = tx.send().await?;
        Ok(*pending_tx)
    }
}
