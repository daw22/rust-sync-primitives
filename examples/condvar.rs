use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;


fn main() {
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    let listner = thread::spawn(move || {
        let (lock, cvar) = & *pair2;
        // get a lock
        let mut gaurd = lock.lock().unwrap();
        while *gaurd < 5 {
            println!("waiting for a change/signal");
            gaurd = cvar.wait(gaurd).unwrap();
            println!("checking if i have 5");
        }
        println!("Got 5 starting work..");
    }); 


    // producer
    for _ in 0..5 {
        let (lock, cvar) = & *pair;
        thread::sleep(Duration::from_millis(500));
        let  mut gaurd = lock.lock().unwrap();
        *gaurd += 1;
        // notify of change
        cvar.notify_one();
        println!("incrimented by one");
    }
    listner.join().unwrap();
}
