use std::path::PathBuf;
use std::fs;
use anyhow::Result;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    data_path: PathBuf,
}

impl Args {
    pub fn into_inputs(self) -> Result<Vec<crate::calc::Input>> {
        let data = fs::read_to_string(self.data_path)?;
        Ok(serde_json::from_str(&data)?)
    }
}
