//! A custom allocator that allows a singular allocation of a specific size
//! known ahead of time.
//!
//! # Getting Started
//! This example allocates once by creating an `Arc` of the unit tuple `()`.
//! Requires libc for printing, but can be replaced with a serial port
//! implementation.
//!
//! ```rust
#![doc = include_str!("../examples/main.rs")]
//! ```
//! 
//! Run with `cargo +nightly run --example main` from within the repo.

#![no_std]

use core::{
    alloc::{GlobalAlloc, Layout},
    cell::UnsafeCell,
    ptr::null_mut,
    sync::atomic::{AtomicBool, Ordering},
};

const MAX_SUPPORTED_ALIGN: usize = 16;
/// A fixed-size single allocation allocator.
#[repr(C, align(16))]
pub struct Allocator<const SIZE: usize>(UnsafeCell<[u8; SIZE]>);

impl<const SIZE: usize> Allocator<SIZE> {
    /// Create a new one-time allocator
    #[inline]
    pub const fn new() -> Self {
        Self(UnsafeCell::new([0; SIZE]))
    }
}

static FULL: AtomicBool = AtomicBool::new(false);

unsafe impl<const SIZE: usize> Sync for Allocator<SIZE> {}

unsafe impl<const SIZE: usize> GlobalAlloc for Allocator<SIZE> {
    // Provide pointer so long as expected size, alignment, and called once.
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() != SIZE
            || layout.align() > MAX_SUPPORTED_ALIGN
            || FULL.fetch_or(true, Ordering::SeqCst)
        {
            return null_mut();
        }

        UnsafeCell::raw_get(&self.0).cast()
    }

    // Don't do anything on deallocation.
    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
