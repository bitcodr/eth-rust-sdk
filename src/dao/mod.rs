
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    DAO,
    r#"[function proposals(uint256) view returns (uint256 id, string description, uint256 votes)
        function vote(uint256 proposalId, bool support)]"#
);

pub struct DaoClient<M> {
    contract: DAO<M>,
}

impl<M: Middleware> DaoClient<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let contract = DAO::new(address, client);
        Self { contract }
    }

    pub async fn get_proposal(&self, proposal_id: U256) -> Result<(U256, String, U256), ContractError<M>> {
        self.contract.proposals(proposal_id).call().await
    }

    pub async fn vote(&self, proposal_id: U256, support: bool) -> Result<TxHash, ContractError<M>> {
        let tx = self.contract.vote(proposal_id, support);
        let pending_tx = tx.send().await?;
        Ok(*pending_tx)
    }
}
