
use std::{collections::{HashMap, HashSet, VecDeque}, ops::Add};

use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{all_consuming, map_res}, character::complete::{multispace1, one_of}, multi::{separated_list1, many1}};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Ground {
    Garden,
    Rock,
    Start
}
impl TryFrom<char> for Ground {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Ground::*;
        match value {
            '.' => Ok(Garden),
            '#' => Ok(Rock),
            'S' => Ok(Start),
            _ => Err("Invalid ground character"),
        }
    }
}


fn parse_line(input: &str) -> IResult<&str, Vec<Ground>> {
    many1(map_res(one_of(".#S"), Ground::try_from))(input)
}
fn parse(input: &str) -> Vec<Vec<Ground>> {
    let (_, output) = all_consuming(separated_list1(multispace1, parse_line))(input).unwrap();
    output
}


#[aoc(day21, part1)]
fn part1(input: &str) -> u64 {
    part1_with_steps(input, 64)
}
fn part1_with_steps(input: &str, target: usize) -> u64 {
    let input = parse(input);

    let mut start_pos = None;
    'outer: for (row_ix, row) in input.iter().enumerate() {
        for (col_ix, &col) in row.iter().enumerate() {
            if col == Ground::Start {
                start_pos = Some((row_ix, col_ix));
                break 'outer;
            }
        }
    }
    let start_pos = start_pos.unwrap(); // discard mut
    let mut neighbor_map = HashMap::with_capacity(target);
    let mut possible_positions = HashSet::from([start_pos]);
    for _ in 0..target {
        let curr_positions = possible_positions.drain().collect_vec();
        curr_positions.into_iter().for_each(|pos| {
            if !neighbor_map.contains_key(&pos) {
                let (row, col) = pos;
                let row_range = if row > 0 { row-1..row+2 } else { 0..2 };
                let col_range = if col > 0 { col-1..col+2 } else { 0..2 };
                let new_neighbors = row_range.map(|r| (r, col))
                    .chain(col_range.map(|c| (row, c)))
                    .filter(|&test_pos| test_pos != pos)
                    .filter(|&(test_row_ix, test_col_ix)| {
                        if let Some(&ground_cell) = input.get(test_row_ix).and_then(|test_row| test_row.get(test_col_ix)) {
                            ground_cell != Ground::Rock
                        } else {
                            false
                        }
                    }
                ).collect_vec();
                neighbor_map.insert(pos, new_neighbors.clone());
            }
            neighbor_map.get(&pos).unwrap().iter().for_each(|&neighbor_pos| {
                possible_positions.insert(neighbor_pos);
            });
        });
    }
    
    possible_positions.len() as u64
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GardenPos {
    row: usize,
    col: usize,
}
impl From<(usize, usize)> for GardenPos {
    fn from((row, col): (usize, usize)) -> Self {
        Self { row, col }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct PlanarPos {
    x: i32,
    y: i32,
}
impl Add for PlanarPos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Parity {
    Even,
    Odd
}
impl Parity {
    fn opposite(&self) -> Self {
        use Parity::*;
        match self {
            Even => Odd,
            Odd => Even
        }
    }
}
impl From<usize> for Parity {
    fn from(value: usize) -> Self {
        use Parity::*;
        match value % 2 {
            0 => Even,
            1 => Odd,
            _ => unreachable!()
        }
    }
}
impl Add for Parity {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        use Parity::*;
        match self {
            Even => rhs,
            Odd => rhs.opposite()
        }
    }
}


#[aoc(day21, part2)]
fn part2(input: &str) -> u64 {
    part2_with_steps(input, 26501365)
}
fn part2_with_steps(input: &str, target: usize) -> u64 {
    println!("target: {target}");
    let input = parse(input);
    let max_row = input[0].len() - 1;
    let max_col = input.len() - 1;

    let mut start_loc = None;
    let mut open_positions = HashSet::new();
    input.iter().enumerate().for_each(|(row_ix, row)| {
        row.iter().enumerate().for_each(|(col_ix, &col)| {
            if col != Ground::Rock {
                open_positions.insert(GardenPos { row: row_ix, col: col_ix });
                if col == Ground::Start {
                    start_loc = Some((GardenPos {row: row_ix, col: col_ix}, PlanarPos {x: 0, y: 0}));
                }
            }
        });
    });
    let (start_pos, start_plane) = start_loc.unwrap(); // discard mut

    // precalculate neighbor map
    let neighbor_map = HashMap::<_, _>::from_iter(open_positions.iter().map(|&pos| {
        let GardenPos { row, col } = pos;
        let possible_neighbors = vec![
            // row-1, col
            if row == 0 {(GardenPos { row: max_row, col }, Some(PlanarPos { x: 0, y: 1 }))}
                else {(GardenPos { row: row - 1, col }, None)},
            // row+1, col
            if row == max_row {(GardenPos { row: 0, col }, Some(PlanarPos { x: 0, y: -1 }))}
                else {(GardenPos { row: row + 1, col }, None)},
            // row, col-1
            if col == 0 {(GardenPos { row, col: max_col }, Some(PlanarPos { x: -1, y: 0 }))}
                else {(GardenPos { row, col: col - 1 }, None)},
            // row, col+1
            if col == max_col {(GardenPos { row, col: 0 }, Some(PlanarPos { x: 1, y: 0 }))}
                else {(GardenPos { row, col: col + 1 }, None)}
        ];
        (pos, possible_neighbors.into_iter().filter(|&(test_pos, _)| {
            let GardenPos { row: test_row_ix, col: test_col_ix } = test_pos;
            if let Some(&ground_cell) = input.get(test_row_ix).and_then(|test_row| test_row.get(test_col_ix)) {
                ground_cell != Ground::Rock
            } else {
                false
            }
        }).collect_vec())
    }));
    // precalculate min distances to cover
    let min_distances_from_start = HashMap::<_, _>::from_iter(open_positions.iter().filter_map(|&pos| {
        let GardenPos { row: row_ix, col: col_ix } = pos;
        if row_ix == 0 || row_ix == max_row || col_ix == 0 || col_ix == max_col || pos == start_pos {
            let mut max_dist = 0;
            let mut parity_counts = HashMap::from([(Parity::Even, 1), (Parity::Odd, 0)]);
            let mut planar_neighbors = HashMap::new();
            let mut curr_parity = Parity::Even;
            let mut found_locs = HashSet::from([pos]);
            while found_locs != open_positions {
                max_dist += 1;
                curr_parity = curr_parity.opposite();
                let curr_locs = found_locs.clone();
                curr_locs.into_iter().for_each(|loc| {
                    let neighbors = neighbor_map.get(&loc).unwrap();
                    neighbors.iter().for_each(|&(neighbor_pos, neighbor_plane_adj)| {
                        if let Some(neighbor_plane_adj) = neighbor_plane_adj {
                            let neighbor_entry = planar_neighbors.entry(neighbor_plane_adj).or_insert(HashMap::new());
                            neighbor_entry.entry(neighbor_pos)
                                    .and_modify(|prev_min| {
                                        let (prev_max_step, _) = prev_min;
                                        if max_dist < *prev_max_step {
                                            *prev_min = (max_dist, curr_parity);
                                        }
                                    }).or_insert((max_dist, curr_parity));
                        } else {
                            if !found_locs.contains(&neighbor_pos) {
                                found_locs.insert(neighbor_pos);
                                parity_counts.entry(curr_parity).and_modify(|cnt| *cnt += 1);
                            }
                        }
                    })
                });
            }
            Some((pos, (max_dist, parity_counts, planar_neighbors)))
        } else {
            None
        }
    }));

    let mut possible_positions = 0;
    let mut maps_to_check = VecDeque::from([(start_plane, vec![(start_pos, Parity::Even, target)], None)]);
    let mut incomplete_planes = HashMap::with_capacity(1);
    let mut complete_planes = HashSet::new();
    let target_parity = Parity::from(target);

    while let Some((map_start_plane, map_starting_positions, triggered_by)) = maps_to_check.pop_front() {
        
        if complete_planes.contains(&map_start_plane) {
            continue;
        }

        if let Some((precalc_parity, neighbors)) = map_starting_positions.iter().fold(None, |acc, &(start_pos, start_parity, max_steps)| {
            if let Some((steps_req, parity_counts, planar_neighbors)) = min_distances_from_start.get(&start_pos) {
                if max_steps > *steps_req {
                    if let Some((parity_map, prev_planar_neighbors)) = acc {
                        // parity map should never change

                        // merge acc with this data
                        // check planar_neighbors to see if any can start with a lower step_count
                        // just fuckin ignore all this shit for now and hope the first one we found is good enough
                        // planar_neighbors.iter().for_each(|(&planar_offset, planar_starting_info)| {});
                        return Some((parity_map, prev_planar_neighbors));
                    } else {
                        // map planar_neighbors offsets to actual planar_coords
                        // need to feed (PlanarPos, HashMap<GardenPos, (usize, Parity)>) to rest of fold
                        return Some((parity_counts.iter().map(|(&orig_parity, &cell_count)| {
                            (start_parity + orig_parity, cell_count)
                        }).collect::<HashMap<_, _>>(), planar_neighbors.iter().map(|(&planar_offset, planar_starting_info)| {
                            (map_start_plane + planar_offset, planar_starting_info.iter().map(|(&neighbor_starting_pos, &(step_offset, step_parity))| {
                                (neighbor_starting_pos, (max_steps - step_offset, start_parity + step_parity))
                            }).collect::<HashMap<_, _>>())
                        }).collect::<HashMap<_, _>>()));
                    }
                }
            }
            acc
        }) {
            println!("{map_start_plane:?} plane prefilled");
            complete_planes.insert(map_start_plane);
            possible_positions += precalc_parity.get(&target_parity).unwrap();
            incomplete_planes.remove(&map_start_plane);
            neighbors.into_iter().for_each(|(neighbor_plane, neighbor_pos_map)| {
                maps_to_check.push_back((neighbor_plane,
                neighbor_pos_map.into_iter().map(|(neighbor_pos, (max_steps, neighbor_parity))|
                    (neighbor_pos, neighbor_parity, max_steps)
                ).collect_vec(), Some(map_start_plane)));
            });
            continue;
        }

        let position_parity_map = incomplete_planes.entry(map_start_plane).or_insert(HashMap::from_iter(
            map_starting_positions.iter().map(|&(map_pos, parity, _)| (map_pos, parity)
        )));

        println!("running plane {map_start_plane:?} with starting positions {map_starting_positions:?}");
        let mut planar_neighbors_to_check = HashMap::new();

        map_starting_positions.into_iter().for_each(|(starting_pos, starting_parity, max_steps)| {
            let mut known_steps_from_start = HashSet::from([starting_pos]);
            for curr_step in 1..=max_steps {
                let curr_steps = known_steps_from_start.clone();
                curr_steps.into_iter().for_each(|pos| {
                    let neighbors = neighbor_map.get(&pos).unwrap();
                    neighbors.iter().for_each(|&(neighbor_pos, neighbor_plane_adj)| {
                        let curr_parity = starting_parity + Parity::from(curr_step);
                        if let Some(neighbor_plane_adj) = neighbor_plane_adj {
                            let neighbor_plane = map_start_plane + neighbor_plane_adj;
                            if triggered_by.map_or(true, |pl: PlanarPos| pl != neighbor_plane) {
                                let curr_max = max_steps - curr_step;
                                let neighbor_positions = planar_neighbors_to_check.entry(neighbor_plane).or_insert(HashMap::new());
                                neighbor_positions.entry(neighbor_pos)
                                    .and_modify(|prev_max| {
                                        let (prev_max_step, _) = prev_max;
                                        if curr_max > *prev_max_step {
                                            *prev_max = (curr_max, curr_parity);
                                        }
                                    }).or_insert((curr_max, curr_parity));
                            }
                        } else {
                            known_steps_from_start.insert(neighbor_pos);
                            if !position_parity_map.contains_key(&neighbor_pos) {
                                position_parity_map.insert(neighbor_pos, curr_parity);
                            }
                        }
                    });
                });
            }
        });

        if position_parity_map.keys().cloned().collect::<HashSet<_>>() == open_positions {
            println!("plane filled");
            complete_planes.insert(map_start_plane);
            possible_positions += position_parity_map.clone().into_values().fold(0, |acc, pos_parity| {
                if pos_parity == target_parity {
                    acc + 1
                } else {
                    acc
                }
            });
            incomplete_planes.remove(&map_start_plane);
        }

        // println!("planar_neighbors: {planar_neighbors_to_check:#?}");
        planar_neighbors_to_check.into_iter().for_each(|(neighbor_plane, neighbor_pos_map)| {
            maps_to_check.push_back((neighbor_plane,
            neighbor_pos_map.into_iter().map(|(neighbor_pos, (max_steps, neighbor_parity))|
                (neighbor_pos, neighbor_parity, max_steps)
            ).collect_vec(), Some(map_start_plane)));
        });

        println!();
    }

    // only complete planes add their spots in advance to possible_positions
    // enumerate through incomplete planes to calculate their contribution
    incomplete_planes.into_values().fold(possible_positions, |acc, parity_map: HashMap<_, _>| {
        parity_map.into_values().fold(acc, |acc, pos_parity| {
            if pos_parity == target_parity {
                acc + 1
            } else {
                acc
            }
        })
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1_with_steps("...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........", 6), 16);
    }

    #[test]
    fn part2_example() {
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 1), 2);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 2), 4);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 3), 6);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 6), 16);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 10), 50);
        assert_eq!(part2_with_steps("...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........", 50), 1594);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 100), 6536);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 500), 167004);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 1000), 668697);
//         assert_eq!(part2_with_steps("...........
// .....###.#.
// .###.##..#.
// ..#.#...#..
// ....#.#....
// .##..S####.
// .##..#...#.
// .......##..
// .##.#.####.
// .##..##.##.
// ...........", 5000), 16733044);
    }

}