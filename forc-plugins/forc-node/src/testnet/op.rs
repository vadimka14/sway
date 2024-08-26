use super::cmd::TestnetCmd;
use crate::run::{run_mode, Mode};

pub(crate) async fn run(_cmd: TestnetCmd) -> anyhow::Result<()> {
    let mode = Mode::Testnet;
    run_mode(mode).await?;
    Ok(())
}
