#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classloader::{ClassLoader, ClassPath, Result};
use std::sync::Arc;

/// Example that uses a class loader to load a class.
#[tokio::main]
async fn main() -> Result<()> {
    let class_path = Arc::new(ClassPath::from("classes"));
    let class_loader = Arc::new(ClassLoader::new("example", class_path));
    let class = ClassLoader::load_class(&class_loader, "HelloWorld").await?;
    println!("{class:?}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
