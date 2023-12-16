
use std::collections::HashSet;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{all_consuming, map_res}, character::complete::{one_of, multispace1}, multi::{separated_list1, many1}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParsedCell {
    Empty,
    MirrorSlash,
    MirrorBackslash,
    SplitterVertical,
    SplitterHorizontal,
}
impl TryFrom<char> for ParsedCell {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::MirrorSlash),
            '\\' => Ok(Self::MirrorBackslash),
            '|' => Ok(Self::SplitterVertical),
            '-' => Ok(Self::SplitterHorizontal),
            _ => Err("Invalid cell character"),
        }
    }
}
impl ParsedCell {
    fn reflect(&self, dir: Direction) -> impl Iterator<Item = Direction> {
        use ParsedCell::*;
        use Direction::*;
        match (self, dir) {
            (Empty, _) => vec![dir].into_iter(),
            (MirrorSlash, North) => vec![East].into_iter(),
            (MirrorSlash, East) => vec![North].into_iter(),
            (MirrorSlash, South) => vec![West].into_iter(),
            (MirrorSlash, West) => vec![South].into_iter(),
            (MirrorBackslash, North) => vec![West].into_iter(),
            (MirrorBackslash, East) => vec![South].into_iter(),
            (MirrorBackslash, South) => vec![East].into_iter(),
            (MirrorBackslash, West) => vec![North].into_iter(),
            (SplitterVertical, North | South) => vec![dir].into_iter(),
            (SplitterVertical, East | West) => vec![North, South].into_iter(),
            (SplitterHorizontal, North | South) => vec![East, West].into_iter(),
            (SplitterHorizontal, East | West) => vec![dir].into_iter(),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}
impl Direction {
    fn travel(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        match self {
            North => Some((x, y.checked_sub(1)?)),
            East => Some((x + 1, y)),
            South => Some((x, y + 1)),
            West => Some((x.checked_sub(1)?, y)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LightBeam {
    pos: (usize, usize),
    dir: Direction
}


fn parse_line(input: &str) -> IResult<&str, Vec<ParsedCell>> {
    many1(map_res(one_of("./\\|-"), |chr| chr.try_into()))(input)
}
fn parse(input: &str) -> Vec<Vec<ParsedCell>> {
    let (_, output) = all_consuming(separated_list1(multispace1, parse_line))(input).unwrap();
    output
}


#[aoc(day16, part1)]
fn part1(input: &str) -> u64 {
    let cells = parse(input);
    count_energized(&cells, LightBeam { pos: (0, 0), dir: Direction::East })
}


fn count_energized(cells: &Vec<Vec<ParsedCell>>, init_beam: LightBeam) -> u64 {
    let (init_x, init_y) = init_beam.pos;
    let mut light_beams = cells[init_y][init_x].reflect(init_beam.dir).map(|dir|
        LightBeam { pos: (init_x, init_y), dir }
    ).collect_vec();
    let mut beam_history = HashSet::new();
    let mut energized = HashSet::new();
    while let Some(beam) = light_beams.pop() {
        energized.insert(beam.pos);
        let new_pos = beam.dir.travel(beam.pos);
        if let Some((new_x, new_y)) = new_pos {
            if let Some(target) = cells.get(new_y).and_then(|row| row.get(new_x)) {
                let new_dirs = target.reflect(beam.dir);
                new_dirs.for_each(|dir| {
                    let new_beam = LightBeam { pos: (new_x, new_y), dir };
                    if !beam_history.contains(&new_beam) {
                        beam_history.insert(new_beam.clone());
                        light_beams.push(new_beam);
                    }
                });
            }
        }
    }
    energized.len() as u64
}


#[aoc(day16, part2)]
fn part2(input: &str) -> u64 {
    let cells = parse(input);
    let max_y = cells.len();
    let max_x = cells[0].len();
    
    (0..max_x).map(|x| {
        LightBeam { pos: (x, max_y - 1), dir: Direction::North }
    }).chain((0..max_y).map(|y| {
        LightBeam { pos: (0, y), dir: Direction::East }
    })).chain((0..max_x).map(|x| {
        LightBeam { pos: (x, 0), dir: Direction::South }
    })).chain((0..max_y).map(|y| {
        LightBeam { pos: (max_x - 1, y), dir: Direction::West }
    })).map(|init_beam| {
        count_energized(&cells, init_beam)
    }).max().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."), 46);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."), 51);
    }

}