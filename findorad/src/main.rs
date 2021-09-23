#![feature(generic_associated_types)]

pub mod coinbase;
pub mod command;
pub mod utxo;

use sha3::Sha3_512;
use libfindora::transaction::Transaction;
use clap::Clap;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "Transaction"
)]
pub struct Findorad {
    pub coinbase: coinbase::CoinbaseModule,
    pub utxo: utxo::UtxoModule,
}

fn main() {
    env_logger::init();

    let opts = command::Opts::parse();

}
