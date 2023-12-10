use std::collections::{HashMap, VecDeque, HashSet};

use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, all_consuming, opt}, character::complete::{line_ending, one_of}, multi::many1, sequence::terminated};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start
}
impl Pipe {
    fn vec_neighbors(&self) -> Option<HashSet<Direction>> {
        if *self != Pipe::Start {
            Some(
                HashSet::from_iter([
                    {if [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest].contains(self) { Some(Direction::North) } else { None }},
                    {if [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(self) { Some(Direction::East) } else { None }},
                    {if [Pipe::Vertical, Pipe::SouthWest, Pipe::SouthEast].contains(self) { Some(Direction::South) } else { None }},
                    {if [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest].contains(self) { Some(Direction::West) } else { None }},
                ].into_iter().flatten())
            )
        } else {
            None
        }
    }

    fn connects_to(&self, dir: Direction) -> bool {
        match dir {
            Direction::North => [Pipe::Vertical, Pipe::NorthEast, Pipe::NorthWest].contains(self),
            Direction::East => [Pipe::Horizontal, Pipe::NorthEast, Pipe::SouthEast].contains(self),
            Direction::South => [Pipe::Vertical, Pipe::SouthWest, Pipe::SouthEast].contains(self),
            Direction::West => [Pipe::Horizontal, Pipe::NorthWest, Pipe::SouthWest].contains(self),
        }
    }
}
impl TryFrom<HashSet<Direction>> for Pipe {
    type Error = &'static str;
    fn try_from(value: HashSet<Direction>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            Err("expected exactly two directions to connect to")
        } else if value == HashSet::from([Direction::North, Direction::East]) {
            Ok(Pipe::NorthEast)
        } else if value == HashSet::from([Direction::North, Direction::South]) {
            Ok(Pipe::Vertical)
        } else if value == HashSet::from([Direction::North, Direction::West]) {
            Ok(Pipe::NorthWest)
        } else if value == HashSet::from([Direction::East, Direction::South]) {
            Ok(Pipe::SouthEast)
        } else if value == HashSet::from([Direction::East, Direction::West]) {
            Ok(Pipe::Horizontal)
        } else if value == HashSet::from([Direction::South, Direction::West]) {
            Ok(Pipe::SouthWest)
        } else {
            Err("invalid directions pair")
        }
    }
}

fn parse_pipe(input: &str) -> IResult<&str, Pipe> {
    map_res(
        one_of("|-LJ7F.S"),
        |chr| {
            match chr {
                '|' => Ok(Pipe::Vertical),
                '-' => Ok(Pipe::Horizontal),
                'L' => Ok(Pipe::NorthEast),
                'J' => Ok(Pipe::NorthWest),
                '7' => Ok(Pipe::SouthWest),
                'F' => Ok(Pipe::SouthEast),
                '.' => Ok(Pipe::Ground),
                'S' => Ok(Pipe::Start),
                _ => Err("Invalid pipe")
            }
        }
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Pipe>> {
    terminated(
        many1(parse_pipe),
        opt(line_ending)
    )(input)
}

fn parse(input: &str) -> Vec<Vec<Pipe>> {
    let (_, output) = all_consuming(many1(parse_line))(input).unwrap();

    output
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West
}
impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn get_loc(&self, (col, row): (i32, i32)) -> (i32, i32) {
        match *self {
            Direction::North => (col - 1, row),
            Direction::East => (col, row + 1),
            Direction::South => (col + 1, row),
            Direction::West => (col, row - 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    location: (i32, i32),
    shape: Pipe,
    distance: Option<u32>,
}
impl From<((i32, i32), Pipe)> for Node {
    fn from((location, shape): ((i32, i32), Pipe)) -> Self {
        Self { location, shape, distance: None }
    }
}

#[aoc(day10, part1)]
fn part1(input: &str) -> u32 {
    let pipe_input = parse(input);

    let mut starting_loc = None;
    let mut loc_node_map = pipe_input.into_iter().enumerate().fold(HashMap::new(), |mut map, (col, pipe_row)| {
        map.extend(pipe_row.into_iter().enumerate().filter_map(|(row, pipe)| {
            let (col, row) = (col as i32, row as i32);
            if pipe == Pipe::Start { starting_loc = Some((col, row)); }
            if let Ok(node) = Node::try_from(((col, row), pipe)) {
                Some(((col, row), node))
            } else {
                None
            }
        }));
        map
    });
    let starting_loc = starting_loc.unwrap();

    loc_node_map.entry(starting_loc).and_modify(|node| { node.distance = Some(0); });
    let mut max_distance = 0;
    let mut locs_to_check = VecDeque::from_iter(vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ].into_iter().filter_map(|dir| {
        let neighbor_loc = dir.get_loc(starting_loc);
        if let Some(node) = loc_node_map.get_mut(&neighbor_loc) {
            if node.shape.connects_to(dir.opposite()) {
                node.distance = Some(1);
                max_distance = 1;
                Some(neighbor_loc)
            } else {
                None
            }
        } else {
            None
        }
    }));

    while let Some(location) = locs_to_check.pop_front() {
        let node = loc_node_map.get(&location).unwrap().clone();
        node.shape.vec_neighbors().unwrap().into_iter().for_each(|dir| {
            if let Some(neighbor_node) = loc_node_map.get_mut(&dir.get_loc(location)) {
                if neighbor_node.shape != Pipe::Start && neighbor_node.distance.is_none() {
                    let new_dist = node.distance.unwrap() + 1;
                    neighbor_node.distance = Some(new_dist);
                    max_distance = new_dist;
                    locs_to_check.push_back(neighbor_node.location);
                }
            }
        });
    }

    max_distance
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Enclosed {
    Inside,
    Outside,
    FromInside(Direction),
    FromOutside(Direction),
}

#[aoc(day10, part2)]
fn part2(input: &str) -> u32 {
    let pipe_input = parse(input);
    let max_col = pipe_input.len() as i32;
    let max_row = pipe_input[0].len() as i32;

    let mut starting_loc = None;
    let mut loc_node_map = pipe_input.into_iter().enumerate().fold(HashMap::new(), |mut map, (col, pipe_row)| {
        map.extend(pipe_row.into_iter().enumerate().filter_map(|(row, pipe)| {
            let (col, row) = (col as i32, row as i32);
            if pipe == Pipe::Start { starting_loc = Some((col, row)); }
            if let Ok(node) = Node::try_from(((col, row), pipe)) {
                Some(((col, row), node))
            } else {
                None
            }
        }));
        map
    });
    let starting_loc = starting_loc.unwrap();

    let all_directions = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    
    let locs_to_check = all_directions.clone().into_iter().filter_map(|dir| {
        let neighbor_loc = dir.get_loc(starting_loc);
        if let Some(node) = loc_node_map.get_mut(&neighbor_loc) {
            if node.shape.connects_to(dir.opposite()) {
                node.distance = Some(1);
                Some(neighbor_loc)
            } else {
                None
            }
        } else {
            None
        }
    });
    let mut locs_to_check = VecDeque::from_iter(locs_to_check);

    let directions = all_directions.clone().into_iter().filter_map(|dir| {
        let neighbor_loc = dir.get_loc(starting_loc);
        if let Some(node) = loc_node_map.get_mut(&neighbor_loc) {
            if node.shape.connects_to(dir.opposite()) {
                node.distance = Some(1);
                Some(dir)
            } else {
                None
            }
        } else {
            None
        }
    });
    let mut direction_set = HashSet::new();
    direction_set.extend(directions);

    loc_node_map.entry(starting_loc).and_modify(|node| {
        node.distance = Some(0);
        node.shape = Pipe::try_from(direction_set).unwrap()
    });

    while let Some(location) = locs_to_check.pop_front() {
        let node = loc_node_map.get(&location).unwrap().clone();
        node.shape.vec_neighbors().unwrap().into_iter().for_each(|dir| {
            if let Some(neighbor_node) = loc_node_map.get_mut(&dir.get_loc(location)) {
                if neighbor_node.shape != Pipe::Start && neighbor_node.distance.is_none() {
                    let new_dist = node.distance.unwrap() + 1;
                    neighbor_node.distance = Some(new_dist);
                    locs_to_check.push_back(neighbor_node.location);
                }
            }
        });
    }

    (0..max_col).map(|col| {
        let (spaces, _) = (0..max_row).fold((0, Enclosed::Outside), |(count, enclosed), row| {
            let node = loc_node_map.get(&(col, row)).unwrap();
            let part_of_main_loop = node.distance.is_some();
            match (node.shape, enclosed) {
                (Pipe::NorthEast, Enclosed::Inside) => if part_of_main_loop { (count, Enclosed::FromInside(Direction::North)) }
                        else { (count + if enclosed == Enclosed::Inside { 1 } else { 0 }, enclosed) }, // I [L] ?
                (Pipe::NorthEast, Enclosed::Outside) => (count, if part_of_main_loop { Enclosed::FromOutside(Direction::North) }
                        else { enclosed }), // O [L] ?
                (Pipe::NorthWest, Enclosed::FromInside(Direction::North)) => (count, Enclosed::Inside), // I L [J] I
                (Pipe::NorthWest, Enclosed::FromInside(Direction::South)) => (count, Enclosed::Outside), // I F [J] O
                (Pipe::NorthWest, Enclosed::FromOutside(Direction::North)) => (count, Enclosed::Outside), // O L [J] O
                (Pipe::NorthWest, Enclosed::FromOutside(Direction::South)) => (count, Enclosed::Inside), // O F [J] I
                (Pipe::SouthWest, Enclosed::FromInside(Direction::North)) => (count, Enclosed::Outside), // I L [7] O
                (Pipe::SouthWest, Enclosed::FromInside(Direction::South)) => (count, Enclosed::Inside), // I F [7] I
                (Pipe::SouthWest, Enclosed::FromOutside(Direction::North)) => (count, Enclosed::Inside), // O L [7] I
                (Pipe::SouthWest, Enclosed::FromOutside(Direction::South)) => (count, Enclosed::Outside), // O F [7] O
                (Pipe::SouthEast, Enclosed::Inside) => if part_of_main_loop { (count, Enclosed::FromInside(Direction::South)) }
                        else { (count + if enclosed == Enclosed::Inside { 1 } else { 0 }, enclosed) }, // I [F] ?
                (Pipe::SouthEast, Enclosed::Outside) => (count, if part_of_main_loop { Enclosed::FromOutside(Direction::South) } else { enclosed }), // O [F] ?
                (Pipe::Vertical, _) => if part_of_main_loop { (count, if enclosed == Enclosed::Inside { Enclosed::Outside } else { Enclosed::Inside }) }
                        else { (count + if enclosed == Enclosed::Inside { 1 } else { 0 }, enclosed) },
                _ => (count + if enclosed == Enclosed::Inside { 1 } else { 0 }, enclosed)
            }
        });
        spaces
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(".....
.S-7.
.|.|.
.L-J.
....."), 4);
        assert_eq!(part1("..F7.
.FJ|.
SJ.L7
|F--J
LJ..."), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."), 4);
        assert_eq!(part2(".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."), 8);
        assert_eq!(part2("FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"), 10);
    }

}