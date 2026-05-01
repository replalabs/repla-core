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
