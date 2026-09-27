# Contract Deployments

This document records deployed smart contract instances for StellarTickets across Stellar networks (`testnet`, `futurenet`, `mainnet`).

---

## Active Deployments

| Network | Contract ID | WASM Hash | Date (UTC) | Status | Notes |
|---|---|---|---|---|---|
| `testnet` | `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAK3IM` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` | 2026-09-20 | Reference / Active | Testnet soak test deployment |
| `testnet` | *Pending next deploy* | *Pending* | *Pending* | Staged | Built with `scripts/build.sh` |

---

## Deployment Details

### Testnet

- **Network:** `testnet` (Passphrase: `Test SDF Network ; September 2015`, RPC: `https://soroban-testnet.stellar.org`)
- **Contract Name:** `stellar-tickets-ticketing`
- **Contract ID:** `CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAK3IM`
- **WASM Hash:** `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- **Date:** 2026-09-20 12:00:00 UTC
- **Explorer:** [Stellar Expert (Testnet)](https://stellar.expert/explorer/testnet/contract/CAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAK3IM)
- **Initialization:**
  - Admin: `GDADMIN...`
  - Payment Token: Testnet native XLM / SEP-41 token wrapper
  - Minimum Purchase Spacing: `0` ledgers (disabled)

---

## Deployment Record Template

Use the following template to log every new contract deployment. Copy and fill out the table below when deploying to `testnet`, `futurenet`, or `mainnet`.

```markdown
### Deployment Record: [<NETWORK>] - YYYY-MM-DD

| Field | Value |
|---|---|
| **Network** | `testnet` \| `futurenet` \| `mainnet` |
| **Contract ID** | `C...` |
| **WASM Hash** | `<sha256-hash-of-optimized-wasm>` |
| **Deployment Date (UTC)** | `YYYY-MM-DD HH:MM:SS UTC` |
| **Deployer Identity** | `<deployer-identity-or-address>` |
| **Admin Address** | `<admin-address-or-multisig>` |
| **Payment Token Address** | `<sep41-token-contract-id>` |
| **Stellar CLI Version** | `stellar X.Y.Z` |
| **Git Commit** | `<git-commit-sha>` |
| **WASM File Path** | `target/wasm32v1-none/release/stellar_tickets_ticketing.optimized.wasm` |
| **Notes / Purpose** | `<e.g., v0.1.0 release, staging soak test, security audit fix>` |
```

---

## Verification & Inspection

### 1. Compute WASM SHA-256 Hash
After building and optimizing with `scripts/build.sh`:
```bash
sha256sum target/wasm32v1-none/release/stellar_tickets_ticketing.wasm
# or if optimized:
sha256sum target/wasm32v1-none/release/stellar_tickets_ticketing.optimized.wasm
```

### 2. Inspect Contract on Network
Inspect contract metadata and specs using the Stellar CLI:
```bash
stellar contract info --id <CONTRACT_ID> --network <NETWORK>
```

### 3. Verify Contract Code on Explorer
- **Testnet:** `https://stellar.expert/explorer/testnet/contract/<CONTRACT_ID>`
- **Mainnet:** `https://stellar.expert/explorer/public/contract/<CONTRACT_ID>`
