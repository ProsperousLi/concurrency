use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = Metrics::new();

    //println!("{:?}", metrics.snapshot());

    for idx in 0..N {
        task_worker(idx, metrics.clone()).unwrap();
    }

    for _ in 0..M {
        request_worker(metrics.clone()).unwrap();
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics)
    }

    //Ok(())
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_secs(rng.gen_range(1..10)));
            metrics.increment(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_secs(rng.gen_range(1..10)));
            let page = rng.gen_range(1..30);
            metrics.increment(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
