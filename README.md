# Anchor Escrow 

## Using Solana Kit, Kite, and Codama 🚀

## Introduction

This Solana program implements an **escrow**, enabling secure token swaps between users. For example, Alice can offer 10 USDC in exchange for 100 WIF.

Without an escrow, users face significant risks:

- **Traditional finance** charges 1-6% in fees, eating into your funds.
- **Manual swaps** are prone to fraud. If Bob takes Alice's 10 USDC but doesn't send the 100 WIF, or if Alice fails to deliver after receiving Bob's tokens, someone gets burned.

The **Anchor Escrow** program acts as a trusted intermediary, releasing tokens only when both parties meet the agreed terms. This ensures Alice and Bob each receive 100% of their desired tokens, securely and without middleman fees.

## Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/jeevan4476/Escrow.git
   cd Escrow
   ```

2. Install dependencies:

   ```bash
   npm install
   ```
   **or**
   ```bash
   yarn install
   ```

4. Run tests:

   ```bash
   # RUSTUP_TOOLCHAIN is needed for consistent builds per
   # https://solana.stackexchange.com/questions/21664/why-is-the-same-commit-of-an-anchor-repo-giving-different-results-when-run-at-di
   # TODO: remove when no longer necessary
   RUSTUP_TOOLCHAIN=nightly-2025-04-16 anchor test
   ```

5. Deploy the program:
   ```bash
   anchor deploy
   ```

## Changelog and Credits

See the [CHANGELOG](CHANGELOG.md) for updates and contributor credits.
