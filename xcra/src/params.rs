use std::path::PathBuf;
use clap::{Parser, Subcommand};

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
    Db {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf
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
    /// Get tests result report
    Report {
        /// Path to the xcresult file
        #[clap(short = 'p', long = "path")]
        xcresult_path: PathBuf
    }
}
