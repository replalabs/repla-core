use crate::hash::state_root;
use crate::state::{Action, StateDelta};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::RwLock;

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
}

#[allow(dead_code)]
fn _unused_helper(_d: &StateDelta) -> [u8; 32] {
    state_root(&[])
}

#[allow(dead_code)]
fn _unused_result() -> Result<()> {
    Ok(())
}
