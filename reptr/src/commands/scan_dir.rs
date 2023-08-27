use std::{
    env, io,
    time::{Duration, Instant},
};

use crate::common::{is_dart_pkg, try_load, ErrorContext, ReadDirExt};

#[derive(Default, Debug)]
pub struct Options {
    pub quiet: bool,
}

pub fn scan_dir(dir_path: Option<std::path::PathBuf>, options: Options) -> io::Result<()> {
    let println = move |s: String| {
        if !options.quiet {
            println!("{s}");
        }
    };

    let dir = dir_path.map_or_else(env::current_dir, Ok)?;

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

    let mut total_count = 0usize;
    let mut parsed_count = 0usize;
    let mut parsed_duration = Duration::ZERO;
    let mut parsed_size = 0usize;

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

                let t = Instant::now();
                match dart_parser::parse(&source) {
                    Ok(_) => {
                        parsed_duration += Instant::now() - t;
                        parsed_size += source.len();
                        parsed_count += 1;
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
        "\nParsed {parsed_count} out of {total_count} files, {:.4}",
        parsed_count as f64 / total_count as f64
    ));
    println(format!(
        "Parsed {:.2} MiB in {:.2} s, {:.2} MiB/s",
        parsed_size as f64 / (1024.0 * 1024.0),
        parsed_duration.as_secs_f64(),
        parsed_size as f64 / parsed_duration.as_secs_f64() / (1024.0 * 1024.0)
    ));

    Ok(())
}
