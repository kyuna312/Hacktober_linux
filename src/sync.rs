//! Synchronization primitives

use spin::mutex::SpinMutex;

/// A safe mutex implementation using spin locks
pub struct SafeMutex<T> {
    inner: SpinMutex<T>,
}

impl<T> SafeMutex<T> {
    /// Create a new mutex
    pub const fn new(value: T) -> Self {
        Self {
            inner: SpinMutex::new(value),
        }
    }

    /// Lock the mutex and get mutable access to the value
    pub fn lock(&self) -> spin::mutex::SpinMutexGuard<T> {
        self.inner.lock()
    }
}
