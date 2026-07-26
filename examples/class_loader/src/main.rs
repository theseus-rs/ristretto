#![forbid(unsafe_code)]

use ristretto_classloader::{Class, ClassLoader, ClassPath, JavaStr, Result};
use std::sync::Arc;

/// Example that uses a class loader to load a class.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let class = load_class().await?;
    println!("{class:?}");
    Ok(())
}

async fn load_class() -> Result<Arc<Class>> {
    let class_path = ClassPath::from(&["classes"]);
    let class_loader = ClassLoader::new("example", class_path);
    let class = class_loader
        .load(JavaStr::try_from_str("HelloWorld")?)
        .await?;
    Ok(class)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn test_load_class() -> Result<()> {
        let class = load_class().await?;

        assert_eq!("HelloWorld", class.name());
        assert!(class.main_method().is_some());
        Ok(())
    }
}
