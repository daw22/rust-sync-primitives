use std::sync::{Arc, Mutex};
use std::thread;


fn main(){
    let count = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for i in 0..10 {
        let cl = Arc::clone(&count);
        let handle = thread::spawn(move || {
            let mut val = cl.lock().unwrap();
            *val += 1;
            println!("thread {} val {}", i, val)
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
