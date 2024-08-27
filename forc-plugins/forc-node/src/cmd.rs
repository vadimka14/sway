use crate::{local::cmd::LocalCmd, testnet::cmd::TestnetCmd};
use clap::Parser;

#[derive(Parser)]
pub enum ForcNode {
    /// Start a local node for development purposes.
    Local(LocalCmd),
    /// Starts a node that will connect to latest testnet.
    Testnet(TestnetCmd),
}
