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
    let mut producer_handles = vec![];
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let handle1 = thread::spawn(move || {
        producer(1, tx1, ITEM_COUNT / 2);
    });

    let handle2 = thread::spawn(move || {
        producer(2, tx2, ITEM_COUNT / 2);
    });

    producer_handles.push(handle1);
    producer_handles.push(handle2);

    // TODO: Create 3 consumer threads
    let mut consumer_handles = vec![];
    for id in 1..=3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        consumer_handles.push(handle);
    }

    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send termination signal to each consumer
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let received = {
            let rx_locked = rx.lock().unwrap();
            rx_locked.recv()
        };

        match received {
            Ok(value) => {
                if value == TERMINATION_SIGNAL {
                    println!("Consumer {} received termination signal.", id);
                    break;
                } else {
                    println!("Consumer {} processing {}", id, value);
                    thread::sleep(Duration::from_millis(150));
                }
            }
            Err(_) => {
                println!("Consumer {} encountered an error. Exiting.", id);
                break;
            }
        }
    }
}
