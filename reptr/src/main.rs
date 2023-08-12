use clap::Parser;
use commands::scan_dir;
use std::io;

mod commands;
mod common;
mod run_conf;

use crate::run_conf::{RunCmd, RunConf};

fn main() -> io::Result<()> {
    let run_conf = RunConf::parse();

    match run_conf.cmd.unwrap_or(RunCmd::Scan { dir: None }) {
        RunCmd::Scan { dir } => scan_dir(dir)?,
        RunCmd::Parse { file: _ } => todo!(),
    }

    Ok(())
}
