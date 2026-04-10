# REPLA

**Solana app-specific rollup framework for games.**

A small, opinionated framework on top of MagicBlock's Ephemeral Rollup primitive. The crate ships:

- A canonical `Action` / `StateDelta` wire format (Borsh, length-prefixed SHA-256 root).
- A `compute_state_root` contract that the TypeScript SDK has to match byte-for-byte.
- A `compute` function for the buyback-and-burn fee split.

The sequencer runtime, the on-chain settler client, and the engine SDKs land in later milestones.

Apache-2.0.
