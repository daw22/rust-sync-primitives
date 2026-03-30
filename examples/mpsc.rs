use std::sync::mpsc::channel;
use std::thread;

fn main() {

    let (tx, rx) = channel();
    for i in 0..10 {
        let sender = tx.clone();
        let _ = thread::spawn(move || {
           let _ = sender.send(i);
        }).join();
    }
    // drop the original tx
    drop(tx);
    for r in rx {
        println!("{}", r);
    }
    println!("All messages recieved and cahnnel closed!");
}
