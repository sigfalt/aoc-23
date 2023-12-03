use std::collections::HashMap;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{all_consuming, map_res, map}, multi::{many1_count, many1}, character::complete::{one_of, digit1, char, line_ending}, branch::alt};


#[derive(Debug, PartialEq)]
enum MatchType {
    Empty(u32),
    Digit(u32),
    Gear,
    Symbol,
    Newline
}

fn parse_empty(input: &str) -> IResult<&str, MatchType> {
    map(
        many1_count(char('.')),
        |len| MatchType::Empty(len as u32)
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, MatchType> {
    map(
        map_res(digit1, |num| u32::from_str_radix(num, 10)),
        |num| MatchType::Digit(num)
    )(input)
}

fn parse_gear(input: &str) -> IResult<&str, MatchType> {
    map(
        char('*'),
        |_| MatchType::Gear
    )(input)
}

fn parse_symbol(input: &str) -> IResult<&str, MatchType> {
    map(
        one_of("@#$%&-=+/"),
        |_| MatchType::Symbol
    )(input)
}

fn parse_newline(input: &str) -> IResult<&str, MatchType> {
    map(
        line_ending,
        |_| MatchType::Newline
    )(input)
}

fn parse(input: &str) -> Vec<MatchType> {
    all_consuming(
        many1(alt((parse_empty, parse_number, parse_gear, parse_symbol, parse_newline)))
    )(input).unwrap().1
}

#[aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    let input = parse(input);

    let mut symbol_locs = vec![];
    let mut digit_adj_map: HashMap<u32, Vec<Vec<(i32, i32)>>> = HashMap::new();
    let mut pos: (i32, i32) = (0, 0); // row, col
    input.into_iter().for_each(|e| {
        match e {
            MatchType::Digit(num) => {
                let mut neighbors_to_check = vec![];

                let num_len = (num.checked_ilog10().unwrap_or(0) + 1) as i32;
                for i in -1..=1 {
                    neighbors_to_check.append(&mut vec![(pos.0 + i, pos.1 - 1), (pos.0 + i, pos.1 + num_len)]);
                }
                for i in 0..num_len {
                    neighbors_to_check.append(&mut vec![(pos.0 - 1, pos.1 + i), (pos.0 + 1, pos.1 + i)]);
                }

                if let Some(existing_adj_lists) = digit_adj_map.get_mut(&num) {
                    existing_adj_lists.push(neighbors_to_check);
                } else {
                    digit_adj_map.insert(num, vec![neighbors_to_check]);
                }
                pos.1 += num_len;
            },
            MatchType::Symbol | MatchType::Gear => {
                symbol_locs.push(pos);
                pos.1 += 1;
            },
            MatchType::Empty(len) => { pos.1 += len as i32; },
            MatchType::Newline => { pos = (pos.0 + 1, 0); }
        };
    });

    digit_adj_map.into_iter().map(|(k, v)| {
        v.into_iter().map(|instance|
            if instance.into_iter().any(|pos| symbol_locs.contains(&pos)) { k } else { 0 }
        ).sum::<u32>()
    }).sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    let input = parse(input);

    let mut gear_locs = vec![];
    let mut adj_to_head_map = HashMap::new();
    let mut head_pos_map = HashMap::new();
    let mut pos: (i32, i32) = (0, 0); // row, col
    input.into_iter().for_each(|e| {
        match e {
            MatchType::Digit(num) => {
                head_pos_map.insert(pos, num);
                let num_len = (num.checked_ilog10().unwrap_or(0) + 1) as i32;
                for i in 0..num_len {
                    adj_to_head_map.insert((pos.0, pos.1 + i), pos);
                }
                pos.1 += num_len;
            },
            MatchType::Gear => {
                gear_locs.push(pos);
                pos.1 += 1;
            },
            MatchType::Symbol => { pos.1 += 1; },
            MatchType::Empty(len) => { pos.1 += len as i32; },
            MatchType::Newline => { pos = (pos.0 + 1, 0); }
        };
    });

    gear_locs.into_iter().map(|pos| {
        let mut adj_pos = vec![];
        for i in -1..=1 {
            for j in -1..=1 {
                if i != 0 || j != 0 {
                    adj_pos.push((pos.0 + i, pos.1 + j));
                }
            }
        }

        let adj_num_heads = adj_pos.into_iter().filter_map(|pos| {
            adj_to_head_map.get(&pos)
        }).sorted_unstable().dedup().collect_vec();
        if adj_num_heads.len() == 2 {
            head_pos_map[adj_num_heads[0]] * head_pos_map[adj_num_heads[1]]
        } else {
            0
        }
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        use MatchType::*;
        assert_eq!(parse_empty("......"), Ok(("", Empty(6))));
        assert_eq!(parse_number("467"), Ok(("", Digit(467))));
        assert_eq!(parse_gear("*"), Ok(("", Gear)));
        assert_eq!(parse_symbol("/"), Ok(("", Symbol)));
        assert_eq!(parse_newline("
"), Ok(("", Newline)));

        assert_eq!(parse("467..114.*."), vec![
            Digit(467), Empty(2), Digit(114), Empty(1), Gear, Empty(1)
        ]);
        assert_eq!(parse("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), vec![
            Digit(467), Empty(2), Digit(114), Empty(2), Newline,
            Empty(3), Gear, Empty(6), Newline,
            Empty(2), Digit(35), Empty(2), Digit(633), Empty(1), Newline,
            Empty(6), Symbol, Empty(3), Newline,
            Digit(617), Gear, Empty(6), Newline,
            Empty(5), Symbol, Empty(1), Digit(58), Empty(1), Newline,
            Empty(2), Digit(592), Empty(5), Newline,
            Empty(6), Digit(755), Empty(1), Newline,
            Empty(3), Symbol, Empty(1), Gear, Empty(4), Newline,
            Empty(1), Digit(664), Empty(1), Digit(598), Empty(2)
        ]);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 4361);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."), 467835);
    }

}