// Minimal example -- spawns the sequencer in-process and feeds it synthetic actions.
// `cargo run --example launch_local`

use anyhow::Result;
use repla::runtime::{SequencerConfig, SequencerRuntime};
use repla::state::Action;
use std::sync::Arc;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();

    let cfg = SequencerConfig {
        l3_id: [0xa1u8; 32],
        slot_time_ms: 20,
        settle_interval_slots: 50,
    };
    let rt = Arc::new(SequencerRuntime::new(cfg));

    let rt_feeder = Arc::clone(&rt);
    tokio::spawn(async move {
        let mut tick = tokio::time::interval(Duration::from_millis(40));
        let mut i: u32 = 0;
        loop {
            tick.tick().await;
            rt_feeder
                .submit_action(Action {
                    l3_id: [0xa1u8; 32],
                    actor: [0u8; 32],
                    payload: i.to_le_bytes().to_vec(),
                    slot_hint: 0,
                })
                .await;
            i += 1;
        }
    });

    rt.run(|delta| async move {
        println!(
            "settled · slot={}..{} · actions={} · root[..4]={:02x?}",
            delta.from_slot, delta.to_slot, delta.action_count, &delta.state_root[..4],
        );
        Ok(())
    })
    .await
}
