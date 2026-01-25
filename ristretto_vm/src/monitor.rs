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
    /// Count of waiting threads to optimize notify (optional, but good for debug/stats)
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
        // Fast path: Reentrancy
        {
            let owner = self.owner.lock();
            if *owner == Some(thread_id) {
                let mut count = self.entry_count.lock();
                *count += 1;
                return Ok(());
            }
        }

        // Slow path: Acquire semaphore
        // We acquire a permit and "forget" it to manually manage the lock lifetime
        let permit = self
            .lock
            .acquire()
            .await
            .map_err(|_| Error::InternalError("Monitor semaphore closed".into()))?;
        permit.forget(); // We will add the permit back manually in release()

        // We own the lock now
        {
            let mut owner = self.owner.lock();
            *owner = Some(thread_id);
            let mut count = self.entry_count.lock();
            *count = 1;
        }

        Ok(())
    }

    /// Release the monitor lock.
    /// Decrements entry count. If 0, releases the actual lock (semaphore).
    /// Returns true if the lock was fully released, false if it's still held (nested exit).
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
            // Add permit back to semaphore
            self.lock.add_permits(1);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Wait for notification.
    /// Releases the lock, waits for notify, and re-acquires the lock.
    ///
    /// # Errors
    /// if the current thread does not own the monitor.
    pub async fn wait(&self, thread_id: u64) -> Result<()> {
        // 1. Verify ownership and get current entry count
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

        // 2. Fully release the lock
        {
            let mut owner = self.owner.lock();
            *owner = None;
            let mut count = self.entry_count.lock();
            *count = 0;
            self.lock.add_permits(1);
        }

        // 3. Wait for notification
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        notified.await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);

        // 4. Re-acquire the lock
        // We must re-acquire fully, respecting the semaphore
        let permit = self
            .lock
            .acquire()
            .await
            .map_err(|_| Error::InternalError("Monitor semaphore closed".into()))?;
        permit.forget();

        // Restore state
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
        // 1. Verify ownership and get current entry count
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

        // 2. Fully release the lock
        {
            let mut owner = self.owner.lock();
            *owner = None;
            let mut count = self.entry_count.lock();
            *count = 0;
            self.lock.add_permits(1);
        }

        // 3. Wait for notification or timeout
        // We use tokio::time::timeout
        self.wait_count.fetch_add(1, Ordering::SeqCst);
        let notified = self.notify.notified();
        let result = tokio::time::timeout(duration, notified).await;
        self.wait_count.fetch_sub(1, Ordering::SeqCst);

        // 4. Re-acquire the lock
        // We must re-acquire fully, respecting the semaphore
        // Whether we timed out or not, we MUST re-acquire the lock before returning.
        let permit = self
            .lock
            .acquire()
            .await
            .map_err(|_| Error::InternalError("Monitor semaphore closed".into()))?;
        permit.forget();

        // Restore state
        {
            let mut owner = self.owner.lock();
            *owner = Some(thread_id);
            let mut count = self.entry_count.lock();
            *count = saved_count;
        }

        match result {
            Ok(()) => Ok(true),  // Notified
            Err(_) => Ok(false), // Timed out
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
    // We use a parking_lot Mutex for the registry itself to avoid async in simple map lookups,
    // as fetching the Monitor Arc is fast.
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

    /// Cleanup a monitor (if we had GC hooks, we'd call this).
    pub fn remove(&self, object_id: usize) {
        let mut monitors = self.monitors.lock();
        monitors.remove(&object_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_monitor_reentrancy() -> Result<()> {
        let monitor = Monitor::new();
        let thread_id = 1;

        monitor.acquire(thread_id).await?;
        // Re-enter
        monitor.acquire(thread_id).await?;

        assert_eq!(*monitor.entry_count.lock(), 2);
        assert_eq!(*monitor.owner.lock(), Some(thread_id));

        // First exit, still owned
        assert!(!monitor.release(thread_id)?);
        assert_eq!(*monitor.entry_count.lock(), 1);
        assert_eq!(*monitor.owner.lock(), Some(thread_id));

        // Second exit, released
        assert!(monitor.release(thread_id)?);
        assert_eq!(*monitor.entry_count.lock(), 0);
        assert_eq!(*monitor.owner.lock(), None);

        Ok(())
    }

    #[tokio::test]
    async fn test_monitor_mutual_exclusion() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        let monitor1 = monitor.clone();
        let monitor2 = monitor.clone();

        let thread1 = 1;
        let thread2 = 2;

        monitor1.acquire(thread1).await?;

        // Spawn a task that tries to acquire
        let handle = tokio::spawn(async move {
            monitor2.acquire(thread2).await.unwrap();
            assert_eq!(*monitor2.owner.lock(), Some(thread2));
            monitor2.release(thread2).unwrap();
        });

        // Sleep a bit to ensure the other task is blocked
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Release from thread 1
        monitor1.release(thread1)?;

        // Now the other task should complete
        timeout(Duration::from_secs(1), handle)
            .await
            .expect("timeout")
            .expect("join failed");

        Ok(())
    }

    #[tokio::test]
    async fn test_wait_notify() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        let monitor_wait = monitor.clone();
        let monitor_notify = monitor.clone();

        let thread_wait = 1;
        let thread_notify = 2;

        // Task 1: Acquire and wait
        let wait_handle = tokio::spawn(async move {
            monitor_wait.acquire(thread_wait).await.unwrap();
            monitor_wait.wait(thread_wait).await.unwrap();
            // Should verify we own it again
            assert_eq!(*monitor_wait.owner.lock(), Some(thread_wait));
            monitor_wait.release(thread_wait).unwrap();
        });

        // Wait for the waiter to be waiting (check wait_count)
        // We might need to yield loop a bit because there's no direct callback
        let start = std::time::Instant::now();
        while monitor.wait_count.load(Ordering::SeqCst) == 0 {
            assert!(
                start.elapsed() <= Duration::from_secs(1),
                "Timed out waiting for thread to wait"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        // Task 2: Acquire and notify
        monitor_notify.acquire(thread_notify).await?;
        monitor_notify.notify(thread_notify)?;
        monitor_notify.release(thread_notify)?;

        // Task 1 should complete
        timeout(Duration::from_secs(1), wait_handle)
            .await
            .expect("timeout")
            .expect("join failed");

        Ok(())
    }

    #[tokio::test]
    async fn test_wait_timeout() -> Result<()> {
        let monitor = Monitor::new();
        let thread_id = 1;

        monitor.acquire(thread_id).await?;

        // Wait with a short timeout, nobody notifies
        let start = std::time::Instant::now();
        let notified = monitor
            .wait_timeout(thread_id, Duration::from_millis(50))
            .await?;
        let elapsed = start.elapsed();

        assert!(!notified); // Timed out
        assert!(elapsed >= Duration::from_millis(50));
        assert_eq!(*monitor.owner.lock(), Some(thread_id));

        monitor.release(thread_id)?;
        Ok(())
    }

    #[tokio::test]
    async fn test_wait_timeout_success() -> Result<()> {
        let monitor = Arc::new(Monitor::new());
        let monitor_wait = monitor.clone();
        let monitor_notify = monitor.clone();
        let thread_wait = 1;
        let thread_notify = 2;

        let wait_handle = tokio::spawn(async move {
            monitor_wait.acquire(thread_wait).await.unwrap();
            let notified = monitor_wait
                .wait_timeout(thread_wait, Duration::from_secs(10))
                .await
                .unwrap();
            monitor_wait.release(thread_wait).unwrap();
            notified
        });

        // Wait for waiter
        let start = std::time::Instant::now();
        while monitor.wait_count.load(Ordering::SeqCst) == 0 {
            assert!(
                start.elapsed() <= Duration::from_secs(1),
                "Timed out waiting for thread to wait"
            );
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        monitor_notify.acquire(thread_notify).await?;
        monitor_notify.notify(thread_notify)?;
        monitor_notify.release(thread_notify)?;

        let notified = timeout(Duration::from_secs(1), wait_handle)
            .await
            .expect("timeout")
            .expect("join failed");
        assert!(notified);

        Ok(())
    }

    #[test]
    fn test_monitor_registry() {
        let registry = MonitorRegistry::new();
        let obj_id = 12345;

        let m1 = registry.monitor(obj_id);
        let m2 = registry.monitor(obj_id);

        assert!(Arc::ptr_eq(&m1, &m2));

        let m3 = registry.monitor(9999);
        assert!(!Arc::ptr_eq(&m1, &m3));

        registry.remove(obj_id);
        let m4 = registry.monitor(obj_id);
        // Should be a new one
        assert!(!Arc::ptr_eq(&m1, &m4));
    }

    #[tokio::test]
    async fn test_illegal_monitor_state() -> Result<()> {
        let monitor = Monitor::new();
        let thread_1 = 1;
        let thread_2 = 2;

        // Acquire with thread 1
        monitor.acquire(thread_1).await?;

        // Try to release with thread 2
        let result = monitor.release(thread_2);
        // Should be IllegalMonitorStateException
        assert!(matches!(
            result,
            Err(Error::JavaError(
                crate::JavaError::IllegalMonitorStateException(_)
            ))
        ));

        // Try to wait with thread 2
        let result = monitor.wait(thread_2).await;
        // Should be IllegalMonitorStateException
        assert!(matches!(
            result,
            Err(Error::JavaError(
                crate::JavaError::IllegalMonitorStateException(_)
            ))
        ));

        // Try to notify with thread 2
        let result = monitor.notify(thread_2);
        assert!(matches!(
            result,
            Err(Error::JavaError(
                crate::JavaError::IllegalMonitorStateException(_)
            ))
        ));

        // Cleanup
        monitor.release(thread_1)?;
        Ok(())
    }
}
