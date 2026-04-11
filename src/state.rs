use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

pub type L3Id = [u8; 32];

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Action {
    pub l3_id: L3Id,
    pub actor: [u8; 32],
    pub payload: Vec<u8>,
    pub slot_hint: u64,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct StateDelta {
    pub l3_id: L3Id,
    pub from_slot: u64,
    pub to_slot: u64,
    pub state_root: [u8; 32],
    pub action_count: u32,
}
