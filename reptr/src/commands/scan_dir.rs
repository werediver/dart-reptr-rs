use std::{
    env, fs, io, path,
    time::{Duration, Instant},
};

use crate::common::{is_dart_pkg, read, ErrorContext, MapDirResult, ReadDirExt};

#[derive(Default, Debug)]
pub struct Options {
    pub quiet: bool,
}

pub fn scan_dir(dir_paths: Vec<path::PathBuf>, options: Options) -> io::Result<()> {
    let println = move |s: String| {
        if !options.quiet {
            println!("{s}");
        }
    };

    let dir_paths = if dir_paths.is_empty() {
        Vec::from_iter([env::current_dir()?])
    } else {
        dir_paths
    };

    let mut total_count = 0usize;
    let mut parsed_count = 0usize;
    let mut loaded_size = 0usize;
    let mut parsed_size = 0usize;
    let mut loading_duration = Duration::ZERO;
    let mut utf8_validation_duration = Duration::ZERO;
    let mut parsing_duration = Duration::ZERO;

    for dir in dir_paths {
        let read_dir_ext = ReadDirExt::new(
            dir.clone(),
            |context: Option<&String>, path| {
                let dir_name = file_name(path)?;
                let context = if dir_name.starts_with('.') {
                    // Ignore dot-directories
                    MapDirResult::Ignore
                } else if let Some(context) = context {
                    MapDirResult::Mark(context.to_owned())
                } else if is_dart_pkg(path)? {
                    MapDirResult::Mark(dir_name.to_owned())
                } else {
                    MapDirResult::Clear
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

        for entry in read_dir_ext {
            let result = entry
                .context_lazy(|| "Error scanning filesystem".to_owned())
                .and_then(|(context, path)| {
                    let t = Instant::now();
                    let source = read(&path)?;
                    loading_duration += Instant::now() - t;
                    loaded_size += source.len();

                    let t = Instant::now();
                    let source = String::from_utf8(source)
                        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
                    utf8_validation_duration += Instant::now() - t;

                    Ok((context, path, source))
                });

            match result {
                Ok((context, path, source)) => {
                    total_count += 1;

                    let rel_path = path.strip_prefix(&dir).unwrap();

                    let t = Instant::now();
                    match dart_parser::parse(&source) {
                        Ok(_) => {
                            parsing_duration += Instant::now() - t;
                            parsed_size += source.len();
                            parsed_count += 1;
                            // println(format!("[PARSED] [{context}] {rel_path:?}"));
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
    }

    println(format!(
        "\nLoaded {:.2} MiB in {:.2} s, {:.2} MiB/s",
        loaded_size as f64 / (1024.0 * 1024.0),
        loading_duration.as_secs_f64(),
        loaded_size as f64 / loading_duration.as_secs_f64() / (1024.0 * 1024.0)
    ));
    println(format!(
        "UTF-8 validation took {:.2} s, {:.2} MiB/s",
        utf8_validation_duration.as_secs_f64(),
        loaded_size as f64 / utf8_validation_duration.as_secs_f64() / (1024.0 * 1024.0)
    ));
    println(format!(
        "Parsed {parsed_count} out of {total_count} files, {:.4}",
        parsed_count as f64 / total_count as f64
    ));
    println(format!(
        "Parsed {:.2} MiB in {:.2} s, {:.2} MiB/s",
        parsed_size as f64 / (1024.0 * 1024.0),
        parsing_duration.as_secs_f64(),
        parsed_size as f64 / parsing_duration.as_secs_f64() / (1024.0 * 1024.0)
    ));

    Ok(())
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
        // The path terminates in "..".
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
