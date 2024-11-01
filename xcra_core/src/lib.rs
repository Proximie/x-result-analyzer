#[macro_use]
mod macros;
pub mod dba;
pub mod error;

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
    let results = dba.get_test_results()?;
    println!("{:#?}", results);
    Ok(())
}
