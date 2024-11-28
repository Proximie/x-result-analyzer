use crate::templates::FailureResultTemplate;
use crate::templates::ResultTemplate;
use clap::Parser;
use clap::Subcommand;
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Cli {
    #[command(subcommand)]
    Generate(Generate),
    #[command(subcommand)]
    Get(Get),
}

#[derive(Subcommand)]
pub enum Generate {
    /// Generates xcresult sqlite3 database
    Database {
        /// Path to the xcresult file
        #[clap(long = "path")]
        xcresult_path: PathBuf,
    },
    /// Generates failed tests report
    FailureReport {
        /// Path to the xcresult file
        #[clap(long = "path")]
        xcresult_path: PathBuf,
        /// Template
        #[command(flatten)]
        template: Template<FailureResultTemplate>,
        /// Path to output file
        #[clap(long = "output")]
        output: PathBuf,
    },
    /// Generates tests report
    Report {
        /// Path to the xcresult file
        #[clap(long = "path")]
        xcresult_path: PathBuf,
        /// Template
        #[command(flatten)]
        template: Template<ResultTemplate>,
        /// Path to output file
        #[clap(long = "output")]
        output: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum Get {
    /// Get tests result summary
    Summary {
        /// Path to the xcresult file
        #[clap(long = "path")]
        xcresult_path: PathBuf,
    },
    /// Get failed tests result report
    FailureReport {
        /// Path to the xcresult file
        #[clap(long = "path")]
        xcresult_path: PathBuf,
    },
    /// Get tests result report
    Report {
        /// Path to the xcresult file
        #[clap(long = "path")]
        xcresult_path: PathBuf,
    },
}

#[derive(clap::Args, Debug)]
pub struct Template<T>
where
    T: Clone + Debug + Send + Sync + clap::ValueEnum + 'static,
{
    /// Path to the file
    #[arg(long = "template-path", conflicts_with = "template")]
    pub custom: Option<PathBuf>,

    /// Name of the built-in template
    #[arg(long = "template", conflicts_with = "template-path")]
    pub builtin: Option<T>,
}
