#![forbid(unsafe_code)]

use ristretto_classloader::{JavaStr, Result, runtime};

/// Example that loads a class from the Java runtime.
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let (_java_home, java_version, class_loader) = runtime::version_class_loader("21").await?;
    let class_name = "java.util.HashMap";
    println!("Loading {class_name} from Java version {java_version}");
    let class = class_loader
        .load(JavaStr::try_from_str(class_name)?)
        .await?;
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
