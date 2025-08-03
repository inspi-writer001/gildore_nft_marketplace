# 🪙 Gildore — Tokenized Precious Metals on Solana

**Turbin3 Builders Cohort — Assignment 2**  
A Solana-based digital savings platform enabling users to save in **tokenized gold and silver**, represented by **NFTs backed 1:1** with real-world precious metals.

---

## Value Proposition

Gildore democratizes access to precious metals using Web3 tools, allowing:

- **Crypto-native savers** to diversify into stable, tangible assets.
- **Newcomers** to Web3 to enter with embedded wallets and fiat options.
- **DeFi users** to collateralize and trade metal-backed NFTs.

Each NFT represents real, physical **gold or silver** securely stored by licensed vault partners, with full transparency and blockchain-backed verification.

### 📂 [Architecture Diagram](https://drive.google.com/file/d/1qSXJ8ne9ilTnkDDEiij6_XwdV6a40cty/view?usp=sharing)

## Core User Personas

| Persona                       | Description                                    |
| ----------------------------- | ---------------------------------------------- |
| 🧠 **Crypto-Native Saver**    | Seeks stable stores of value in gold/silver    |
| 👶 **First-Time Crypto User** | Wants asset exposure without crypto complexity |
| 🛠️ **Platform Admin**         | Manages compliance, vaults, and pricing        |
| 🏦 **Vault Partner**          | Custodian of real-world precious metals        |

## 📚 Core Functionalities (User Stories)

### NFT Purchase Flow

- View gold/silver spot prices
- Calculate and preview metal quantity
- Pay in USDC/USDT (or fiat for new users)
- NFT is minted with:
  - Metal type, purity, weight
  - Vault ID
  - Purchase metadata

### Portfolio Management

- View NFT holdings and real-time valuation
- Transfer NFTs to others
- List NFTs on secondary markets

### Physical Redemption

- Burn NFT to redeem real metal
- Provide KYC + shipping details
- Vault ships to user after validation

---

## ⛓️ On-Chain Requirements

| Epic                | On-Chain Actions                                          |
| ------------------- | --------------------------------------------------------- |
| **NFT Minting**     | Token program mints NFT with vault metadata               |
| **Vault Inventory** | State tracking of metal availability and reservations     |
| **Price Oracles**   | Spot price feeds for gold/silver in USD                   |
| **Redemption Flow** | NFT burning, identity locking, vault partner notification |
| **Escrow**          | Stablecoin escrow until NFT successfully minted           |

---

## 🏗️ Project Structure

```yaml
.
├── Anchor.toml # Anchor config for the program
├── Cargo.toml # Rust dependencies
├── programs/
│ └── anchor_marketplace/ # Solana program code
│ ├── instructions/ # Create, list, purchase, update NFTs
│ ├── state/ # NFT + marketplace state definitions
│ └── lib.rs # Entrypoint and processor
├── app/ # Frontend or client (WIP)
├── migrations/deploy.ts # Anchor deployment script
├── tests/anchor_marketplace.ts # Mocha test suite
```

---

## 🚀 Getting Started

### 📦 Prerequisites

- [Anchor](https://www.anchor-lang.com/docs/installation)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [pnpm](https://pnpm.io/) (for Node dependencies)

### 🧪 Local Testing

```bash
# Install dependencies
pnpm install

# Build the Solana program
anchor build

# Run tests
anchor test
```
