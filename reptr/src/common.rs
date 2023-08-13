use std::{fs, io, path::Path};

mod error_context;
mod read_dir_ext;
mod source;

pub use error_context::ErrorContext;
pub use read_dir_ext::ReadDirExt;
pub use source::Source;

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

pub fn try_load(path: &Path) -> io::Result<Source> {
    fs::read_to_string(path)
        .context_lazy(|| format!("Cannot read file at path {path:?}"))
        .map(|s| s.into())
}

pub fn _try_mmap(path: &Path) -> io::Result<Source> {
    let f = fs::File::open(path).context_lazy(|| format!("Cannot open file at path {path:?}"))?;
    let mmap = unsafe { memmap2::Mmap::map(&f) }
        .context_lazy(|| format!("Cannot memory-map file at path {path:?}"))?;

    mmap.try_into()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
