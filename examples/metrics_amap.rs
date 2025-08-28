use std::{thread, time::Duration};

use anyhow::Result;
use concurrency2::metrics::amap::AmapMetrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 3;

fn main() -> Result<()> {
    let metrics = AmapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "request.page.1",
        "request.page.2",
        "request.page.3",
        "request.page.4",
    ]);
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

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            thread::sleep(Duration::from_secs(rng.random_range(1..3)));
            metrics.inc(format!("call.thread.worker.{idx}")).unwrap();
        }
    });
    Ok(())
}

fn request_worker(metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::rng();
            thread::sleep(Duration::from_secs(rng.random_range(1..3)));
            let page = rng.random_range(1..5);
            metrics.inc(format!("request.page.{page}")).unwrap();
        }
    });
    Ok(())
}
