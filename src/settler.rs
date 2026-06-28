//! On-chain settler client. Builds the wire-format payload for `settle_state` so a
//! signer can submit the instruction with their wallet. Network access is intentionally
//! out of scope -- pair this with `solana-sdk` for live submission.

use borsh::BorshSerialize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, BorshSerialize, Serialize, Deserialize)]
pub struct SettleStateArgs {
    pub l3_id: [u8; 32],
    pub from_slot: u64,
    pub to_slot: u64,
    pub state_root: [u8; 32],
    pub action_count: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum SettleError {
    #[error("to_slot ({to}) < from_slot ({from})")]
    InvalidSlotRange { from: u64, to: u64 },
    #[error("from_slot ({from}) <= last_settled ({last})")]
    NonMonotonic { from: u64, last: u64 },
}

pub fn build_settle_args(
    l3_id: [u8; 32],
    from_slot: u64,
    to_slot: u64,
    state_root: [u8; 32],
    action_count: u32,
    last_settled: u64,
) -> Result<SettleStateArgs, SettleError> {
    if to_slot < from_slot {
        return Err(SettleError::InvalidSlotRange {
            from: from_slot,
            to: to_slot,
        });
    }
    if from_slot <= last_settled {
        return Err(SettleError::NonMonotonic {
            from: from_slot,
            last: last_settled,
        });
    }
    Ok(SettleStateArgs {
        l3_id,
        from_slot,
        to_slot,
        state_root,
        action_count,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_monotonic() {
        let r = build_settle_args([0u8; 32], 100, 200, [0u8; 32], 1, 100);
        assert!(matches!(r, Err(SettleError::NonMonotonic { .. })));
    }

    #[test]
    fn rejects_inverted_range() {
        let r = build_settle_args([0u8; 32], 200, 100, [0u8; 32], 1, 0);
        assert!(matches!(r, Err(SettleError::InvalidSlotRange { .. })));
    }

    #[test]
    fn accepts_clean_args() {
        let r = build_settle_args([0u8; 32], 101, 200, [0u8; 32], 1, 100);
        assert!(r.is_ok());
    }
}
