//! A forc plugin to start a fuel core instance, preconfigured for generic
//! usecases.
mod cmd;
mod local;

use clap::Parser;
use forc_tracing::{init_tracing_subscriber, println_error};

fn main() {
    init_tracing_subscriber(Default::default());
    let command = cmd::ForcNode::parse();
    /*
    if let Err(err) = op::run(command) {
        println_error(&format!("{}", err));
        std::process::exit(1);
    }
    */
}
