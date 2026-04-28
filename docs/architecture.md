# Architecture

Four layers stacked on top of MagicBlock and the host engine.

- Engine SDK (Unity / Unreal / Godot)
- magicblock-adapter (TypeScript)
- sequencer-runtime (Rust)
- anchor-settler (Solana mainnet)

Game state never lives on L1 -- only the batched root.
