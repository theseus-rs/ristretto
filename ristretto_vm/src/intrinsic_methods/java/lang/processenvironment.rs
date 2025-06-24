use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classfile::mutf8;
use ristretto_classloader::{Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/ProcessEnvironment.environ()[[B", Any)]
#[async_recursion(?Send)]
pub(crate) async fn environ(thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    let mut values = Vec::new();
    for (key, value) in std::env::vars() {
        let key = Some(Reference::from(mutf8::to_bytes(key)?));
        values.push(key);
        let value = Some(Reference::from(mutf8::to_bytes(value)?));
        values.push(value);
    }
    let class = thread.class("[[B").await?;
    let result = Reference::from((class, values));
    Ok(Some(Value::Object(Some(result))))
}
