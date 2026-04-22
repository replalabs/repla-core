use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FeeConfig {
    pub per_action_lamports: u64,
    pub buyback_bps: u16,
}

impl Default for FeeConfig {
    fn default() -> Self {
        Self {
            per_action_lamports: 1_000,
            buyback_bps: 5_000,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FeeSplit {
    pub total: u64,
    pub burn: u64,
    pub sequencer: u64,
}

pub fn compute(cfg: FeeConfig, action_count: u32) -> FeeSplit {
    let total = cfg.per_action_lamports.saturating_mul(action_count as u64);
    let burn = total.saturating_mul(cfg.buyback_bps as u64) / 10_000;
    let sequencer = total.saturating_sub(burn);
    FeeSplit { total, burn, sequencer }
}
