#![allow(unused)]

use std::thread;
use std::time::Duration;
use std::sync::{ mpsc, Arc, Mutex };

/*
type Int = usize;

const NUM_TIMERS: Int = 24;

fn timer(d: usize, tx: mpsc::Sender<Int>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        println!("{}: setting timer...", d);
        thread::sleep(Duration::from_secs(d as u64));
        tx.send(d).unwrap();
        println!("{} sent!", d);
    })
}

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..NUM_TIMERS {
        timer(i, tx.clone());
    }

    for v in rx.iter().take(NUM_TIMERS) {
        println!("{}: received!", v);
    }
}
*/

/*
fn main() {
    let c = Arc::new(Mutex::new(0));
    let mut hs = vec![];

    for _ in 0..10 {
        let _c = c.clone();
        
        hs.push(thread::spawn( move || {
            let mut num = _c.lock().unwrap();

            *num +=1;
        }));
    }

    for h in hs {
        h.join().unwrap();
    }

    println!("Result: {}", *c.lock().unwrap()); // 10
}
*/

fn is_prime(n: usize) -> bool {
    (2..n).all(|i| { n % i != 0 })
}

fn producer(tx :mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
    thread::spawn(move || for i in 100_000_000.. {
        tx.send(i).unwrap();
    })
}

fn worker(id: u64, shared_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
    thread::spawn(move || loop {
        {
            match shared_rx.lock() {
                Ok(rx) => {
                    match rx.try_recv() {
                        Ok(n) => {
                            if is_prime(n) {
                                println!("worker {} found a prime: {}", id, n);
                            }
                        },
                        Err(_) => ()
                    }
                },
                Err(_) => ()
            } 
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(1024);

    let shared_rx = Arc::new(Mutex::new(rx));

    for i in 1..5 {
        worker(i, shared_rx.clone());
    }

    producer(tx).join().unwrap();
}