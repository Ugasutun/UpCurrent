# UpCurrent Smart Contracts

This directory contains all Soroban smart contracts for the UpCurrent invoice factoring platform.

## Structure

The contracts are organized as a Cargo workspace, allowing multiple contracts to coexist and share dependencies:

```
contracts/
├── Cargo.toml              # Workspace configuration
├── upcurrent_escrow/       # Main escrow contract
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # Contract implementation
│       └── test.rs         # Unit tests
└── [future_contract]/      # Additional contracts can be added here
```

## Prerequisites

- Rust toolchain (latest stable)
- Soroban CLI v21+
- Stellar SDK

### Install Soroban CLI

```bash
cargo install --locked soroban-cli --features opt
```

### Add WebAssembly target

```bash
rustup target add wasm32-unknown-unknown
```

## Building Contracts

Build all contracts in the workspace:

```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM files will be located at:
```
target/wasm32-unknown-unknown/release/[contract_name].wasm
```

## Testing Contracts

Run all tests:

```bash
cargo test
```

Run tests for a specific contract:

```bash
cargo test -p upcurrent_escrow
```

## Deploying Contracts

### Deploy to Testnet

```bash
# Deploy the contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Initialize the contract
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <YOUR_SECRET_KEY> \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015" \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

### Generate TypeScript Bindings

Generate TypeScript bindings for frontend integration:

```bash
stellar contract bindings typescript \
  --wasm ./target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
  --output-dir ../src/contracts/upcurrent
```

## Current Contracts

### upcurrent_escrow

The main escrow contract that handles:
- **Invoice Tokenization**: SMEs can mint invoices as on-chain debt tokens
- **Invoice Funding**: Investors can purchase invoices at a discount
- **Settlement**: Debtors pay full invoice amount, triggering automated distribution
- **Dispute Handling**: Lock mechanisms for disputed payments

#### Key Functions

- `initialize(admin: Address)` - Initialize the contract
- `create_invoice(issuer, debtor, amount, discount_rate, maturity_date)` - Create a new invoice
- `fund_invoice(invoice_id, investor, token)` - Investor purchases an invoice
- `settle_invoice(invoice_id, debtor, token)` - Debtor settles the invoice
- `get_invoice(invoice_id)` - Retrieve invoice details
- `get_invoice_count()` - Get total number of invoices

## Adding New Contracts

To add a new contract to the workspace:

1. Create a new directory under `contracts/`:
   ```bash
   mkdir contracts/new_contract
   ```

2. Create a `Cargo.toml` for the new contract:
   ```toml
   [package]
   name = "new_contract"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib"]

   [dependencies]
   soroban-sdk = { workspace = true }

   [dev-dependencies]
   soroban-sdk = { workspace = true, features = ["testutils"] }
   ```

3. Add the new contract to the workspace in `contracts/Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "upcurrent_escrow",
       "new_contract",  # Add this line
   ]
   ```

4. Create your contract implementation in `contracts/new_contract/src/lib.rs`

## Development Guidelines

- Always write comprehensive unit tests for contract functions
- Use `#[contractimpl]` for all public contract methods
- Validate all inputs and use `panic!` for error conditions
- Use `require_auth()` for functions that modify state
- Follow Rust naming conventions (snake_case for functions)
- Document all public functions with doc comments
- Keep contracts modular and focused on single responsibilities

## Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Stellar Developer Discord](https://discord.gg/stellardev)
