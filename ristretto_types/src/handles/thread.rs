use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::task::JoinHandle;

/// Represents a handle to a thread.
#[derive(Debug)]
pub struct ThreadHandle<T: Send + Sync> {
    pub thread: Arc<T>,
    /// Whether this thread is a daemon thread. Daemon threads do not prevent the VM from exiting.
    pub daemon: bool,
    /// The join handle for the thread, if it exists.
    #[cfg(not(target_family = "wasm"))]
    pub join_handle: Option<JoinHandle<()>>,
}

impl<T: Send + Sync> From<Arc<T>> for ThreadHandle<T> {
    fn from(thread: Arc<T>) -> Self {
        ThreadHandle {
            thread,
            daemon: false,
            #[cfg(not(target_family = "wasm"))]
            join_handle: None,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl<T: Send + Sync> From<(Arc<T>, JoinHandle<()>, bool)> for ThreadHandle<T> {
    fn from((thread, join_handle, daemon): (Arc<T>, JoinHandle<()>, bool)) -> Self {
        ThreadHandle {
            thread,
            daemon,
            join_handle: Some(join_handle),
        }
    }
}

impl<T: Send + Sync> TryInto<Arc<T>> for ThreadHandle<T> {
    type Error = crate::Error;

    fn try_into(self) -> Result<Arc<T>, Self::Error> {
        let ThreadHandle { thread, .. } = self;
        Ok(thread)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;
    use std::fmt::Debug;

    #[derive(Debug)]
    struct MockThread {
        id: u64,
    }

    #[test]
    fn test_thread_handle_from_arc() {
        let thread = Arc::new(MockThread { id: 1 });
        let handle: ThreadHandle<MockThread> = thread.clone().into();
        assert_eq!(handle.thread.id, 1);
        assert!(!handle.daemon);
        assert!(handle.join_handle.is_none());
    }

    #[tokio::test]
    async fn test_thread_handle_from_tuple() {
        let thread = Arc::new(MockThread { id: 2 });
        let join_handle = tokio::spawn(async {});
        let handle: ThreadHandle<MockThread> = (thread, join_handle, true).into();
        assert_eq!(handle.thread.id, 2);
        assert!(handle.daemon);
        assert!(handle.join_handle.is_some());
    }

    #[tokio::test]
    async fn test_thread_handle_from_tuple_non_daemon() {
        let thread = Arc::new(MockThread { id: 3 });
        let join_handle = tokio::spawn(async {});
        let handle: ThreadHandle<MockThread> = (thread, join_handle, false).into();
        assert!(!handle.daemon);
        assert!(handle.join_handle.is_some());
    }

    #[test]
    fn test_thread_handle_try_into() -> Result<()> {
        let thread = Arc::new(MockThread { id: 42 });
        let handle: ThreadHandle<MockThread> = thread.into();
        let extracted: Arc<MockThread> = handle.try_into()?;
        assert_eq!(extracted.id, 42);
        Ok(())
    }

    #[test]
    fn test_thread_handle_debug() {
        let thread = Arc::new(MockThread { id: 1 });
        let handle: ThreadHandle<MockThread> = thread.into();
        let debug = format!("{handle:?}");
        assert!(debug.contains("ThreadHandle"));
    }
}
