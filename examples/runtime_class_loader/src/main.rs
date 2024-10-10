#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classloader::{runtime, Result};

/// Example that loads a class from the Java runtime.
fn main() -> Result<()> {
    let (version, class_loader) = runtime::class_loader("21")?;
    let class_name = "java/util/HashMap";
    println!("Loading {class_name} from Java runtime {version}");
    let class = class_loader.load(class_name)?;
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
