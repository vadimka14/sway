use super::cmd::TestnetCmd;
use crate::{
    cmd::ask_user_yes_no_question,
    pkg::{create_chainconfig_dir, ChainConfig},
    run::{run_mode, Mode},
};

pub(crate) async fn run(cmd: TestnetCmd) -> anyhow::Result<()> {
    let mode = Mode::Testnet(cmd);
    create_chainconfig_dir(ChainConfig::Testnet)?;
    // Ask if the user already have a key-pair generated.
    let has_keypair = ask_user_yes_no_question("Do you have a keypair in hand?")?;
    

    run_mode(mode).await?;
    Ok(())
}
