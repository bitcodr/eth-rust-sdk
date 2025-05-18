
# eth-rust-sdk

A modular, async-ready Rust SDK for interacting with Ethereum smart contracts using [ethers-rs](https://github.com/gakonst/ethers-rs).  
Includes support for:
- **ERC-721 (NFTs)**
- **DAOs**
- **Staking contracts**
- **ERC-20 tokens**
- **Vault contracts**

---

## Features

- Modular design: each contract type is a separate module
- Uses `ethers-rs` for provider, signer, and contract bindings
- Works with any Ethereum-compatible chain (Mainnet, Goerli, Polygon, etc.)
- Includes usage examples and `.env` configuration

---

## Setup

1. Clone this repo and enter the directory:
```bash
git clone https://github.com/bitcodr/eth-rust-sdk.git
cd eth-rust-sdk
```

2. Copy the `.env.example` to `.env` and add your Infura/Alchemy RPC + private key:
```env
RPC_URL=https://mainnet.infura.io/v3/your_project_id
PRIVATE_KEY=your_private_key_here
```

3. Build the project:
```bash
cargo build
```

---

## Examples

Each module includes a sample usage file inside `/examples`.

Run them like this:

```bash
cargo run --example fetch_nft_metadata
cargo run --example dao_info
cargo run --example staking_balance
```

---

## Modules

- `nft`: Query token URI, owner, and transfer
- `dao`: Read proposals and vote
- `staking`: Stake/unstake and read earnings
- `erc20`: Token transfers, approvals, balances
- `vault`: Deposit, withdraw, and check balances

---

## Development

- Format code:
```bash
cargo fmt
```

- Run unit tests:
```bash
cargo test
```

---

## License

MIT © [bitcodr](https://github.com/bitcodr)
