use crate::prelude::*;
#[allow(unused_imports)]
use std::str::FromStr;
#[allow(unused_imports)]
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, EnumString};

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
    // static
    fn search_seq_xmas() -> Vec<CellValue> {
        const NEEDLE: [CellValue; 4] = [CellValue::X, CellValue::M, CellValue::A, CellValue::S];
        NEEDLE.iter().map(|v| v.clone()).collect()
    }
    fn search_seq_mas() -> Vec<CellValue> {
        const NEEDLE: [CellValue; 3] = [CellValue::M, CellValue::A, CellValue::S];
        NEEDLE.iter().map(|v| v.clone()).collect()
    }
}
/// (y, x) to match with grid crate
type GridCoord2 = (usize, usize);
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
    fn is_valid_coord(&self, (y, x): GridCoord2) -> bool {
        return (0..self.grid.rows()).contains(&y) && (0..self.grid.cols()).contains(&x);
    }
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
    /// Vertically Stacked 'M's + to the right
    fn x_east(
        &self,
        head: GridCoord2,
        tail_coords: &std::collections::HashSet<GridCoord2>,
    ) -> Option<Vec<DirectedPath>> {
        let south_two = TravelDirection::S.next_coord_with_dist(head, 2)?;
        let mid = TravelDirection::E
            .next_coord_with_dist(TravelDirection::S.next_coord_with_dist(head, 1)?, 1)?;
        let east_two = TravelDirection::E.next_coord_with_dist(head, 2)?;
        let south_two_east_two = TravelDirection::E.next_coord_with_dist(south_two, 2)?;
        if self.grid.get(mid.0, mid.1).unwrap_or(&CellValue::Unknown) != &CellValue::A {
            return None;
        }
        if tail_coords.contains(&east_two) && tail_coords.contains(&south_two_east_two) {
            return Some(vec![
                (TravelDirection::SE, vec![head, mid, south_two_east_two]),
                (TravelDirection::NE, vec![south_two, mid, east_two]),
            ]);
        }
        return None;
    }
    /// Vertically Stacked 'M's + to the left
    fn x_west(
        &self,
        head: GridCoord2,
        tail_coords: &std::collections::HashSet<GridCoord2>,
    ) -> Option<Vec<DirectedPath>> {
        let south_two = TravelDirection::S.next_coord_with_dist(head, 2)?;
        let mid = TravelDirection::W
            .next_coord_with_dist(TravelDirection::S.next_coord_with_dist(head, 1)?, 1)?;
        let west_two = TravelDirection::W.next_coord_with_dist(head, 2)?;
        let south_two_west_two = TravelDirection::W.next_coord_with_dist(south_two, 2)?;

        if self.grid.get(mid.0, mid.1).unwrap_or(&CellValue::Unknown) != &CellValue::A {
            return None;
        }
        if tail_coords.contains(&west_two) && tail_coords.contains(&south_two_west_two) {
            return Some(vec![
                (TravelDirection::SW, vec![head, mid, south_two_west_two]),
                (TravelDirection::NW, vec![south_two, mid, west_two]),
            ]);
        }
        return None;
    }
    /// Horizontal 'M's + going down
    fn x_south(
        &self,
        head: GridCoord2,
        tail_coords: &std::collections::HashSet<GridCoord2>,
    ) -> Option<Vec<DirectedPath>> {
        let east_two = TravelDirection::E.next_coord_with_dist(head, 2)?;
        let mid = TravelDirection::S
            .next_coord_with_dist(TravelDirection::E.next_coord_with_dist(head, 1)?, 1)?;
        let south_two = TravelDirection::S.next_coord_with_dist(head, 2)?;
        let east_two_south_two = TravelDirection::S.next_coord_with_dist(east_two, 2)?;

        if self.grid.get(mid.0, mid.1).unwrap_or(&CellValue::Unknown) != &CellValue::A {
            return None;
        }
        if tail_coords.contains(&south_two) && tail_coords.contains(&east_two_south_two) {
            return Some(vec![
                (TravelDirection::SE, vec![head, mid, east_two_south_two]),
                (TravelDirection::SW, vec![east_two, mid, south_two]),
            ]);
        }
        return None;
    }
    /// Horizontal 'M's + going up
    fn x_north(
        &self,
        head: GridCoord2,
        tail_coords: &std::collections::HashSet<GridCoord2>,
    ) -> Option<Vec<DirectedPath>> {
        let east_two = TravelDirection::E.next_coord_with_dist(head, 2)?;
        let mid = TravelDirection::E
            .next_coord_with_dist(TravelDirection::N.next_coord_with_dist(head, 1)?, 1)?;
        let north_two = TravelDirection::N.next_coord_with_dist(head, 2)?;
        let east_two_north_two = TravelDirection::N.next_coord_with_dist(east_two, 2)?;
        if self.grid.get(mid.0, mid.1).unwrap_or(&CellValue::Unknown) != &CellValue::A {
            return None;
        }
        if tail_coords.contains(&north_two) && tail_coords.contains(&east_two_north_two) {
            return Some(vec![
                (TravelDirection::NE, vec![head, mid, east_two_north_two]),
                (TravelDirection::NW, vec![east_two, mid, north_two]),
            ]);
        }
        return None;
    }
    fn valid_cross_plans(&self) -> Vec<Vec<DirectedPath>> {
        let mas_paths = self.valid_plans_for_directions(
            self.find_iter(CellValue::M).map(|(start, _)| start),
            CellValue::search_seq_mas(),
            TravelDirection::cross(),
        );
        let heads_tails: Vec<(GridCoord2, GridCoord2)> = mas_paths
            .iter()
            .map(|path| (path.1[0], path.1[2]))
            .collect();
        let head_coords: std::collections::HashSet<GridCoord2> =
            heads_tails.iter().map(|(head, _)| *head).collect();
        let tail_coords: std::collections::HashSet<GridCoord2> =
            heads_tails.iter().map(|(_, tail)| *tail).collect();

        let head_vals = dbg!(head_coords
            .iter()
            .map(|c| self.grid[*c].to_string())
            .collect::<Vec<_>>()
            .join(""));
        let tail_vals = dbg!(tail_coords
            .iter()
            .map(|c| self.grid[*c].to_string())
            .collect::<Vec<_>>()
            .join(""));
        // Cross types
        let cross_plans = head_coords
            .iter()
            .map(|head| {
                let tmp: Vec<DirectedPath> = vec![
                    self.x_east(*head, &tail_coords),
                    self.x_west(*head, &tail_coords),
                    self.x_north(*head, &tail_coords),
                    self.x_south(*head, &tail_coords),
                ]
                .iter()
                .filter_map(|o| o.clone())
                .flatten()
                .collect();
                return tmp;
            })
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
    fn all() -> Vec<TravelDirection> {
        TravelDirection::iter().collect()
    }
    fn cardinal() -> Vec<TravelDirection> {
        const CARDINAL_PLAN: [TravelDirection; 4] = [
            TravelDirection::N,
            TravelDirection::E,
            TravelDirection::S,
            TravelDirection::W,
        ];
        Vec::from(CARDINAL_PLAN)
    }
    fn cross() -> Vec<TravelDirection> {
        const CROSS_PLAN: [TravelDirection; 4] = [
            TravelDirection::NE,
            TravelDirection::SE,
            TravelDirection::SW,
            TravelDirection::NE,
        ];
        Vec::from(CROSS_PLAN)
    }
    // fn next_coord_unchecked(&self, (y, x): GridCoord2) -> GridCoord2 {
    //     match self {
    //         Self::N => (y - 1, x),
    //         Self::NE => (y - 1, x + 1),
    //         Self::E => (y, x + 1),
    //         Self::SE => (y + 1, x + 1),
    //         Self::S => (y + 1, x),
    //         Self::SW => (y - 1, x - 1),
    //         Self::W => (y, x - 1),
    //         Self::NW => (y - 1, x - 1),
    //     }
    // }

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
