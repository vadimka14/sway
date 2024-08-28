use super::cmd::LocalCmd;
use crate::{
    pkg::{create_chainconfig_dir, ChainConfig},
    run::{run_mode, Mode},
};

pub(crate) async fn run(cmd: LocalCmd) -> anyhow::Result<()> {
    let mut cmd = cmd.clone();
    create_chainconfig_dir(ChainConfig::Local)?;
    if cmd.chain_config.is_none() {
        cmd.chain_config = Some(ChainConfig::Local.into());
    }
    let mode = Mode::Local(cmd);
    run_mode(mode).await?;
    Ok(())
}
