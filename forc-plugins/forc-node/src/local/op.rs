use super::cmd::LocalCmd;
use crate::run::{run_mode, Mode};

pub(crate) async fn run(_cmd: LocalCmd) -> anyhow::Result<()> {
    let mode = Mode::Local;
    run_mode(mode).await?;
    Ok(())
}
