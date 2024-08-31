use crate::{local::cmd::LocalCmd, testnet::cmd::TestnetCmd};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Confirm};

#[derive(Parser)]
pub enum ForcNode {
    /// Start a local node for development purposes.
    Local(LocalCmd),
    /// Starts a node that will connect to latest testnet.
    Testnet(TestnetCmd),
}

pub(crate) fn ask_user_yes_no_question(question: &str) -> anyhow::Result<bool> {
    let answer = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(question)
        .default(false)
        .show_default(false)
        .interact()?;
    Ok(answer)
}
