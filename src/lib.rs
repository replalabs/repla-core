//! repla -- Solana app-specific rollup framework for games.
//!
//! This crate is the open-source reference implementation of the REPLA sequencer runtime
//! and a thin client into the Anchor settler program. It is deliberately small: read it
//! end-to-end in an afternoon.
//!
//! ## Pieces
//!
//! - [`runtime`] -- the sequencer loop. Deterministic slot-based batching, designed to drive a MagicBlock ephemeral rollup.
//! - [`settler`] -- on-chain settler client. Builds `settle_state` instructions.
//! - [`state`] -- the canonical state delta + Borsh wire format.
//! - [`hash`] -- the canonical state-root hash (also implemented in TypeScript for parity).
//! - [`merkle`] -- a binary Merkle tree for per-action membership proofs.
//! - [`fee`] -- fee math (action × per-action × buyback split).
//!
//! ## Quickstart
//!
//! ```no_run
//! use repla::runtime::{SequencerConfig, SequencerRuntime};
//!
//! # async fn run() -> anyhow::Result<()> {
//! let cfg = SequencerConfig::default();
//! let rt = SequencerRuntime::new(cfg);
//! rt.run(|delta| async move {
//!     println!("settled {:?}", delta);
//!     Ok(())
//! }).await?;
//! # Ok(()) }
//! ```

pub mod fee;
pub mod hash;
pub mod merkle;
pub mod runtime;
pub mod settler;
pub mod state;

pub use merkle::{leaf_hash, merkle_root, verify_merkle_proof};
pub use runtime::{SequencerConfig, SequencerRuntime};
pub use state::{Action, StateDelta};

/// Crate version as exposed in `--version` flags and health probes.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Apache-2.0 license SPDX identifier, surfaced by the dashboard.
pub const LICENSE_SPDX: &str = "Apache-2.0";

/// Default Anchor settler program id on Solana devnet (base58).
pub const DEFAULT_PROGRAM_ID: &str = "42LxZbUQHUSiBvuVzo1YtAxbjDbxLDLHNmQhyG5wabVV";

/// Returns a short human-readable build banner. Use in CLI startup logs.
pub fn build_banner() -> String {
    format!(
        "repla v{} · {} · program {}",
        VERSION, LICENSE_SPDX, DEFAULT_PROGRAM_ID
    )
}
