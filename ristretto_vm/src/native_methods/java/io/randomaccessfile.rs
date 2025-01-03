use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `java.io.RandomAccessFile`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/RandomAccessFile";
    let java_version = registry.java_version();

    if java_version <= &JAVA_18 {
        registry.register(class_name, "length", "()J", length);
        registry.register(class_name, "readBytes", "([BII)I", read_bytes);
        registry.register(class_name, "setLength", "(J)V", set_length);
        registry.register(class_name, "writeBytes", "([BII)V", write_bytes);
    } else {
        registry.register(class_name, "length0", "()J", length_0);
        registry.register(class_name, "readBytes0", "([BII)I", read_bytes_0);
        registry.register(class_name, "setLength0", "(J)V", set_length_0);
        registry.register(class_name, "writeBytes0", "([BII)V", write_bytes_0);
    }

    registry.register(class_name, "getFilePointer", "()J", get_file_pointer);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(class_name, "open0", "(Ljava/lang/String;I)V", open_0);
    registry.register(class_name, "read0", "()I", read_0);
    registry.register(class_name, "seek0", "(J)V", seek_0);
    registry.register(class_name, "write0", "(I)V", write_0);
}

#[async_recursion(?Send)]
async fn close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.close0()V")
}

#[async_recursion(?Send)]
async fn get_file_pointer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.getFilePointer()J")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn length(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.length()J")
}

#[async_recursion(?Send)]
async fn length_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.length0()J")
}

#[async_recursion(?Send)]
async fn open_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.open0(Ljava/lang/String;I)V")
}

#[async_recursion(?Send)]
async fn read_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.read0()I")
}

#[async_recursion(?Send)]
async fn read_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.readBytes([BII)I")
}

#[async_recursion(?Send)]
async fn read_bytes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.readBytes0([BII)I")
}

#[async_recursion(?Send)]
async fn seek_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.seek0(J)V")
}

#[async_recursion(?Send)]
async fn set_length(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.setLength(J)V")
}

#[async_recursion(?Send)]
async fn set_length_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.setLength0(J)V")
}

#[async_recursion(?Send)]
async fn write_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.write0(I)V")
}

#[async_recursion(?Send)]
async fn write_bytes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.writeBytes([BII)V")
}

#[async_recursion(?Send)]
async fn write_bytes_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.RandomAccessFile.writeBytes0([BII)V")
}
