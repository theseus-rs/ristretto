#![forbid(unsafe_code)]
#![deny(clippy::pedantic)]

use ristretto_classfile::{ClassFile, Result};
use std::fs;
use std::io::Cursor;
use std::{env, process};

/// Example that reads a class file and verifies it.
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    process::exit(run(&args)?)
}

fn run(args: &[String]) -> Result<i32> {
    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        return Ok(1);
    }
    let file_name = &args[1];
    let bytes = fs::read(file_name)?;
    let class_file = ClassFile::from_bytes(&mut Cursor::new(bytes))?;
    class_file.verify()?;
    println!("{class_file}");
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() -> Result<()> {
        let args = vec![
            "read_class".to_string(),
            "classes/HelloWorld.class".to_string(),
        ];
        let exit_code = run(&args)?;
        assert_eq!(0, exit_code);
        Ok(())
    }

    #[test]
    fn test_main_error() -> Result<()> {
        let args = vec!["read_class".to_string()];
        let exit_code = run(&args)?;
        assert_eq!(1, exit_code);
        Ok(())
    }
}
