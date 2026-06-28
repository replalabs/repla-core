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
    FeeSplit {
        total,
        burn,
        sequencer,
    }
}

/// Reverse of `compute`: what action count would have produced this fee total?
/// Useful for replay / accounting cross-checks against on-chain Settlement records.
pub fn actions_for_total(cfg: FeeConfig, total: u64) -> u64 {
    if cfg.per_action_lamports == 0 {
        return 0;
    }
    total / cfg.per_action_lamports
}

/// Apply a registered fee config to an iterator of action batches and return the cumulative split.
pub fn aggregate<I: IntoIterator<Item = u32>>(cfg: FeeConfig, batches: I) -> FeeSplit {
    let mut sum = FeeSplit {
        total: 0,
        burn: 0,
        sequencer: 0,
    };
    for n in batches {
        let s = compute(cfg, n);
        sum.total = sum.total.saturating_add(s.total);
        sum.burn = sum.burn.saturating_add(s.burn);
        sum.sequencer = sum.sequencer.saturating_add(s.sequencer);
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_default_is_50_50() {
        let s = compute(FeeConfig::default(), 100);
        assert_eq!(s.total, 100_000);
        assert_eq!(s.burn, 50_000);
        assert_eq!(s.sequencer, 50_000);
    }

    #[test]
    fn burn_zero_when_bps_zero() {
        let s = compute(
            FeeConfig {
                per_action_lamports: 1_000,
                buyback_bps: 0,
            },
            10,
        );
        assert_eq!(s.burn, 0);
        assert_eq!(s.sequencer, 10_000);
    }

    #[test]
    fn burn_total_when_bps_max() {
        let s = compute(
            FeeConfig {
                per_action_lamports: 1_000,
                buyback_bps: 10_000,
            },
            10,
        );
        assert_eq!(s.burn, 10_000);
        assert_eq!(s.sequencer, 0);
    }
}
