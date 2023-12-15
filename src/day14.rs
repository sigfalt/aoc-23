
use std::{collections::{HashMap, BTreeMap}, ops::ControlFlow, cmp::Ordering, fmt::Display};

use itertools::Itertools;
use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, all_consuming}, character::complete::{line_ending, one_of}, multi::{many1, separated_list1}};
use num::Integer;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Rock {
    Rounded,
    Square,
    Empty
}
impl TryFrom<char> for Rock {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Rounded),
            '#' => Ok(Self::Square),
            '.' => Ok(Self::Empty),
            _ => Err("Invalid rock input")
        }
    }
}


fn parse_line(input: &str) -> IResult<&str, Vec<Rock>> {
    many1(map_res(one_of("O#."), |chr| chr.try_into()))(input)
}

fn parse(input: &str) -> Vec<Vec<Rock>> {
    let (_, output) = all_consuming(separated_list1(line_ending, parse_line))(input).unwrap();
    output
}


#[derive(Debug, Clone)]
struct RockWeight {
    sum: u64,
    to_add: u64
}

#[aoc(day14, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);

    // fake actually rolling the rocks and just calculate the weight if they moved north as far as they could
    let col_num = input.len() as u64;
    let row_length = input[0].len();
    input.into_iter().enumerate().fold(vec![RockWeight {sum:0, to_add:col_num}; row_length], |init, (row_ix, row)| {
        let row_ix = row_ix as u64;
        row.into_iter().zip_eq(init.into_iter()).map(|(rock, rock_weight)| {
            match rock {
                // no rock so no additional weight,
                // but anything below might still roll up and contribute max
                Rock::Empty => rock_weight,

                // this rock contributes whatever the max was,
                // and anything below can roll up behind it and contribute max-1
                Rock::Rounded => RockWeight { sum: rock_weight.sum + rock_weight.to_add, to_add: rock_weight.to_add - 1 },

                // this rock doesn't contribute anything,
                // and anything below would get stuck behind it (at row_ix-1)
                Rock::Square => RockWeight { sum: rock_weight.sum, to_add: col_num - (row_ix + 1) },
                
            }
        }).collect_vec()
    }).into_iter().fold(0, |sum, rock| {
        sum + rock.sum
    })
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn spin_cycle() -> &'static [Self] {
        &[
            Self::North,
            Self::West,
            Self::South,
            Self::East
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Grid {
    data: BTreeMap<(usize, usize), Rock>,
    rows: usize,
    cols: usize,
}
impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        (0..self.rows).for_each(|row_ix| {
            (0..self.cols).for_each(|col_ix| {
                output.push(match self.data.get(&(row_ix, col_ix)) {
                    Some(Rock::Rounded) => 'O',
                    Some(Rock::Square) => '#',
                    _ => '.'
                });
            });
            output.push('\n');
        });
        write!(f, "{}", output)
    }
}
impl Grid {
    fn from_vec(input: Vec<Vec<Rock>>) -> Self {
        let rows = input.len();
        let cols = input[0].len();
        let mut data = BTreeMap::new();
        input.into_iter().enumerate().for_each(|(row_ix, rocks)| {
            rocks.into_iter().enumerate().for_each(|(col_ix, rock)| {
                if rock != Rock::Empty {
                    data.insert((row_ix, col_ix), rock);
                }
            })
        });
        Self {data, rows, cols}
    }
    fn spin_cycle(&mut self) {
        Direction::spin_cycle().iter().for_each(|&dir| {
            self.push_rocks(dir);
        })
    }
    fn push_rocks(&mut self, dir: Direction) {
        let rock_pos_iter = match dir {
            Direction::North => (0..self.rows).into_iter().flat_map(|row| (0..self.cols).into_iter().map(move |col| (row, col)).collect_vec()).collect_vec(),
            Direction::South => (0..self.rows).rev().flat_map(|row| (0..self.cols).into_iter().map(move |col| (row, col)).collect_vec()).collect_vec(),
            Direction::East => (0..self.rows).rev().flat_map(|row| (0..self.cols).rev().map(move |col| (row, col)).collect_vec()).collect_vec(),
            Direction::West => (0..self.rows).rev().flat_map(|row| (0..self.cols).into_iter().map(move |col| (row, col)).collect_vec()).collect_vec(),
        }.into_iter();

        let try_roll = |(row, col): (usize, usize), (rows, cols): &(usize, usize), dir| {
            match dir {
                Direction::North => row.checked_sub(1).map(|new_row| (new_row, col)),
                Direction::South => row.checked_add(1).and_then(|new_row| if new_row.cmp(rows) == Ordering::Less { Some((new_row, col)) } else { None }),
                Direction::East => col.checked_add(1).and_then(|new_col| if new_col.cmp(cols) == Ordering::Less { Some((row, new_col)) } else { None }),
                Direction::West => col.checked_sub(1).map(|new_col| (row, new_col)),
            }
        };

        rock_pos_iter.for_each(|pos| {
            // only rounded rocks can move, so only do anything for them
            if let Some(Rock::Rounded) = self.data.get(&pos) {
                let mut curr_pos = pos;
                let mut new_pos = None;
                while let Some(rolled_pos) = try_roll(curr_pos, &(self.rows, self.cols), dir) {
                    if self.data.get(&rolled_pos).is_none() {
                        // nothing is in the way, keep trying to roll
                        curr_pos = rolled_pos;
                        new_pos = Some(rolled_pos);
                    } else {
                        break;
                    }
                }
                if let Some(new_pos) = new_pos {
                    self.data.remove(&pos).unwrap();
                    self.data.insert(new_pos, Rock::Rounded);
                }
            }
        });
    }
}

#[aoc(day14, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    // I think we actually need to roll the rocks around now
    let init_grid = Grid::from_vec(input);
    let mut memoized_cycles: HashMap<Grid, u64> = HashMap::new();
    let max_cycles = 1000000000;
    let cycle_result = (0..max_cycles).into_iter().try_fold(init_grid, |grid, step| {
        let mut new_grid = grid.clone();
        new_grid.spin_cycle();

        // check if we've previously generated this grid from a previous step
        if let Some(&prev_step) = memoized_cycles.get(&new_grid) {
            // we've cycled to this grid state before after {prev_step} steps
            // if we repeat the cycle enough times, will we end up at this state after {max_cycles}?
            if (max_cycles - step).is_multiple_of(&(step - prev_step)) {
                return ControlFlow::Break(grid);
            }
        }
        // insert new grid into memoized map
        memoized_cycles.insert(new_grid.clone(), step);
        ControlFlow::Continue(new_grid)
    });
    let result_grid = match cycle_result {
        ControlFlow::Continue(r) => r,
        ControlFlow::Break(r) => r,
    };

    let rows = result_grid.rows as u64;
    result_grid.data.into_iter().fold(0, |sum, ((row, _), rock)| {
        sum + if rock == Rock::Rounded { rows - row as u64 } else { 0 }
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."), 136);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."), 64);
    }

}