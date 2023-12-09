use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{map_res, all_consuming}, character::{complete::{space1, line_ending}, is_digit}, multi::separated_list1, bytes::complete::take_while1};


fn parse_i32(input: &str) -> IResult<&str, i32> {
    map_res(
        take_while1(|c: char| is_digit(c.try_into().unwrap()) || c == '-'),
        |num| i32::from_str_radix(num, 10)
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    separated_list1(space1, parse_i32)(input)
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let (_, output) = all_consuming(separated_list1(line_ending, parse_line))(input).unwrap();

    output
}


#[aoc(day9, part1)]
fn part1(input: &str) -> i32 {
    let input = parse(input);

    input.into_iter().map(|line| {
        let mut line_derivs = vec![line];
        loop {
            let curr_line = line_derivs.last().unwrap();
            let next_line = curr_line.windows(2).map(|window| {
                window[1] - window[0]
            }).collect_vec();
            if next_line.iter().all(|&x| x == 0) {
                break;
            }
            line_derivs.push(next_line);
        }
        let history = line_derivs.into_iter().rev().fold(0, |last_deriv_num, deriv| {
            deriv.last().unwrap() + last_deriv_num
        });
        history
    }).sum()
}


#[aoc(day9, part2)]
fn part2(input: &str) -> i32 {
    let input = parse(input);

    input.into_iter().map(|line| {
        let mut line_derivs = vec![line];
        loop {
            let curr_line = line_derivs.last().unwrap();
            let next_line = curr_line.windows(2).map(|window| {
                window[1] - window[0]
            }).collect_vec();
            if next_line.iter().all(|&x| x == 0) {
                break;
            }
            line_derivs.push(next_line);
        }
        let history = line_derivs.into_iter().rev().fold(0, |last_deriv_num, deriv| {
            deriv.first().unwrap() - last_deriv_num
        });
        history
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"), 114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"), 2);
    }

}