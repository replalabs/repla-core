//! repla -- Solana app-specific rollup framework for games.
//!
//! This crate is the open-source reference implementation of the REPLA sequencer runtime
//! and a thin client into the Anchor settler program. It is deliberately small: read it
//! end-to-end in an afternoon.
//!
//! ## Pieces
//!
//! - [`runtime`] -- the sequencer loop. Wraps MagicBlock ER with deterministic batching.
//! - [`settler`] -- on-chain settler client. Builds `settle_state` instructions.
//! - [`state`] -- the canonical state delta + Borsh wire format.
//! - [`hash`] -- the canonical state-root hash (also implemented in TypeScript for parity).
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
pub mod runtime;
pub mod settler;
pub mod state;

pub use state::{Action, StateDelta};
pub use runtime::{SequencerConfig, SequencerRuntime};

/// Crate version as exposed in `--version` flags and health probes.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Apache-2.0 license SPDX identifier, surfaced by the dashboard.
pub const LICENSE_SPDX: &str = "Apache-2.0";

/// Default Anchor settler program id on Solana mainnet (base58).
pub const DEFAULT_PROGRAM_ID: &str = "42LxZbUQHUSiBvuVzo1YtAxbjDbxLDLHNmQhyG5wabVV";

/// Returns a short human-readable build banner. Use in CLI startup logs.
pub fn build_banner() -> String {
    format!(
        "repla v{} · {} · program {}",
        VERSION, LICENSE_SPDX, DEFAULT_PROGRAM_ID
    )
}
