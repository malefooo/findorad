use crate::command::{Init, Node};
use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Findora Network")]
pub struct Opts {
    /// Findorad home, include config and data.
    #[clap(short = 'h', long = "home", default_value = "")]
    home: String,
    ///Start a findorad dev node.
    #[clap(short = 'd', long = "dev")]
    dev: bool,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn execute(&self) {

    }
}

#[derive(Clap)]
pub enum SubCommand {
    #[clap(name = "init", version = "1.0", author = "Findora Network")]
    Init(Init),
    #[clap(name = "node", version = "1.0", author = "Findora Network")]
    Node(Node),
}

impl SubCommand {
    pub fn execute(&self) {

    }
}
