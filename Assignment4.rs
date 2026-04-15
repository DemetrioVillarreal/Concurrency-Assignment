use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();

    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: Create 2 producer threads
    let mut handles = Vec::new();

    for i in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(i, tx_clone, ITEM_COUNT);
        });
        handles.push(handle);
    }
    
    // TODO: Create 3 consumer threads
    for i in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(i, rx_clone);
        });
        handles.push(handle);
    }
    
    // TODO: Wait for all threads to finish

    // wait for producers first
    for i in 0..2 {
        handles[i].join().unwrap();
    }

    // send termination signal for each consumer
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // wait for consumers
    for i in 2..handles.len() {
        handles[i].join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }

    println!("Producer {} finished", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap();

        if value == TERMINATION_SIGNAL {
            println!("Consumer {} terminating", id);
            break;
        }

        println!("Consumer {} received {}", id, value);
        thread::sleep(Duration::from_millis(200));
    }
}