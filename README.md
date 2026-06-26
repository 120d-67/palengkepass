# PalengkePass

**Escrow-powered supply chain protection for Filipino micro-retailers.**

---

## Problem

Maria, a sari-sari store owner in Taguig, Philippines, loses ₱2,000/month in spoiled inventory because her vegetable supplier demands cash upfront but delivers inconsistently—leaving her no leverage to dispute late or partial orders.

## Solution

PalengkePass lets Maria fund a Soroban escrow contract with USDC that auto-releases payment only when she confirms delivery via SMS shortcode, giving her enforceable leverage over suppliers without needing a bank or lawyer.

---

## Timeline

| Phase | Duration | Deliverable |
|-------|----------|-------------|
| Smart contract development | 2 days | Core escrow logic |
| Testing & security review | 1 day | 5+ unit tests passing |
| Frontend MVP | 2 days | Mobile-responsive web app |
| Anchor integration | 1 day | PHP↔USDC on/off ramp |
| Demo polish | 1 day | End-to-end flow video |

---

## Stellar Features Used

- **USDC transfers** — Stable value for escrow deposits
- **Soroban smart contracts** — Conditional release logic with dispute resolution

---

## Vision and Purpose

PalengkePass transforms informal supply relationships into enforceable contracts. By requiring suppliers to earn payment through verified delivery, we shift power toward micro-retailers who currently have zero recourse against unreliable partners.

---

## Prerequisites

- Rust 1.74+ with `wasm32-unknown-unknown` target
- Soroban CLI v21.0.0+
- Stellar testnet account with test XLM

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf [sh.rustup.rs](https://sh.rustup.rs) | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli

## Deployed Contract

| Field | Value |
|-------|-------|
| Contract ID | `CCYGT52QKL3FNMCZIHE2ZTA5T2EI5WA76EV5FFADUOAEC3LB3ICTFEWO` |
| Network | testnet |
| Explorer | [View on stellar.expert](https://stellar.expert/explorer/testnet/contract/CCYGT52QKL3FNMCZIHE2ZTA5T2EI5WA76EV5FFADUOAEC3LB3ICTFEWO) |
| Deploy Tx | [View transaction](https://stellar.expert/explorer/testnet/tx/ddaf4c246bb8b780724ab7909a63cf1097edba04177a155bc2941cf15d7809a8) |
| Deployed | 2026-06-26 06:42:51 UTC |
| Wallet | freighter (`GDFW…JLE2`) |
