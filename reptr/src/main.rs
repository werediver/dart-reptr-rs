use std::{env, io, path::Path};

mod error_context;
mod read_dir_ext;

use read_dir_ext::ReadDirExt;

fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;

    let read_dir_ext = ReadDirExt::new(
        cwd,
        |context, path| {
            let dir_name = path.file_name().and_then(|s| s.to_str());
            let context = if dir_name.is_some_and(|s| s.starts_with(".")) {
                // Ignore dot-directories
                None
            } else if context.is_some() {
                context.cloned()
            } else {
                if is_dart_pkg(path)? {
                    dir_name.map(|s| s.to_owned())
                } else {
                    None
                }
            };

            Ok(context)
        },
        |context, path| Ok(context.map(|context| (context.clone(), path))),
    );

    for entry in read_dir_ext {
        match entry {
            Ok((context, path)) => {
                println!("[{context:?}] {path:?}");
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }

    Ok(())
}

fn is_dart_pkg(path: &Path) -> io::Result<bool> {
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
