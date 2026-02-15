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
        let saved_count = self.release_for_wait(thread_id)?;
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        notified.await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);
        self.reacquire_after_wait(thread_id, saved_count).await
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
        let saved_count = self.release_for_wait(thread_id)?;
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        let result = tokio::time::timeout(duration, notified).await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);
        self.reacquire_after_wait(thread_id, saved_count).await?;

        match result {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Wait for notification, checking interrupt flag periodically.
    /// Returns true if interrupted, false if notified.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub async fn wait_interruptibly<F>(&self, thread_id: u64, is_interrupted: F) -> Result<bool>
    where
        F: Fn() -> bool,
    {
        let saved_count = self.release_for_wait(thread_id)?;
        self.wait_count.fetch_add(1, Ordering::SeqCst);

        let mut interrupted = false;
        loop {
            if is_interrupted() {
                interrupted = true;
                break;
            }
            let notified = self.notify.notified();
            if let Ok(()) =
                tokio::time::timeout(std::time::Duration::from_millis(10), notified).await
            {
                break; // Notified
            }
        }

        self.wait_count.fetch_sub(1, Ordering::SeqCst);
        self.reacquire_after_wait(thread_id, saved_count).await?;
        Ok(interrupted)
    }

    /// Wait for notification with timeout, checking interrupt flag periodically.
    /// Returns true if interrupted, false if notified or timed out.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub async fn wait_timeout_interruptibly<F>(
        &self,
        thread_id: u64,
        duration: std::time::Duration,
        is_interrupted: F,
    ) -> Result<bool>
    where
        F: Fn() -> bool,
    {
        let saved_count = self.release_for_wait(thread_id)?;
        self.wait_count.fetch_add(1, Ordering::SeqCst);

        let deadline = tokio::time::Instant::now() + duration;
        let mut interrupted = false;
        loop {
            if is_interrupted() {
                interrupted = true;
                break;
            }
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                break; // Timed out
            }
            let poll_duration = remaining.min(std::time::Duration::from_millis(10));
            let notified = self.notify.notified();
            if let Ok(()) = tokio::time::timeout(poll_duration, notified).await {
                break; // Notified
            }
        }

        self.wait_count.fetch_sub(1, Ordering::SeqCst);
        self.reacquire_after_wait(thread_id, saved_count).await?;
        Ok(interrupted)
    }

    /// Release the monitor for wait, returning the saved entry count.
    fn release_for_wait(&self, thread_id: u64) -> Result<usize> {
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

        Ok(saved_count)
    }

    /// Re-acquire the monitor after wait, restoring the saved entry count.
    async fn reacquire_after_wait(&self, thread_id: u64, saved_count: usize) -> Result<()> {
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
