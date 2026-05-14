# REPLA

**Solana app-specific rollup framework for games. A workbench, not another chain.**

REPLA sits on top of MagicBlock's Ephemeral Rollup primitive and gives game teams a sequencer runtime, an Anchor settler client, and the engine SDK surface. The reference crate is deliberately small: read it end-to-end in an afternoon.

## What is in the crate

- `runtime` -- sequencer loop with deterministic batching.
- `settler` -- on-chain settle-instruction argument builder.
- `state` -- `Action` and `StateDelta` wire types.
- `hash` -- canonical length-prefixed SHA-256 state root.
- `fee` -- buyback-and-burn fee split.

Engine SDKs (Unity / Unreal / Godot) and the `repla-cli` are shipped from the framework monorepo and depend on this crate as the canonical reference.

Apache-2.0.
