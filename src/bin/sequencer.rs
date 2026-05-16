use anyhow::Result;
use clap::Parser;
use repla::runtime::{SequencerConfig, SequencerRuntime};

#[derive(Parser, Debug)]
#[command(version, about = "REPLA sequencer node")]
struct Args {
    #[arg(long, default_value_t = 50)]
    slot_time_ms: u32,

    #[arg(long, default_value_t = 200)]
    settle_interval: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = SequencerConfig {
        l3_id: [0u8; 32],
        slot_time_ms: args.slot_time_ms,
        settle_interval_slots: args.settle_interval,
    };
    let rt = SequencerRuntime::new(cfg);
    rt.run(|_delta| async move { Ok(()) }).await?;
    Ok(())
}
