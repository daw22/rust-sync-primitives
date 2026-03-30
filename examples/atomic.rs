use std::sync::atomic::AtomicU32;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::Ordering;

fn main() {
    let x = Arc::new(AtomicU32::new(0));

    let mut threads = vec![];

    for _i in 0..10 {
        let xref = Arc::clone(&x);
        let handler = thread::spawn(move || {
            let val = xref.fetch_add(1, Ordering::Relaxed);
        });
        threads.push(handler);
    }
    for thread in threads {
        thread.join().unwrap();
    }
    println!("{}", x.load(Ordering::Relaxed));
}
