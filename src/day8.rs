use std::{collections::HashMap, ops::ControlFlow, cell::RefCell, rc::{Rc, Weak}};

use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, all_consuming, map}, multi::{separated_list1, many_till, many1}, character::complete::{line_ending, one_of}, bytes::complete::{take, tag}, sequence::{tuple, separated_pair}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}
impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err("invalid direction")
        }
    }
}

#[derive(Debug, Clone)]
struct ParsedNode {
    name: String,
    left: String,
    right: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeType {
    Starting,
    Ending,
}
impl TryFrom<&str> for NodeType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.ends_with("A") {
            Ok(Self::Starting)
        } else if value.ends_with("Z") {
            Ok(Self::Ending)
        } else {
            Err("invalid special node character")
        }
    }
}

#[derive(Debug, Clone)]
struct Node {
    name: String,
    location: Option<NodeType>,
    left: RefCell<Option<Weak<Node>>>,
    right: RefCell<Option<Weak<Node>>>,
}
impl From<ParsedNode> for Node {
    fn from(value: ParsedNode) -> Self {
        Self::from(value.name.as_str())
    }
}
impl From<&str> for Node {
    fn from(value: &str) -> Self {
        Self {
            location: value.try_into().ok(),
            name: value.to_string(),
            left: RefCell::new(None),
            right: RefCell::new(None),
        }
    }
}
impl Node {
    fn get(&self, dir: Direction) -> Rc<Node> {
        match dir {
            Direction::Left => Weak::clone(&self.left.borrow().as_ref().expect("node missing left")).upgrade().expect("failed to upgrade left"),
            Direction::Right => Weak::clone(&self.right.borrow().as_ref().expect("node missing right")).upgrade().expect("failed to upgrade right"),
        }
    }
}

fn parse_directions(input: &str) -> IResult<&str, Vec<Direction>> {
    map(many_till(map_res(
        one_of("LR"),
        |chr| chr.try_into()
    ), line_ending),
        |(directions, _)| directions
    )(input)
}

fn line_ending1(input: &str) -> IResult<&str, ()> {
    let (output, _) = many1(line_ending)(input)?;
    Ok((output, ()))
}

fn parse_node(input: &str) -> IResult<&str, ParsedNode> {
    let (output, (name, _, left, _, right, _)) = tuple((
        take(3u8),
        tag(" = ("),
        take(3u8),
        tag(", "),
        take(3u8),
        tag(")")
    ))(input)?;

    Ok((output, ParsedNode {
        name: name.to_string(),
        left: left.to_string(),
        right: right.to_string()
    }))
}

fn parse_nodes(input: &str) -> IResult<&str, Vec<ParsedNode>> {
    separated_list1(line_ending, parse_node)(input)
}

fn parse(input: &str) ->(Vec<Direction>, Vec<ParsedNode>) {
    let (_, output) = all_consuming(
        separated_pair(parse_directions, line_ending1, parse_nodes)
    )(input).unwrap();

    output
}


#[aoc(day8, part1)]
fn part1(input: &str) -> u32 {
    let (directions, nodes) = parse(input);
    let mut node_map: HashMap<String, Rc<Node>> = HashMap::with_capacity(nodes.len());
    nodes.into_iter().for_each(|node| {
        let left = Rc::downgrade(node_map.entry(node.left.clone()).or_insert(Rc::new(node.left.as_str().into())));
        let right = Rc::downgrade(node_map.entry(node.right.clone()).or_insert(Rc::new(node.right.as_str().into())));

        let new_node = node_map.entry(node.name.clone()).or_insert(Rc::new(node.name.as_str().into()));
        new_node.left.replace(Some(left));
        new_node.right.replace(Some(right));
    });

    let starting_node = Rc::clone(node_map.get("AAA").unwrap());

    let ControlFlow::Break(steps) = directions.iter().cycle().try_fold((0, starting_node), |(steps, curr_node), &dir| {
        if curr_node.name == "ZZZ" {
            ControlFlow::Break(steps)
        } else {
            ControlFlow::Continue((steps + 1, curr_node.get(dir)))
        }
    }) else {
        panic!("broke out of infinite loop without result?");
    };

    steps
}


#[aoc(day8, part2)]
fn part2(input: &str) -> u64 {
    let (directions, nodes) = parse(input);
    let mut node_map: HashMap<String, Rc<Node>> = HashMap::with_capacity(nodes.len());
    let mut starting_nodes = Vec::new();
    nodes.into_iter().for_each(|node| {
        let left = Rc::downgrade(node_map.entry(node.left.clone()).or_insert(Rc::new(node.left.as_str().into())));
        let right = Rc::downgrade(node_map.entry(node.right.clone()).or_insert(Rc::new(node.right.as_str().into())));

        let new_node = node_map.entry(node.name.clone()).or_insert(Rc::new(node.name.as_str().into()));
        new_node.left.replace(Some(left));
        new_node.right.replace(Some(right));

        if let Some(NodeType::Starting) = new_node.location {
            starting_nodes.push(Rc::clone(new_node));
        }
    });

    starting_nodes.into_iter().map(|node| {
        if let ControlFlow::Break(fold_val) = directions.iter().cycle().try_fold(
            (node, 0u64),
            |(curr_node, steps), &dir| {
                if let Some(NodeType::Ending) = curr_node.location {
                    ControlFlow::Break(steps)
                } else {
                    ControlFlow::Continue((curr_node.get(dir), steps + 1))
                }
        }) {
            fold_val
        } else {
            panic!("broke out of infinite loop without result?");
        }
    }).reduce(num::integer::lcm).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"), 2);

        assert_eq!(part1("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"), 6);
    }

}