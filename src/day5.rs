use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{map_res, all_consuming}, multi::{separated_list1, many0, many1}, character::complete::{digit1, space1, line_ending}, sequence::{tuple, pair, preceded, terminated}, bytes::complete::{tag, is_not}};


fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |num| u64::from_str_radix(num, 10))(input)
}

fn parse_seed_list(input: &str) -> IResult<&str, Vec<u64>> {
    preceded(
        tag("seeds: "),
        separated_list1(
            space1,
            parse_u64
        )
    )(input)
}

#[derive(Debug, PartialEq)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    range_len: u64
}
impl TryFrom<Vec<u64>> for Mapping {
    type Error = String;

    fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
        if value.len() != 3 {
            Err(format!("Mapping created with {} values, expected 3", value.len()))
        } else {
            Ok(Mapping {
                dest_start: value[0],
                src_start: value[1],
                range_len: value[2]
            })
        }
    }
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    map_res(
        separated_list1(
            space1,
            parse_u64
        ),
        |list| Mapping::try_from(list)
    )(input)
}

fn parse_mapping_list(input: &str) -> IResult<&str, Vec<Mapping>> {
    separated_list1(line_ending, parse_mapping)(input)
}

fn parse_map(input: &str) -> IResult<&str, Vec<Mapping>> {
    preceded(
        tuple((is_not(" "), space1, tag("map:"), line_ending)),
        parse_mapping_list
    )(input)
}

fn line_ending1(input: &str) -> IResult<&str, Vec<&str>> {
    many1(line_ending)(input)
}

fn parse_all_maps(input: &str) -> IResult<&str, Vec<Vec<Mapping>>> {
    separated_list1(
        line_ending1,
        parse_map
    )(input)
}

fn line_ending0(input: &str) -> IResult<&str, Vec<&str>> {
    many0(line_ending)(input)
}

fn parse(input: &str) -> (Vec<u64>, Vec<Vec<Mapping>>) {
    let (_, result) = all_consuming(pair(
        terminated(parse_seed_list, line_ending0),
        terminated(parse_all_maps, line_ending0)
    ))(input).unwrap();

    result
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    let (seed_list, maps_vec) = parse(input);

    seed_list.into_iter().map(|seed| {
        maps_vec.iter().fold(seed, |src, map| {
            map.iter().find_map(|mapping| {
                if src >= mapping.src_start && src < mapping.src_start + mapping.range_len {
                    Some(src - mapping.src_start + mapping.dest_start)
                } else {
                    None
                }
            }).unwrap_or(src)
        })
    }).min().unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    let (seed_list, maps_vec) = parse(input);

    seed_list.iter().tuples().flat_map(
        |(&start, &end)| (start..start+end)
    ).map(|seed| {
        maps_vec.iter().fold(seed, |src, map| {
            map.iter().find_map(|mapping| {
                if src >= mapping.src_start && src < mapping.src_start + mapping.range_len {
                    Some(src - mapping.src_start + mapping.dest_start)
                } else {
                    None
                }
            }).unwrap_or(src)
        })
    }).min().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        assert_eq!(parse_seed_list("seeds: 79 14 55 13"), Ok(("", vec![79, 14, 55, 13])));
        assert_eq!(parse_map("seed-to-soil map:
50 98 2
52 50 48"), Ok(("", vec![
            Mapping {
                dest_start: 50,
                src_start: 98,
                range_len: 2
            }, Mapping {
                dest_start: 52,
                src_start: 50,
                range_len: 48
            }
        ])));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"), 46);
    }

}