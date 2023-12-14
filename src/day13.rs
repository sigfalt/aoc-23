
use itertools::Itertools;
use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, all_consuming, map}, character::complete::{line_ending, one_of}, multi::{many1, separated_list1, many1_count}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ground {
    Ash,
    Rock
}
impl TryFrom<char> for Ground {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ash),
            '#' => Ok(Self::Rock),
            _ => Err("Invalid ground character")
        }
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    cells: Vec<Vec<Ground>>
}


fn parse_line(input: &str) -> IResult<&str, Vec<Ground>> {
    many1(map_res(
        one_of(".#"),
        |chr| chr.try_into()
    ))(input)
}

fn parse_pattern(input: &str) -> IResult<&str, Pattern> {
    map(
        separated_list1(line_ending, parse_line),
        |cells| Pattern { cells }
    )(input)
}

fn line_ending1(input: &str) -> IResult<&str, usize> {
    many1_count(line_ending)(input)
}

fn parse(input: &str) -> Vec<Pattern> {
    let (_, patterns) = all_consuming(separated_list1(line_ending1, parse_pattern))(input).unwrap();
    patterns
}

fn detect_row_mirror(input: &Pattern, smudge: bool) -> Option<u64> {
    if smudge { detect_smudged_mirror(&input.cells) } else { detect_mirror(&input.cells) }
}

fn detect_column_mirror(input: &Pattern, smudge: bool) -> Option<u64> {
    let rows = input.cells.len();
    let cols = input.cells[0].len();
    let transposed = (0..cols).map(|col| {
        (0..rows).map(|row| input.cells[row][col]).collect()
    }).collect();

    if smudge { detect_smudged_mirror(&transposed) } else { detect_mirror(&transposed) }
}

fn detect_mirror(input: &Vec<Vec<Ground>>) -> Option<u64> {
    'outer: for start_ix in 0..(input.len() - 1) {
        let mut curr_ix = start_ix;
        let mut next_ix = start_ix + 1;
        while let (Some(curr_line), Some(next_line)) = (input.get(curr_ix), input.get(next_ix)) {
            if curr_line != next_line {
                continue 'outer;
            }
            curr_ix -= 1;
            next_ix += 1;
        }
        return Some(1 + start_ix as u64);
    }
    None
}

fn detect_smudged_mirror(input: &Vec<Vec<Ground>>) -> Option<u64> {
    'outer: for start_ix in 0..(input.len() - 1) {
        let mut curr_defects = 0;
        let mut curr_ix = start_ix;
        let mut next_ix = start_ix + 1;
        while let (Some(curr_line), Some(next_line)) = (input.get(curr_ix), input.get(next_ix)) {
            curr_defects += curr_line.iter().zip_eq(next_line.iter()).fold(0, |diffs, (x, y)| {
                if x == y { diffs } else { diffs + 1 }
            });
            if curr_defects > 1 {
                continue 'outer;
            }

            if curr_ix == 0 {
                // avoid usize underflow
                break;
            }
            curr_ix -= 1;
            next_ix += 1;
        }
        if curr_defects == 1 {
            return Some(1 + start_ix as u64);
        }
    }
    None
}

#[aoc(day13, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);

    input.into_iter().map(|pattern| {
        if let Some(val) = detect_row_mirror(&pattern, false) {
            val * 100
        } else if let Some(val) = detect_column_mirror(&pattern, false) {
            val
        } else {
            panic!("Did not detect any mirror from pattern {pattern:?}");
        }
    }).sum()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    input.into_iter().map(|pattern| {
        if let Some(val) = detect_row_mirror(&pattern, true) {
            val * 100
        } else if let Some(val) = detect_column_mirror(&pattern, true) {
            val
        } else {
            panic!("Did not detect any mirror from pattern {pattern:?}");
        }
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"), 405);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"), 400);
    }

}