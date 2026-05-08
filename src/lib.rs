//! repla -- Solana app-specific rollup framework for games.
//!
//! The crate is split into a small set of focused modules:
//!
//! - [`runtime`] -- the sequencer loop.
//! - [`settler`] -- on-chain settler argument builder.
//! - [`state`] -- canonical Action and StateDelta types.
//! - [`hash`] -- length-prefixed SHA-256 used as the state root.
//! - [`fee`] -- fee math for sequencer rewards and buyback-and-burn.

pub mod fee;
pub mod hash;
pub mod runtime;
pub mod settler;
pub mod state;

pub use runtime::{SequencerConfig, SequencerRuntime};
pub use state::{Action, StateDelta};
