use std::env;
use std::sync::atomic::{ Ordering, AtomicUsize};
use std::thread;
use std::time::Duration;
use std::alloc::{System, GlobalAlloc};

use crate::vec::MyVec;

mod arc;
mod thread_pool;
mod vec;

// custom allocator
struct CountingAllocator {
    allocated_bytes: AtomicUsize,
    deallocated_bytes: AtomicUsize
}

unsafe impl GlobalAlloc for CountingAllocator {
   unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        unsafe {
            self.allocated_bytes.fetch_add(layout.size(), Ordering::SeqCst);
            // delegate the real allocation to the os
            System.alloc(layout)
        }
   } 

   unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
       unsafe {
           self.deallocated_bytes.fetch_add(layout.size(), Ordering::SeqCst);

            // delegate
            System.dealloc(ptr, layout);
       }
   }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator {
    allocated_bytes: AtomicUsize::new(0),
    deallocated_bytes: AtomicUsize::new(0)
};

fn main() {

    // let var = env::var("THREADS").expect("Number of threads not provided!!");
    // let num_threads: usize = var.parse().expect("Number of thread must be a number!!");
    // println!("{}", num_threads);
    // // initialaize the thread pool
    // let pool = thread_pool::Pool::init(num_threads);
    // for i in 0..100 {
    //     let job = move |id: usize| {
    //         println!("Job {} being done by a thread: {}", i, id);
    //         thread::sleep(Duration::from_millis(3000));
    //     }; 
    //
    //     pool.execute(job);
    //     // give time for the workers to swoop in
    //     thread::sleep(Duration::from_millis(100));
    // }
    //
    // // construct MyArc
    // let my_arc = arc::MyArc::new(String::from("dawit"));
    // let clone1 = my_arc.clone(); 
    //
    // thread::spawn(move || {
    //     println!("{}", *clone1);
    // }).join().unwrap();
    //
    // {
    //     let _clone2 = my_arc.clone();
    // }
    // test the custom allocator
    // println!("Before creating the vector: {}", GLOBAL.allocated_bytes.load(Ordering::SeqCst));
    // {
    //     let _ = vec![1, 2, 3, 4];
    //     println!("After creating a vector: {}", GLOBAL.allocated_bytes.load(Ordering::SeqCst));
    // }
    // println!("After the vector is droped, deallocated size: {}", GLOBAL.deallocated_bytes.load(Ordering::SeqCst));
    

    // try initialize my custom vec
    // let mut vec: vec::MyVec<u32> = vec::MyVec::new();
    // vec.push(1);
    // vec.push(2);
    // vec.push(3);
    // println!("{:?}", vec);
    // let _ = vec.pop();
    // println!("{:?}", vec);
    // // try initalizing it with ZST
    // // let _vec1: vec::MyVec<()> = vec::MyVec::new();
    // vec.insert(1, 12);
    // println!("{:?}", vec);
    // for i in vec.iter() {
    //     println!("{}", i);
    // }
    // vec.remove(1);
    // for i in vec.iter() {
    //     println!("{}", i);
    // }

    let mut vec: MyVec<String> = MyVec::new();
    

    vec.push(String::from("DAwit"));
    vec.push(String::from("DAwit"));
    vec.push(String::from("DAwit"));

    vec.drain();
    for (i, val) in vec.into_iter().enumerate() {
        println!("{}", val); 
        if i == 0 { 
            break;
        }
    }
    
}


