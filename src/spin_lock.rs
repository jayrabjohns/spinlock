use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicBool, Ordering},
};

use crate::spin_guard::SpinGuard;

pub struct SpinLock<T> {
    pub(crate) is_locked: AtomicBool,
    pub(crate) value: UnsafeCell<T>,
}

// Only spinlock is shared between threads, so interior value doesn't have to be sync
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            is_locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinGuard<T> {
        while self.is_locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        SpinGuard { lock: self }
    }

    /// Safety relies on &mut T not being used anymore
    pub fn unlock(&self) {
        self.is_locked.store(false, Ordering::Release)
    }
}
