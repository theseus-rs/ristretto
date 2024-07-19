#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classfile::{ClassFile, Result};
use std::env;
use std::fs;
use std::io::Cursor;
use std::process;

/// Example that reads a class file and verifies it.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        process::exit(1);
    }
    let file_name = &args[1];
    let bytes = fs::read(file_name)?;
    let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
    class_file.verify()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}
