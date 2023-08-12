use std::{env, io};

use crate::common::{is_dart_pkg, try_load_parse, ReadDirExt};

pub fn scan_dir(dir: Option<std::path::PathBuf>) -> io::Result<()> {
    let dir = dir.map_or_else(env::current_dir, Ok)?;

    let read_dir_ext = ReadDirExt::new(
        dir.clone(),
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
            Ok((context, path)) => {
                total_count += 1;
                let result = try_load_parse(&path);
                // let result = _try_mmap_parse(&path);

                let rel_path = path.strip_prefix(&dir).unwrap();

                match result {
                    Ok(_) => {
                        success_count += 1;
                        println!("[PARSED] [{context}] {rel_path:?}");
                    }
                    Err(e) => {
                        println!("[FAILED] [{context}] {rel_path:?}\n{e}");
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
