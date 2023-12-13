
use std::{iter::once, collections::HashMap};
// use rayon::prelude::*;

use itertools::Itertools;
use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, all_consuming, map}, character::complete::{line_ending, one_of, space1, digit1, char}, multi::{many1, separated_list1}, sequence::separated_pair};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown
}
impl TryFrom<char> for Spring {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err("Invalid character converted to Spring")
        }
    }
}
impl PartialEq<Spring> for &Spring {
    fn eq(&self, other: &Spring) -> bool {
        (*self).eq(other)
    }
    fn ne(&self, other: &Spring) -> bool {
        !self.eq(other)
    }
}
impl Spring {
    fn can_be(&self, status: Spring) -> bool {
        if self == Spring::Unknown {
            true
        } else {
            self == status
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SpringRow {
    springs: Vec<Spring>,
    rules: Vec<usize>
}
impl From<(Vec<Spring>, Vec<usize>)> for SpringRow {
    fn from((springs, rules): (Vec<Spring>, Vec<usize>)) -> Self {
        Self { springs, rules }
    }
}


fn parse_springs(input: &str) -> IResult<&str, Vec<Spring>> {
    many1(map_res(
        one_of(".#?"),
        |chr| chr.try_into()
    ))(input)
}

fn parse_rule(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |num| usize::from_str_radix(num, 10))(input)
}

fn parse_rules(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(char(','), parse_rule)(input)
}

fn parse_line(input: &str) -> IResult<&str, SpringRow> {
    map(
        separated_pair(parse_springs, space1, parse_rules),
        |parsed| parsed.into()
    )(input)
}

fn parse(input: &str) -> Vec<SpringRow> {
    let (_, output) = all_consuming(separated_list1(line_ending, parse_line))(input).unwrap();
    output
}

fn num_possibilities(input: &SpringRow) -> u64 {
    num_possibilities_memoized(input.clone(), &mut HashMap::new())
}

fn num_possibilities_memoized(input: SpringRow, memoized: &mut HashMap<SpringRow, u64>) -> u64 {
    let rules = &input.rules;
    let cells = &input.springs;
    // println!("input: {input:?}");

    if rules.len() == 0 && cells.iter().all(|cell| cell.can_be(Spring::Operational)) {
        // println!("no cells and no rules");
        return 1;
    }

    let min_remaining_cells = rules.iter().sum::<usize>() + rules.len() - 1;
    if cells.len() < min_remaining_cells {
        // println!("no possibilities left due to length");
        0
    } else if cells.len() == min_remaining_cells {
        // println!("maybe one possibility left due to length");
        let expected = rules.into_iter().map(|&rule| { vec![Spring::Damaged; rule] }).intersperse(vec![Spring::Operational]).flatten();
        if expected.zip_eq(cells).all(|(first, second)| { second.can_be(first) }) { 1 } else { 0 }
    } else {
        match memoized.get(&input).cloned() {
            Some(val) => val,
            None => {
                let (head_cell, tail_cells) = cells.split_first().unwrap();
                let (head_rule, tail_rules) = rules.split_first().unwrap();

                let mut possibilities = 0;
                // what is the first cell?
                // println!("head cell: {head_cell:?}");
                possibilities += if head_cell.can_be(Spring::Operational) {
                    // cell already decided and doesn't affect rules, strip it and recurse
                    // println!("recursing, stripped operational cell");
                    num_possibilities_memoized(
                        SpringRow { springs: tail_cells.into_iter().copied().collect(), rules: rules.to_vec() },
                        memoized
                    )
                } else { 0 };
                possibilities += if head_cell.can_be(Spring::Damaged) {
                    // are the next {head_rule} cells all damaged and the one after operational?
                    let (rule_head_cells, tail) = cells.split_at(head_rule.clone());
                    // println!("checking cells {rule_head_cells:?}");
                    
                    if rule_head_cells.into_iter().all(|e| e.can_be(Spring::Damaged)) {
                        if tail_rules.is_empty() {
                            // no more rules, don't need to check for any more empty cells
                            // println!("recursing, stripped matching last rule");
                            num_possibilities_memoized(
                                SpringRow { springs: tail.into_iter().copied().collect(), rules: tail_rules.to_vec() },
                                memoized
                            )
                        } else {
                            let (rule_tail_cell, tail_rule_cells) = tail.split_first().unwrap();
                            // the rule is satisfied and thus recurse into the remaining cells/rules
                            if rule_tail_cell.can_be(Spring::Operational) {
                                // println!("recursing, stripped matching rule");
                                num_possibilities_memoized(
                                    SpringRow { springs: tail_rule_cells.into_iter().copied().collect(), rules: tail_rules.to_vec() },
                                    memoized
                                )
                            } else {
                                0
                            }
                        }
                    } else {
                        // rule doesn't work here, return no possibilities
                        // println!("failed recursion");
                        0
                    }
                } else { 0 };
                memoized.insert(input, possibilities);

                possibilities
            }
        }
    }
}

#[aoc(day12, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);
    
    input.into_iter().fold(0, |sum, row| {
        sum + num_possibilities(&row)
    })
}

#[aoc(day12, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    let input = input.into_iter().map(|row| {
        let rules = row.rules.into_iter();
        let new_rules = rules.clone().chain(rules.clone()).chain(rules.clone()).chain(rules.clone()).chain(rules);
        let cells = row.springs.into_iter();
        let cells_with_gap = cells.clone().chain(once(Spring::Unknown));
        let new_cells = cells_with_gap.clone().chain(cells_with_gap.clone()).chain(cells_with_gap.clone()).chain(cells_with_gap).chain(cells);
        (new_cells.collect_vec(), new_rules.collect_vec()).into()
    }).collect_vec();
    
    input.into_iter().fold(0, |sum, row| {
        sum + num_possibilities(&row)
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"), 525152);
    }

}