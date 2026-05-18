# Quick Start Guide for Contributors

Welcome to UpCurrent smart contract development! This guide will get you up and running quickly.

## 🚀 Quick Setup (5 minutes)

### 1. Install Rust (if not already installed)

**Windows:**
```powershell
winget install Rustlang.Rustup
```

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. Install Soroban Dependencies

```bash
cd contracts
make install-deps
```

Or manually:
```bash
rustup target add wasm32-unknown-unknown
cargo install --locked soroban-cli --features opt
```

### 3. Build and Test

```bash
make build
make test
```

## 📁 Project Structure

```
contracts/
├── upcurrent_escrow/          # Main escrow contract
│   ├── src/
│   │   ├── lib.rs            # Contract logic
│   │   └── test.rs           # Unit tests
│   └── Cargo.toml
├── Cargo.toml                 # Workspace config
├── Makefile                   # Build commands
└── README.md                  # Full documentation
```

## 🛠️ Common Commands

| Command | Description |
|---------|-------------|
| `make build` | Build all contracts |
| `make test` | Run all tests |
| `make clean` | Clean build artifacts |
| `make check` | Quick syntax check |
| `make fmt` | Format code |
| `make clippy` | Run linter |
| `make bindings` | Generate TypeScript bindings |

## 📝 Working on an Issue

### 1. Create a New Contract

If your issue requires a new contract:

```bash
# Create contract directory
mkdir contracts/my_new_contract
cd contracts/my_new_contract

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "my_new_contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = { workspace = true }

[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
EOF

# Create src directory
mkdir src
```

Add your contract to the workspace in `contracts/Cargo.toml`:
```toml
[workspace]
members = [
    "upcurrent_escrow",
    "my_new_contract",  # Add this line
]
```

### 2. Modify Existing Contract

Edit files in `contracts/upcurrent_escrow/src/`:
- `lib.rs` - Main contract logic
- `test.rs` - Unit tests

### 3. Test Your Changes

```bash
# Run all tests
make test

# Run tests for specific contract
cargo test -p upcurrent_escrow

# Run specific test
cargo test -p upcurrent_escrow test_create_invoice
```

### 4. Build for Production

```bash
make build
```

The compiled WASM will be at:
```
target/wasm32-unknown-unknown/release/[contract_name].wasm
```

## 🧪 Testing Best Practices

Always write tests for new functionality:

```rust
#[test]
fn test_my_feature() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, UpCurrentEscrow);
    let client = UpCurrentEscrowClient::new(&env, &contract_id);
    
    // Your test logic here
    assert_eq!(expected, actual);
}
```

## 🔍 Code Review Checklist

Before submitting a PR:

- [ ] Code builds without warnings: `make build`
- [ ] All tests pass: `make test`
- [ ] Code is formatted: `make fmt`
- [ ] No clippy warnings: `make clippy`
- [ ] Added tests for new functionality
- [ ] Updated documentation if needed

## 📚 Key Concepts

### Invoice Lifecycle

1. **Create** - SME creates invoice (`create_invoice`)
2. **Fund** - Investor purchases at discount (`fund_invoice`)
3. **Settle** - Debtor pays full amount (`settle_invoice`)

### Contract Storage

- **Instance Storage** - Contract-level data (admin, counters)
- **Persistent Storage** - Long-term data (invoices)

### Authentication

Use `require_auth()` for functions that modify state:

```rust
pub fn my_function(env: Env, caller: Address) {
    caller.require_auth();  // Verify caller signature
    // ... rest of function
}
```

## 🆘 Getting Help

- Check [SETUP.md](./SETUP.md) for detailed setup instructions
- Read [README.md](./README.md) for full contract documentation
- Review [Soroban docs](https://soroban.stellar.org/docs)
- Ask in [Stellar Discord](https://discord.gg/stellardev)

## 🎯 Example: Adding a New Feature

Let's say you want to add a "cancel invoice" feature:

1. **Add the function** in `lib.rs`:
```rust
pub fn cancel_invoice(env: Env, invoice_id: u64, issuer: Address) {
    issuer.require_auth();
    
    let mut invoice: Invoice = env
        .storage()
        .persistent()
        .get(&DataKey::Invoice(invoice_id))
        .expect("Invoice not found");
    
    if invoice.issuer != issuer {
        panic!("Only issuer can cancel");
    }
    
    if invoice.status != InvoiceStatus::Active {
        panic!("Can only cancel active invoices");
    }
    
    invoice.status = InvoiceStatus::Cancelled;
    env.storage()
        .persistent()
        .set(&DataKey::Invoice(invoice_id), &invoice);
}
```

2. **Add the status** to the enum:
```rust
pub enum InvoiceStatus {
    Active,
    Funded,
    Settled,
    Disputed,
    Cancelled,  // Add this
}
```

3. **Write tests** in `test.rs`:
```rust
#[test]
fn test_cancel_invoice() {
    // Test implementation
}
```

4. **Build and test**:
```bash
make build
make test
```

That's it! You're ready to contribute. Happy coding! 🎉
