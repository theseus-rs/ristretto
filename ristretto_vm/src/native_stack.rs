//! Bound Java transitions nested under synchronous compiled execution.
//!
//! The JIT checks the generated frame's backend size before native entry. This
//! module reserves room for outer entry and exception construction. Arbitrary
//! native helpers and interpreter-only native reentry need separate qualification.
use crate::Result;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/// A suspended call owns a boxed future, never a native stack segment. Polling
/// and recursive cancellation each obtain temporary stack room when needed.
/// Java frame/slot limits remain the bound on logical recursion.
pub(crate) struct CallFuture<F> {
    future: Option<Pin<Box<F>>>,
}

impl<F> CallFuture<F> {
    pub(crate) fn new(future: F) -> Self {
        Self {
            future: Some(Box::pin(future)),
        }
    }
}

impl<F: Future<Output = Result<T>>, T> Future for CallFuture<F> {
    type Output = Result<T>;
    #[expect(
        clippy::expect_used,
        reason = "the private future is taken only during Drop"
    )]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let future = self
            .get_mut()
            .future
            .as_mut()
            .expect("call future remains owned until drop");
        #[cfg(not(target_family = "wasm"))]
        {
            if COMPILED_DEPTH.get() != 0 {
                if let Err(error) = check_entry() {
                    return Poll::Ready(Err(error));
                }
                return future.as_mut().poll(cx);
            }
            stacker::maybe_grow(EXCEPTION_RED_ZONE, EXCEPTION_STACK_BYTES, || {
                future.as_mut().poll(cx)
            })
        }
        #[cfg(target_family = "wasm")]
        future.as_mut().poll(cx)
    }
}

impl<F> Drop for CallFuture<F> {
    fn drop(&mut self) {
        let future = self.future.take();
        #[cfg(not(target_family = "wasm"))]
        stacker::maybe_grow(EXCEPTION_RED_ZONE, EXCEPTION_STACK_BYTES, || drop(future));
        #[cfg(target_family = "wasm")]
        drop(future);
    }
}

#[cfg(not(target_family = "wasm"))]
thread_local! {
    // Synchronous compiled execution cannot migrate while the native frame is active.
    static COMPILED_DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

#[cfg(not(target_family = "wasm"))]
struct CompiledEntry(usize);

#[cfg(not(target_family = "wasm"))]
impl Drop for CompiledEntry {
    fn drop(&mut self) {
        COMPILED_DEPTH.set(self.0);
    }
}

#[cfg(not(target_family = "wasm"))]
const ENTRY_RED_ZONE: usize = ristretto_jit::Function::STACK_RESERVE_BYTES;
#[cfg(not(target_family = "wasm"))]
const EXCEPTION_RED_ZONE: usize = 2 * ENTRY_RED_ZONE;
#[cfg(not(target_family = "wasm"))]
const EXCEPTION_STACK_BYTES: usize = 1024 * 1024;

/// Check before resolving/entering another method or acquiring its monitor.
#[inline]
#[cfg(not(target_family = "wasm"))]
pub(crate) fn check_entry() -> Result<()> {
    if COMPILED_DEPTH.get() != 0
        && stacker::remaining_stack().is_some_and(|bytes| bytes < ENTRY_RED_ZONE)
    {
        return Err(
            crate::JavaError::StackOverflowError("Native stack exhausted".to_owned()).into(),
        );
    }
    Ok(())
}

/// Give the outermost compiled entry room even when entered from deep bootstrap
/// code. Nested entries consume this bounded segment and reject low capacity.
pub(crate) fn with_compiled_stack<T>(
    frame_bytes: usize,
    execute: impl FnOnce() -> Result<T>,
) -> Result<T> {
    #[cfg(not(target_family = "wasm"))]
    {
        check_entry()?;
        let previous = COMPILED_DEPTH.get();
        let run = || {
            COMPILED_DEPTH.set(previous + 1);
            let _entry = CompiledEntry(previous);
            execute()
        };
        if previous == 0 {
            let reserve = frame_bytes.checked_add(EXCEPTION_RED_ZONE).ok_or_else(|| {
                crate::Error::InternalError("native stack requirement overflow".to_owned())
            })?;
            stacker::maybe_grow(reserve, EXCEPTION_STACK_BYTES.max(reserve), run)
        } else {
            run()
        }
    }
    #[cfg(target_family = "wasm")]
    {
        let _ = frame_bytes;
        execute()
    }
}

/// Exception constructors can re-enter Java. Poll them with reserved native
/// space, on the same OS thread (and therefore the same GC context). The segment
/// is released before returning `Pending`; the pinned future retains its state.
pub(crate) async fn with_exception_stack<F: Future>(future: F) -> F::Output {
    #[cfg(not(target_family = "wasm"))]
    {
        let mut future = std::pin::pin!(future);
        std::future::poll_fn(|cx| {
            stacker::maybe_grow(EXCEPTION_RED_ZONE, EXCEPTION_STACK_BYTES, || {
                future.as_mut().poll(cx)
            })
        })
        .await
    }
    #[cfg(target_family = "wasm")]
    future.await
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::task::{Context, Poll, Waker};

    #[test]
    fn suspended_call_chains_poll_migrate_and_drop_on_small_stacks() -> Result<()> {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        struct Held(Arc<AtomicUsize>);
        impl Drop for Held {
            fn drop(&mut self) {
                self.0.fetch_sub(1, Ordering::Relaxed);
            }
        }
        async fn chain(depth: usize, alive: Arc<AtomicUsize>) -> Result<()> {
            alive.fetch_add(1, Ordering::Relaxed);
            let _held = Held(alive.clone());
            if depth == 0 {
                std::future::pending().await
            } else {
                CallFuture::new(chain(depth - 1, alive)).await
            }
        }
        const CHILD: &str = "RISTRETTO_CALL_STACK_CHILD";
        if std::env::var_os(CHILD).is_none() {
            let mut command = std::process::Command::new(std::env::current_exe()?);
            command.args(["--exact", "native_stack::tests::suspended_call_chains_poll_migrate_and_drop_on_small_stacks", "--nocapture"]).env(CHILD, "1");
            ristretto_test_util::process::run(&mut command, std::time::Duration::from_secs(30))?
                .require_success()?;
            return Ok(());
        }
        let alive = Arc::new(AtomicUsize::new(0));
        let count = alive.clone();
        let future = std::thread::Builder::new()
            .stack_size(512 * 1024)
            .spawn(move || {
                let mut future = Box::pin(CallFuture::new(chain(2048, count)));
                assert!(
                    future
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
                future
            })?
            .join()
            .map_err(|_| crate::Error::InternalError("call chain poll panicked".to_owned()))?;
        assert_eq!(alive.load(Ordering::Relaxed), 2049);
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                let mut future = future;
                assert!(
                    future
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop()))
                        .is_pending()
                );
                drop(future);
            })?
            .join()
            .map_err(|_| {
                crate::Error::InternalError("call chain cancellation panicked".to_owned())
            })?;
        assert_eq!(alive.load(Ordering::Relaxed), 0);
        with_compiled_stack(0, || {
            stacker::grow(64 * 1024, || {
                let mut future = Box::pin(CallFuture::new(async { Ok(42) }));
                assert!(matches!(
                    future
                        .as_mut()
                        .poll(&mut Context::from_waker(Waker::noop())),
                    Poll::Ready(Err(crate::Error::JavaError(
                        crate::JavaError::StackOverflowError(_)
                    )))
                ));
            });
            Ok(())
        })
    }

    #[test]
    fn exception_stack_preserves_thread_and_pending_future() -> Result<()> {
        let result = with_compiled_stack(0, || {
            check_exception_stack();
            Ok(())
        });
        assert_eq!(COMPILED_DEPTH.get(), 0);
        result
    }

    #[test]
    fn first_compiled_entry_grows_and_nested_rejection_restores_depth() -> Result<()> {
        stacker::grow(64 * 1024, || {
            assert_eq!(COMPILED_DEPTH.get(), 0);
            let result: Result<()> = with_compiled_stack(0, || {
                assert_eq!(COMPILED_DEPTH.get(), 1);
                assert!(stacker::remaining_stack().is_some_and(|bytes| bytes > EXCEPTION_RED_ZONE));
                stacker::grow(64 * 1024, || {
                    // A nested entry must reject low capacity without growing another segment.
                    with_compiled_stack(0, || Ok(()))
                })
            });
            assert!(matches!(
                result,
                Err(crate::Error::JavaError(
                    crate::JavaError::StackOverflowError(_)
                ))
            ));
            assert_eq!(COMPILED_DEPTH.get(), 0);
            Ok(())
        })
    }

    #[test]
    fn first_entry_reserves_the_generated_frame_beyond_the_default_segment() -> Result<()> {
        let frame_bytes = 2 * 1024 * 1024;
        stacker::grow(64 * 1024, || {
            with_compiled_stack(frame_bytes, || {
                assert!(
                    stacker::remaining_stack()
                        .is_some_and(|bytes| bytes >= frame_bytes + ENTRY_RED_ZONE)
                );
                assert_eq!(COMPILED_DEPTH.get(), 1);
                Ok(())
            })?;
            assert_eq!(COMPILED_DEPTH.get(), 0);
            Ok(())
        })
    }

    fn check_exception_stack() {
        let thread = std::thread::current().id();
        let mut polls = 0;
        let future = std::future::poll_fn(|_| {
            assert_eq!(std::thread::current().id(), thread);
            assert!(stacker::remaining_stack().is_some_and(|bytes| bytes > EXCEPTION_RED_ZONE));
            polls += 1;
            if polls < 3 {
                Poll::Pending
            } else {
                Poll::Ready(polls)
            }
        });
        let mut future = std::pin::pin!(with_exception_stack(future));
        let mut context = Context::from_waker(Waker::noop());
        for expected in [Poll::Pending, Poll::Pending, Poll::Ready(3)] {
            stacker::grow(64 * 1024, || {
                assert!(check_entry().is_err());
                assert_eq!(future.as_mut().poll(&mut context), expected);
                // The temporary exception segment must be released after every poll.
                assert!(check_entry().is_err());
            });
            assert!(check_entry().is_ok());
        }
    }
}
