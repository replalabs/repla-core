# REPLA

**Solana app-specific rollup framework for games.**

A small, opinionated framework on top of MagicBlock's Ephemeral Rollup primitive. The crate now ships:

- A canonical `Action` / `StateDelta` wire format.
- A length-prefixed SHA-256 state root, with a TypeScript-parity test.
- `compute` for the sequencer / buyback-and-burn fee split.
- A first cut of `SequencerRuntime` that owns slot cadence and batching.

The on-chain settler client and the engine SDKs land next.

Apache-2.0.
