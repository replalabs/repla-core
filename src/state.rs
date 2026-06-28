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

    pub fn is_empty(&self) -> bool {
        self.action_count == 0
    }
}

/// Convenience: turn a 64-character hex string into the 32-byte L3 ID used as a PDA seed.
pub fn l3_id_from_hex(hex: &str) -> Result<L3Id, &'static str> {
    let s = hex.strip_prefix("0x").unwrap_or(hex);
    if s.len() != 64 {
        return Err("expected 64 hex chars");
    }
    let mut out = [0u8; 32];
    for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
        let hi = decode_nibble(chunk[0])?;
        let lo = decode_nibble(chunk[1])?;
        out[i] = (hi << 4) | lo;
    }
    Ok(out)
}

fn decode_nibble(c: u8) -> Result<u8, &'static str> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(10 + c - b'a'),
        b'A'..=b'F' => Ok(10 + c - b'A'),
        _ => Err("invalid hex digit"),
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
