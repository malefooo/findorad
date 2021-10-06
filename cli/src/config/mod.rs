mod node;
mod cli;

use std::{
    path::{Path, PathBuf},
};

pub use node::Node;
pub use cli::Cli;

use ruc::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub node: Node,
    pub cli: Cli,
    #[serde(skip)]
    config_path: PathBuf,
}

impl Config {
    pub fn load(home_path: &Path, config_path: &Path) -> Result<Self> {
        let default_config = Config::default();

        let mut config = config::Config::try_from(&default_config).c(d!())?;

        let c_path = if home_path != Path::new(concat!(env!("HOME"), "/.findora/fn")) {
            let p = home_path.join("config.toml");
            p
        } else {
            config_path.to_path_buf()
        };

        config.merge(config::File::from(c_path)).c(d!())?;
        config.merge(config::Environment::with_prefix("fn")).c(d!())?;

        Ok(config.try_into().c(d!())?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node: Node::default(),
            cli: Cli::default(),
            config_path: PathBuf::from(concat!(env!("HOME"), "/.findora/fn/config.toml")),
        }
    }
}
