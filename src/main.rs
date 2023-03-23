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

/*

fn is_prime(n: usize) -> bool {
    (2..n).all(|i| { n % i != 0 })
}

fn producer(tx :mpsc::SyncSender<usize>) -> thread::JoinHandle<()> {
    thread::spawn(move || for i in 100_000_000.. {
        tx.send(i).unwrap();
    })
}

fn worker(id: u64, shared_rx: Arc<Mutex<mpsc::Receiver<usize>>>) {
    let builder = thread::Builder::new().name(format!("thread-{}", id));

    thread::spawn(move || loop {
        {
            if let Ok(rx) = shared_rx.lock() {
                if let Ok(n) = rx.try_recv() {
                    if is_prime(n) {
                        // println!("worker {} found a prime: {}", id, n);
                        println!("worker {} found a prime: {}", thread::current().name().unwrap(), n);
                    }
                }
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
*/

/*

fn plus_one_by_ref(n: &mut usize) {
    *n = *n + 1;
}

fn main() {
    let mut i = 17;
    plus_one_by_ref(&mut i);
    println!("i: {}", i);
    plus_one_by_ref(&mut i);
    println!("i: {}", i);
}
*/

/*
fn main() {
    // ----------------------- immutable ptr
    let a = 13;
    let a_ptr: *const i32 = &a;
    println!("{:#?}", a_ptr);
    */

    /*
    // ----------------------- mutable ptr
    let mut b = 10;
    let b_ptr: *mut i32 = &mut b;
    
    println!("{:#?}", b_ptr);
    unsafe {
        *b_ptr = 20;
    }
    println!("{:#?}", b_ptr);
    println!("{}", b);
    */

    /*
    let bx = Box::new(16);
    let bx_ptr: *const i32 = &*bx;

    println!("{:#?}", bx_ptr);
    
    let mut boxy = Box::new(17);
    let boxy_ptr: *mut i32 = &mut *boxy;
    
    println!("{:#?}", boxy_ptr);
    */

    /*
    fn foo(x: &mut i32, y: &mut i32) -> i32 {
        *x = 50;
        *y = 99;
        *x
    }

    let mut n = 0;
    let n_ptr = &mut n as *mut i32;
    let n_mut1 = unsafe { &mut *n_ptr };
    let n_mut2 = unsafe { &mut *n_ptr };

    println!("OUTPUT: {}", foo(n_mut1, n_mut2));

    let v = String::from("Hello");
}
*/

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("Hello world");

    let word = first_word(&s);

    println!("{}", word);
    
    s.clear();

    // ------------------

    let a = [0, 1, 2, 3];
    let v = &a[..2];

    let v = vec![1, 2, 3, 4, 5];
    let z = &v[..2];
}