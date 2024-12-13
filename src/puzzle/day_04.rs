use crate::prelude::*;
use grid::Grid;

pub struct Day04;

impl Puzzle for Day04 {
	fn new(_ops: &super::RootOpt) -> Box<dyn Puzzle> {
		Box::new(Self)
	}

	fn part_one(&self, _input: &str) -> super::PuzzleResult {
		let grid = FBGrid::from_str(_input);
		let paths = grid.valid_xmas_paths();

		grid.visualize(&paths);

		Ok(paths.len().to_string())
	}

	fn part_two(&self, _input: &str) -> super::PuzzleResult {
		let grid = FBGrid::from_str(_input);
		let paths = grid.valid_cross_plans();
		let flattened_paths = paths
			.iter()
			// .inspect(|el| dbg!(el))
			.map(|a_cross| {
				let tmp: Vec<DirectedPath> =
					a_cross.iter().map(|subpath| subpath.clone()).collect();
				return tmp;
			})
			.flatten()
			.collect();
		grid.visualize(&flattened_paths);

		Ok(paths.len().to_string())
	}
}
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

#[derive(Debug, Clone, PartialEq, EnumString, Display, Default)]
enum CellValue {
	#[strum(serialize = ".")]
	#[default]
	Unknown,
	X,
	M,
	A,
	S,
}
impl CellValue {
	fn search_seq_xmas() -> Vec<CellValue> {
		const NEEDLE: [CellValue; 4] = [CellValue::X, CellValue::M, CellValue::A, CellValue::S];
		NEEDLE.iter().map(|v| v.clone()).collect()
	}
}
/// (y, x) to match with grid crate
type GridCoord2 = (usize, usize);
#[derive(Clone)]
struct CrossCoordTray {
	top_left: GridCoord2,
	top_right: GridCoord2,
	mid: GridCoord2,
	bottom_left: GridCoord2,
	bottom_right: GridCoord2,
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
	// fn is_valid_coord(&self, (y, x): GridCoord2) -> bool {
	//     return (0..self.grid.rows()).contains(&y) && (0..self.grid.cols()).contains(&x);
	// }
}
type DirectedPath = (TravelDirection, Vec<GridCoord2>);
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
					.map(|c| CellValue::from_str(&c.to_string()).unwrap())
					.collect::<Vec<CellValue>>()
			})
			.collect::<Vec<Vec<CellValue>>>();
		let grid: Grid<CellValue> = grid_from_vec_vec(g_data);
		Self { grid }
	}
	fn valid_plans_for_directions<'a, I>(
		&self,
		origins: I,
		search_seq: Vec<CellValue>,
		travel_plan: Vec<TravelDirection>,
	) -> Vec<DirectedPath>
	where
		I: Iterator<Item = GridCoord2> + 'a,
	{
		origins
			.map(|start| {
				travel_plan
					.iter()
					.filter_map(|dir| {
						// let _ = dbg!(&dir);
						let mut path: Vec<GridCoord2> = Vec::new();
						let mut coords = std::iter::successors(Some(start), |c| dir.next_coord(*c));
						for target in search_seq.iter() {
							// if !self.is_valid_coord(current) {
							//     return None;
							// }
							if let Some(current) = coords.next() {
								if let Some(entry) = self.grid.get(current.0, current.1) {
									if target != entry {
										return None;
									}
									path.push(current);
								} else {
									// Out of bounds (too big)
									return None;
								}
							} else {
								// Negative out of bounds (numerical wrapping)
								return None;
							}
						}
						return Some((*dir, path));
					})
					.collect::<Vec<_>>()
			})
			.flatten()
			.collect::<Vec<_>>()
	}
	fn valid_xmas_paths(&self) -> Vec<DirectedPath> {
		self.valid_plans_for_directions(
			self.find_iter(CellValue::X).map(|(start, _)| start),
			CellValue::search_seq_xmas(),
			TravelDirection::all(),
		)
	}
	fn check_cross(
		&self,
		mid: GridCoord2,
		template: &Vec<CellValue>,
	) -> Option<(CrossCoordTray, bool)> {
		let top_left = TravelDirection::NW.next_coord_with_dist(mid, 1)?;
		let top_right = TravelDirection::NE.next_coord_with_dist(mid, 1)?;
		let bottom_left = TravelDirection::SW.next_coord_with_dist(mid, 1)?;
		let bottom_right = TravelDirection::SE.next_coord_with_dist(mid, 1)?;
		let coords = CrossCoordTray {
			top_left,
			top_right,
			mid,
			bottom_left,
			bottom_right,
		};

		let current_coords: Vec<GridCoord2> = vec![top_left, top_right, bottom_left, bottom_right];
		let current_vals: Vec<&CellValue> = current_coords
			.iter()
			.filter_map(|c| self.grid.get(c.0, c.1))
			.collect();
		if current_vals.len() != template.len() {
			return Some((coords, false));
		}
		return Some((
			coords,
			std::iter::zip(current_vals, template).all(|(a, b)| a == b),
		));
	}
	/// Vertically Stacked 'M's + to the right
	fn x_east(&self, mid: GridCoord2) -> Option<Vec<DirectedPath>> {
		let (coords, valid) = self.check_cross(
			mid,
			&vec![CellValue::M, CellValue::S, CellValue::M, CellValue::S],
		)?;
		if !valid {
			return None;
		}
		return Some(vec![
			(
				TravelDirection::SE,
				vec![coords.top_left, coords.mid, coords.bottom_right],
			),
			(
				TravelDirection::NE,
				vec![coords.bottom_left, coords.mid, coords.top_right],
			),
		]);
	}
	/// Vertically Stacked 'M's + to the left
	fn x_west(&self, mid: GridCoord2) -> Option<Vec<DirectedPath>> {
		let (coords, valid) = self.check_cross(
			mid,
			&vec![CellValue::S, CellValue::M, CellValue::S, CellValue::M],
		)?;
		if !valid {
			return None;
		}
		return Some(vec![
			(
				TravelDirection::SW,
				vec![coords.top_right, coords.mid, coords.bottom_left],
			),
			(
				TravelDirection::NW,
				vec![coords.bottom_right, coords.mid, coords.top_left],
			),
		]);
	}
	/// Horizontal 'M's + going down
	fn x_south(&self, mid: GridCoord2) -> Option<Vec<DirectedPath>> {
		let (coords, valid) = self.check_cross(
			mid,
			&vec![CellValue::M, CellValue::M, CellValue::S, CellValue::S],
		)?;
		if !valid {
			return None;
		}
		return Some(vec![
			(
				TravelDirection::SE,
				vec![coords.top_left, coords.mid, coords.bottom_right],
			),
			(
				TravelDirection::SW,
				vec![coords.top_right, coords.mid, coords.bottom_left],
			),
		]);
	}
	/// Horizontal 'M's + going up
	fn x_north(&self, mid: GridCoord2) -> Option<Vec<DirectedPath>> {
		let (coords, valid) = self.check_cross(
			mid,
			&vec![CellValue::S, CellValue::S, CellValue::M, CellValue::M],
		)?;
		if !valid {
			return None;
		}
		return Some(vec![
			(
				TravelDirection::NE,
				vec![coords.bottom_left, coords.mid, coords.top_right],
			),
			(
				TravelDirection::NW,
				vec![coords.bottom_right, coords.mid, coords.top_left],
			),
		]);
	}
	fn valid_cross_plans(&self) -> Vec<Vec<DirectedPath>> {
		let mid_points = self.find_iter(CellValue::A).map(|(start, _)| start);

		let cross_plans = mid_points
			.map(|mid| {
				let tmp: Vec<Vec<DirectedPath>> = vec![
					self.x_east(mid),
					self.x_west(mid),
					self.x_north(mid),
					self.x_south(mid),
				]
				.iter()
				.filter_map(|o| {
					o.clone().map(|entry| {
						if entry.is_empty() {
							return None;
						}
						return Some(entry);
					})
				})
				.filter_map(|o| o)
				.collect();
				return tmp;
			})
			.flatten()
			.collect();
		return cross_plans;
	}
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
	fn visualize(&self, paths: &Vec<DirectedPath>) {
		let mut mask = self.clone();
		paths
			.iter()
			.map(|(_, path)| path)
			.flatten()
			.for_each(|coord| mask.grid[*coord] = CellValue::Unknown);

		let mut visualized = self.clone();
		mask.grid
			.indexed_iter()
			.filter(|(_, val)| CellValue::Unknown != **val)
			.for_each(|(coord, _)| visualized.grid[coord] = CellValue::Unknown);

		println!("{}", visualized.to_string());
	}
}
#[test]
fn sample_day04_1() {
	let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
	// println!("{:?}", grid_from(input));
	println!("{:?}", Day04.part_one(input).unwrap());
}

#[test]
fn sample_day04_2() {
	let input = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;
	// println!("{:?}", grid_from(input));
	println!("{:?}", Day04.part_two(input).unwrap());
}
