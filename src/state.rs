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

impl StateDelta {
    pub fn slot_range(&self) -> u64 {
        self.to_slot.saturating_sub(self.from_slot) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_range_inclusive() {
        let d = StateDelta {
            l3_id: [0u8; 32],
            from_slot: 10,
            to_slot: 19,
            state_root: [0u8; 32],
            action_count: 0,
        };
        assert_eq!(d.slot_range(), 10);
    }
}
