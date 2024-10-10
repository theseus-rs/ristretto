#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classloader::{ClassLoader, ClassPath, Result};

/// Example that uses a class loader to load a class.
fn main() -> Result<()> {
    let class_path = ClassPath::from("classes");
    let class_loader = ClassLoader::new("example", class_path);
    let class = class_loader.load("HelloWorld")?;
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
