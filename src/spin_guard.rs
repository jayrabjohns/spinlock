use std::ops::{Deref, DerefMut};

use crate::spin_lock::SpinLock;

pub struct SpinGuard<'a, T> {
    pub(crate) lock: &'a SpinLock<T>,
}

unsafe impl<T> Send for SpinGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for SpinGuard<'_, T> where T: Sync {}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.unlock()
    }
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: This guard's lifetime provides a guarantee that the locks
        // value is acquired exclusively

        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: This guard's lifetime provides a guarantee that the locks
        // value is acquired exclusively

        unsafe { &mut *self.lock.value.get() }
    }
}
