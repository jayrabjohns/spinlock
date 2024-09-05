use std::{
    cell::UnsafeCell,
    collections::btree_map::Values,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// Only spinlock is shared between threads, so interior value doesn't have to be sync
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> &mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        unsafe { &mut *self.value.get() }
    }

    /// Safety relies on &mut T not being used anymore
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release)
    }
}
