mod templates;
mod params;

use clap::Parser;
use params::{Cli, Generate, Get};

fn main() -> anyhow::Result<()> {
    let args = Cli::try_parse()?;
    match args {
        Cli::Generate(Generate::Db { xcresult_path }) => {
            xcra_core::generate_xcresult_db(&xcresult_path)?;
        },
        Cli::Get(Get::Summary { xcresult_path }) => {
            xcra_core::get_summary(&xcresult_path)?;
        },
        Cli::Get(Get::Report { xcresult_path }) => {
            xcra_core::get_failure_report(&xcresult_path)?;
        },
    }
    Ok(())
}
