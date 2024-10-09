﻿extern crate alloc;

use alloc::alloc::{alloc, dealloc};

use core::{
    alloc::Layout,
    fmt,
    ptr::{self, NonNull},
};

use abs_mm::mem_alloc::TrMalloc;

type MemAddr = NonNull<[u8]>;

#[derive(Debug, Clone)]
pub struct CoreAllocError;

impl fmt::Display for CoreAllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "alloc returns null pointer.")
    }
}

/// A wrapper for `alloc::alloc` and `alloc::dealloc`
#[derive(Debug, Default, Clone, Copy)]
pub struct CoreAlloc;

impl CoreAlloc {
    pub fn shared() -> &'static Self {
        static CORE_ALLOC: CoreAlloc = CoreAlloc;
        &CORE_ALLOC
    }

    pub const fn new() -> Self {
        CoreAlloc
    }

    pub fn can_support(&self, layout: Layout) -> bool {
        let _ = layout;
        true
    }

    pub fn allocate(
        &self,
        layout: Layout,
    ) -> Result<MemAddr, CoreAllocError> {
        unsafe {
            if let Option::Some(p) = NonNull::new(alloc(layout)) {
                #[cfg(test)]
                log::trace!(
                    "[CoreAlloc::allocate]({}, {}) returns {:?}",
                    layout.size(),
                    layout.align(),
                    p.as_ptr()
                );
                let slice = ptr::slice_from_raw_parts_mut(
                    p.as_ptr(),
                    layout.size(),
                );
                Result::Ok(NonNull::new_unchecked(slice))
            } else {
                Result::Err(CoreAllocError)
            }
        }
    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// * `ptr` must denote a block of memory [*currently allocated*] via this 
    ///   allocator, and
    /// * `layout` must [*fit*] that block of memory.
    ///
    /// [*currently allocated*]: #currently-allocated-memory
    /// [*fit*]: #memory-fitting
    pub unsafe fn deallocate(
        &self,
        ptr: MemAddr,
        layout: Layout,
    ) -> Result<usize, CoreAllocError> {
        #[cfg(test)]
        log::trace!(
            "[CoreAlloc::deallocate]({:?}) len: {}, layout: ({}, {})",
            ptr.as_ptr(),
            ptr.as_ref().len(),
            layout.size(),
            layout.align()
        );
        dealloc(ptr.as_ptr() as *mut _, layout);
        Result::Ok(layout.size())
    }
}

unsafe impl TrMalloc for CoreAlloc {
    type Err = CoreAllocError;

    #[inline(always)]
    fn can_support(&self, layout: Layout) -> bool {
        CoreAlloc::can_support(self, layout)
    }

    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<MemAddr, Self::Err> {
        CoreAlloc::allocate(self, layout)
    }

    #[inline(always)]
    unsafe fn deallocate(
        &self,
        ptr: MemAddr,
        layout: Layout,
    ) -> Result<usize, Self::Err> {
        CoreAlloc::deallocate(self, ptr, layout)
    }
}
