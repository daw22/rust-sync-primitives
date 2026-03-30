use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let count = Arc::new(Mutex::new(0));

    let th1cl = Arc::clone(&count);
    let _ = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let _unused = th1cl.lock().unwrap();
        panic!("somethink whorable happend!!");
    }).join();

     let x= Arc::clone(&count);
     let _ = thread::spawn(move || {
        match count.lock() {
            Ok(mut val) => *val += 1,
            Err(poison) => {
                println!("poisoned lock aquired!! value inside: {}", poison.into_inner());
                count.clear_poison();
            }
        }
    }).join();

     match x.lock() {
         Ok(mut val) => {
             println!("recieved the recoverd mutex: {}", val);
             *val += 1;
             println!("incrimemted to : {}", val);
         },
         Err(_poison) => print!("still poisoned!")
     }
}
