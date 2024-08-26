use crate::local::cmd::LocalCmd;
use clap::Parser;

#[derive(Parser)]
pub enum ForcNode {
    Local(LocalCmd),
}
