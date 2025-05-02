use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};

const TERMINATION_SIGNAL: i32 = -1;

pub fn run() {
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles: Vec<Option<thread::JoinHandle<()>>> = vec![];

    // Producers
    for i in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        handles.push(Some(thread::spawn(move || {
            producer(i + 1, tx_clone, ITEM_COUNT / NUM_PRODUCERS);
        })));
    }

    // Consumers
    for i in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        handles.push(Some(thread::spawn(move || {
            consumer(i + 1, rx_clone);
        })));
    }

    // Wait for producers
    for i in 0..NUM_PRODUCERS {
        let handle = handles[i].take().unwrap();
        handle.join().unwrap();
    }

    // Send termination signals
    for _ in 0..NUM_CONSUMERS {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers
    for i in NUM_PRODUCERS..(NUM_PRODUCERS + NUM_CONSUMERS) {
        let handle = handles[i].take().unwrap();
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed.");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..=99);
        println!("Producer {} sent {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let num = rx.lock().unwrap().recv().unwrap();
        if num == TERMINATION_SIGNAL {
            println!("Consumer {} received termination", id);
            break;
        }
        println!("Consumer {} received {}", id, num);
        thread::sleep(Duration::from_millis(150));
    }
}