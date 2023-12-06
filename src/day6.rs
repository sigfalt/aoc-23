use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{map_res, all_consuming}, multi::separated_list1, character::complete::{digit1, space1, alpha1, char, line_ending}, sequence::{preceded, tuple, separated_pair}};


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
    time: u32,
    distance: u32
}
impl From<(u32, u32)> for Race {
    fn from((time, distance): (u32, u32)) -> Self {
        Self{time, distance}
    }
}

fn parse(input: &str) -> impl Iterator<Item = Race> {
    let (_, (times, distances)) = all_consuming(separated_pair(
        parse_input_line,
        line_ending,
        parse_input_line
    ))(input).unwrap();

    times.into_iter().zip_eq(distances.into_iter()).map(|r| r.into())
}

#[aoc(day6, part1)]
fn part1(input: &str) -> u32 {
    let races = parse(input);

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
fn part2(input: &str) -> u32 {
    let races = parse(input);

    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        assert_eq!(parse("Time:      7  15   30
Distance:  9  40  200").collect_vec(), vec![Race::from((7, 9)), Race::from((15, 40)), Race::from((30, 200))]);
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