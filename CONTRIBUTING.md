# Contributing to UpCurrent 🌊

Thank you for your interest in contributing to UpCurrent! We're building decentralized invoice factoring infrastructure on Stellar and Soroban, and every contribution — from a typo fix to a new smart contract feature — moves us closer to that goal. This guide covers everything you need to get started.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Branch Naming Convention](#branch-naming-convention)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Smart Contract Development](#smart-contract-development)
- [Testing](#testing)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)
- [Community & Support](#community--support)

---

## Code of Conduct

By participating in this project, you agree to uphold our community standards:

- Be respectful and inclusive in all interactions.
- Give constructive, specific feedback — not personal criticism.
- Welcome contributors of all experience levels, including those new to Web3 or Soroban.
- Keep discussions focused on ideas and solutions, not individuals.

Violations may be reported to the maintainers and will be addressed promptly.

---

## Getting Started

### Prerequisites

Make sure you have the following installed before contributing:

**Frontend:**
- **Node.js** v18 or higher
- **npm** v9 or higher
- **Git**
- A **Freighter Wallet** browser extension (for local testing)

**Smart Contracts:**
- **Rust** (stable toolchain) — install via [rustup](https://rustup.rs/)
- **Soroban CLI** v21 or higher
- **wasm32 target** for Rust:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Local Setup

1. **Fork** the repository on GitHub.

2. **Clone** your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/upcurrent.git
   cd upcurrent
   ```

3. **Add the upstream remote:**
   ```bash
   git remote add upstream https://github.com/yourusername/upcurrent.git
   ```

4. **Install frontend dependencies:**
   ```bash
   npm install
   ```

5. **Build smart contract bindings** and pull them into the frontend:
   ```bash
   stellar contract bindings typescript \
     --wasm ./target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
     --output-dir ./src/contracts/upcurrent
   ```

6. **Start the development server:**
   ```bash
   npm run dev
   ```

   The app will be available at `http://localhost:5173` (Vite default).

---

## Development Workflow

1. Sync your fork with the latest upstream changes before starting any work:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. Create a new branch for your changes (see [Branch Naming Convention](#branch-naming-convention)).

3. Make your changes, following the [Coding Standards](#coding-standards).

4. Write or update tests where applicable (see [Testing](#testing)).

5. Commit your changes using the [Commit Message Guidelines](#commit-message-guidelines).

6. Push your branch and open a Pull Request.

---

## Branch Naming Convention

Use the following prefixes to keep branches organized:

| Prefix | Purpose | Example |
|---|---|---|
| `feat/` | New features | `feat/investor-desk-yield-matrix` |
| `fix/` | Bug fixes | `fix/escrow-payout-race-condition` |
| `contract/` | Smart contract changes | `contract/dispute-lock-logic` |
| `docs/` | Documentation updates | `docs/update-setup-guide` |
| `chore/` | Maintenance tasks | `chore/upgrade-stellar-sdk` |
| `test/` | Adding or fixing tests | `test/invoice-mint-unit-tests` |
| `refactor/` | Code refactoring | `refactor/horizon-indexer-hooks` |

---

## Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification. Each commit message should be structured as:

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

**Types:** `feat`, `fix`, `docs`, `chore`, `test`, `refactor`, `style`, `perf`

**Scope** refers to the area of the codebase affected (e.g., `escrow`, `marketplace`, `sme-dashboard`, `investor-desk`, `horizon`, `contracts`, `wallet`).

**Examples:**

```
feat(investor-desk): add IRR calculator with risk tier adjustment

fix(escrow): resolve time-lock expiry check on early settlement

contract(escrow): add dispute-lock handshake verification logic

docs(readme): fix broken mermaid diagram closing tag

chore(deps): upgrade @stellar/wallets-kit to v2.3.0
```

Keep the subject line under 72 characters and write in the imperative mood ("add feature", not "added feature").

---

## Pull Request Process

1. Ensure your branch is up to date with `upstream/main` before opening a PR.

2. Fill out the PR template completely, including:
   - A clear description of what your change does and why.
   - Steps to reproduce (for bug fixes) or steps to test (for features).
   - Screenshots or screen recordings for UI changes.
   - For contract changes, include the Testnet contract ID used for verification.
   - Reference to any related issues (e.g., `Closes #42`).

3. All PRs must pass the following checks before review:
   - `npm run lint` — no linting errors
   - `npm run type-check` — no TypeScript errors
   - `npm run test` — all frontend tests passing
   - `cargo test` — all smart contract tests passing (for contract changes)

4. Request a review from at least one maintainer. Contract changes require review from a maintainer familiar with Soroban.

5. Address all review comments before merging. Resolved conversations should be marked as resolved by the reviewer.

6. PRs are merged using **squash and merge** to keep the commit history clean.

---

## Project Structure

```
upcurrent/
├── src/
│   ├── components/             # Reusable UI components
│   │   ├── sme/                # SME Dashboard components
│   │   └── investor/           # Investor Desk components
│   ├── contracts/              # Auto-generated Soroban TypeScript bindings
│   │   └── upcurrent/          # Escrow contract bindings
│   ├── hooks/                  # Custom React hooks (Horizon indexers, state)
│   ├── lib/                    # Utility functions and SDK wrappers
│   │   ├── stellar/            # Horizon API and network helpers
│   │   └── wallet/             # Stellar Wallets Kit integration
│   ├── pages/                  # Route-level page components
│   │   ├── dashboard/          # SME portal pages
│   │   └── investor/           # Investor portal pages
│   └── types/                  # Shared TypeScript type definitions
├── contracts/                  # Soroban smart contract source (Rust)
│   └── escrow/                 # Core escrow vault contract
│       ├── src/
│       └── Cargo.toml
├── public/                     # Static assets
├── vite.config.ts
└── .env.local                  # Local environment variables (not committed)
```

---

## Coding Standards

### Frontend (React / TypeScript)

- Write **TypeScript** for all new files — avoid `any` types.
- Prefer **functional components** and React Hooks.
- Keep components small and single-purpose; separate logic into custom hooks.
- Use **TailwindCSS** utility classes for all styling — avoid inline styles and custom CSS files unless absolutely necessary.
- Follow the existing dual-portal design system. Do not introduce new color or spacing values outside Tailwind's config.

### Stellar & Horizon Integration

- All Stellar SDK and Horizon API calls must be wrapped in `try/catch` with meaningful error states surfaced to the UI.
- Never expose private keys or sensitive wallet data — always use Stellar Wallets Kit for signing.
- Horizon indexer logic should be isolated in `src/hooks/` or `src/lib/stellar/` and not scattered across components.
- Default to **Testnet** in all development and PR environments. Mainnet usage is restricted to the production deployment.

---

## Smart Contract Development

UpCurrent's escrow logic lives in `contracts/escrow/` and is written in **Rust** targeting the Soroban environment.

### Building Contracts

```bash
# Build the escrow contract
cd contracts/escrow
cargo build --target wasm32-unknown-unknown --release

# Regenerate TypeScript bindings after contract changes
stellar contract bindings typescript \
  --wasm ./target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
  --output-dir ../../src/contracts/upcurrent
```

### Deploying to Testnet

```bash
stellar contract deploy \
  --wasm ./target/wasm32-unknown-unknown/release/upcurrent_escrow.wasm \
  --source YOUR_ACCOUNT_ALIAS \
  --network testnet
```

### Contract Guidelines

- All public contract functions must include doc comments explaining inputs, outputs, and error conditions.
- Time-lock and escrow logic must be covered by unit tests using Soroban's native test environment.
- Never introduce breaking changes to existing contract interfaces without a deprecation notice and migration path.
- Contract changes must be deployed and verified on **Testnet** before a PR can be merged.
- Do not deploy to **Mainnet** outside of the official release process managed by maintainers.

---

## Testing

### Frontend

We use **Vitest** and **React Testing Library** for unit and integration tests.

```bash
# Run all frontend tests
npm run test

# Run tests in watch mode
npm run test:watch

# Check TypeScript types
npm run type-check

# Lint the codebase
npm run lint
```

### Smart Contracts

Soroban contracts are tested using Rust's native test framework with the Soroban SDK test utilities.

```bash
cd contracts/escrow
cargo test
```

### Guidelines

- Write tests for all new utility functions in `src/lib/`.
- Cover key user flows (invoice minting, marketplace listing, escrow settlement, investor claim) with integration tests.
- Mock Stellar SDK, Horizon API, and Wallets Kit calls in frontend tests — do not make real network requests in the test suite.
- Every new contract function must have at least one unit test covering the happy path and one covering a failure/edge case.

---

## Reporting Bugs

Found a bug? Please [open an issue](https://github.com/yourusername/upcurrent/issues/new?template=bug_report.md) and include:

- A clear, descriptive title.
- Steps to reproduce the issue.
- Expected vs. actual behavior.
- Your environment (OS, browser, Node.js version, Soroban CLI version, network: Testnet/Mainnet).
- Any relevant console errors, transaction hashes, or screenshots.

> ⚠️ **Security vulnerabilities** — especially those affecting escrow or settlement logic — should **not** be reported in public issues. Please contact the maintainers directly and privately.

---

## Suggesting Features

Have an idea? We'd love to hear it. [Open a feature request](https://github.com/yourusername/upcurrent/issues/new?template=feature_request.md) and describe:

- The problem your feature would solve.
- Your proposed solution or approach.
- Any alternatives you've considered.
- Whether it aligns with the current [SCF Milestone Roadmap](README.md#-scf-milestone-roadmap-build-award-open-track).

Feature requests are discussed openly and prioritized based on community feedback and roadmap alignment.

---

## Community & Support

- **GitHub Discussions** — for questions, ideas, and general conversation.
- **Issues** — for confirmed bugs and tracked feature requests.
- **Stellar Discord** — join the `#dev-upcurrent` channel for real-time discussion with the team.

We appreciate every contribution. Whether you're fixing a typo, improving test coverage, or writing Soroban escrow logic — you're helping unlock liquidity for businesses around the world. Thank you. 🌍
