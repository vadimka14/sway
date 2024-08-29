use clap::Parser;

#[derive(Parser)]
pub struct TestnetCmd {
    #[clap(long = "port")]
    pub port: Option<u16>,
}
