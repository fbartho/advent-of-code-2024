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
        let paths = grid.valid_paths();

        let mut mask = grid.clone();
        paths
            .iter()
            .flatten()
            .for_each(|coord| mask.grid[*coord] = CellValue::Unknown);
        let mut visualized = grid.clone();
        mask.grid
            .indexed_iter()
            .filter(|(_, val)| CellValue::Unknown != **val)
            .for_each(|(coord, _)| visualized.grid[coord] = CellValue::Unknown);
        println!("{}", visualized.to_string());

        Ok(paths.len().to_string())
    }

    fn part_two(&self, _input: &str) -> super::PuzzleResult {
        todo!("implement part two")
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
const NEEDLE_BITS: usize = 4;
impl CellValue {
    // static
    fn search_seq() -> impl Iterator<Item = &'static CellValue> {
        const NEEDLE: [CellValue; NEEDLE_BITS] =
            [CellValue::X, CellValue::M, CellValue::A, CellValue::S];
        NEEDLE.iter()
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
    // fn is_valid_coord(&self, (y, x): GridCoord2) -> bool {
    //     return (0..self.grid.rows()).contains(&y) && (0..self.grid.cols()).contains(&x);
    // }
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
                    .map(|c| CellValue::from_str(&c.to_string()).unwrap())
                    .collect::<Vec<CellValue>>()
            })
            .collect::<Vec<Vec<CellValue>>>();
        let grid: Grid<CellValue> = grid_from_vec_vec(g_data);
        Self { grid }
    }
    fn valid_paths(&self) -> Vec<Vec<GridCoord2>> {
        self.find_iter(CellValue::X)
            .map(|(start, _)| {
                TravelDirection::iter()
                    .filter_map(|dir| {
                        let _ = dbg!(&dir);
                        let mut path: Vec<GridCoord2> = Vec::new();
                        let mut coords = std::iter::successors(Some(start), |c| dir.next_coord(*c));
                        for target in CellValue::search_seq() {
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
                        return Some(path);
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>()
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
}
#[derive(Debug, Clone, PartialEq, EnumString, Display, EnumIter)]
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

    fn next_coord(&self, (y, x): GridCoord2) -> Option<GridCoord2> {
        Some(match self {
            Self::N => (y.checked_sub(1)?, x),
            Self::NE => (y.checked_sub(1)?, x + 1),
            Self::E => (y, x + 1),
            Self::SE => (y + 1, x + 1),
            Self::S => (y + 1, x),
            Self::SW => (y + 1, x.checked_sub(1)?),
            Self::W => (y, x.checked_sub(1)?),
            Self::NW => (y.checked_sub(1)?, x.checked_sub(1)?),
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
