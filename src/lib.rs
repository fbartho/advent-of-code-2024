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
/// (vertical, horizontal) to be consistent with GridCoord2
#[derive(Clone, Copy, Debug, Default)]
pub struct GridDistance2<T>(T, T);
impl GridDistance2<i32> {
	pub fn normalize(&self) -> GridDistance2<i32>
// where
	// 	T: Sized
	// 		+ From<T>
	// 		+ From<std::num::NonZero<T>>
	// 		+ Copy
	// 		+ std::ops::Div<Output = T>
	// 		+ std::ops::Rem<Output = T>
	// 		+ std::cmp::PartialOrd,
	{
		if self.0 <= self.1 {
			if self.1 % self.0 == 0 {
				return GridDistance2(self.0 / self.0, self.1 / self.0);
			} else {
				return *self;
			}
		} else {
			if self.0 % self.1 == 0 {
				return GridDistance2(self.0 / self.1, self.1 / self.1);
			}
			return *self;
		}
	}
}
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
impl<CellType> FBGrid<CellType>
where
	CellType: Default + PartialEq,
{
	pub fn distance_generic<T>(a: &GridCoord2, b: &GridCoord2) -> GridDistance2<T>
	where
		T: From<usize> + std::ops::Sub<Output = T>,
	{
		// Small is upwards, and to the left
		let a0: T = a.0.into();
		let a1: T = a.1.into();
		let b0: T = b.0.into();
		let b1: T = b.1.into();
		return GridDistance2(b0 - a0, b1 - a1);
	}
	pub fn distance(&self, a: &GridCoord2, b: &GridCoord2) -> GridDistance2<i32>
// where
	// 	T: From<usize> + std::ops::Sub<Output = T>,
	{
		// Small is upwards, and to the left
		let a0: i32 = a.0 as i32;
		let a1: i32 = a.1 as i32;
		let b0: i32 = b.0 as i32;
		let b1: i32 = b.1 as i32;
		return GridDistance2(b0 - a0, b1 - a1);
	}

	pub fn travel<T>(&self, coord: GridCoord2, distance: GridDistance2<T>) -> Option<GridCoord2>
	where
		T: From<usize> + Into<usize> + std::ops::Add<Output = T> + std::cmp::PartialOrd<T>,
	{
		let coord0: T = coord.0.into();
		let coord1: T = coord.1.into();
		let (row, col) = (coord0 + distance.0, coord1 + distance.1);
		if row < 0.into()
			|| col < 0.into()
			|| row >= self.grid.rows().into()
			|| col >= self.grid.cols().into()
		{
			return None;
		}

		return Some((row.into(), col.into()));
	}
}
