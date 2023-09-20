mod find_dart_files;
mod parse_raw;
mod stats;

use futures::FutureExt;
use std::{env, io, path, sync::Arc};

use crate::common::poisoned;

use self::{find_dart_files::find_dart_files, parse_raw::parse_raw_async, stats::Counter};

macro_rules! time {
    ($x:expr) => {{
        use std::time::Instant;

        let t = Instant::now();
        let value = $x;
        let duration = Instant::now() - t;

        (value, duration)
    }};
}

pub(crate) use time;

#[derive(Default, Debug)]
pub struct Options {
    pub ignore: Vec<path::PathBuf>,
    pub quiet: bool,
}

pub async fn scan_dirs(dir_paths: Vec<path::PathBuf>, options: Options) -> io::Result<()> {
    let stats = Arc::new(std::sync::Mutex::new(stats::Stats::default()));

    let println = move |s: String| {
        if !options.quiet {
            println!("{s}");
        }
    };

    let cwd = env::current_dir()?;
    let dir_paths = if dir_paths.is_empty() {
        Vec::from_iter([cwd.clone()])
    } else {
        dir_paths
    };

    let mut tasks = tokio::task::JoinSet::new();
    let async_read_throttle = Arc::new(tokio::sync::Semaphore::new(16));

    for dir in dir_paths {
        for entry in find_dart_files(&dir, options.ignore.clone()) {
            match entry {
                Ok((context, path)) => {
                    let async_read_throttle = async_read_throttle.clone();
                    let stats = stats.clone();
                    let cwd = cwd.clone();

                    let rel_path = path.strip_prefix(&cwd).unwrap_or(&path).to_owned();
                    let rel_path_copy = rel_path.clone();

                    tasks.spawn(
                        async move {
                            let source =
                                read_async(&path, async_read_throttle, stats.clone()).await?;

                            parse_raw_async(source, stats).await.map_err(|e| {
                                io::Error::new(
                                    io::ErrorKind::InvalidData,
                                    format!("Error parsing file at path {rel_path:?}\n\n{e}"),
                                )
                            })
                        }
                        .map(|result| (context, rel_path_copy, result)),
                    );
                }
                Err(e) => {
                    eprintln!("Error scanning filesystem: {e}");
                }
            }
        }
    }

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(result) => {
                let (context, _rel_path, result) = result;
                match result {
                    Ok(_ast) => {
                        // println(format!(
                        //     "[PARSED] [{context}] Parsed file at path {rel_path:?}"
                        // ));
                    }
                    Err(e) => {
                        let pkg_name = context.pkg_name;
                        println(format!("[FAILED] [{pkg_name}] {e}"));
                    }
                }
            }
            Err(join_error) => {
                eprintln!("Async task failed: {join_error}");
            }
        }
    }

    println(format!("\n{}\n", stats.lock().map_err(poisoned)?));

    Ok(())
}

async fn read_async(
    path: &path::Path,
    throttle: Arc<tokio::sync::Semaphore>,
    stats: Arc<std::sync::Mutex<stats::Stats>>,
) -> io::Result<Vec<u8>> {
    let _permit = throttle.acquire_owned().await.unwrap();

    let (source, duration) = time! { tokio::fs::read(path).await? };

    stats
        .lock()
        .map_err(poisoned)?
        .count(stats::event::FileLoaded {
            size: source.len(),
            duration,
        });

    Ok(source)
}
