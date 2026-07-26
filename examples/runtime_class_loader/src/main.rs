#![forbid(unsafe_code)]

use ristretto_classloader::{Class, JavaStr, Result, runtime};
use std::sync::Arc;

/// Example that loads a class from the Java runtime.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let (java_version, class) = load_class().await?;
    let class_name = "java.util.HashMap";
    println!("Loading {class_name} from Java version {java_version}");
    println!("{class:?}");
    Ok(())
}

async fn load_class() -> Result<(String, Arc<Class>)> {
    let (_java_home, java_version, class_loader) = runtime::version_class_loader("21").await?;
    let class = class_loader
        .load(JavaStr::try_from_str("java.util.HashMap")?)
        .await?;
    Ok((java_version, class))
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    #[ignore = "requires network access"]
    async fn test_load_class() -> Result<()> {
        let (java_version, class) = load_class().await?;

        assert!(java_version.starts_with("21"));
        assert_eq!("java/util/HashMap", class.name());
        Ok(())
    }
}
