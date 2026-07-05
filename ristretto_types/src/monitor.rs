use crate::{Error, Result};
use ristretto_gc::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;
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
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        // Register for notification before releasing the monitor to prevent lost notifications
        let notified = self.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        let saved_count = self.release_for_wait(thread_id)?;
        notified.await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);
        self.reacquire_after_wait(thread_id, saved_count).await
    }

    /// Wait for notification with a timeout.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub async fn wait_timeout(&self, thread_id: u64, duration: Duration) -> Result<bool> {
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        let saved_count = self.release_for_wait(thread_id)?;
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
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        // Register for notification before releasing the monitor to prevent lost notifications
        let notified = self.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        let saved_count = self.release_for_wait(thread_id)?;

        let mut interrupted = false;
        loop {
            if is_interrupted() {
                interrupted = true;
                break;
            }
            if tokio::time::timeout(Duration::from_millis(10), notified.as_mut())
                .await
                .is_ok()
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
        duration: Duration,
        is_interrupted: F,
    ) -> Result<bool>
    where
        F: Fn() -> bool,
    {
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        let saved_count = self.release_for_wait(thread_id)?;

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
            let poll_duration = remaining.min(Duration::from_millis(10));
            if let Ok(()) = tokio::time::timeout(poll_duration, notified.as_mut()).await {
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

    /// Check if the monitor is owned by the given thread.
    #[must_use]
    pub fn is_owned_by(&self, thread_id: u64) -> bool {
        let owner = self.owner.lock();
        *owner == Some(thread_id)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JavaError;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    static WAIT_INTERRUPT_ON_FIRST: AtomicBool = AtomicBool::new(false);
    static WAIT_INTERRUPT_AFTER_POLL: AtomicBool = AtomicBool::new(false);
    static WAIT_TIMEOUT_INTERRUPT_ON_FIRST: AtomicBool = AtomicBool::new(false);

    fn assert_illegal_monitor_state(result: &Result<impl Sized>) {
        assert!(matches!(
            result,
            Err(Error::JavaError(JavaError::IllegalMonitorStateException(_)))
        ));
    }

    fn never_interrupted() -> bool {
        false
    }

    fn wait_interrupt_on_first_check() -> bool {
        !WAIT_INTERRUPT_ON_FIRST.swap(true, Ordering::SeqCst)
    }

    fn wait_interrupt_after_poll() -> bool {
        WAIT_INTERRUPT_AFTER_POLL.swap(true, Ordering::SeqCst)
    }

    fn wait_timeout_interrupt_on_first_check() -> bool {
        !WAIT_TIMEOUT_INTERRUPT_ON_FIRST.swap(true, Ordering::SeqCst)
    }

    #[tokio::test]
    async fn test_acquire_release_reentrant_and_errors() -> Result<()> {
        let monitor = Monitor::default();
        monitor.acquire(1).await?;
        assert!(monitor.is_owned_by(1));
        monitor.acquire(1).await?;
        assert!(!monitor.release(1)?);
        assert!(monitor.is_owned_by(1));
        assert!(monitor.release(1)?);
        assert!(!monitor.is_owned_by(1));
        assert_illegal_monitor_state(&monitor.release(1));
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_notify_and_notify_errors() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        assert_illegal_monitor_state(&monitor.notify(1));
        assert_illegal_monitor_state(&monitor.notify_all(1));
        monitor.acquire(1).await?;

        let waiter = monitor.clone();
        let wait_task = tokio::spawn(async move { waiter.wait(1).await });
        tokio::time::sleep(Duration::from_millis(10)).await;
        monitor.acquire(2).await?;
        assert!(monitor.is_owned_by(2));
        assert_illegal_monitor_state(&monitor.notify(1));
        monitor.notify(2)?;
        monitor.release(2)?;
        wait_task.await.expect("wait task")?;
        assert!(monitor.is_owned_by(1));
        monitor.release(1)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_timeout_notified_and_timed_out() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        monitor.acquire(1).await?;
        let waiter = monitor.clone();
        let wait_task =
            tokio::spawn(async move { waiter.wait_timeout(1, Duration::from_secs(5)).await });
        tokio::time::sleep(Duration::from_millis(10)).await;
        monitor.acquire(2).await?;
        monitor.notify_all(2)?;
        monitor.release(2)?;
        assert!(wait_task.await.expect("wait task")?);
        monitor.release(1)?;

        monitor.acquire(3).await?;
        assert!(!monitor.wait_timeout(3, Duration::from_millis(1)).await?);
        monitor.release(3)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_interruptibly_notified_and_interrupted() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        monitor.acquire(1).await?;
        let waiter = monitor.clone();
        let wait_task = tokio::spawn(async move {
            waiter
                .wait_interruptibly(1, never_interrupted as fn() -> bool)
                .await
        });
        tokio::time::sleep(Duration::from_millis(10)).await;
        monitor.acquire(2).await?;
        monitor.notify(2)?;
        monitor.release(2)?;
        assert!(!wait_task.await.expect("wait task")?);
        monitor.release(1)?;

        WAIT_INTERRUPT_ON_FIRST.store(false, Ordering::SeqCst);
        monitor.acquire(3).await?;
        assert!(
            monitor
                .wait_interruptibly(3, wait_interrupt_on_first_check as fn() -> bool)
                .await?
        );
        monitor.release(3)?;

        WAIT_INTERRUPT_AFTER_POLL.store(false, Ordering::SeqCst);
        monitor.acquire(4).await?;
        assert!(
            monitor
                .wait_interruptibly(4, wait_interrupt_after_poll as fn() -> bool)
                .await?
        );
        monitor.release(4)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_timeout_interruptibly_paths() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        monitor.acquire(1).await?;
        assert!(
            !monitor
                .wait_timeout_interruptibly(
                    1,
                    Duration::from_millis(1),
                    never_interrupted as fn() -> bool
                )
                .await?
        );
        monitor.release(1)?;

        WAIT_TIMEOUT_INTERRUPT_ON_FIRST.store(false, Ordering::SeqCst);
        monitor.acquire(2).await?;
        assert!(
            monitor
                .wait_timeout_interruptibly(
                    2,
                    Duration::from_secs(1),
                    wait_timeout_interrupt_on_first_check as fn() -> bool,
                )
                .await?
        );
        monitor.release(2)?;

        monitor.acquire(3).await?;
        let waiter = monitor.clone();
        let wait_task = tokio::spawn(async move {
            waiter
                .wait_timeout_interruptibly(
                    3,
                    Duration::from_secs(5),
                    never_interrupted as fn() -> bool,
                )
                .await
        });
        tokio::time::sleep(Duration::from_millis(10)).await;
        monitor.acquire(4).await?;
        monitor.notify(4)?;
        monitor.release(4)?;
        assert!(!wait_task.await.expect("wait task")?);
        monitor.release(3)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_requires_owner() {
        let monitor = Monitor::new();
        let result = monitor.wait(1).await;
        assert_illegal_monitor_state(&result);
        let result = monitor.wait_timeout(1, Duration::from_millis(1)).await;
        assert_illegal_monitor_state(&result);
        let result = monitor
            .wait_interruptibly(1, never_interrupted as fn() -> bool)
            .await;
        assert_illegal_monitor_state(&result);
        let result = monitor
            .wait_timeout_interruptibly(
                1,
                Duration::from_millis(1),
                never_interrupted as fn() -> bool,
            )
            .await;
        assert_illegal_monitor_state(&result);
    }

    #[tokio::test]
    async fn test_closed_semaphore_errors() -> Result<()> {
        let monitor = Monitor::new();
        monitor.lock.close();
        assert!(matches!(
            monitor.acquire(1).await,
            Err(Error::InternalError(_))
        ));

        let monitor = Monitor::new();
        monitor.acquire(1).await?;
        let saved_count = monitor.release_for_wait(1)?;
        monitor.lock.close();
        assert!(matches!(
            monitor.reacquire_after_wait(1, saved_count).await,
            Err(Error::InternalError(_))
        ));
        Ok(())
    }

    #[test]
    fn test_monitor_registry_new_monitor_and_remove() {
        let registry = MonitorRegistry::new();
        let first = registry.monitor(123);
        let second = registry.monitor(123);
        assert!(Arc::ptr_eq(&first, &second));
        registry.remove(123);
        let third = registry.monitor(123);
        assert!(!Arc::ptr_eq(&first, &third));
    }
}
