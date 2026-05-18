# Contract Development Setup Guide

This guide will help you set up your development environment for building Soroban smart contracts.

## Step 1: Install Rust

### Windows

Download and run the Rust installer from [rustup.rs](https://rustup.rs/):

```powershell
# Download and run rustup-init.exe
# Follow the on-screen instructions
```

Or using winget:

```powershell
winget install Rustlang.Rustup
```

After installation, restart your terminal and verify:

```bash
cargo --version
rustc --version
```

### macOS/Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Step 2: Add WebAssembly Target

Soroban contracts compile to WebAssembly:

```bash
rustup target add wasm32-unknown-unknown
```

## Step 3: Install Soroban CLI

```bash
cargo install --locked soroban-cli --features opt
```

Verify installation:

```bash
soroban --version
```

## Step 4: Build the Contracts

Navigate to the contracts directory and build:

```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

## Step 5: Run Tests

```bash
cargo test
```

## Step 6: Deploy to Testnet (Optional)

First, configure your Stellar account:

```bash
# Generate a new keypair for testing
soroban keys generate --global test-account --network testnet

# Fund your account with testnet tokens
soroban keys address test-account | xargs -I {} curl "https://friendbot.stellar.org?addr={}"
```

Deploy the contract:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
  --source test-account \
  --network testnet
```

## Troubleshooting

### "cargo: command not found"

Make sure Rust is installed and your PATH is configured correctly. Restart your terminal after installation.

### Build errors

Make sure you have the wasm32 target installed:

```bash
rustup target add wasm32-unknown-unknown
```

### Soroban CLI not found

Ensure cargo's bin directory is in your PATH:

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$HOME/.cargo/bin:$PATH"
```

## Next Steps

- Read the [contracts/README.md](./README.md) for detailed contract documentation
- Check out the [Soroban documentation](https://soroban.stellar.org/docs)
- Join the [Stellar Developer Discord](https://discord.gg/stellardev)
