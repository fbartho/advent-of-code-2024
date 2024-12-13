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

#[allow(unused_imports)]
use strum::IntoEnumIterator;
/// (y, x) to match with grid crate
pub type GridCoord2 = (usize, usize);
#[derive(
	Debug,
	Clone,
	Copy,
	PartialEq,
	strum_macros::EnumString,
	strum_macros::Display,
	strum_macros::EnumIter,
)]
pub enum TravelDirection {
	N,
	NE,
	E,
	SE,
	S,
	SW,
	W,
	NW,
}
impl TravelDirection {
	pub fn all() -> Vec<TravelDirection> {
		TravelDirection::iter().collect()
	}
	pub fn cardinal() -> Vec<TravelDirection> {
		const CARDINAL_PLAN: [TravelDirection; 4] = [
			TravelDirection::N,
			TravelDirection::E,
			TravelDirection::S,
			TravelDirection::W,
		];
		Vec::from(CARDINAL_PLAN)
	}
	pub fn ordinal() -> Vec<TravelDirection> {
		const ORDINAL_PLAN: [TravelDirection; 4] = [
			TravelDirection::NE,
			TravelDirection::SE,
			TravelDirection::SW,
			TravelDirection::NW,
		];
		Vec::from(ORDINAL_PLAN)
	}
	pub fn rt90(&self) -> TravelDirection {
		match self {
			TravelDirection::N => TravelDirection::E,
			TravelDirection::E => TravelDirection::S,
			TravelDirection::S => TravelDirection::W,
			TravelDirection::W => TravelDirection::N,

			TravelDirection::NE => TravelDirection::SE,
			TravelDirection::SE => TravelDirection::SW,
			TravelDirection::SW => TravelDirection::NW,
			TravelDirection::NW => TravelDirection::NE,
		}
	}

	pub fn next_coord(&self, coord: GridCoord2) -> Option<GridCoord2> {
		self.next_coord_with_dist(coord, 1)
	}
	pub fn next_coord_with_dist(&self, (y, x): GridCoord2, distance: usize) -> Option<GridCoord2> {
		Some(match self {
			Self::N => (y.checked_sub(distance)?, x),
			Self::NE => (y.checked_sub(distance)?, x + distance),
			Self::E => (y, x + distance),
			Self::SE => (y + distance, x + distance),
			Self::S => (y + distance, x),
			Self::SW => (y + distance, x.checked_sub(distance)?),
			Self::W => (y, x.checked_sub(distance)?),
			Self::NW => (y.checked_sub(distance)?, x.checked_sub(distance)?),
		})
	}
}

use grid::Grid;

pub fn grid_from_vec_vec<I, O>(data: Vec<Vec<I>>) -> Grid<O>
where
	I: Into<O> + Clone,
	O: Default + From<I> + Sized,
{
	let mut grid = Grid::new(data.len(), data[0].len());
	for (y, row) in data.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			grid[(y, x)] = O::from(cell.clone());
		}
	}
	grid
}

#[derive(Clone)]
pub struct FBGrid<T>
where
	T: Default + PartialEq,
{
	grid: Grid<T>,
}
// impl<T> FBGrid<T> {
// }
