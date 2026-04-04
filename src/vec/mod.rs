// custom vec implementation

// layout
// the pointer to the allocation, the size of the allocation, number of initialized elements

use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;
use std::alloc::{Layout, alloc, dealloc, realloc};

#[derive(Debug)]
pub struct MyVec<T> {
    // ptr: *mut T, // too strict -- invariant
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "not handling ZSTs");
        MyVec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0
        }
    }
    // impl grow
    // check for OOM
    // allcation rule --> if cap == 0 ,allocate(1) cap =1 ::: if cap =1, cap *=2, allocate(cap)
    // check new_cap < isize::MAX, on allocation/reallocation
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1))
        } else {
            (self.cap * 2, Layout::array::<T>(self.cap * 2))
        };
        
        let new_layout = new_layout.expect("Allocation too large!");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        }else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size())}
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(ptr) => ptr,
            None => std::alloc::handle_alloc_error(new_layout)
        }; 
        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: T) {
        if self.cap == self.len { self.grow();}

        // do a blind write
        unsafe { std::ptr::write(self.ptr.as_ptr().add(self.len), elem);}

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        }else {

            unsafe {
                self.len -= 1;
                Some(std::ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // drop every element
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.len).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
} 

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T]{
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        } 
    }
}
