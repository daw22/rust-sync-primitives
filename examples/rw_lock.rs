 use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main(){

    let config: Arc<RwLock<&str>> = Arc::new(RwLock::new("version 1"));
    // create multiple read threads
    let mut handles = vec![];
    for i in 0..3 {
        let config = Arc::clone(&config);
        handles.push(thread::spawn(move || {
            println!("thread {} sees {}", i, *config.read().unwrap());
            // hold read lock for some time
            thread::sleep(Duration::from_millis(500));
        }));
    }
    //trying to starve writers
    let conf = Arc::clone(&config);
    let mut count = 0;
    handles.push(thread::spawn(move || {
        loop {
            println!("starver thread sees: {}", *conf.read().unwrap());
            // hold it for a bit
            thread::sleep(Duration::from_millis(100));
            count += 1;
            if count > 100 {
                break;
            }
        }
    }));

    for handle in handles {
        handle.join().unwrap();
    }
    // try write lock
    println!("main thread trying to aquire write lock");
    let mut val = config.write().unwrap();
    *val = "version 2 (updated!)";
    println!("main thread finished writing");
}
