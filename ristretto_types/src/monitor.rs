use crate::{Error, Result};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::{Notify, Semaphore};

/// A monitor is a synchronization mechanism that allows threads to have:
/// 1. Mutual exclusion (via locks)
/// 2. Cooperation (via wait/notify)
#[derive(Debug)]
pub struct Monitor {
    /// The semaphore for mutual exclusion. Capacity 1 acts as a mutex.
    lock: Semaphore,
    /// The thread ID that currently owns the monitor.
    owner: Mutex<Option<u64>>,
    /// The number of times the owner has entered the monitor (reentrancy).
    entry_count: Mutex<usize>,
    /// The notification mechanism for wait/notify.
    notify: Notify,
    /// Count of waiting threads to optimize notify
    wait_count: AtomicUsize,
}

impl Default for Monitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Monitor {
    /// Create a new monitor.
    #[must_use]
    pub fn new() -> Self {
        Self {
            lock: Semaphore::new(1),
            owner: Mutex::new(None),
            entry_count: Mutex::new(0),
            notify: Notify::new(),
            wait_count: AtomicUsize::new(0),
        }
    }

    /// Acquire the monitor lock.
    /// Handles reentrancy: if the current thread already owns the monitor, it increments the count.
    ///
    /// # Errors
    /// if the lock cannot be acquired.
    pub async fn acquire(&self, thread_id: u64) -> Result<()> {
        {
            let owner = self.owner.lock();
            if *owner == Some(thread_id) {
                let mut count = self.entry_count.lock();
                *count += 1;
                return Ok(());
            }
        }

        let permit = self
            .lock
            .acquire()
            .await
            .map_err(|_| Error::InternalError("Monitor semaphore closed".into()))?;
        permit.forget();

        {
            let mut owner = self.owner.lock();
            *owner = Some(thread_id);
            let mut count = self.entry_count.lock();
            *count = 1;
        }

        Ok(())
    }

    /// Release the monitor lock.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub fn release(&self, thread_id: u64) -> Result<bool> {
        let mut owner = self.owner.lock();
        if *owner != Some(thread_id) {
            return Err(crate::JavaError::IllegalMonitorStateException(
                "Current thread does not own the monitor".into(),
            )
            .into());
        }

        let mut count = self.entry_count.lock();
        *count -= 1;

        if *count == 0 {
            *owner = None;
            self.lock.add_permits(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Wait for notification.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub async fn wait(&self, thread_id: u64) -> Result<()> {
        let saved_count = {
            let owner = self.owner.lock();
            if *owner != Some(thread_id) {
                return Err(crate::JavaError::IllegalMonitorStateException(
                    "Current thread does not own the monitor".into(),
                )
                .into());
            }
            *self.entry_count.lock()
        };

        {
            let mut owner = self.owner.lock();
            *owner = None;
            let mut count = self.entry_count.lock();
            *count = 0;
            self.lock.add_permits(1);
        }

        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        notified.await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);

        let permit = self
            .lock
            .acquire()
            .await
            .map_err(|_| Error::InternalError("Monitor semaphore closed".into()))?;
        permit.forget();

        {
            let mut owner = self.owner.lock();
            *owner = Some(thread_id);
            let mut count = self.entry_count.lock();
            *count = saved_count;
        }

        Ok(())
    }

    /// Wait for notification with a timeout.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub async fn wait_timeout(
        &self,
        thread_id: u64,
        duration: std::time::Duration,
    ) -> Result<bool> {
        let saved_count = {
            let owner = self.owner.lock();
            if *owner != Some(thread_id) {
                return Err(crate::JavaError::IllegalMonitorStateException(
                    "Current thread does not own the monitor".into(),
                )
                .into());
            }
            *self.entry_count.lock()
        };

        {
            let mut owner = self.owner.lock();
            *owner = None;
            let mut count = self.entry_count.lock();
            *count = 0;
            self.lock.add_permits(1);
        }

        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        let result = tokio::time::timeout(duration, notified).await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);

        let permit = self
            .lock
            .acquire()
            .await
            .map_err(|_| Error::InternalError("Monitor semaphore closed".into()))?;
        permit.forget();

        {
            let mut owner = self.owner.lock();
            *owner = Some(thread_id);
            let mut count = self.entry_count.lock();
            *count = saved_count;
        }

        match result {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Notify one waiting thread.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub fn notify(&self, thread_id: u64) -> Result<()> {
        let owner = self.owner.lock();
        if *owner != Some(thread_id) {
            return Err(crate::JavaError::IllegalMonitorStateException(
                "Current thread does not own the monitor".into(),
            )
            .into());
        }
        self.notify.notify_one();
        Ok(())
    }

    /// Notify all waiting threads.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub fn notify_all(&self, thread_id: u64) -> Result<()> {
        let owner = self.owner.lock();
        if *owner != Some(thread_id) {
            return Err(crate::JavaError::IllegalMonitorStateException(
                "Current thread does not own the monitor".into(),
            )
            .into());
        }
        self.notify.notify_waiters();
        Ok(())
    }
}

/// Registry to map Object identities to Monitors.
#[derive(Debug, Default)]
pub struct MonitorRegistry {
    monitors: Mutex<HashMap<usize, Arc<Monitor>>>,
}

impl MonitorRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get or create a monitor for the given object identifier.
    pub fn monitor(&self, object_id: usize) -> Arc<Monitor> {
        let mut monitors = self.monitors.lock();
        monitors
            .entry(object_id)
            .or_insert_with(|| Arc::new(Monitor::new()))
            .clone()
    }

    /// Cleanup a monitor.
    pub fn remove(&self, object_id: usize) {
        let mut monitors = self.monitors.lock();
        monitors.remove(&object_id);
    }
}
