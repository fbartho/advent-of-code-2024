use crate::prelude::*;

use grid::Grid;
#[allow(unused_imports)]
use std::str::FromStr;
#[allow(unused_imports)]
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};
pub struct Day06;

impl Puzzle for Day06 {
	fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
		Box::new(Self)
	}

	fn part_one(&self, _input: &str) -> super::PuzzleResult {
		let the_map = FBGrid::from_str(_input);

		println!("{}", the_map.to_string());

		let path_to_exit = the_map.exit_map();
		println!("{}", path_to_exit.to_string());

		Ok(path_to_exit
			.find_iter(CellValue::Visited)
			.count()
			.to_string())
	}

	fn part_two(&self, _input: &str) -> super::PuzzleResult {
		todo!("implement part two")
	}
}

/// (y, x) to match with grid crate
type GridCoord2 = (usize, usize);

fn grid_from_vec_vec<I, O>(data: Vec<Vec<I>>) -> Grid<O>
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

#[derive(Debug, Clone, Copy, PartialEq, EnumString, Display, EnumIter)]
enum TravelDirection {
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
	// fn all() -> Vec<TravelDirection> {
	// 	TravelDirection::iter().collect()
	// }
	// fn cardinal() -> Vec<TravelDirection> {
	// 	const CARDINAL_PLAN: [TravelDirection; 4] = [
	// 		TravelDirection::N,
	// 		TravelDirection::E,
	// 		TravelDirection::S,
	// 		TravelDirection::W,
	// 	];
	// 	Vec::from(CARDINAL_PLAN)
	// }
	fn rt90(&self) -> TravelDirection {
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

	fn next_coord(&self, coord: GridCoord2) -> Option<GridCoord2> {
		self.next_coord_with_dist(coord, 1)
	}
	fn next_coord_with_dist(&self, (y, x): GridCoord2, distance: usize) -> Option<GridCoord2> {
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

#[derive(Clone)]
struct FBGrid<T>
where
	T: Default + PartialEq,
{
	grid: Grid<T>,
}

impl<T> FBGrid<T>
where
	T: Default + PartialEq,
{
	fn find_iter(&self, needle: T) -> impl Iterator<Item = ((usize, usize), &T)> {
		self.grid
			.indexed_iter()
			.filter(move |(_, val)| **val == needle)
	}
}
#[derive(Debug, Copy, Clone, PartialEq, EnumString, Display, Default)]
enum SerializedCellValue {
	#[strum(serialize = ".")]
	#[default]
	Unknown,

	#[strum(serialize = "X")]
	Visited,
	#[strum(serialize = "#")]
	Obstacle,
	#[strum(serialize = "^")]
	PlayerFacingNorth,
	#[strum(serialize = ">")]
	PlayerFacingEast,
	#[strum(serialize = "<")]
	PlayerFacingWest,
	#[strum(serialize = "v")]
	PlayerFacingSouth,
}

#[derive(Debug, Copy, Clone, PartialEq, Display, Default)]
enum CellValue {
	#[default]
	Unknown,
	Visited,
	Obstacle,
	PlayerFacing(TravelDirection),
}
impl From<SerializedCellValue> for CellValue {
	fn from(value: SerializedCellValue) -> Self {
		match value {
			SerializedCellValue::Unknown => CellValue::Unknown,
			SerializedCellValue::Visited => CellValue::Visited,
			SerializedCellValue::Obstacle => CellValue::Obstacle,
			SerializedCellValue::PlayerFacingNorth => CellValue::PlayerFacing(TravelDirection::N),
			SerializedCellValue::PlayerFacingEast => CellValue::PlayerFacing(TravelDirection::E),
			SerializedCellValue::PlayerFacingWest => CellValue::PlayerFacing(TravelDirection::S),
			SerializedCellValue::PlayerFacingSouth => CellValue::PlayerFacing(TravelDirection::W),
		}
	}
}
impl From<CellValue> for SerializedCellValue {
	fn from(value: CellValue) -> Self {
		match value {
			CellValue::Unknown => SerializedCellValue::Unknown,
			CellValue::Visited => SerializedCellValue::Visited,
			CellValue::Obstacle => SerializedCellValue::Obstacle,
			CellValue::PlayerFacing(dir) => match dir {
				TravelDirection::N => SerializedCellValue::PlayerFacingNorth,
				TravelDirection::E => SerializedCellValue::PlayerFacingEast,
				TravelDirection::S => SerializedCellValue::PlayerFacingWest,
				TravelDirection::W => SerializedCellValue::PlayerFacingSouth,
				_ => panic!("Unimplemented player direction"),
			},
		}
	}
}
impl FBGrid<CellValue> {
	// fn new(grid: Grid<CellValue>) -> Self {
	//     Self { grid }
	// }
	fn from_str(input: &str) -> Self {
		let g_data = input
			.trim()
			.lines()
			.map(|l| {
				l.chars()
					.map(|c| SerializedCellValue::from_str(&c.to_string()).unwrap())
					.map(|c| c.into())
					.collect::<Vec<CellValue>>()
			})
			.collect::<Vec<Vec<CellValue>>>();
		let grid: Grid<CellValue> = grid_from_vec_vec(g_data);
		Self { grid }
	}
	fn find_start(&self) -> Option<(GridCoord2, TravelDirection)> {
		if let Some((loc, dir_source)) = self.grid.indexed_iter().find(|(_, val)| {
			if let CellValue::PlayerFacing(_) = val {
				return true;
			}
			return false;
		}) {
			if let CellValue::PlayerFacing(dir) = *dir_source {
				return Some((loc, dir));
			}
		}
		return None;
	}
	/// Create a new map where we've marked all the paths we'd take to exit the maze
	fn exit_map(&self) -> Self {
		let start = self.find_start().expect("No starting location found");
		let mut result = self.clone();
		let mut current = start;
		while let Some(next_loc) = current.1.next_coord(current.0) {
			// println!(
			// 	"{:?} {}:\n {}",
			// 	current.0,
			// 	current.1.to_string(),
			// 	result.to_string()
			// );
			// println!("{}", result.to_string());

			result.grid[current.0] = CellValue::Visited;
			if let Some(next_cell) = self.grid.get(next_loc.0, next_loc.1) {
				match next_cell {
					CellValue::Obstacle => {
						let next_dir = current.1.rt90();
						current.1 = next_dir
					}
					// Just advance forwards if it's not an obstacle
					_ => current.0 = next_loc,
				}
			} else {
				// We found an edge! (E, S)
				break;
			}
		}
		// When you exit off the top/left, we don't enter the loop and mark the cell as visited
		result.grid[current.0] = CellValue::Visited;

		println!("last: {:?}", current);
		return result;
	}
	fn to_string(&self) -> String {
		// let gs = self.grid.to_string()
		let mut result = String::new();
		for row in self.grid.iter_rows() {
			for cell in row {
				result.push_str(&SerializedCellValue::from(*cell).to_string());
			}
			result.push('\n');
		}
		return result;
	}
	// fn visualize(&self, path: &Vec<GridCoord2>) {
	// 	let mut visualized = self.clone();
	// 	path.iter()
	// 		.for_each(|coord| visualized.grid[*coord] = CellValue::Visited);

	// 	println!("{}", visualized.to_string());
	// }
}
#[test]
fn sample_day06_1() {
	let input = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;
	println!("{:?}", Day06.part_one(input).unwrap());
}
