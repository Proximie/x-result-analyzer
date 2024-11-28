mod hbs_helpers;
mod params;
mod templates;

use anyhow::bail;
use clap::Parser;
use handlebars::Handlebars;
use params::Cli;
use params::Generate;
use params::Get;
use params::Template;
use std::fs;

fn main() -> anyhow::Result<()> {
    let args = Cli::try_parse()?;
    match args {
        Cli::Generate(Generate::Database { xcresult_path }) => {
            xra_core::generate_xcresult_db(&xcresult_path)?;
        }
        Cli::Generate(Generate::FailureReport {
            xcresult_path,
            template:
                Template {
                    builtin: Some(_),
                    custom: Some(_),
                },
            output,
        }) => {
            bail!("Only one template is allowed");
        }
        Cli::Generate(Generate::FailureReport {
            xcresult_path,
            template:
                Template {
                    builtin: Some(template),
                    custom: None,
                },
            output,
        }) => {
            let params = xra_core::generate_failure_report(&xcresult_path)?;
            let reg = Handlebars::new();
            let content = template.template();
            let content = reg.render_template(&content, &params)?;
            fs::write(output, content)?;
        }
        Cli::Generate(Generate::FailureReport {
            xcresult_path,
            template:
                Template {
                    builtin: None,
                    custom: Some(path),
                },
            output,
        }) => {
            todo!("Support custom templates");
        }
        Cli::Generate(Generate::FailureReport {
            xcresult_path,
            template:
                Template {
                    builtin: None,
                    custom: None,
                },
            output,
        }) => {
            bail!("Template is required");
        }
        Cli::Generate(Generate::Report {
            xcresult_path,
            template:
                Template {
                    builtin: Some(_),
                    custom: Some(_),
                },
            output,
        }) => {
            bail!("Only one template is allowed");
        }
        Cli::Generate(Generate::Report {
            xcresult_path,
            template:
                Template {
                    builtin: Some(template),
                    custom: None,
                },
            output,
        }) => {
            let params = xra_core::generate_tests_report(&xcresult_path)?;
            let mut reg = Handlebars::new();
            reg.register_helper("toEmoji", Box::new(hbs_helpers::result_to_emoji));
            let content = template.template();
            let content = reg.render_template(&content, &params)?;
            fs::write(output, content)?;
        }
        Cli::Generate(Generate::Report {
            xcresult_path,
            template:
                Template {
                    builtin: None,
                    custom: Some(path),
                },
            output,
        }) => {
            todo!("Support custom templates");
        }
        Cli::Generate(Generate::Report {
            xcresult_path,
            template:
                Template {
                    builtin: None,
                    custom: None,
                },
            output,
        }) => {
            todo!("Support custom templates");
        }
        Cli::Get(Get::Summary { xcresult_path }) => {
            xra_core::get_summary(&xcresult_path)?;
        }
        Cli::Get(Get::FailureReport { xcresult_path }) => {
            xra_core::get_failure_report(&xcresult_path)?;
        }
        Cli::Get(Get::Report { xcresult_path }) => {
            xra_core::get_tests_report(&xcresult_path)?;
        }
    }
    Ok(())
}
