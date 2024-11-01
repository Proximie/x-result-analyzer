mod hbs_helpers;
mod params;
mod templates;

use clap::Parser;
use handlebars::Handlebars;
use params::Cli;
use params::Generate;
use params::Get;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args = Cli::try_parse()?;
    match args {
        Cli::Generate(Generate::Database { xcresult_path }) => {
            xcra_core::generate_xcresult_db(&xcresult_path)?;
        }
        Cli::Generate(Generate::FailureReport {
            xcresult_path,
            template,
            output,
        }) => {
            let params = xcra_core::generate_failure_report(&xcresult_path)?;
            let reg = Handlebars::new();
            let content = template.template();
            let content = reg.render_template(&content, &params)?;
            fs::write(output, content)?;
        }
        Cli::Generate(Generate::Report {
            xcresult_path,
            template,
            output,
        }) => {
            let params = xcra_core::generate_tests_report(&xcresult_path)?;
            let mut reg = Handlebars::new();
            reg.register_helper("toEmoji", Box::new(hbs_helpers::result_to_emoji));
            let content = template.template();
            let content = reg.render_template(&content, &params)?;
            fs::write(output, content)?;
        }
        Cli::Get(Get::Summary { xcresult_path }) => {
            xcra_core::get_summary(&xcresult_path)?;
        }
        Cli::Get(Get::FailureReport { xcresult_path }) => {
            xcra_core::get_failure_report(&xcresult_path)?;
        }
        Cli::Get(Get::Report { xcresult_path }) => {
            xcra_core::get_tests_report(&xcresult_path)?;
        }
    }
    Ok(())
}
