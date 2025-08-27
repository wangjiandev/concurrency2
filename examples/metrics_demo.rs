use std::{thread, time::Duration};

use anyhow::Result;
use concurrency2::metrics::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 3;

fn main() -> Result<()> {
    let metrics = Metrics::new();
    println!("{metrics}");

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(3));
        println!("{metrics}");
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            thread::sleep(Duration::from_secs(rng.random_range(1..3)));
            metrics.inc(format!("call.thread.worker.{idx}"));
        }
    });
    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            thread::sleep(Duration::from_secs(rng.random_range(1..3)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("request.page.{page}"));
        }
    });
    Ok(())
}
