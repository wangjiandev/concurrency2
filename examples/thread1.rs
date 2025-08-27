use anyhow::{Result, anyhow};
use std::{sync::mpsc, thread, time::Duration};

const N: usize = 5;

#[allow(dead_code)]
#[derive(Debug)]
struct Message {
    id: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..N {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    drop(tx); // 多一个tx， 否则rx无法结束

    let consumer = thread::spawn(|| {
        for msg in rx {
            println!("Received message: {msg:?}");
        }
        println!("Consumer finished");
    });

    println!("Consumer started");

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn producer(id: usize, tx: mpsc::Sender<Message>) -> Result<()> {
    loop {
        let value = rand::random::<u64>() as usize;
        tx.send(Message::new(id, value))?;
        let time_sleep = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(time_sleep));

        if rand::random::<u8>() % 10 == 0 {
            println!("Producer {id} finished");
            return Ok(());
        }
    }
}

impl Message {
    fn new(id: usize, value: usize) -> Self {
        Self { id, value }
    }
}
