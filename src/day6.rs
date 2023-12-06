use anyhow::Result;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{map_res, all_consuming}, multi::{separated_list1, fold_many1}, character::complete::{digit1, space1, alpha1, char, line_ending}, sequence::{preceded, tuple, separated_pair, pair}};


fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |num| u32::from_str_radix(num, 10))(input)
}

fn parse_num_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, parse_u32)(input)
}

fn parse_input_line(input: &str) -> IResult<&str, Vec<u32>> {
    preceded(
        tuple((
            alpha1,
            char(':'),
            space1
        )),
        parse_num_list
    )(input)
}

#[derive(Debug, PartialEq)]
struct Race {
    time: u64,
    distance: u64
}
impl From<(u32, u32)> for Race {
    fn from((time, distance): (u32, u32)) -> Self {
        Self{time: time.into(), distance: distance.into()}
    }
}
impl TryFrom<(i32, i32)> for Race {
    type Error = anyhow::Error;
    fn try_from((time, distance): (i32, i32)) -> Result<Race> {
        Ok(Self{time: time.try_into()?, distance: distance.try_into()?})
    }
}
impl From<(u64, u64)> for Race {
    fn from((time, distance): (u64, u64)) -> Self {
        Self{time, distance}
    }
}

fn parse_part1(input: &str) -> impl Iterator<Item = Race> {
    let (_, (times, distances)) = all_consuming(separated_pair(
        parse_input_line,
        line_ending,
        parse_input_line
    ))(input).unwrap();

    times.into_iter().zip_eq(distances.into_iter()).map(|r| r.into())
}

fn parse_separated_u64(input: &str) -> IResult<&str, u64> {
    map_res(fold_many1(
        preceded(space1, digit1),
        String::new,
        |acc, digits| {
            acc + digits
        }
    ), |num| u64::from_str_radix(&num, 10))(input)
}

fn parse_bad_kerning_line(input: &str) -> IResult<&str, u64> {
    preceded(
        pair(alpha1, char(':')),
        parse_separated_u64
    )(input)
}

fn parse_part2(input: &str) -> Race {
    let (_, race) = all_consuming(separated_pair(
        parse_bad_kerning_line,
        line_ending,
        parse_bad_kerning_line
    ))(input).unwrap();

    race.into()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u64 {
    let races = parse_part1(input);

    races.map(|race| {
        // check from 1 to time/2 as that will be the maximum distance
        // skip 0 as that will always result in 0 distance
        let min_time = (1..race.time / 2).find(
            |button_time| button_time * (race.time - button_time) > race.distance
        ).unwrap();
        ((race.time / 2 + 1) - min_time) * 2 - (if race.time % 2 == 0 { 1 } else { 0 })
    }).product()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> u64 {
    let race = parse_part2(input);

    let min_time = (1..race.time / 2).find(|button_time|
        button_time * (race.time - button_time) > race.distance
    ).unwrap();
    ((race.time / 2 + 1) - min_time) * 2 - (if race.time % 2 == 0 { 1 } else { 0 })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        assert_eq!(parse_part1("Time:      7  15   30
Distance:  9  40  200").collect_vec(), vec![Race::try_from((7, 9)).unwrap(), Race::try_from((15, 40)).unwrap(), Race::try_from((30, 200)).unwrap()]);

        assert_eq!(parse_separated_u64("      7  15   30"), Ok(("", 71530)));
        assert_eq!(parse_part2("Time:      7  15   30
Distance:  9  40  200"), Race::try_from((71530, 940200)).unwrap());
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("Time:      7  15   30
Distance:  9  40  200"), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("Time:      7  15   30
Distance:  9  40  200"), 71503);
    }

}