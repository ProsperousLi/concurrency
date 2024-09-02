use anyhow::Result;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_THREADS: usize = 4;

#[derive(Debug)]
struct Message {
    id: usize,
    data: String,
}
fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_THREADS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }

    let consumer_thread = thread::spawn(move || consumer(rx));

    let _ = consumer_thread
        .join()
        .map_err(|e| anyhow::anyhow!("Error joining thread error : {:?}", e))?;

    Ok(())
}

fn producer(i: usize, tx: mpsc::Sender<Message>) -> anyhow::Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Message::new(i, format!("{}", value)))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

fn consumer(rx: mpsc::Receiver<Message>) -> anyhow::Result<()> {
    loop {
        let msg = rx.recv()?;
        println!("{:?} and {:?}", msg.data, msg.id);
    }
}

impl Message {
    fn new(id: usize, data: String) -> Self {
        Message { id, data }
    }
}
