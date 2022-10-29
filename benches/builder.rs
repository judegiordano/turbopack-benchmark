use std::{process::{Command, Stdio}, time::Duration, io::Write};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn shell() -> Command {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    }
}

pub fn builder(turbo: bool) -> anyhow::Result<Duration> {
    let build_cmd = if turbo {
        "npx turbo build --no-cache"
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

fn turbo_vs_next(c: &mut Criterion) {
    let mut group = c.benchmark_group("turbo-vs-next-build");
    group.significance_level(0.1).sample_size(10);
    group.bench_function("npx next build", |b| b.iter(|| builder(black_box(false))));
    group.bench_function("npx turbo build", |b| b.iter(|| builder(black_box(true))));
    group.finish();
}

criterion_group!(benches, turbo_vs_next);
criterion_main!(benches);
