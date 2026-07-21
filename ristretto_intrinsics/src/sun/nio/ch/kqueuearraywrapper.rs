use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/ch/KQueueArrayWrapper.init()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let vm = thread.vm()?;
    Ok(Some(Value::Int(super::kqueue::create_queue(&*vm)?)))
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.initStructSizes()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_struct_sizes<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let class = thread.class("sun.nio.ch.KQueueArrayWrapper").await?;
    class.set_static_value("EVFILT_READ", Value::Int(-1))?;
    class.set_static_value("EVFILT_WRITE", Value::Int(-2))?;
    class.set_static_value("SIZEOF_KEVENT", Value::Int(32))?;
    class.set_static_value("FD_OFFSET", Value::Int(0))?;
    class.set_static_value("FILTER_OFFSET", Value::Int(8))?;
    Ok(None)
}

#[intrinsic_method("sun/nio/ch/KQueueArrayWrapper.interrupt(I)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn interrupt<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let fd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let fd = super::posix::raw_descriptor(&*vm, fd).await;
    let _ = super::posix::write_descriptor(fd, &[1])?;
    Ok(None)
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.kevent0(IJIJ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn kevent_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let timeout = parameters.pop_long()?;
    let event_count = parameters.pop_int()?;
    let address = parameters.pop_long()?;
    let kqfd = parameters.pop_int()?;
    Ok(Some(Value::Int(
        super::kqueue::wait(&thread, kqfd, address, event_count, timeout).await?,
    )))
}

#[intrinsic_method(
    "sun/nio/ch/KQueueArrayWrapper.register0(IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn register_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let flags = parameters.pop_int()?;
    let filter = parameters.pop_int()?;
    let fd = parameters.pop_int()?;
    let kqfd = parameters.pop_int()?;
    let vm = thread.vm()?;
    let errno = super::kqueue::change(&*vm, kqfd, fd, filter, flags).await?;
    if errno != 0 {
        return Err(super::posix::io_exception(
            "kevent",
            &std::io::Error::from_raw_os_error(errno),
        ));
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;
    use ristretto_types::VM;

    async fn socket_pair<T: Thread + 'static>(thread: Arc<T>) -> Result<(i32, i32)> {
        let vm = thread.vm()?;
        let pair = Value::new_object(vm.garbage_collector(), Reference::from(vec![0_i32, 0_i32]));
        let result =
            super::super::kqueueport::socketpair(thread, Parameters::new(vec![pair.clone()]))
                .await?;
        assert_eq!(None, result);
        let values = pair.as_int_vec_ref()?;
        let [first, second, ..] = values.as_ref() else {
            return Err(ristretto_types::Error::InternalError(
                "KQueueArrayWrapper test socket pair is incomplete".to_string(),
            ));
        };
        Ok((*first, *second))
    }

    async fn close<T: Thread + 'static>(thread: Arc<T>, fd: i32) -> Result<()> {
        let result =
            super::super::kqueueport::close_0(thread, Parameters::new(vec![Value::Int(fd)]))
                .await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let Some(Value::Int(fd)) = init(thread.clone(), Parameters::default()).await? else {
            return Err(ristretto_types::Error::InternalError(
                "KQueueArrayWrapper.init returned no descriptor".to_string(),
            ));
        };
        assert!(fd >= 0);
        close(thread, fd).await
    }

    #[tokio::test]
    async fn test_init_struct_sizes() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init_struct_sizes(thread.clone(), Parameters::default()).await?;
        assert_eq!(None, result);
        let class = thread.class("sun.nio.ch.KQueueArrayWrapper").await?;
        assert_eq!(-1, class.static_value("EVFILT_READ")?.as_i32()?);
        assert_eq!(-2, class.static_value("EVFILT_WRITE")?.as_i32()?);
        assert_eq!(32, class.static_value("SIZEOF_KEVENT")?.as_i32()?);
        assert_eq!(0, class.static_value("FD_OFFSET")?.as_i32()?);
        assert_eq!(8, class.static_value("FILTER_OFFSET")?.as_i32()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_interrupt() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let (writer, reader) = socket_pair(thread.clone()).await?;
        let result = interrupt(thread.clone(), Parameters::new(vec![Value::Int(writer)])).await?;
        assert_eq!(None, result);
        let result = super::super::kqueueport::drain_1(
            thread.clone(),
            Parameters::new(vec![Value::Int(reader)]),
        )
        .await?;
        assert_eq!(None, result);
        close(thread.clone(), writer).await?;
        close(thread, reader).await
    }

    #[tokio::test]
    async fn test_kevent_0() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let Some(Value::Int(kqfd)) = init(thread.clone(), Parameters::default()).await? else {
            return Err(ristretto_types::Error::InternalError(
                "KQueueArrayWrapper.init returned no descriptor".to_string(),
            ));
        };
        let result = kevent_0(
            thread.clone(),
            Parameters::new(vec![
                Value::Int(kqfd),
                Value::Long(0),
                Value::Int(0),
                Value::Long(0),
            ]),
        )
        .await?;
        assert_eq!(Some(Value::Int(0)), result);
        close(thread, kqfd).await
    }

    #[tokio::test]
    async fn test_register_0() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let Some(Value::Int(kqfd)) = init(thread.clone(), Parameters::default()).await? else {
            return Err(ristretto_types::Error::InternalError(
                "KQueueArrayWrapper.init returned no descriptor".to_string(),
            ));
        };
        let (writer, reader) = socket_pair(thread.clone()).await?;
        let result = register_0(
            thread.clone(),
            Parameters::new(vec![
                Value::Int(kqfd),
                Value::Int(reader),
                Value::Int(i32::from(libc::EVFILT_READ)),
                Value::Int(i32::from(libc::EV_ADD)),
            ]),
        )
        .await?;
        assert_eq!(None, result);

        let result = interrupt(thread.clone(), Parameters::new(vec![Value::Int(writer)])).await?;
        assert_eq!(None, result);
        let address = vm.native_memory().allocate(32);
        let result = kevent_0(
            thread.clone(),
            Parameters::new(vec![
                Value::Int(kqfd),
                Value::Long(address),
                Value::Int(1),
                Value::Long(1_000),
            ]),
        )
        .await?;
        assert_eq!(Some(Value::Int(1)), result);
        let ident = vm.native_memory().read_bytes(address, 8);
        let ident: [u8; 8] = ident.try_into().map_err(|_| {
            ristretto_types::Error::InternalError(
                "one kevent ident must contain eight bytes".to_string(),
            )
        })?;
        assert_eq!(u64::try_from(reader)?, u64::from_ne_bytes(ident));

        close(thread.clone(), kqfd).await?;
        close(thread.clone(), writer).await?;
        close(thread, reader).await
    }
}
