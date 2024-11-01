use std::path::PathBuf;
use clap::{Parser, Subcommand};

use crate::templates::{FailureResultTemplate, ResultTemplate};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub enum Cli {
    #[command(subcommand)]
    Generate(Generate),
    #[command(subcommand)]
    Get(Get)
}

#[derive(Subcommand)]
pub enum Generate {
    /// Generates xcresult sqlite3 database
    Database {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf
    },
    /// Generates failed tests report
    FailureReport {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf,
        /// Template
        #[clap(short = 't', long = "template")]
        template: FailureResultTemplate,
        /// Path to output file
        #[clap(short = 'o', long = "output")]
        output: PathBuf,
    },
    /// Generates tests report
    Report {
       /// Path to the xcresult file
       #[clap(short = 'p', long = "path")]
       xcresult_path: PathBuf,
        /// Template
       #[clap(short = 't', long = "template")]
       template: ResultTemplate,
        /// Path to output file
       #[clap(short = 'o', long = "output")]
       output: PathBuf,
    }
}

#[derive(Subcommand)]
pub enum Get {
    /// Get tests result summary
    Summary {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf
    },
    /// Get failed tests result report
    FailureReport {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf
    },
    /// Get tests result report
    Report {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf
    }
}
