use std::collections::VecDeque;
use std::sync::atomic::AtomicUsize;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};

pub struct SharedData {
    pub jobs: VecDeque<Box<dyn FnOnce(usize) + Send + 'static>>,
    pub is_dead: bool
}

pub struct Pool {
    pub threads: Vec<thread::JoinHandle<()>>,
    pub data: Arc<(Mutex<SharedData>, Condvar)>,
    pub job_done_count: Arc<AtomicUsize>
}

impl Pool {
    pub fn init(num: usize) -> Self {

        let shared_data = SharedData {
            jobs: VecDeque::new(),
            is_dead: false
        }; 
        let data = Arc::new((Mutex::new(shared_data), Condvar::new()));
        let mut handles = vec![];
        for i in 0..num {
            let cloned_data = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let (lock, cvar) = & *cloned_data;
                loop {
                    let mut data = lock.lock().unwrap();

                    while data.jobs.is_empty() && !data.is_dead {
                        data = cvar.wait(data).unwrap();
                    }
                    // check if it is dead/ the producer
                    if data.is_dead {
                        println!("thread {} sees master is dead!", i);
                        break;
                    }
                    // pop and execute
                    match data.jobs.pop_front() {
                        Some(job) => {
                            //drop the lock and execute
                            drop(data);
                            job(i);
                        },
                        None => {}
                    }
                }
                println!("Thread {} closed down.", i);

            });
            handles.push(handle);
        }
        Pool{threads: handles, data, job_done_count: Arc::new(AtomicUsize::new(0))}

    }

    pub fn execute<F>(&self, f: F)
        where 
            F: FnOnce(usize) + Send + 'static
            {
                let (lock, cvar) = & *self.data;
                let mut data = lock.lock().unwrap();

                data.jobs.push_back(Box::new(f));
                println!("-- Added a new job to the queue --");
                cvar.notify_one();
            }
}
