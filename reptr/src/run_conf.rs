use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(name = "Dart Repointer")]
#[command(version = clap::crate_version!())]
#[command(about = "Fast code generation for Dart. Eventually. Maybe.", long_about = None)]
pub struct RunConf {
    #[command(subcommand)]
    pub cmd: Option<RunCmd>,
}

#[derive(clap::Subcommand)]
pub enum RunCmd {
    Scan { dir: Option<PathBuf> },
    Parse { file: PathBuf },
}
