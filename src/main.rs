use anyhow::Result;
use clap::Parser;
use std::{
    io::Write,
    process::{Command, Stdio},
    time::Duration,
};
use tracing_subscriber::FmtSubscriber;

fn shell() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    }
}

pub fn builder(turbo: bool) -> Result<Duration> {
    let build_cmd = if turbo {
        "npx turbo build --no-cache"
    } else {
        "npx next build"
    };
    tracing::info!("running {build_cmd}...");
    // let out_file = if turbo { "out-turbo.txt" } else { "out.txt" };
    // let out = File::create(out_file)?;
    let mut cmd = shell();
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    // let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::from(out)).spawn()?;

    let a = format!("{}\n", build_cmd);
    let bytes = a.as_bytes();
    let now = std::time::Instant::now();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"cd twoyay-web\n")?;
    child.stdin.as_mut().unwrap().write_all(bytes)?;
    child.wait_with_output()?;
    let done = now.elapsed();
    Ok(done)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable building with turbopack
    #[arg(short, long, default_value_t = false)]
    turbo: bool,
}

fn main() -> Result<()> {
    let lvl = tracing::Level::INFO;
    let subscriber = FmtSubscriber::builder().with_max_level(lvl).finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let Args { turbo } = Args::parse();
    let elapsed = builder(turbo)?;
    tracing::info!("build complete in {}ms", elapsed.as_millis());
    Ok(())
}
