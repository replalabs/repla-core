# Security

## Threat model

The REPLA settler is an Anchor program on Solana. The table below lists only the
mitigations that are **implemented and enforced on-chain today** — each is
verifiable in `programs/repla-settler/src/lib.rs`. Mitigations that are part of
the design but not yet implemented are kept separate under
[Planned mitigations](#planned-mitigations) so this document does not overstate
the current guarantees.

| Actor | Capability | Implemented mitigation |
|-------|------------|------------------------|
| Replay attacker | Re-submits an old state delta | Monotonic settlement: `from_slot > last_settled_slot` is required on every `settle_state` (`NonMonotonicSlot`). |
| L1 reorg | A recent settlement reverts | `settle_lag`: a settlement is accepted only once `to_slot + N <= current slot` (`SettleLagNotMet`). |
| Duplicate settlement | Settles the same slot twice | Each settlement is a PDA keyed by `(l3_id, to_slot)`; re-initializing the same slot fails. |
| Stake griefer | Registers with a tiny stake to spam | `min_stake` floor enforced at registration (`StakeTooLow`). The stake is real lamports escrowed into a per-operator vault PDA, not a bookkeeping number. |
| Misbehaving sequencer | Operator must be penalized | `slash_sequencer` moves the slashed lamports from the operator's vault PDA to the protocol treasury PDA, bounded by the escrowed stake (`SlashExceedsStake`). This is **admin-gated** (see below). |

Stake lifecycle: `register_sequencer` transfers the stake from the operator into
a vault PDA (a real `SystemProgram` transfer); `withdraw_stake` returns unspent
stake to the operator; `slash_sequencer` routes a slashed amount to the treasury
PDA. All three are real lamport movements — the vault is a system-owned PDA that
signs its own outflows. Slashed lamports are **routed to the treasury PDA, not
burned**.

## Planned mitigations

The following are part of the design but are **not implemented yet**. They are
listed explicitly so there is no ambiguity about what the current program does
and does not enforce.

- **Automatic fraud detection.** Detecting an invalid state root (e.g. a
  two-sequencer disagreement or fraud proofs) and triggering slashing without
  operator intervention. Today `slash_sequencer` is **admin-gated and manual**
  (`has_one = admin`) — the program does not itself detect an invalid root.
- **Censorship resistance.** Stake-weighted sequencer election plus a user
  escape-hatch path for when a sequencer drops actions.
- **Reward-race protection.** First-valid-wins claiming with a cooldown to deter
  front-running of sequencer rewards.
- **Registration fee.** A protocol fee at registration, on top of the refundable
  `min_stake` escrow.

## RPC hygiene

- Client RPC is always the public Solana endpoint.
- DAS / `getAsset` calls go through a backend proxy.
- A build-time grep rejects any binary that ships a Helius / QuickNode key.

## CORS

The backend allows exactly four explicit origins. No wildcard; credentials enabled.

## Anchor profile

`lto = "fat"`, `overflow-checks = true`, `codegen-units = 1`. Settlement accounts
are `Box<>`-allocated to keep stack usage bounded.

## Audit plan

Pre-launch independent review of the anchor-settler crate. Bug bounty live at
launch. Reproducible builds via `anchor verify` against the deployed program hash.
