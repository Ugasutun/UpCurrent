# Contributing to UpCurrent Smart Contracts

Thank you for your interest in contributing to UpCurrent's smart contracts! This guide will help you get started.

## 🎯 Before You Start

1. **Read the documentation**:
   - [QUICKSTART.md](./QUICKSTART.md) - Get up and running quickly
   - [SETUP.md](./SETUP.md) - Detailed setup instructions
   - [README.md](./README.md) - Full contract documentation

2. **Set up your environment**:
   ```bash
   cd contracts
   make install-deps
   make build
   make test
   ```

3. **Find an issue to work on**:
   - Check the [GitHub Issues](https://github.com/yourusername/upcurrent/issues)
   - Look for issues labeled `good first issue` or `help wanted`
   - Comment on the issue to let others know you're working on it

## 🔄 Development Workflow

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/upcurrent.git
cd upcurrent/contracts
```

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/issue-number-description
```

### 3. Make Your Changes

- Write clean, well-documented code
- Follow Rust naming conventions
- Add comprehensive tests
- Keep commits atomic and well-described

### 4. Test Your Changes

```bash
# Run all tests
make test

# Run specific contract tests
cargo test -p upcurrent_escrow

# Check for warnings
make clippy

# Format code
make fmt
```

### 5. Commit Your Changes

```bash
git add .
git commit -m "feat: add invoice cancellation feature"
```

**Commit message format**:
- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks

### 6. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

## 📋 Code Standards

### Rust Style Guide

- Use `snake_case` for functions and variables
- Use `PascalCase` for types and structs
- Use `SCREAMING_SNAKE_CASE` for constants
- Maximum line length: 100 characters
- Use `cargo fmt` to format code

### Contract Best Practices

1. **Always validate inputs**:
   ```rust
   if amount <= 0 {
       panic!("Amount must be positive");
   }
   ```

2. **Use require_auth for state changes**:
   ```rust
   pub fn my_function(env: Env, caller: Address) {
       caller.require_auth();
       // ... modify state
   }
   ```

3. **Write comprehensive tests**:
   ```rust
   #[test]
   fn test_success_case() { /* ... */ }
   
   #[test]
   #[should_panic(expected = "Error message")]
   fn test_failure_case() { /* ... */ }
   ```

4. **Document public functions**:
   ```rust
   /// Creates a new invoice token
   /// 
   /// # Arguments
   /// * `issuer` - The SME creating the invoice
   /// * `amount` - Invoice amount in smallest units
   /// 
   /// # Panics
   /// Panics if amount is not positive
   pub fn create_invoice(/* ... */) { /* ... */ }
   ```

## 🧪 Testing Requirements

All PRs must include tests:

- **Unit tests** for all new functions
- **Integration tests** for complex workflows
- **Failure tests** for error conditions

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_happy_path() {
        // Setup
        let env = Env::default();
        env.mock_all_auths();
        
        // Execute
        let result = my_function(&env);
        
        // Assert
        assert_eq!(result, expected);
    }
    
    #[test]
    #[should_panic(expected = "Error message")]
    fn test_error_case() {
        // Test that should panic
    }
}
```

## 📝 Pull Request Guidelines

### PR Title Format

- `feat(escrow): add invoice cancellation`
- `fix(escrow): correct discount calculation`
- `docs(contracts): update deployment guide`

### PR Description Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass locally
- [ ] Added new tests for changes
- [ ] Tested on Stellar testnet (if applicable)

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Comments added for complex logic
- [ ] Documentation updated
- [ ] No new warnings generated
```

## 🔍 Code Review Process

1. **Automated checks** run on all PRs:
   - Build verification
   - Test suite
   - Code formatting
   - Linting

2. **Manual review** by maintainers:
   - Code quality
   - Test coverage
   - Documentation
   - Security considerations

3. **Feedback and iteration**:
   - Address review comments
   - Push updates to your branch
   - Request re-review when ready

## 🚀 Adding New Contracts

When adding a new contract to the workspace:

1. **Create the contract structure**:
   ```bash
   mkdir contracts/new_contract
   cd contracts/new_contract
   mkdir src
   ```

2. **Create Cargo.toml**:
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

3. **Update workspace** in `contracts/Cargo.toml`:
   ```toml
   [workspace]
   members = [
       "upcurrent_escrow",
       "new_contract",
   ]
   ```

4. **Implement contract** in `src/lib.rs`

5. **Add tests** in `src/test.rs`

6. **Update documentation** in `contracts/README.md`

## 🐛 Reporting Bugs

When reporting bugs in contracts:

1. **Check existing issues** first
2. **Provide details**:
   - Contract name and function
   - Expected behavior
   - Actual behavior
   - Steps to reproduce
   - Test case if possible

3. **Use the bug report template**

## 💡 Suggesting Features

For new contract features:

1. **Open a discussion** first for major changes
2. **Describe the use case** clearly
3. **Consider security implications**
4. **Propose an implementation approach**

## 🔒 Security

- **Never commit private keys** or secrets
- **Report security issues privately** to maintainers
- **Consider attack vectors** in your code
- **Use safe math operations** (Soroban SDK handles this)

## 📚 Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Stellar Developer Discord](https://discord.gg/stellardev)

## 🎉 Recognition

Contributors will be:
- Listed in project documentation
- Mentioned in release notes
- Invited to community calls

Thank you for contributing to UpCurrent! 🌊
