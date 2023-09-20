use std::{fs, io, path};

use crate::common::{MapDirResult, ReadDirExt};

#[derive(Clone, Debug)]
pub struct FileContext {
    pub pkg_name: String,
    pub pkg_dir: path::PathBuf,
}

pub fn find_dart_files(
    dir: &path::Path,
    pkg_dirs_to_ignore: Vec<impl AsRef<path::Path>>,
) -> impl Iterator<Item = io::Result<(FileContext, path::PathBuf)>> {
    ReadDirExt::new(
        dir.to_owned(),
        move |context: Option<&FileContext>, path| {
            match file_name(path) {
                Ok(dir_name) => {
                    let context = if dir_name.starts_with('.') {
                        // Ignore dot-directories
                        MapDirResult::Ignore
                    } else if let Some(context) = context {
                        let should_ignore =
                            path.strip_prefix(&context.pkg_dir).is_ok_and(|rel_path| {
                                pkg_dirs_to_ignore
                                    .iter()
                                    .map(|dir| dir.as_ref())
                                    .any(|dir| dir == rel_path)
                            });
                        if should_ignore {
                            MapDirResult::Ignore
                        } else {
                            MapDirResult::Mark(context.to_owned())
                        }
                    } else if is_dart_pkg(path)? {
                        MapDirResult::Mark(FileContext {
                            pkg_name: dir_name.to_owned(),
                            pkg_dir: path.to_owned(),
                        })
                    } else {
                        MapDirResult::Clear
                    };

                    Ok(context)
                }
                Err(_) => Ok(MapDirResult::Clear),
            }
        },
        |context, path| {
            Ok(context.and_then(|context| {
                if path
                    .extension()
                    .and_then(|s| s.to_str())
                    .is_some_and(|ext| ext == "dart")
                {
                    Some((context.clone(), path))
                } else {
                    None
                }
            }))
        },
    )
}

fn file_name(p: impl AsRef<path::Path>) -> io::Result<String> {
    fn cannot_convert_to_utf8() -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, "Cannot convert path to UTF-8")
    }

    if let Some(s) = p.as_ref().file_name() {
        s.to_str()
            .map(ToOwned::to_owned)
            .ok_or_else(cannot_convert_to_utf8)
    } else {
        // The path terminates in ".." or is the f/s root "/".
        let p = fs::canonicalize(p)?;
        if let Some(s) = p.file_name() {
            s.to_str()
                .map(ToOwned::to_owned)
                .ok_or_else(cannot_convert_to_utf8)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot extract file or directory name from path",
            ))
        }
    }
}

pub fn is_dart_pkg(path: &path::Path) -> io::Result<bool> {
    let has_manifest = {
        let manifest = path.join("pubspec.yaml");
        manifest.try_exists()? && manifest.is_file()
    };

    fn has_lib(path: &path::Path) -> io::Result<bool> {
        let lib = path.join("lib");
        Ok(lib.try_exists()? && lib.is_dir())
    }

    fn has_bin(path: &path::Path) -> io::Result<bool> {
        let bin = path.join("bin");
        Ok(bin.try_exists()? && bin.is_dir())
    }

    Ok(has_manifest && (has_lib(path)? || has_bin(path)?))
}
