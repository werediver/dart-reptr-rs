use clap::Parser;
use commands::{
    parse_file,
    scan::{self, scan_dirs},
};
use std::io;

mod commands;
mod common;
mod run_conf;

use crate::run_conf::{RunCmd, RunConf};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let run_conf = RunConf::parse();

    if let Some(jobs) = run_conf.jobs {
        rayon::ThreadPoolBuilder::new()
            .num_threads(jobs)
            .build_global()
            .unwrap();
    }

    let cmd = run_conf.cmd.unwrap_or(RunCmd::Scan {
        dirs: Vec::new(),
        ignore: Vec::new(),
        quiet: false,
    });

    match cmd {
        RunCmd::Scan {
            dirs,
            ignore,
            quiet,
        } => scan_dirs(dirs, scan::Options { ignore, quiet }).await?,
        RunCmd::Parse { file } => parse_file(file)?,
    }

    Ok(())
}
