use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::JAVA_11;
use ristretto_macros::intrinsic_method;

#[intrinsic_method("pkg/Mixed.sync()I", Any)]
pub fn sync<T>() -> u8 {
    let _ = std::marker::PhantomData::<T>;
    7
}

#[intrinsic_method("pkg/Mixed.async()I", GreaterThanOrEqual(JAVA_11))]
pub async fn r#async<T>() -> u8 {
    let _ = std::marker::PhantomData::<T>;
    let mut pending = true;
    std::future::poll_fn(move |context| {
        if pending {
            pending = false;
            context.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(11)
        }
    })
    .await
}
