
use ethers::prelude::*;
use std::sync::Arc;

abigen!(
    ERC721,
    r#"[function ownerOf(uint256 tokenId) view returns (address)
        function tokenURI(uint256 tokenId) view returns (string)
        function transferFrom(address from, address to, uint256 tokenId)]"#,
);

pub struct NftClient<M> {
    contract: ERC721<M>,
}

impl<M: Middleware> NftClient<M> {
    pub fn new(address: Address, client: Arc<M>) -> Self {
        let contract = ERC721::new(address, client);
        Self { contract }
    }

    pub async fn owner_of(&self, token_id: U256) -> Result<Address, ContractError<M>> {
        self.contract.owner_of(token_id).call().await
    }

    pub async fn token_uri(&self, token_id: U256) -> Result<String, ContractError<M>> {
        self.contract.token_uri(token_id).call().await
    }
}
