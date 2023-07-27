use std::{env, fs, io, path::Path};

mod error_context;
mod read_dir_ext;

use read_dir_ext::ReadDirExt;

use crate::error_context::ErrorContext;

fn main() -> io::Result<()> {
    let cwd = env::current_dir()?;

    let read_dir_ext = ReadDirExt::new(
        cwd,
        |context, path| {
            let dir_name = path.file_name().and_then(|s| s.to_str());
            let context = if dir_name.is_some_and(|s| s.starts_with('.')) {
                // Ignore dot-directories
                None
            } else if context.is_some() {
                context.cloned()
            } else if is_dart_pkg(path)? {
                dir_name.map(|s| s.to_owned())
            } else {
                None
            };

            Ok(context)
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
    );

    let mut success_count = 0;
    let mut total_count = 0;

    for entry in read_dir_ext {
        match entry {
            Ok((_context, path)) => {
                total_count += 1;
                let result = try_load_parse(&path);
                // let result = try_mmap_parse(&path);
                match result {
                    Ok(_) => {
                        success_count += 1;
                        // println!("[Success] [{context}] {path:?}");
                        print!("*");
                    }
                    Err(_) => {
                        // println!("[Failure] [{context}] {path:?}");
                        print!(" ");
                    }
                }
            }
            Err(err) => {
                println!("{err}");
            }
        }
    }

    println!(
        "\nSuccess / total: {success_count} / {total_count} ({:.4})",
        success_count as f64 / total_count as f64
    );

    Ok(())
}

fn try_load_parse(path: &Path) -> io::Result<()> {
    let content =
        fs::read_to_string(path).context_lazy(|| format!("Cannot read file at path {path:?}"))?;
    let _items = dart_parser::parse(&content)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))
        .context_lazy(|| format!("Cannot parse file at path {path:?}"))?;

    Ok(())
}

fn _try_mmap_parse(path: &Path) -> io::Result<()> {
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
