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
        return Err(SettleError::InvalidSlotRange { from: from_slot, to: to_slot });
    }
    if from_slot <= last_settled {
        return Err(SettleError::NonMonotonic { from: from_slot, last: last_settled });
    }
    Ok(SettleStateArgs {
        l3_id,
        from_slot,
        to_slot,
        state_root,
        action_count,
    })
}
