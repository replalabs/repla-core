use anyhow::Result;
use clap::Parser;
use repla::runtime::{SequencerConfig, SequencerRuntime};
use tracing::info;
use tracing_subscriber::EnvFilter;

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
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();
    let args = Args::parse();
    let cfg = SequencerConfig {
        l3_id: [0u8; 32],
        slot_time_ms: args.slot_time_ms,
        settle_interval_slots: args.settle_interval,
    };
    info!("repla-sequencer starting (open-source build)");
    let rt = SequencerRuntime::new(cfg);
    rt.run(|delta| async move {
        info!(?delta, "delta emitted");
        Ok(())
    })
    .await?;
    Ok(())
}
