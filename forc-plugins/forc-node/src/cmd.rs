use crate::{local::cmd::LocalCmd, testnet::cmd::TestnetCmd};
use clap::Parser;

#[derive(Parser)]
pub enum ForcNode {
    Local(LocalCmd),
    Testnet(TestnetCmd),
}
