use crate::hash::state_root;
use crate::state::{Action, StateDelta};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencerConfig {
    pub l3_id: [u8; 32],
    pub slot_time_ms: u32,
    pub settle_interval_slots: u32,
}

impl Default for SequencerConfig {
    fn default() -> Self {
        Self {
            l3_id: [0u8; 32],
            slot_time_ms: 50,
            settle_interval_slots: 200,
        }
    }
}

pub struct SequencerRuntime {
    config: SequencerConfig,
    pending: Arc<RwLock<VecDeque<Action>>>,
    current_slot: Arc<RwLock<u64>>,
    last_settled_slot: Arc<RwLock<u64>>,
}

impl SequencerRuntime {
    pub fn new(config: SequencerConfig) -> Self {
        Self {
            config,
            pending: Arc::new(RwLock::new(VecDeque::new())),
            current_slot: Arc::new(RwLock::new(0)),
            last_settled_slot: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn submit_action(&self, action: Action) {
        self.pending.write().await.push_back(action);
    }

    pub async fn pending_count(&self) -> usize {
        self.pending.read().await.len()
    }

    pub async fn current_slot(&self) -> u64 {
        *self.current_slot.read().await
    }

    pub async fn last_settled(&self) -> u64 {
        *self.last_settled_slot.read().await
    }

    pub async fn run<F, Fut>(&self, on_delta: F) -> Result<()>
    where
        F: Fn(StateDelta) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send,
    {
        let mut ticker = interval(Duration::from_millis(self.config.slot_time_ms as u64));
        let mut batch_count: u32 = 0;
        let mut batch_payloads: Vec<Vec<u8>> = Vec::new();
        let mut batch_from: u64 = 1;

        info!(
            slot_time_ms = self.config.slot_time_ms,
            settle_interval = self.config.settle_interval_slots,
            "sequencer started"
        );

        loop {
            ticker.tick().await;
            let slot = {
                let mut s = self.current_slot.write().await;
                *s += 1;
                *s
            };
            debug!(slot, "tick");

            let drained: Vec<Action> = self.pending.write().await.drain(..).collect();
            for action in drained {
                batch_count = batch_count.saturating_add(1);
                batch_payloads.push(action.payload);
            }

            if slot % self.config.settle_interval_slots as u64 == 0 {
                let refs: Vec<&[u8]> = batch_payloads.iter().map(|p| p.as_slice()).collect();
                let root = state_root(&refs);
                let delta = StateDelta {
                    l3_id: self.config.l3_id,
                    from_slot: batch_from,
                    to_slot: slot,
                    state_root: root,
                    action_count: batch_count,
                };
                info!(from = batch_from, to = slot, actions = batch_count, "emit delta");
                if on_delta(delta).await.is_ok() {
                    *self.last_settled_slot.write().await = slot;
                }
                batch_count = 0;
                batch_payloads.clear();
                batch_from = slot + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn submit_then_pending_counts() {
        let rt = SequencerRuntime::new(SequencerConfig::default());
        rt.submit_action(Action {
            l3_id: [0u8; 32],
            actor: [1u8; 32],
            payload: vec![1, 2, 3, 4],
            slot_hint: 5,
        })
        .await;
        assert_eq!(rt.pending_count().await, 1);
    }
}
