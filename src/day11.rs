
use std::collections::{HashSet, HashMap};

use itertools::Itertools;
use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, all_consuming}, character::complete::{line_ending, one_of}, multi::{many1, separated_list1}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImagePixel {
    Empty,
    Galaxy
}
impl TryFrom<char> for ImagePixel {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Galaxy),
            _ => Err("invalid image pixel character")
        }
    }
}

fn parse_line(input: &str) -> IResult<&str, Vec<ImagePixel>> {
    many1(map_res(
        one_of(".#"),
        |chr| ImagePixel::try_from(chr)
    ))(input)
}

fn parse(input: &str) -> Vec<Vec<ImagePixel>> {
    let (_, output) = all_consuming(separated_list1(line_ending, parse_line))(input).unwrap();
    output
}


fn expand(input: Vec<Vec<ImagePixel>>) -> Vec<Vec<ImagePixel>> {
    let row_adjusted = input.into_iter().flat_map(|row| {
        if row.iter().all(|&pixel| pixel == ImagePixel::Empty) {
            vec![row.clone(), row]
        } else {
            vec![row]
        }
    }).collect_vec();

    let empty_cols: HashSet<usize> = row_adjusted.iter().fold(HashSet::from_iter(0..row_adjusted[0].len()), |mut set, row| {
        set.retain(|&ix| row[ix] == ImagePixel::Empty);
        set
    });
    row_adjusted.into_iter().map(|row| {
        row.into_iter().enumerate().flat_map(|(ix, pixel)| {
            if empty_cols.contains(&ix) {
                vec![pixel, pixel]
            } else {
                vec![pixel]
            }
        }).collect_vec()
    }).collect_vec()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GalaxyNode {
    location: (u64, u64)
}
impl From<(usize, usize)> for GalaxyNode {
    fn from((x_usize, y_usize): (usize, usize)) -> Self {
        let location = (x_usize as u64, y_usize as u64);
        Self { location }
    }
}
impl GalaxyNode {
    fn distance_between(&self, other: &GalaxyNode) -> u64 {
        let (self_x, self_y) = self.location;
        let (other_x, other_y) = other.location;
        self_x.abs_diff(other_x) + self_y.abs_diff(other_y)
    }
    fn set_row(&mut self, new_row: u64) {
        let (_, orig_col) = self.location;
        self.location = (new_row, orig_col);
    }
    fn set_col(&mut self, new_col: u64) {
        let (orig_row, _) = self.location;
        self.location = (orig_row, new_col);
    }
}

#[aoc(day11, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);
    let expanded = expand(input);
    let galaxies: Vec<GalaxyNode> = expanded.into_iter().enumerate().fold(Vec::new(), |acc, (row_ix, row)| {
        row.into_iter().enumerate().fold(acc, |mut row_acc, (col_ix, pixel)| {
            if pixel == ImagePixel::Galaxy {
                row_acc.push((row_ix, col_ix).into());
            }
            row_acc
        })
    });

    galaxies.iter().combinations(2).map(|el| {
        if el.len() != 2 {
            panic!("combinations(2) produced combinations not containing 2 elements");
        } else {
            el[0].distance_between(el[1])
        }
    }).sum()
}

fn process_true_locations(input: Vec<Vec<ImagePixel>>, expansion_factor: u64) -> Vec<GalaxyNode> {
    let mut adjusted_location_map = HashMap::new();
    input.iter().enumerate().for_each(|(row_ix, row)| {
        row.iter().enumerate().for_each(|(col_ix, &pixel)| {
            if pixel == ImagePixel::Galaxy {
                let galaxy_node: GalaxyNode = (row_ix, col_ix).into();
                adjusted_location_map.insert(galaxy_node.clone(), galaxy_node);
            }
        });
    });

    input.iter().enumerate().for_each(|(row_ix, row)| {
        if row.iter().all(|&pixel| pixel == ImagePixel::Empty) {
            adjusted_location_map.iter_mut().for_each(|(orig_galaxy, true_galaxy)| {
                let (orig_row, _) = orig_galaxy.location;
                let (mod_row, _) = true_galaxy.location;
                if orig_row > row_ix as u64 {
                    true_galaxy.set_row(mod_row + expansion_factor - 1);
                }
            });
        }
    });

    input.iter().fold(HashSet::from_iter(0..input[0].len()), |mut set: HashSet<usize>, row| {
        set.retain(|&ix| row[ix] == ImagePixel::Empty);
        set
    }).into_iter().for_each(|col_ix| {
        adjusted_location_map.iter_mut().for_each(|(orig_galaxy, true_galaxy)| {
            let (_, orig_col) = orig_galaxy.location;
            let (_, mod_col) = true_galaxy.location;
            if orig_col > col_ix as u64 {
                true_galaxy.set_col(mod_col + expansion_factor - 1);
            }
        });
    });

    adjusted_location_map.into_values().collect_vec()
}

fn process_galaxy_image_with_expansion(input: Vec<Vec<ImagePixel>>, expansion_factor: u64) -> u64 {
    let galaxies = process_true_locations(input, expansion_factor);

    galaxies.iter().combinations(2).map(|el| {
        if el.len() != 2 {
            panic!("combinations(2) produced combinations not containing 2 elements");
        } else {
            el[0].distance_between(el[1])
        }
    }).sum()
}

#[aoc(day11, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);
    process_galaxy_image_with_expansion(input, 1_000_000)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 374);
    }

    #[test]
    fn part2_example() {
        assert_eq!(process_galaxy_image_with_expansion(parse("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 2), 374);
        assert_eq!(process_galaxy_image_with_expansion(parse("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 10), 1030);
        assert_eq!(process_galaxy_image_with_expansion(parse("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."), 100), 8410);
    }

}