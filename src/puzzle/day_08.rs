use crate::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Day08;

impl Puzzle for Day08 {
	fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
		Box::new(Self)
	}

	fn part_one(&self, _input: &str) -> super::PuzzleResult {
		let radios = RadioMap::new(_input, PuzzlePart::P1);
		println!("{}", radios.to_string());
		return Ok(radios.antinode_locations.len().to_string());
	}

	fn part_two(&self, _input: &str) -> super::PuzzleResult {
		let radios = RadioMap::new(_input, PuzzlePart::P2);
		println!("{}", radios.to_string());
		return Ok(radios.antinode_locations.len().to_string());
	}
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum CellConfigValue {
	#[default]
	Unknown,
	Antenna(char),
	Obstacle,
}
impl std::str::FromStr for CellConfigValue {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"" => Ok(CellConfigValue::Unknown),
			"." => Ok(CellConfigValue::Unknown),
			_ => Ok(CellConfigValue::Antenna(s.chars().nth(0).unwrap())),
		}
	}
}
impl CellConfigValue {
	fn to_string(&self) -> String {
		match self {
			CellConfigValue::Unknown => String::from("."),
			CellConfigValue::Antenna(c) => c.to_string(),
			CellConfigValue::Obstacle => String::from("#"),
		}
	}
}

fn parse_map(input: &str) -> FBGrid<CellConfigValue> {
	let parsed = input
		.trim()
		.split("\n")
		.map(|line| {
			line.chars()
				.filter_map(|c| CellConfigValue::from_str(&c.to_string()).ok())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();
	return FBGrid {
		grid: grid_from_vec_vec(parsed),
	};
}
impl FBGrid<CellConfigValue> {
	fn to_string(&self) -> String {
		// let gs = self.grid.to_string()
		let mut result = String::new();
		for row in self.grid.iter_rows() {
			for cell in row {
				result.push_str(&cell.to_string());
			}
			result.push('\n');
		}
		return result;
	}
	fn make_antinode_left(
		&self,
		a: &GridCoord2,
		_b: &GridCoord2,
		dist: GridDistance2<i32>,
	) -> Option<GridCoord2> {
		let r0 = (a.0 as i32).checked_sub(dist.0)?;
		let r1 = (a.1 as i32).checked_sub(dist.1)?;
		if r0 < 0 || r1 < 0 {
			return None;
		}
		if r0 as usize > self.grid.rows() || r1 as usize > self.grid.cols() {
			return None;
		}
		return Some((r0 as usize, r1 as usize));
	}
	fn make_antinode_right(
		&self,
		_a: &GridCoord2,
		b: &GridCoord2,
		dist: GridDistance2<i32>,
	) -> Option<GridCoord2> {
		let r0 = (b.0 as i32) + (dist.0);
		let r1 = (b.1 as i32) + (dist.1);
		if r0 < 0 || r1 < 0 {
			return None;
		}
		if r0 as usize > self.grid.rows() || r1 as usize > self.grid.cols() {
			return None;
		}
		return Some((r0 as usize, r1 as usize));
	}
}
struct RadioMap {
	grid: FBGrid<CellConfigValue>,
	// antenna_locations: HashMap<char, Vec<GridCoord2>>,
	antinode_locations: Vec<GridCoord2>,
}
enum PuzzlePart {
	P1,
	P2,
}
impl RadioMap {
	fn new(input: &str, part: PuzzlePart) -> Self {
		let grid = parse_map(input);
		let mut antenna_locations: HashMap<char, Vec<GridCoord2>> = HashMap::new();
		let mut antinode_locations: HashSet<GridCoord2> = HashSet::new();

		grid.grid.indexed_iter().for_each(|(loc, val)| match val {
			CellConfigValue::Antenna(c) => {
				if !antenna_locations.contains_key(c) {
					antenna_locations.insert(*c, Vec::new());
				}
				antenna_locations.get_mut(c).unwrap().push(loc);
			}
			_ => {}
		});
		match part {
			PuzzlePart::P1 => {
				antenna_locations
					.values()
					.map(|locations| {
						return locations
							.iter()
							.combinations(2)
							.map(|combo| {
								let dist = grid.distance(combo[0], combo[1]);
								vec![
									grid.make_antinode_left(combo[0], combo[1], dist),
									grid.make_antinode_right(combo[0], combo[1], dist),
								]
							})
							.flatten()
							.filter_map(|n| {
								if let Some(coord) = n {
									if let Some(_) = grid.grid.get(coord.0, coord.1) {
										return n;
									}
								}
								return None;
							})
							.collect::<Vec<GridCoord2>>();
					})
					.flatten()
					.for_each(|loc| {
						antinode_locations.insert(loc);
					});
			}
			PuzzlePart::P2 => {
				antenna_locations
					.values()
					.map(|locations| {
						return locations
							.iter()
							.combinations(2)
							.map(|combo| {
								let dist = grid.distance(combo[0], combo[1]).normalize();
								let mut curr = *combo[0];
								let mut batch: Vec<GridCoord2> = Vec::new();

								// Include the starting node in the batch as make_antinode_left and make_antinode_right would skip it
								batch.push(curr);
								while let Some(next) =
									grid.make_antinode_left(&curr, combo[1], dist)
								{
									batch.push(next);
									curr = next;
								}

								// Go to the right from the left starting point to catch points between the antennas
								curr = *combo[0];
								while let Some(next) =
									grid.make_antinode_right(combo[0], &curr, dist)
								{
									batch.push(next);
									curr = next;
								}

								return batch;
							})
							.flatten()
							.filter_map(|n| {
								if let Some(_) = grid.grid.get(n.0, n.1) {
									return Some(n);
								}
								return None;
							})
							.collect::<Vec<GridCoord2>>();
					})
					.flatten()
					.for_each(|loc| {
						antinode_locations.insert(loc);
					});
			}
		}

		return Self {
			grid,
			// antenna_locations,
			antinode_locations: antinode_locations.iter().map(|n| *n).collect::<Vec<_>>(),
		};
	}
	fn to_string(&self) -> String {
		let mut visual = self.grid.clone();
		self.antinode_locations.iter().for_each(|(r, c)| {
			if let Some(cell) = visual.grid.get_mut(*r, *c) {
				*cell = CellConfigValue::Obstacle;
			}
		});
		return format!("{}\n--\n{}", self.grid.to_string(), visual.to_string());
	}
}

#[test]
fn sample_day08_1() {
	let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
	println!("{:?}", Day08.part_one(input).unwrap());
}

#[test]
fn sample_day08_2() {
	let input = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"#;
	println!("{:?}", Day08.part_two(input).unwrap());
}
