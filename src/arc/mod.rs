
// layout

use std::isize;
use std::ops::Deref;
use std::sync::atomic::AtomicUsize;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct InnerArc<T> {
    count: AtomicUsize,
    data: T
}

pub struct MyArc<T> {
    // ptr: *mut InnerArc<T> -> this is invariant and gives a false info about ownership to the
    // dropck since raw pointers are treated as a loose reference to data owned by someone else
    ptr: NonNull<InnerArc<T>>,
    _ph: PhantomData<InnerArc<T>> // this solves the ownership problem and makes sure that we are
                                  // not lef with adangling ptr to T and helps the dropck to
                                  // claculate a corect drop order for the struct
}

impl<T> MyArc<T> {
    pub fn new(data: T) -> Self {
        let boxed = Box::new(InnerArc {
            count: AtomicUsize::new(1),
            data
        });

        MyArc {
            ptr: NonNull::new(Box::into_raw(boxed)).unwrap(),
            _ph: PhantomData
        }
    }
}

//implement sync and send so we can send it across threads
unsafe  impl<T: Send + Sync> Send for MyArc<T> {}
unsafe impl<T: Send + Sync> Sync for MyArc<T> {}

// implement Dref
impl<T> Deref for MyArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref()};
        &inner.data
    }
}

// clone
impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        // increment the counter
        let counter = unsafe { self.ptr.as_ref()};
        let old_counter = counter.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        /*
         * prevents an overflow of the counter(on a 32bit sys, usize ~4B)
         * if the counter overflows/wrap around to zero, one of the owners could think it is the last
         * one and clear the Arc form memory - relsulting in USE-AFTER-FREE on the next
         * access(likely from a leaked owner)
         * */
        if old_counter >= isize::MAX as usize {
            std::process::abort();
        } 
        println!("Refs Num: {}", old_counter + 1);
        Self {
            ptr: self.ptr,
            _ph: PhantomData
        }
    }
}

// implement Drop
impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        // check if counter is zero after decrementing
        let counter = unsafe { self.ptr.as_ref()};
        let old_count = counter.count.fetch_sub(1, std::sync::atomic::Ordering::Release);
        
        if old_count != 1 {
            return;
        }

        // put the fence
        std::sync::atomic::fence(std::sync::atomic::Ordering::Acquire);

        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}
