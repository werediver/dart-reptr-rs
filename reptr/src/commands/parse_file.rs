use std::{io, path::PathBuf};

use crate::common::try_load;

pub fn parse_file(file_path: PathBuf) -> io::Result<()> {
    let source = try_load(&file_path)?;
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
