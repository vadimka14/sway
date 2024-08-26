use clap::Parser;
use fuel_core::service::DbType;
use fuel_core_bin::cli::run::Command as RunCmd;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct RunOpts {
    command: RunCmd,
}

impl Default for RunOpts {
    fn default() -> Self {
        let default_input = vec![""];
        let command = RunCmd::parse_from(default_input);
        Self { command }
    }
}

impl FromStr for RunOpts {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" ");
        let command = RunCmd::parse_from(s);

        let run_opts = RunOpts { command };
        Ok(run_opts)
    }
}

/// Supported mode of operations by `forc-node`.
#[derive(Debug)]
pub enum Mode {
    /// Local is a local node suited for local development.
    /// By default, the node is in `debug` mode and the db used is `in-memory`.
    Local,
    /// Testnet is the configuration to connect the node to latest testnet.
    Testnet,
    /// Custom is basically equivalent to running `fuel-core run` with some params.
    Custom(RunOpts),
}

impl From<Mode> for RunOpts {
    fn from(value: Mode) -> Self {
        let run_cmd = match value {
            Mode::Local => {
                let mut opts = RunOpts::default();
                opts.command.database_type = DbType::InMemory;
                opts.command.debug = true;
                opts.command
            }
            Mode::Testnet => unimplemented!("testnet is not implemented yet"),
            Mode::Custom(cmd) => cmd.command,
        };
        Self { command: run_cmd }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self::Custom(RunOpts::default())
    }
}

pub async fn run_mode(mode: Mode) -> anyhow::Result<()> {
    let opts: RunOpts = mode.into();
    fuel_core_bin::cli::run::exec(opts.command).await?;
    Ok(())
}
