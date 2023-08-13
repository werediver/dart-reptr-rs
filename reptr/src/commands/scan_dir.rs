use std::{env, io};

use crate::common::{is_dart_pkg, try_load, ErrorContext, ReadDirExt};

#[derive(Default, Debug)]
pub struct Options {
    pub quiet: bool,
}

pub fn scan_dir(dir: Option<std::path::PathBuf>, options: Options) -> io::Result<()> {
    let println = move |s: String| {
        if !options.quiet {
            println!("{s}");
        }
    };

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
        let result = entry
            .context_lazy(|| "Error scanning filesystem".to_owned())
            .and_then(|(context, path)| {
                let source = try_load(&path)?;

                Ok((context, path, source))
            });

        match result {
            Ok((context, path, source)) => {
                total_count += 1;

                let rel_path = path.strip_prefix(&dir).unwrap();

                match dart_parser::parse(&source) {
                    Ok(_) => {
                        success_count += 1;
                        println(format!("[PARSED] [{context}] {rel_path:?}"));
                    }
                    Err(e) => {
                        println(format!("[FAILED] [{context}] {rel_path:?}\n{e}"));
                    }
                }
            }
            Err(err) => {
                println(format!("{err}"));
            }
        }
    }

    println(format!(
        "\nSuccess / total: {success_count} / {total_count} ({:.4})",
        success_count as f64 / total_count as f64
    ));

    Ok(())
}
