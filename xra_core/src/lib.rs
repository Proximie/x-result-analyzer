#[macro_use]
mod macros;
pub mod dba;
pub mod error;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::{fs, io};

use dba::Dba;

pub fn generate_xcresult_db(xcresult_path: &PathBuf) -> Result<(), error::XcraError> {
    if !fs::exists(xcresult_path)? {
        return Err(error::XcraError::XcresultNotFound);
    }

    Command::new("xcrun")
        .arg("xcresulttool")
        .arg("get")
        .arg("test-results")
        .arg("summary")
        .arg("--path")
        .arg(xcresult_path)
        .output()?;
    Ok(())
}

pub fn generate_failure_report(
    xcresult_path: &PathBuf,
) -> Result<serde_json::Value, error::XcraError> {
    if !fs::exists(pathbuf![xcresult_path, "database.sqlite3"])? {
        generate_xcresult_db(xcresult_path)?;
    }

    let dba = Dba::new(&xcresult_path.display().to_string())?;
    let results = dba.get_failed_test_results()?;

    Ok(serde_json::to_value(&results)?)
}

pub fn generate_tests_report(
    xcresult_path: &PathBuf,
) -> Result<serde_json::Value, error::XcraError> {
    if !fs::exists(pathbuf![xcresult_path, "database.sqlite3"])? {
        generate_xcresult_db(xcresult_path)?;
    }

    let dba = Dba::new(&xcresult_path.display().to_string())?;
    let results: dba::TestResults = dba.get_test_results()?;

    Ok(serde_json::to_value(&results)?)
}

pub fn get_summary(xcresult_path: &PathBuf) -> Result<(), error::XcraError> {
    if !fs::exists(xcresult_path)? {
        return Err(error::XcraError::XcresultNotFound);
    }

    let output = Command::new("xcrun")
        .arg("xcresulttool")
        .arg("get")
        .arg("test-results")
        .arg("summary")
        .arg("--path")
        .arg(xcresult_path)
        .output()?;
    io::stdout().write_all(&output.stdout)?;
    Ok(())
}

pub fn get_failure_report(xcresult_path: &PathBuf) -> Result<(), error::XcraError> {
    if !fs::exists(pathbuf![xcresult_path, "database.sqlite3"])? {
        generate_xcresult_db(xcresult_path)?;
    }

    let dba = Dba::new(&xcresult_path.display().to_string())?;
    let results = dba.get_failed_test_results()?;
    println!("{:#?}", results);
    Ok(())
}

pub fn get_tests_report(xcresult_path: &PathBuf) -> Result<(), error::XcraError> {
    if !fs::exists(pathbuf![xcresult_path, "database.sqlite3"])? {
        generate_xcresult_db(xcresult_path)?;
    }

    let dba = Dba::new(&xcresult_path.display().to_string())?;
    let results = dba.get_test_results()?;
    println!("{:#?}", results);
    Ok(())
}

pub fn log_github_action_annotation(
    file: &str,
    line: u32,
    col: Option<u32>,
    level: &str,
    message: &str,
) -> io::Result<()> {
    let mut log_file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Append to the file
        .open("error_logs.txt")?; // Open the log file (path is "error_logs.txt")

    match col {
        Some(col) => {
            writeln!(
                log_file,
                "::{} file={},line={},col={}::{}",
                level, file, line, col, message
            )?;
        }
        None => {
            writeln!(
                log_file,
                "::{} file={},line={}::{}",
                level, file, line, message
            )?;
        }
    }

    Ok(())
}
