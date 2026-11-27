//! Drive a component request up to a guest yield or an idle Tokio scheduler.
use std::cell::Cell;
use std::future::{Future, poll_fn};
use std::pin::Pin;
use std::task::{Poll, Waker};
use std::time::{Duration, Instant};
use tokio::runtime::{Builder, Runtime};

thread_local! {
    static DRIVER: Cell<Option<Waker>> = const { Cell::new(None) };
    static IDLE: Cell<bool> = const { Cell::new(false) };
}

pub(super) fn runtime() -> std::io::Result<Runtime> {
    Builder::new_current_thread()
        .enable_all()
        .on_thread_park(|| {
            if let Some(waker) = DRIVER.take() {
                // Tokio checks its ready queue and polls timers without blocking
                // after this callback wakes the block_on future.
                IDLE.set(true);
                waker.wake();
            }
        })
        .build()
}

pub(super) fn drive<F: Future + ?Sized>(
    runtime: &Runtime,
    mut request: Pin<&mut F>,
) -> Poll<F::Output> {
    struct Turn;
    impl Drop for Turn {
        fn drop(&mut self) {
            DRIVER.take();
            IDLE.set(false);
        }
    }
    let _turn = Turn;
    let start = Instant::now();
    let mut polled = false;
    runtime.block_on(poll_fn(|cx| {
        if polled && (IDLE.replace(false) || start.elapsed() >= Duration::from_millis(4)) {
            return Poll::Ready(Poll::Pending);
        }
        if !polled {
            DRIVER.set(Some(cx.waker().clone()));
            polled = true;
        }
        match request.as_mut().poll(cx) {
            Poll::Ready(value) => Poll::Ready(Poll::Ready(value)),
            Poll::Pending => Poll::Pending,
        }
    }))
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;

    #[test]
    fn idle_turn_returns_while_the_same_request_still_awaits_a_sender() {
        let runtime = runtime().unwrap();
        let (send, receive) = tokio::sync::oneshot::channel();
        let mut request = std::pin::pin!(async { receive.await.unwrap() });
        assert!(drive(&runtime, request.as_mut()).is_pending());
        send.send(42).unwrap();
        assert_eq!(drive(&runtime, request.as_mut()), Poll::Ready(42));
    }

    #[test]
    fn timers_and_spawned_tasks_progress_across_repeated_host_calls() {
        let runtime = runtime().unwrap();
        let mut request = std::pin::pin!(async {
            tokio::spawn(async {
                tokio::time::sleep(Duration::from_millis(15)).await;
                41
            })
            .await
            .unwrap()
                + 1
        });
        assert!(drive(&runtime, request.as_mut()).is_pending());
        let until = Instant::now() + Duration::from_secs(2);
        loop {
            if let Poll::Ready(result) = drive(&runtime, request.as_mut()) {
                assert_eq!(result, 42);
                break;
            }
            assert!(Instant::now() < until);
            std::thread::sleep(Duration::from_millis(1));
        }
    }
}
