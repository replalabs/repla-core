# Security

## Threat model

| Actor | Capability | Mitigation |
|-------|------------|------------|
| Malicious sequencer | Submits an invalid state root | Two-sequencer detection → `slash_sequencer` |
| Censoring sequencer | Drops user actions | Stake-weighted election; escape hatch path |
| Replay attacker | Re-submits an old delta | Monotonic `from_slot > last_settled_slot` |
| Front-runner | Races to claim a sequencer reward | First-valid-wins + cooldown |
| L1 reorg | Recent settlement reverts | `settle_lag` waits N slots before finality |
| Stake griefer | Tiny stake spam | `min_stake` floor + registration fee |

## RPC hygiene

- Client RPC is always the public Solana endpoint.
- DAS / `getAsset` go through a backend proxy.
- A build-time grep rejects any binary that ships a Helius / QuickNode key.

## CORS

The backend allows exactly four origins. No wildcard, credentials enabled.

## Anchor profile

`lto = "fat"`, `overflow-checks = true`, `codegen-units = 1`. Settle accounts are `Box<>`-allocated.

## Audit plan

Pre-launch independent review on the anchor-settler crate. Bug bounty live at launch. Reproducible builds via `anchor verify` against the deployed program hash.
