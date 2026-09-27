use ristretto_classfile::VersionSpecification::Any;
use ristretto_macros::{async_method, intrinsic_method};

#[intrinsic_method("pkg/Example.intrinsic()V", Any)]
pub fn intrinsic_fixture() {}

#[async_method]
pub async fn async_fixture(value: u8) -> u8 {
    value
}

#[test]
fn intrinsic_method_macro_preserves_function() {
    intrinsic_fixture();
}

#[test]
fn async_method_macro_returns_future() {
    drop(async_fixture(7));
}

#[intrinsic_method("pkg/Example.recursive(I)I", Any)]
#[async_method]
pub async fn recursive_fixture(depth: u8) -> u8 {
    if depth == 0 {
        0
    } else {
        recursive_fixture(depth - 1).await + 1
    }
}

#[test]
fn explicit_recursive_boxing_still_works() {
    use std::task::{Context, Poll, Waker};
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(
        recursive_fixture(4).as_mut().poll(&mut context),
        Poll::Ready(4)
    );
}

#[intrinsic_method("pkg/Example.suspends()I", Any)]
async fn suspends() -> u8 {
    let mut pending = true;
    std::future::poll_fn(move |cx| {
        if pending {
            pending = false;
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(7)
        }
    })
    .await
}

#[test]
fn ordinary_async_body_can_suspend() {
    use std::task::{Context, Poll, Waker};
    let mut future = std::pin::pin!(suspends());
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(7));
}
