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
    // create jobs and push/notify
    let (lock, cvar) = & *pool.data;
    for i in 0..100 {
        let job = move |id: usize| {
            println!("Job {} being done by a thread: {}", i, id);
            thread::sleep(Duration::from_millis(1000));
        }; 

        // let mut jobs = lock.lock().unwrap();
        // jobs.jobs.push_back(Box::new(job));
        // cvar.notify_one();
        // drop(jobs);
        // // give time for the workers to swoop in
        pool.execute(job);
        thread::sleep(Duration::from_millis(100));
    }

    // after pushing all the jobs check all jobs are taken and set is dead true
    loop {
        let mut jobs = lock.lock().unwrap();
        if jobs.jobs.is_empty() {
            jobs.is_dead = true;
            cvar.notify_all();
            break;
        }
        thread::sleep(Duration::from_millis(3000));
        println!("dead locked???");
    }
    for handle in pool.threads {
        handle.join().unwrap();
    }
}
