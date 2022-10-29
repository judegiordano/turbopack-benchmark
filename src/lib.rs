use std::{process::{Command, Stdio}, time::Duration, io::Write};

fn shell() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    }
}

pub fn builder(turbo: bool) -> anyhow::Result<Duration> {
    // idk should these commands be disabling cache?
    let build_cmd = if turbo {
        "npx turbo build"
    } else {
        "npx next build"
    };
    tracing::info!("running {build_cmd}...");
    let mut cmd = shell();
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
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
