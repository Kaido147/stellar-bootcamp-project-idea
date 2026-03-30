# TulongTap: Offline First Disaster Relief

## Problem
During severe typhoons in the Philippines, power grids and cellular networks fail. Standard crypto wallets are useless for buying emergency supplies because they require a live internet connection to sign and broadcast transactions. 

## Solution
TulongTap uses Stellar Soroban's native authorization framework to allow offline signing. Victims tap an NFC card or phone at a local Sari-Sari store to generate a cryptographic signature. The merchant collects these signatures offline and syncs them to the Soroban contract once internet access is restored, releasing the USDC.

## Stellar Advantage
- **Offline Authorization:** Uses Soroban's `require_auth` to process deferred transactions.
- **Gas-less for Victims:** NGOs fund the contract, and merchants pay the sync fee. Victims need zero XLM to buy food.
- **Micro-payments:** Near-zero fees make small Sari-Sari store purchases viable.

## Build & Test
1. Build: `soroban contract build`
2. Test: `cargo test`

## Demo CLI
```bash
# 1. NGO Allocates 100 USDC to Victim
soroban contract invoke --id <CONTRACT_ID> --source admin --network testnet -- allocate_aid --admin <ADMIN_ADDR> --victim <VICTIM_ADDR> --amount 100

# 2. Merchant Syncs the Offline Payment (Requires victim's auth payload in a real-world frontend)
soroban contract invoke --id <CONTRACT_ID> --source merchant --network testnet -- offline_pay --victim <VICTIM_ADDR> --merchant <MERCHANT_ADDR> --amount 20