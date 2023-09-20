use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(name = "Dart Repointer")]
#[command(version = clap::crate_version!())]
#[command(about = "Fast code generation for Dart. Eventually. Maybe.", long_about = None)]
pub struct RunConf {
    /// The size of the global thread pool.
    #[arg(short, long)]
    pub jobs: Option<usize>,
    #[command(subcommand)]
    pub cmd: Option<RunCmd>,
}

#[derive(clap::Subcommand)]
pub enum RunCmd {
    Scan {
        dirs: Vec<PathBuf>,
        /// Directories inside packages to ignore (e.g. "build,test,example").
        ///
        /// Dot-directories are always ignored (e.g. ".git", ".dart_tool").
        #[arg(short, long, default_value = "build", value_delimiter = ',')]
        ignore: Vec<PathBuf>,
        #[arg(short, long)]
        quiet: bool,
    },
    Parse {
        file: PathBuf,
    },
}
