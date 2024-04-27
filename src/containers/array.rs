use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

struct RawArray<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawArray<T> {}

unsafe impl<T: Sync> Sync for RawArray<T> {}

impl<T> RawArray<T> {
    fn with_cap(cap: usize) -> Self {
        let layout = Layout::array::<T>(cap).unwrap();
        assert!(layout.size() <= isize::MAX as usize, "Allocation too large");
        let raw_ptr = unsafe { alloc::alloc(layout) };

        let ptr = match NonNull::new(raw_ptr as *mut T) {
            None => alloc::handle_alloc_error(layout),
            Some(p) => p,
        };

        RawArray { ptr, cap }
    }
}

impl<T> Drop for RawArray<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout) }
        }
    }
}

pub struct Array<T> {
    buf: RawArray<T>,
    size: usize,
}

impl<T> Array<T> {
    fn with_capacity(cap: usize) -> Self {
        let raw = RawArray::with_cap(cap);
        Array { buf: raw, size: 0 }
    }

    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn const_ptr(&self) -> *const T {
        self.buf.ptr.as_ptr().cast_const()
    }

    pub fn at(&self, index: usize) -> T {
        assert!(index < self.size, "Out of bound index");
        unsafe { ptr::read(self.const_ptr().add(index)) }
    }

    pub fn set(&mut self, index: usize, value: T) {
        assert!(index < self.size, "Out of bound index");
        unsafe {
            ptr::write(self.ptr().add(index), value);
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            unsafe {
                Some(ptr::read(self.const_ptr().add(self.size)))
            }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.buf.cap
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }
}

impl<T: Clone> Array<T> {
    fn fill(&mut self, value: T) {
        for i in 0..self.capacity() {
            let val = value.clone();
            unsafe { ptr::write(self.ptr().add(i), val) }
        }
        self.size = self.capacity();
    }

    pub fn with(value: T, cap: usize) -> Self {
        let mut arr = Array::with_capacity(cap);
        arr.fill(value);
        arr
    }
}

impl <T> Drop for Array<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl <T> Deref for Array<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.const_ptr(), self.size)
        }
    }
}

impl <T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.size)
        }
    }
}

#[macro_export]
macro_rules! array {
    ($x:expr; $n:expr) => {
        Array::with($x, $n)
    };
}

#[cfg(test)]
mod tests {
    use crate::containers::array::Array;

    #[test]
    fn test_array() {
        let mut arr = array![5; 10];
        assert_eq!(arr.capacity(), 10);
        assert_eq!(arr.size(), 10);
        assert_eq!(arr.at(3), 5);
        arr.set(3, 56);
        assert_eq!(arr.at(3), 56);
    }
}
