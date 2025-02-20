#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use anyhow::{Result, bail};
use ristretto_classfile::ClassFile;
use std::env;
use std::fs;
use std::io::Cursor;

/// Example that reads a class file and verifies it.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    run(&args)
}

fn run(args: &[String]) -> Result<()> {
    if args.len() != 2 {
        bail!("Usage: {} <file_name>", args[0]);
    }
    let file_name = &args[1];
    let bytes = fs::read(file_name)?;
    let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
    class_file.verify()?;
    println!("{class_file}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() -> Result<()> {
        let args = vec![
            "read_class".to_string(),
            "classes/HelloWorld.class".to_string(),
        ];
        run(&args)
    }

    #[test]
    fn test_main_error() {
        assert!(main().is_err());
    }
}
