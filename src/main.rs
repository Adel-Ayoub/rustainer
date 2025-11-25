mod cgroup;
mod commands;
mod docker;
mod namespace;
mod storage;
mod utils;

use anyhow::Result;
use clap::Parser;
#[cfg(unix)]
use commands::{run, start, RunOpts};
use tracing::debug;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[cfg(unix)]
    /// Run application in container
    Run(RunOpts),

    #[cfg(unix)]
    #[command(hide = true)]
    Start(RunOpts),
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let opts: Opts = Opts::parse();

    match opts.subcmd {
        #[cfg(unix)]
        SubCommand::Run(opts) => {
            run(opts)?;
        }
        #[cfg(unix)]
        SubCommand::Start(opts) => {
            start(opts)?;
        }
    }

    debug!("container exited");

    Ok(())
}

