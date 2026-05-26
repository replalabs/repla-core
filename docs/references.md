# References

Prior art and source material the framework leans on.

- **MagicBlock Labs** -- Ephemeral Rollups. https://docs.magicblock.gg
- **Anchor** -- Coral's Anchor framework. https://www.anchor-lang.com
- **Solana validator** -- the L1 everything else assumes. https://docs.solana.com
- **Polygon CDK** -- AppChain framework for EVM. https://polygon.technology/polygon-cdk
- **Arbitrum Orbit** -- Orbit chains under the Arbitrum stack. https://docs.arbitrum.io/launch-orbit-chain/orbit-gentle-introduction
- **OP Stack** -- modular rollup stack for Optimism. https://stack.optimism.io
- **Borsh** -- canonical Solana serialization. https://borsh.io

The state-root contract and Borsh wire are intentionally simple so both Rust and TypeScript can implement them from scratch.
