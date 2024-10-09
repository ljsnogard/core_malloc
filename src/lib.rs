#![no_std]

mod core_alloc_;

pub use core_alloc_::{CoreAlloc, CoreAllocError};

pub mod x_deps {
    pub use abs_mm;
}