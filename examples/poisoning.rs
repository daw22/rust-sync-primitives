use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let count = Arc::new(Mutex::new(0));

    let th1cl = Arc::clone(&count);
    let _t1 = thread::spawn(move || {
        // thread::sleep(Duration::from_secs(1));
        let _unused = th1cl.lock().unwrap();
        panic!("somethink whorable happend!!"); // now the lock is poisoned
    });

    let x= Arc::clone(&count);
    let t2 = thread::spawn(move || {
        match count.lock() {
            Ok(mut val) => *val += 1,
            Err(poison) => {
                println!("poisoned lock aquired!! value inside: {}", poison.into_inner());
                count.clear_poison();
            }
        }
    });

    t2.join().unwrap(); // so main thread get the lock after the poison is cleared
    match x.lock() {
         Ok(mut val) => {
             println!("recieved the recovered mutex: {}", val);
             *val += 1;
             println!("incrimemted to : {}", val);
         },
         Err(_poison) => print!("still poisoned!")
    }

    // t1.join().unwrap();  - shouldn't join a panicked thread
}
