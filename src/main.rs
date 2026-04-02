use std::clone;
use std::env;
use std::thread;
use std::time::Duration;

mod arc;
mod thread_pool;

fn main() {

    let var = env::var("THREADS").expect("Number of threads not provided!!");
    let num_threads: usize = var.parse().expect("Number of thread must be a number!!");
    println!("{}", num_threads);
    // initialaize the thread pool
    let pool = thread_pool::Pool::init(num_threads);
    for i in 0..100 {
        let job = move |id: usize| {
            println!("Job {} being done by a thread: {}", i, id);
            thread::sleep(Duration::from_millis(3000));
        }; 

        pool.execute(job);
        // give time for the workers to swoop in
        thread::sleep(Duration::from_millis(100));
    }

    // construct MyArc
    let my_arc = arc::MyArc::new(String::from("dawit"));
    let clone1 = my_arc.clone(); 
    
    thread::spawn(move || {
        println!("{}", *clone1);
    }).join().unwrap();

    {
        let _clone2 = my_arc.clone();
    }
}
