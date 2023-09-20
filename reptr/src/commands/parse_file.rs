use std::{fs, io, path};

use crate::common::ErrorContext;

pub fn parse_file(path: path::PathBuf) -> io::Result<()> {
    let source = read_string(&path)?;
    match dart_parser::parse(&source) {
        Ok(ast) => {
            println!("{ast:#?}");
        }
        Err(e) => {
            println!("Error parsing file at path {path:?}\n\n{e}");
        }
    }
    Ok(())
}

fn read_string(path: &path::Path) -> io::Result<String> {
    String::from_utf8(read(path)?).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Cannot load file at path {path:?}: {e}"),
        )
    })
}

fn read(path: &path::Path) -> io::Result<Vec<u8>> {
    fs::read(path).context_lazy(|| format!("Cannot read file at path {path:?}"))
}
