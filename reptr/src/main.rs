use clap::Parser;
use commands::{parse_file, scan_dir};
use std::io;

mod commands;
mod common;
mod run_conf;

use crate::run_conf::{RunCmd, RunConf};

fn main() -> io::Result<()> {
    let run_conf = RunConf::parse();

    let cmd = run_conf.cmd.unwrap_or(RunCmd::Scan {
        dir: None,
        quiet: false,
    });

    match cmd {
        RunCmd::Scan { dir, quiet } => scan_dir(dir, scan_dir::Options { quiet })?,
        RunCmd::Parse { file } => parse_file(file)?,
    }

    Ok(())
}
