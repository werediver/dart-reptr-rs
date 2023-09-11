use std::{fs, io, path::Path};

mod error_context;
mod read_dir_ext;

pub use error_context::ErrorContext;
pub use read_dir_ext::{MapDirResult, ReadDirExt};

pub fn is_dart_pkg(path: &Path) -> io::Result<bool> {
    let has_manifest = {
        let manifest = path.join("pubspec.yaml");
        manifest.try_exists()? && manifest.is_file()
    };

    fn has_lib(path: &Path) -> io::Result<bool> {
        let lib = path.join("lib");
        Ok(lib.try_exists()? && lib.is_dir())
    }

    fn has_bin(path: &Path) -> io::Result<bool> {
        let bin = path.join("bin");
        Ok(bin.try_exists()? && bin.is_dir())
    }

    Ok(has_manifest && (has_lib(path)? || has_bin(path)?))
}

pub fn read_string(path: &Path) -> io::Result<String> {
    String::from_utf8(read(path)?).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Cannot load file at path {path:?}\n\nFile does not contain valid UTF-8",
        )
    })
}

pub fn read(path: &Path) -> io::Result<Vec<u8>> {
    fs::read(path).context_lazy(|| format!("Cannot read file at path {path:?}"))
}
