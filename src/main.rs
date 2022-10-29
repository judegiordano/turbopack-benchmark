use anyhow::Result;
use clap::Parser;
use tracing_subscriber::FmtSubscriber;

use turbopack_benchmark::{builder, Args};

fn main() -> Result<()> {
    let lvl = tracing::Level::INFO;
    let subscriber = FmtSubscriber::builder().with_max_level(lvl).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let Args { directory, turbo } = Args::parse();
    let elapsed = builder(&directory, turbo)?;
    tracing::info!("build complete in {}ms", elapsed.as_millis());
    Ok(())
}
