use std::{
    io::Write,
    process::{Command, Stdio},
    time::Duration,
};

fn shell() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    }
}

pub fn builder(dirname: &str, enable_turbo: bool) -> anyhow::Result<Duration> {
    // idk should these commands be disabling cache?
    let build_cmd = if enable_turbo {
        "npx turbo build"
    } else {
        "npx next build"
    };
    tracing::info!("running {build_cmd}...");
    let mut cmd = shell();
    // spawn child process
    let mut child = cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;
    // cd into next.js directory
    let dir = format!("cd {}\n", dirname);
    let bytes = dir.as_bytes();
    child.stdin.as_mut().unwrap().write_all(bytes)?;
    // build with given command
    let cmd = format!("{}\n", build_cmd);
    let bytes = cmd.as_bytes();
    let now = std::time::Instant::now();
    // write
    child.stdin.as_mut().unwrap().write_all(bytes)?;
    child.wait_with_output()?;
    let done = now.elapsed();
    Ok(done)
}
