
use std::collections::{BinaryHeap, HashMap};

use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{all_consuming, map_res}, character::complete::{one_of, multispace1}, multi::{separated_list1, many1}};


fn parse_line(input: &str) -> IResult<&str, Vec<u64>> {
    many1(map_res(
        one_of("0123456789"),
        |chr| u64::from_str_radix(&chr.to_string(), 10)
    ))(input)
}
fn parse(input: &str) -> Vec<Vec<u64>> {
    let (_, output) = all_consuming(separated_list1(multispace1, parse_line))(input).unwrap();
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
    fn travel(&self, (x, y): (usize, usize)) -> Option<(usize, usize)> {
        use Direction::*;
        match self {
            North => Some((x, y.checked_sub(1)?)),
            East => Some((x + 1, y)),
            South => Some((x, y + 1)),
            West => Some((x.checked_sub(1)?, y)),
        }
    }
    fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
    fn iter() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [ North, East, South, West ].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TravelHistory {
    dir: Direction,
    count: u8
}
impl TravelHistory {
    fn add_step(self, dir: Direction) -> Self {
        TravelHistory { dir, count: if self.dir == dir { self.count + 1 } else { 1 } }
    }
    fn can_travel(self, dir: Direction) -> bool {
        if self.dir == dir.opposite() {
            false
        } else if self.dir == dir && self.count >= 3 {
            false
        } else {
            true
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SearchNode {
    loc: (usize, usize),
    cost: u64,
    history: TravelHistory,
    heuristic: u64,
}
impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}
impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl SearchNode {
    fn new(loc: (usize, usize), cost: u64, history: TravelHistory, (trg_x, trg_y): (usize, usize)) -> Self {
        let (loc_x, loc_y) = loc;
        Self {loc, cost, history, heuristic: cost + (trg_x - loc_x) as u64 + (trg_y - loc_y) as u64}
    }
}


#[aoc(day17, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);
    let max_x = input[0].len();
    let max_y = input.len();
    let target_loc = (max_x - 1, max_y - 1);

    let mut search_nodes = BinaryHeap::new();
    let mut min_cost = HashMap::with_capacity(max_x + max_y);
    Direction::iter().for_each(|dir| {
        min_cost.insert(((0, 0), TravelHistory { dir, count: 0 }), 0);
    });
    // first step does not have any direction to use for the travel history
    // enumerate neighbors and add to search_nodes manually, then start iteration
    search_nodes.push(SearchNode::new((0, 1), input[1][0], TravelHistory { dir: Direction::South, count: 1 }, target_loc));
    search_nodes.push(SearchNode::new((1, 0), input[0][1], TravelHistory { dir: Direction::East, count: 1 }, target_loc));
    while let Some(node) = search_nodes.pop() {
        if (0..=node.history.count).filter_map(|dir_steps| {
            let more_permissive_history = TravelHistory { dir: node.history.dir, count: dir_steps };
            min_cost.get(&(node.loc, more_permissive_history))
        }).any(|&past_cost| past_cost < node.cost) {
            continue;
        }

        min_cost.insert((node.loc, node.history), node.cost);
        if node.loc == target_loc {
            return node.cost;
        }

        // what directions can we travel?
        let possible_dirs = Direction::iter().filter(|&dir| node.history.can_travel(dir));
        let next_locs = possible_dirs.filter_map(|dir| dir.travel(node.loc).and_then(|l| Some((dir, l))));
        // next_locs can still contain items off the grid
        next_locs.for_each(|(dir, new_loc)| {
            let (new_x, new_y) = new_loc;
            if let Some(&node_cost) = input.get(new_y).and_then(|row| row.get(new_x)) {
                search_nodes.push(SearchNode::new(new_loc, node.cost + node_cost, node.history.add_step(dir), target_loc));
            }
        });
    }

    panic!("goal not reached?!");
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UltraCrucibleTravel {
    dir: Direction,
    count: u8
}
impl UltraCrucibleTravel {
    fn add_step(self, dir: Direction) -> Self {
        UltraCrucibleTravel { dir, count: if self.dir == dir { self.count + 1 } else { 1 } }
    }
    fn can_travel(self, dir: Direction) -> bool {
        if self.dir == dir.opposite() {
            false
        } else if self.dir == dir {
            self.count < 10
        } else {
            self.count >= 4
        }
    }
    fn can_stop(self) -> bool {
        self.count >= 4
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UltraCrucibleNode {
    loc: (usize, usize),
    cost: u64,
    history: UltraCrucibleTravel,
    heuristic: u64,
}
impl Ord for UltraCrucibleNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}
impl PartialOrd for UltraCrucibleNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl UltraCrucibleNode {
    fn new(loc: (usize, usize), cost: u64, history: UltraCrucibleTravel, (trg_x, trg_y): (usize, usize)) -> Self {
        let (loc_x, loc_y) = loc;
        Self {loc, cost, history, heuristic: cost + (trg_x - loc_x) as u64 + (trg_y - loc_y) as u64}
    }
    fn can_stop(&self) -> bool {
        self.history.can_stop()
    }
}


#[aoc(day17, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);
    let max_x = input[0].len();
    let max_y = input.len();
    let target_loc = (max_x - 1, max_y - 1);

    let mut search_nodes = BinaryHeap::new();
    let mut min_cost = HashMap::with_capacity(max_x + max_y);
    Direction::iter().for_each(|dir| {
        min_cost.insert(((0, 0), UltraCrucibleTravel { dir, count: 0 }), 0);
    });
    // first step does not have any direction to use for the travel history
    // enumerate neighbors and add to search_nodes manually, then start iteration
    search_nodes.push(UltraCrucibleNode::new((0, 1), input[1][0], UltraCrucibleTravel { dir: Direction::South, count: 1 }, target_loc));
    search_nodes.push(UltraCrucibleNode::new((1, 0), input[0][1], UltraCrucibleTravel { dir: Direction::East, count: 1 }, target_loc));
    while let Some(node) = search_nodes.pop() {
        if node.can_stop() {
            if node.loc == target_loc {
                return node.cost;
            }

            if (0..=node.history.count).filter_map(|dir_steps| {
                let alt_history = UltraCrucibleTravel { dir: node.history.dir, count: dir_steps };
                min_cost.get(&(node.loc, alt_history))
            }).any(|&past_cost| past_cost < node.cost) {
                continue;
            }

            min_cost.insert((node.loc, node.history), node.cost);
        }

        // what directions can we travel?
        let possible_dirs = Direction::iter().filter(|&dir| node.history.can_travel(dir));
        let next_locs = possible_dirs.filter_map(|dir| dir.travel(node.loc).and_then(|l| Some((dir, l))));
        // next_locs can still contain items off the grid
        next_locs.for_each(|(dir, new_loc)| {
            let (new_x, new_y) = new_loc;
            if let Some(&node_cost) = input.get(new_y).and_then(|row| row.get(new_x)) {
                // println!("travelling {:?} to search node at loc {:?}", dir, target_loc);
                search_nodes.push(UltraCrucibleNode::new(new_loc, node.cost + node_cost, node.history.add_step(dir), target_loc));
            }
        });
    }

    panic!("goal not reached?!");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"), 102);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"), 94);
        assert_eq!(part2("111111111111
999999999991
999999999991
999999999991
999999999991"), 71);
    }

}