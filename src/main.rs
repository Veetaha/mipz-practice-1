mod calc;

use clap::Parser;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    data_path: PathBuf,
}

impl Args {
    pub(crate) fn into_inputs(self) -> Result<Vec<crate::calc::Input>> {
        let data = fs::read_to_string(self.data_path)?;
        Ok(serde_json::from_str(&data)?)
    }
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let inputs = args.into_inputs()?;

    for input in inputs {
        let mut outputs = calc::euro_diffusion(&input.countries);

        // Make the outputs ordered by iteration count and country name within them
        outputs.sort_by(|a, b| {
            if a.iter == b.iter {
                return a.country_name.cmp(&b.country_name)
            }
            a.iter.cmp(&b.iter)
        });

        eprintln!("Results for input #{}", input.id);

        for output in outputs {
            eprintln!("{} {}", output.country_name, output.iter);
        }
    }

    Ok(())
}
