use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

use super::common::throw_unix_exception;
use super::managed_files;

#[intrinsic_method("sun/nio/fs/UnixCopyFile.transfer(IIJ)V", LessThanOrEqual(JAVA_17))]
#[async_method]
pub async fn transfer<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _cancel_address = parameters.pop_long()?;
    let src = parameters.pop_int()?;
    let dst = parameters.pop_int()?;

    let vm = thread.vm()?;
    let file_handles = (*vm).file_handles();
    let mut buf = vec![0u8; 8192];

    loop {
        let n = match managed_files::read(file_handles, i64::from(src), &mut buf).await {
            Ok(n) => n,
            Err(e) => {
                let errno = e.raw_os_error().unwrap_or(5);
                return Err(throw_unix_exception(&thread, errno).await);
            }
        };
        if n == 0 {
            break;
        }
        if let Err(e) = managed_files::write_all(file_handles, i64::from(dst), &buf[..n]).await {
            let errno = e.raw_os_error().unwrap_or(5);
            return Err(throw_unix_exception(&thread, errno).await);
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transfer_default_params() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = transfer(thread, Parameters::default()).await;
        assert!(matches!(
            result,
            Err(ristretto_types::Error::ParametersUnderflow)
        ));
    }

    #[tokio::test]
    async fn test_transfer_invalid_fds() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let mut params = Parameters::default();
        params.push_int(0); // dst
        params.push_int(0); // src
        params.push_long(0); // cancel address
        let result = transfer(thread, params).await;
        assert!(matches!(result, Err(ristretto_types::Error::Throwable(_))));
    }

    #[tokio::test]
    #[cfg(target_family = "unix")]
    async fn test_transfer_success() {
        let (vm, thread) = crate::test::java17_thread().await.expect("thread");
        let src_path = std::env::current_dir()
            .unwrap()
            .join("_test_transfer_src.tmp");
        let dst_path = std::env::current_dir()
            .unwrap()
            .join("_test_transfer_dst.tmp");
        std::fs::write(&src_path, b"transfer content").unwrap();

        let file_handles = vm.file_handles();
        let resource_manager = vm.resource_manager();
        let src_fd = managed_files::open(
            file_handles,
            resource_manager,
            src_path.to_str().unwrap(),
            0, // O_RDONLY
            0,
        )
        .await
        .expect("open src");
        let dst_fd = managed_files::open(
            file_handles,
            resource_manager,
            dst_path.to_str().unwrap(),
            libc::O_WRONLY | libc::O_CREAT | libc::O_TRUNC,
            0o644,
        )
        .await
        .expect("open dst");

        let mut params = Parameters::default();
        #[expect(clippy::cast_possible_truncation)]
        {
            params.push_int(dst_fd as i32);
            params.push_int(src_fd as i32);
        }
        params.push_long(0); // cancel address

        let result = transfer(thread, params).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);

        managed_files::close(file_handles, src_fd).await;
        managed_files::close(file_handles, dst_fd).await;

        let content = std::fs::read(&dst_path).unwrap();
        assert_eq!(content, b"transfer content");

        std::fs::remove_file(&src_path).ok();
        std::fs::remove_file(&dst_path).ok();
    }
}
