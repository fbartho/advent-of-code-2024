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

		Ok(path_to_exit.find_any_visited().count().to_string())
	}

	fn part_two(&self, _input: &str) -> super::PuzzleResult {
		let the_map = FBGrid::from_str(_input);
		println!("{}", the_map.to_string());

		let (exit_map, path_to_exit) = the_map
			.exit_map_with_loops(true)
			.expect("Couldn't find an exit without modifications");
		println!("{}", exit_map.to_string());

		let start = path_to_exit[0].0;
		let mut unique_locations: std::collections::HashSet<GridCoord2> =
			std::collections::HashSet::new();
		let candidates = path_to_exit.iter().filter_map(|(loc, dir)| {
			let mut modified_map = the_map.clone();
			if let Some(next_loc) = dir.next_coord(*loc) {
				if unique_locations.contains(&next_loc) {
					return None;
				}
				unique_locations.insert(next_loc);
				if let Some(current) = exit_map.grid.get(next_loc.0, next_loc.1) {
					if next_loc != start && current.can_insert_obstacle(*dir) {
						modified_map.grid[next_loc] = CellValue::Obstacle;
						return Some((next_loc, modified_map));
					}
				}
			}
			return None;
		});

		let maps_that_loop: Vec<(GridCoord2, FBGrid<CellValue>)> = candidates
			.filter_map(|(loc, modified_map)| {
				if modified_map.exit_map_with_loops(false).is_none() {
					return Some((loc, modified_map));
				}
				return None;
			})
			.collect();

		Ok(maps_that_loop.len().to_string())
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

#[derive(Debug, Clone, PartialEq, EnumString, Display, Default)]
enum SerializedCellValue {
	#[strum(serialize = ".")]
	#[default]
	Unknown,

	#[strum(serialize = "{0}")]
	Visited(String),
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
	VisitedTraveling {
		n: bool,
		e: bool,
		s: bool,
		w: bool,
	},
	Obstacle,
	PlayerFacing(TravelDirection),
}
impl From<SerializedCellValue> for CellValue {
	fn from(value: SerializedCellValue) -> Self {
		match value {
			SerializedCellValue::Unknown => CellValue::Unknown,
			SerializedCellValue::Obstacle => CellValue::Obstacle,
			SerializedCellValue::PlayerFacingNorth => CellValue::PlayerFacing(TravelDirection::N),
			SerializedCellValue::PlayerFacingEast => CellValue::PlayerFacing(TravelDirection::E),
			SerializedCellValue::PlayerFacingWest => CellValue::PlayerFacing(TravelDirection::S),
			SerializedCellValue::PlayerFacingSouth => CellValue::PlayerFacing(TravelDirection::W),
			_ => panic!("Unimplemented parsing map for symbol {:?}", value),
		}
	}
}
impl From<CellValue> for SerializedCellValue {
	fn from(value: CellValue) -> Self {
		match value {
			CellValue::Unknown => SerializedCellValue::Unknown,
			CellValue::VisitedTraveling { n, e, s, w } => match (n, e, s, w) {
				// (true, true, true, true) => SerializedCellValue::Visited(String::from("✛")),
				// (true, true, true, false) => SerializedCellValue::Visited(String::from("├")),
				// (true, true, false, true) => SerializedCellValue::Visited(String::from("┴")),
				// (true, true, false, false) => SerializedCellValue::Visited(String::from("└")),
				// (true, false, true, true) => SerializedCellValue::Visited(String::from("┤")),
				// (true, false, true, false) => SerializedCellValue::Visited(String::from("↕")),
				// (true, false, false, true) => SerializedCellValue::Visited(String::from("┘")),
				// (true, false, false, false) => SerializedCellValue::Visited(String::from("↑")),
				// (false, true, true, true) => SerializedCellValue::Visited(String::from("┬")),
				// (false, true, true, false) => SerializedCellValue::Visited(String::from("┌")),
				// (false, true, false, true) => SerializedCellValue::Visited(String::from("↔")),
				// (false, true, false, false) => SerializedCellValue::Visited(String::from("→")),
				// (false, false, true, true) => SerializedCellValue::Visited(String::from("┐")),
				// (false, false, true, false) => SerializedCellValue::Visited(String::from("↓")),
				// (false, false, false, true) => SerializedCellValue::Visited(String::from("←")),
				(true, true, true, true) => SerializedCellValue::Visited(String::from("✛")),
				(true, true, true, false) => SerializedCellValue::Visited(String::from("✛")),
				(true, true, false, true) => SerializedCellValue::Visited(String::from("✛")),
				(true, true, false, false) => SerializedCellValue::Visited(String::from("✛")),
				(true, false, true, true) => SerializedCellValue::Visited(String::from("✛")),
				(true, false, true, false) => SerializedCellValue::Visited(String::from("↕")),
				(true, false, false, true) => SerializedCellValue::Visited(String::from("✛")),
				(true, false, false, false) => SerializedCellValue::Visited(String::from("↑")),
				(false, true, true, true) => SerializedCellValue::Visited(String::from("✛")),
				(false, true, true, false) => SerializedCellValue::Visited(String::from("✛")),
				(false, true, false, true) => SerializedCellValue::Visited(String::from("↔")),
				(false, true, false, false) => SerializedCellValue::Visited(String::from("→")),
				(false, false, true, true) => SerializedCellValue::Visited(String::from("✛")),
				(false, false, true, false) => SerializedCellValue::Visited(String::from("↓")),
				(false, false, false, true) => SerializedCellValue::Visited(String::from("←")),
				(false, false, false, false) => SerializedCellValue::Unknown,
			},
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
impl CellValue {
	/// We can only add an obstacle to Visited tiles in the exit map
	fn can_insert_obstacle(&self, dir: TravelDirection) -> bool {
		match self {
			CellValue::Unknown => panic!("Useless to add an obstacle off the path"),
			CellValue::VisitedTraveling { n, e, s, w } => match dir {
				TravelDirection::N => n == &(dir == TravelDirection::N),
				TravelDirection::E => e == &(dir == TravelDirection::E),
				TravelDirection::S => s == &(dir == TravelDirection::S),
				TravelDirection::W => w == &(dir == TravelDirection::W),
				_ => false,
			},
			CellValue::Obstacle => false,
			CellValue::PlayerFacing(_) => false,
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
	fn find_any_visited(&self) -> impl Iterator<Item = (GridCoord2, &CellValue)> {
		self.grid.indexed_iter().filter(move |(_, val)| match val {
			CellValue::VisitedTraveling { .. } => true,
			_ => false,
		})
	}
	fn visited_for(val: CellValue, dir: TravelDirection) -> Result<CellValue, ()> {
		let next_is_n = TravelDirection::N == dir;
		let next_is_e = TravelDirection::E == dir;
		let next_is_s = TravelDirection::S == dir;
		let next_is_w = TravelDirection::W == dir;

		match val {
			CellValue::VisitedTraveling { n, e, s, w } => {
				if n && next_is_n || e && next_is_e || s && next_is_s || w && next_is_w {
					// Already had marked a direction so this is a loop!
					return Result::Err(());
				}
				return Result::Ok(CellValue::VisitedTraveling {
					n: n || next_is_n,
					e: e || next_is_e,
					s: s || next_is_s,
					w: w || next_is_w,
				});
			}
			CellValue::Obstacle => panic!("Cannot go through an obstacle!"),
			_ => Result::Ok(CellValue::VisitedTraveling {
				n: TravelDirection::N == dir,
				e: TravelDirection::E == dir,
				s: TravelDirection::S == dir,
				w: TravelDirection::W == dir,
			}),
		}
	}
	/// Errs if it detects a loop!
	fn add_travel_direction_to_cell(
		&mut self,
		loc: GridCoord2,
		dir: TravelDirection,
	) -> Result<(), ()> {
		match FBGrid::visited_for(self.grid[loc], dir) {
			Ok(val) => {
				self.grid[loc] = val;
				return Result::Ok(());
			}
			Err(e) => Result::Err(e),
		}
	}
	/// Create a new map where we've marked all the paths we'd take to exit the maze
	/// -- Abort if we detect a loop
	fn exit_map_with_loops(
		&self,
		collect_path: bool,
	) -> Option<(Self, Vec<(GridCoord2, TravelDirection)>)> {
		// println!("{}", self.to_string());
		let start = self.find_start().expect("No starting location found");
		let mut result = self.clone();
		let mut current = start;
		let mut path: Vec<(GridCoord2, TravelDirection)> = Vec::new();
		if collect_path {
			path.push(current);
		}
		while let Some(next_loc) = current.1.next_coord(current.0) {
			// println!(
			// 	"{:?} {}:\n {}",
			// 	current.0,
			// 	current.1.to_string(),
			// 	result.to_string()
			// );
			// println!("{}", result.to_string());

			if let Result::Err(_e) = result.add_travel_direction_to_cell(current.0, current.1) {
				// Loop detected!
				return None;
			}
			if collect_path {
				path.push(current);
			}
			if let Some(next_cell) = self.grid.get(next_loc.0, next_loc.1) {
				match next_cell {
					CellValue::Obstacle => {
						let next_dir = current.1.rt90();
						current.1 = next_dir
					}
					_ => {
						// advance forwards if it's not an obstacle
						current.0 = next_loc
					}
				}
			} else {
				// We found an edge! (E, S)
				break;
			}
		}
		// When you exit off the top/left, we don't mark the cell as visited
		let _ = result.add_travel_direction_to_cell(current.0, current.1);
		if collect_path {
			path.push(current);
		}

		println!("last: {:?}", current);
		return Some((result, path));
	}
	fn exit_map(&self) -> Self {
		return self.exit_map_with_loops(false).expect("No exit found").0;
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

#[test]
fn sample_day06_2() {
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
	println!("{:?}", Day06.part_two(input).unwrap());
}
