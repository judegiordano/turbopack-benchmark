use anyhow::Result;
use clap::Parser;
use tracing_subscriber::FmtSubscriber;

use turbopack_benchmark::builder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Next.js Directory Path
    #[arg(short, long)]
    directory: String,

    /// Enable building with turbopack
    #[arg(short, long, default_value_t = false)]
    turbo: bool,
}

fn main() -> Result<()> {
    let lvl = tracing::Level::INFO;
    let subscriber = FmtSubscriber::builder().with_max_level(lvl).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let Args { directory, turbo } = Args::parse();
    let elapsed = builder(&directory, turbo)?;
    tracing::info!("build complete in {}ms", elapsed.as_millis());
    Ok(())
}
