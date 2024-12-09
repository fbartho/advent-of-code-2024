use advent2024::RootOpt;
use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
	env_logger::init();
	let opt = RootOpt::parse();
	opt.run()
}
