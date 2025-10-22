use clap::Parser;
use real_estate_sim::prelude::*;
use std::fs::File;
use std::io::Write;


#[derive(Parser, Debug)]
#[command(author, version, about = "Multithreaded real-estate simulator in Rust", long_about = None)]
struct Args {
#[arg(short, long, help = "Path to JSON config file")]
config: String,
#[arg(short, long, help = "Where to write JSON results")]
out: Option<String>,
#[arg(long, help = "Number of Monte Carlo runs (overrides config)")]
runs: Option<usize>,
}


fn main() -> anyhow::Result<()> {
let args = Args::parse();
let cfg: sim::config::SimulationConfig = util::load_json(&args.config)?;
let runs = args.runs.unwrap_or(cfg.runs);


// Run scenarios in parallel with Rayon.
let results: Vec<sim::result::SimulationOutcome> = (0..runs).into_par_iter()
.map(|seed| sim::runner::run_scenario(&cfg, seed as u64))
.collect::<Result<_, _>>()?;


let summary = sim::result::aggregate(&results);


if let Some(path) = args.out {
let mut f = File::create(&path)?;
serde_json::to_writer_pretty(&mut f, &summary)?;
writeln!(f)?;
println!("Wrote results to {}", path);
} else {
println!("{}", serde_json::to_string_pretty(&summary)?);
}


Ok(())
}
