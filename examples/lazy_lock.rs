use std::collections::HashMap;
use std::sync::{LazyLock};
use std::thread;
use std::time::Duration;

static DB_URL: LazyLock<String> = LazyLock::new(|| {
    println!("-- First time access...");
    // simulate heavy init process
    thread::sleep(Duration::from_millis(1000));
    String::from("https://blalalalalj")
});

static GLOBAL_DICT: LazyLock<HashMap<&'static str, &str>> = LazyLock::new(|| {
    // intt the HashMap
    let mut map = HashMap::new();
    println!("initialized the hashmap for the first time");
    map.insert("rust", "a systems programming language");
    map.insert("ethiopia", "an east african nation");
    map
});

fn main(){
    let t1 = thread::spawn(|| {
        println!("thread t1 gets: {}",  *DB_URL);
    });
    t1.join().unwrap();
    println!("main thread gets: {}",  *DB_URL);
    // use lazy lock with  hashmap
    
    let _ = thread::spawn(||{
        println!("first thread gets rust: {}", GLOBAL_DICT.get("rust").unwrap());
    }).join();

    println!("main thread gets ethiopia: {}", GLOBAL_DICT.get("ethiopia").unwrap());
}
