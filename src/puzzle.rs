mod day_00; // Template file. Not used, but imported so it will get checked for errors.
macros::import_solutions!(); // Import the rest of the solution files

use std::io::Read;

use aoc_client::SubmissionOutcome;
use clap::Parser;
use macros::get_solution;

use crate::{client::Client, RootOpt};

pub type PuzzleResult = Result<String, anyhow::Error>;

pub trait Puzzle {
	fn new(ops: &RootOpt) -> Box<dyn Puzzle>
	where
		Self: Sized;

	fn part_one(&self, _input: &str) -> PuzzleResult;
	fn part_two(&self, _input: &str) -> PuzzleResult;
}

#[derive(Clone, Debug, Parser, Default)]
pub struct PuzzleCommand {
	/// Submit the result and update the data files
	#[arg(long)]
	submit: bool,
}

impl PuzzleCommand {
	pub fn run(&self, opt: &RootOpt) -> Result<(), anyhow::Error> {
		let client = crate::client::Client::new(opt)?;
		let data = if opt.data {
			let mut data = String::new();
			std::io::stdin().read_to_string(&mut data)?;
			data
		} else {
			client.get_input(opt.day)?
		};

		let day = get_solution!(opt);
		let solution = match opt.part {
			1 => day.part_one(&data)?,
			2 => day.part_two(&data)?,
			_ => todo!("Implement part three"),
		};

		println!("Solution: {}", solution);

		if self.submit {
			let client = Client::new(opt)?;
			let res = client.client.submit_answer(opt.part as i64, solution)?;
			println!("{:?}", res);

			if matches!(res, SubmissionOutcome::Correct) {
				println!("Downloading puzzle update");
				client.clear()?;
				client.download()?;
			}
		}

		Ok(())
	}
}
