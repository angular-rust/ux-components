#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(dead_code)]

use std::cell::Cell;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};

thread_local! {
    static IS_MAIN_THREAD: Cell<bool> = Cell::new(false)
}

static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Asserts that this is the main thread and either `init` has been called.
macro_rules! assert_initialized_main_thread {
    () => {
        if !super::rt::is_initialized_main_thread() {
            if super::rt::is_initialized() {
                panic!("Clutter may only be used from the main thread.");
            } else {
                panic!("Clutter has not been initialized. Call `init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

/// Asserts that neither `init` has been called.
macro_rules! assert_not_initialized {
    () => {
        if super::rt::is_initialized() {
            panic!("This function has to be called before `init`.");
        }
    };
}

/// Returns `true` if Clutter has been initialized.
#[inline]
pub fn is_initialized() -> bool {
    skip_assert_initialized!();
    INITIALIZED.load(Ordering::Acquire)
}

/// Returns `true` if Clutter has been initialized and this is the main thread.
#[inline]
pub fn is_initialized_main_thread() -> bool {
    skip_assert_initialized!();
    IS_MAIN_THREAD.with(|c| c.get())
}

/// Informs this crate that Clutter has been initialized and the current thread is the main one.
pub unsafe fn set_initialized() {
    skip_assert_initialized!();
    if is_initialized_main_thread() {
        return;
    } else if is_initialized() {
        panic!("Attempted to initialize GDK from two different threads.");
    }
    INITIALIZED.store(true, Ordering::Release);
    IS_MAIN_THREAD.with(|c| c.set(true));
}

pub fn init() {
    assert_not_initialized!();
    // unsafe {
    //     ffi::clutter_init(ptr::null_mut(), ptr::null_mut());
    //     set_initialized();
    // }
    unimplemented!()
}

pub fn run() {
    // assert_initialized_main_thread!();
    // unsafe {
    //     ffi::clutter_main();
    //     set_initialized();
    // }
    unimplemented!()
}
