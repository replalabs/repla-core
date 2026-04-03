use serde::{Deserialize, Serialize};

pub type L3Id = [u8; 32];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub l3_id: L3Id,
    pub actor: [u8; 32],
    pub payload: Vec<u8>,
    pub slot_hint: u64,
}
