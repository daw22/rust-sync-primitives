use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

static INIT: OnceLock<String> = OnceLock::new();

fn get_value(name: String) -> String {
    let val = INIT.get_or_init(|| {
        // simulate a heavi initalization
        println!("thread {} doing the heavy initalization", name);
        thread::sleep(Duration::from_millis(1000));
        String::from("super secret password")
    });
    val.clone() 
}

struct LongText {
    text: String,
    len: OnceLock<usize>
}

impl LongText {
    fn new(txt: String) -> Self {
        LongText { text: txt, len: OnceLock::new()}
    }

    fn get_text_len(&self) -> usize {
        *self.len.get_or_init(|| {
            println!("first time access - initializing");
            thread::sleep(Duration::from_millis(500));
            self.text.split_whitespace().count()
        })
    } 
}

fn main() {
   let t1 = thread::spawn(|| {
       let val = get_value(String::from("one"));
        println!("thread one get: {}", val);
   });

   t1.join().unwrap();
   // give thread one a head start
   thread::sleep(Duration::from_millis(100));

    let t2 = thread::spawn(|| {
        let val = get_value(String::from("two"));
        println!("thread two get: {}", val);
    });
    t2.join().unwrap();

    // use oncelock with a struct field
    let x = Arc::new(LongText::new(String::from("dawit")));
    let y = Arc::clone(&x);
    let st = thread::spawn(move || {
        let len = y.get_text_len();
        println!("thread st gets len value: {}", len);
    });
    st.join().unwrap();
    // let st have a slight head start
    thread::sleep(Duration::from_millis(100));

    // access len from main thread later
    let len = x.get_text_len();
    println!("main thread got len: {}", len);
}
