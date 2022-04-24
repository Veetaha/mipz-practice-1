use euro_diffusion::{cli, calc};
use clap::Parser;


fn main() -> anyhow::Result<()> {
    let args = cli::Args::parse();

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
