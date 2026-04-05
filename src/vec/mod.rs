// custom vec implementation

// layout
// the pointer to the allocation, the size of the allocation, number of initialized elements

use std::ops::{Deref, DerefMut};
use std::ptr::{ self, NonNull, copy};
use std::alloc::{Layout, alloc, dealloc, realloc};

#[derive(Debug)]
struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize
}

impl<T> RawVec<T> {
    fn new() -> Self {
        RawVec {
            ptr: NonNull::dangling(),
            cap: 0
        }
    }

    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 }else {self.cap * 2};
        let new_layout = Layout::array::<T>(new_cap).unwrap();
        assert!(new_layout.size() < isize::MAX as usize, "Allocation too Large!!");

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout)}
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
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
       if self.cap != 0 {
           let layout = Layout::array::<T>(self.cap).unwrap();
           unsafe { dealloc(self.ptr.as_ptr() as *mut u8, layout);}
       } 
    }
}

#[derive(Debug)]
pub struct MyVec<T> {
    // ptr: *mut T, // too strict -- invariant
    // ptr: NonNull<T>,
    // cap: usize,
    buf: RawVec<T>,
    len: usize,
}

unsafe impl<T: Send> Send for MyVec<T> {}
unsafe impl<T: Sync> Sync for MyVec<T> {}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        assert!(std::mem::size_of::<T>() != 0, "not handling ZSTs");
        MyVec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }
    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn push(&mut self, elem: T) {
        if self.cap() == self.len { self.buf.grow();}

        // do a blind write
        unsafe { std::ptr::write(self.ptr().add(self.len), elem);}

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        }else {

            unsafe {
                self.len -= 1;
                Some(std::ptr::read(self.ptr().add(self.len)))
            }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bound!!");
        if self.len == self.cap() { self.buf.grow()}
        unsafe {
           copy(
               self.ptr().add(index), 
               self.ptr().add(index + 1), 
               self.len - index
            ); 
           ptr::write(self.ptr().add(index), elem);
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) -> T{
        assert!(index <= self.len, "Index out of bound!!");
        unsafe {
            self.len -= 1;
            let removed_elem = ptr::read(self.ptr().add(index));
            copy(
                self.ptr().add(index + 1), 
                self.ptr().add(index), 
                self.len - index
            );
            removed_elem
        }
    } 
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap() != 0 {
            // drop every element
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.len).unwrap();
            unsafe {
                dealloc(self.ptr() as *mut u8, layout);
            }
        }
    }
} 

impl<T> Deref for MyVec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut [T]{
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        } 
    }
}

// IntoIter impl
pub struct IntoIter<T> {
    _buf: RawVec<T>,
    start: *const T,
    end: *const T,
}

impl<T> IntoIterator for MyVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        
        // construct IntoIter
        let ptr = unsafe {ptr::read(&self.buf)};
        let len = self.len;
        std::mem::forget(self);

        IntoIter {
            start: ptr.ptr.as_ptr(),
            end: if ptr.cap == 0 {
                ptr.ptr.as_ptr()
            }else {
                unsafe { ptr.ptr.as_ptr().add(len)}
            },
            _buf: ptr,
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        }else {
            unsafe {
                let val = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(val)
            }
        }
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
       if self.end == self.start {
           None
       }else {
           unsafe {
               self.end = self.end.offset(-1);
               Some(ptr::read(self.end))
           }
       }
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        // drain the buffer any remaining item
        if self._buf.cap != 0 {
            for _ in &mut *self {}
        }
    }
}
