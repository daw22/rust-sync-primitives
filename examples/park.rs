use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

fn main () {

    let queue = Mutex::new(VecDeque::new());

    thread::scope(|s| {
        let t = s.spawn(|| loop {
            if let Some(item) = queue.lock().unwrap().pop_front() {
                dbg!(item);
            } else {
                thread::park();
            }
        });

        for i in 0.. {
            queue.lock().unwrap().push_back(i);
            t.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
        /*
           -> an unpark call before park, clears the parking call i.e the thread continues execution rather
              than sleeping
           -> unpark calls stack up, two unpark call on a thread that is not parked
                - the first unpark clears the park call
                - the second unpark gets lost, there for it is important we clear the queen rather than sleeping after one operation
           -> a stack unpark call could wake a pared call even when the queue is empty, no problem in code corectness but efficency ..
        */
    });
}

