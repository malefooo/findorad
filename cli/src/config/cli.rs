use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cli {
    pub home: PathBuf,
}

impl Default for Cli {
    fn default() -> Self {
        Self {
            home: PathBuf::from(concat!(env!("HOME"), "/.findora/fn")),
        }
    }
}

