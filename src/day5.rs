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
            Ok(Self {
                dest_start: value[0],
                src_start: value[1],
                range_len: value[2]
            })
        }
    }
}
impl From<(u64, u64, u64)> for Mapping {
    fn from((dest_start, src_start, range_len): (u64, u64, u64)) -> Self {
        Self { dest_start, src_start, range_len }
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

#[aoc(day5, part2, naive)]
fn part2_naive(input: &str) -> u64 {
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

fn process_ranges(input_pairs: &mut Vec<(u64, u64)>, mappings: &Vec<Mapping>) -> Vec<(u64, u64)> {
    let mut output_pairs = vec![];
    'grab_input: while let Some((seed_start, seed_range_len)) = input_pairs.pop() {
        let seed_end = seed_start + seed_range_len;
        // three scenarios we care about:
        //   - first is contained in a mapping
        //   - second is contained in a mapping, but first is not
        //   - a mapping is entirely contained between first and second
        for mapping in mappings {
            let map_src_end = mapping.src_start + mapping.range_len;
            if seed_start >= mapping.src_start && seed_start < map_src_end {
                // is second also contained in the mapping?
                // if not, figure out what portion doesn't overlap and reprocess
                if seed_end <= map_src_end {
                    output_pairs.push((seed_start - mapping.src_start + mapping.dest_start, seed_range_len));
                } else {
                    output_pairs.push((seed_start - mapping.src_start + mapping.dest_start, map_src_end - seed_start));
                    input_pairs.push((map_src_end, seed_end - mapping.src_start - mapping.range_len));
                }
                continue 'grab_input;
            }

            if seed_end > mapping.src_start && seed_end < map_src_end {
                output_pairs.push((mapping.dest_start, seed_range_len + seed_start - mapping.src_start));
                input_pairs.push((seed_start, mapping.src_start - seed_start));
                continue 'grab_input;
            }

            if seed_start <= mapping.src_start && seed_end >= map_src_end {
                output_pairs.push((mapping.dest_start, mapping.range_len));
                input_pairs.push((seed_start, mapping.src_start - seed_start));
                if seed_end - map_src_end > 0 {
                    input_pairs.push((map_src_end, seed_end - map_src_end));
                }
                
                continue 'grab_input;
            }
        }
        
        // if no mappings overlap with this range, it is not mapped and continues to the next stage as-is
        output_pairs.push((seed_start, seed_range_len));
    }
    output_pairs
}

#[aoc(day5, part2, fast)]
fn part2_fast(input: &str) -> u64 {
    let (seed_list, maps_vec) = parse(input);

    maps_vec.into_iter().fold(seed_list.into_iter().tuples().collect_vec(), |mut seeds, mappings| {
        process_ranges(&mut seeds, &mappings)
    }).into_iter().map(|(range_start, _)| range_start).min().unwrap()
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
            Mapping::from((50, 98, 2)),
            Mapping::from((52, 50, 48))
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
    fn part2_naive_example() {
        assert_eq!(part2_naive("seeds: 79 14 55 13

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

    #[test]
    fn test_process_ranges() {
        assert_eq!(process_ranges(&mut vec![(79, 14), (55, 13)], &vec![
            Mapping::from((50, 98, 2)),
            Mapping::from((52, 50, 48)),
        ]), vec![(57, 13), (81, 14)]);
        assert_eq!(process_ranges(&mut vec![(57, 13), (81, 14)], &vec![
            Mapping::from((0, 15, 37)),
            Mapping::from((37, 52, 2)),
            Mapping::from((39, 0, 15))
        ]), vec![(81, 14), (57, 13)]);
        assert_eq!(process_ranges(&mut vec![(81, 14), (57, 13)], &vec![
            Mapping::from((49, 53, 8)),
            Mapping::from((0, 11, 42)),
            Mapping::from((42, 0, 7)),
            Mapping::from((57, 7, 4))
        ]), vec![(53, 4), (61, 9), (81, 14)]);
        assert_eq!(process_ranges(&mut vec![(53, 4), (61, 9), (81, 14)], &vec![
            Mapping::from((88, 18, 7)),
            Mapping::from((18, 25, 70))
        ]), vec![(74, 14), (54, 9), (46, 4)]);
        assert_eq!(process_ranges(&mut vec![(74, 14), (54, 9), (46, 4)], &vec![
            Mapping::from((45, 77, 23)),
            Mapping::from((81, 45, 19)),
            Mapping::from((68, 64, 13))
        ]), vec![(82, 4), (90, 9), (45, 11), (78, 3)]);
    }

    #[test]
    fn part2_fast_example() {
        assert_eq!(part2_fast("seeds: 79 14 55 13

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
56 93 4
"), 46);
    }

}