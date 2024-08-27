use super::cmd::LocalCmd;
use crate::run::{run_mode, Mode};

pub(crate) async fn run(cmd: LocalCmd) -> anyhow::Result<()> {
    let mode = Mode::Local(cmd);
    run_mode(mode).await?;
    Ok(())
}
