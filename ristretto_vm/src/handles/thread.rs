use crate::thread::Thread;
use std::sync::Arc;
#[cfg(not(target_family = "wasm"))]
use tokio::task::JoinHandle;

/// Represents a handle to a thread.
#[derive(Debug)]
pub(crate) struct ThreadHandle {
    pub(crate) thread: Arc<Thread>,
    /// Whether this thread is a daemon thread. Daemon threads do not prevent the VM from exiting.
    pub(crate) daemon: bool,
    /// The join handle for the thread, if it exists.  The primordial thread does not have a join
    /// handle; the primordial thread is the one that started the VM and is not expected to be
    /// joined.
    #[cfg(not(target_family = "wasm"))]
    pub(crate) join_handle: Option<JoinHandle<()>>,
}

impl From<Arc<Thread>> for ThreadHandle {
    fn from(thread: Arc<Thread>) -> Self {
        ThreadHandle {
            thread,
            daemon: false,
            #[cfg(not(target_family = "wasm"))]
            join_handle: None,
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl From<(Arc<Thread>, JoinHandle<()>, bool)> for ThreadHandle {
    fn from((thread, join_handle, daemon): (Arc<Thread>, JoinHandle<()>, bool)) -> Self {
        ThreadHandle {
            thread,
            daemon,
            join_handle: Some(join_handle),
        }
    }
}

impl TryInto<Arc<Thread>> for ThreadHandle {
    type Error = crate::Error;

    fn try_into(self) -> Result<Arc<Thread>, Self::Error> {
        let ThreadHandle { thread, .. } = self;
        Ok(thread)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Result;

    #[tokio::test]
    async fn test_thread_handle_from_thread() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let thread_handle: ThreadHandle = thread.clone().into();
        assert_eq!(thread_handle.thread.id(), thread.id());
    }

    #[tokio::test]
    async fn test_thread_handle_try_into_thread() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let expected_thread_id = thread.id();
        let thread_handle: ThreadHandle = thread.into();
        let extracted_thread: Arc<Thread> = thread_handle.try_into()?;
        assert_eq!(expected_thread_id, extracted_thread.id());
        Ok(())
    }
}
