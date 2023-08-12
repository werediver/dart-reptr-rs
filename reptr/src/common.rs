use std::{fs, io, path::Path};

mod error_context;
mod read_dir_ext;

pub use error_context::ErrorContext;
pub use read_dir_ext::ReadDirExt;

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

pub fn try_load_parse(path: &Path) -> io::Result<()> {
    let content =
        fs::read_to_string(path).context_lazy(|| format!("Cannot read file at path {path:?}"))?;
    let _items = dart_parser::parse(&content)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
        .context_lazy(|| format!("Cannot parse file at path {path:?}"))?;

    Ok(())
}

pub fn _try_mmap_parse(path: &Path) -> io::Result<()> {
    let f = fs::File::open(path).context_lazy(|| format!("Cannot open file at path {path:?}"))?;
    let content = unsafe { memmap2::Mmap::map(&f)? };
    let content =
        std::str::from_utf8(&content).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    // let content = unsafe { std::str::from_utf8_unchecked(&content) };
    let _items = dart_parser::parse(content)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
        .context_lazy(|| format!("Cannot parse file at path {path:?}"))?;

    Ok(())
}
