use std::env;
use std::thread;
use std::time::Duration;

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
}
