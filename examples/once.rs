use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();
fn main() {
    let _ = thread::spawn(||{
        INIT.call_once(|| {
            println!("this should only print once!");
        });
    }).join();

    INIT.call_once(|| {
        println!("this should only print once!-- agian -- this should not print");
    });
}
