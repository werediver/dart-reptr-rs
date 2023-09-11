use std::{io, path::PathBuf};

use crate::common::read_string;

pub fn parse_file(file_path: PathBuf) -> io::Result<()> {
    let source = read_string(&file_path)?;
    match dart_parser::parse(&source) {
        Ok(ast) => {
            println!("{ast:#?}");
        }
        Err(e) => {
            println!("Error parsing file at path {file_path:?}\n\n{e}");
        }
    }
    Ok(())
}
