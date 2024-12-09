mod client;
pub mod prelude;
mod puzzle;

pub use puzzle::Puzzle;

use clap::Parser;
use client::DownloadCommand;
use client::SubmitCommand;
use puzzle::PuzzleCommand;

#[derive(Parser, Debug, Clone)]
pub struct RootOpt {
	/// Year to run (default: 2024)
	#[arg(short, long, default_value_t = 2024)]
	pub year: u16,

	/// Day to run
	#[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=31))]
	pub day: u8,

	/// Part to run
	#[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..=2))]
	pub part: u8,

	/// Read data from stdin instead of file
	#[arg(long)]
	pub data: bool,

	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Commands {
	Puzzle(puzzle::PuzzleCommand),
	Download(DownloadCommand),
	Submit(SubmitCommand),
}

impl RootOpt {
	pub fn run(&self) -> Result<(), anyhow::Error> {
		log::info!("Running day {} part {}", self.day, self.part);

		if let Some(cmd) = &self.command {
			return cmd.run(self);
		}

		PuzzleCommand::default().run(self)?;

		Ok(())
	}
}

impl Commands {
	pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
		match self {
			Commands::Download(cmd) => cmd.run(opt),
			Commands::Submit(cmd) => cmd.run(opt),
			Commands::Puzzle(cmd) => cmd.run(opt),
		}
	}
}
