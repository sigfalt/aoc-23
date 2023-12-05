use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{map_res, map, all_consuming}, multi::separated_list1, character::{complete::{digit1, space1, line_ending, char}, is_space}, sequence::{delimited, tuple, pair}, bytes::complete::{tag, take_while1}};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Scratchcard {
    id: u32,
    winners: Vec<u32>,
    num_list: Vec<u32>
}

fn parse_num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |str| u32::from_str_radix(str, 10))(input)
}

fn parse_card_num(input: &str) -> IResult<&str, u32> {
    delimited(pair(tag("Card"), space1), parse_num, pair(char(':'), space1))(input)
}

fn parse_num_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, parse_num)(input)
}

fn parse_separator(input: &str) -> IResult<&str, &str> {
    take_while1(|c| is_space(c as u8) || c == '|')(input)
}

fn parse_line(input: &str) -> IResult<&str, Scratchcard> {
    map(
        tuple((parse_card_num, parse_num_list, parse_separator, parse_num_list)),
        |(id, winners, _, num_list)| Scratchcard {id, winners, num_list}
    )(input)
}

fn parse(input: &str) -> Vec<Scratchcard> {
    let (_, output) = all_consuming(
        separated_list1(line_ending, parse_line)
    )(input).unwrap();
    output
}

#[aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    let input = parse(input);

    input.into_iter().map(|card| {
        let exp = card.num_list.into_iter().filter(|num| card.winners.contains(num)).count();
        if exp == 0 {
            0
        } else {
            2u32.pow((exp - 1) as u32)
        }
    }).sum()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    let input = parse(input);

    let mut card_counts = vec![1u32; input.len()];
    input.into_iter().for_each(|card| {
        let id = (card.id - 1) as usize;
        let copies = card_counts[id];
        let num_winners = card.num_list.into_iter().filter(|num| card.winners.contains(num)).count();
        for i in 0..num_winners {
            card_counts[id + i + 1] += copies;
        }
    });

    card_counts.into_iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        assert_eq!(parse_num("41"), Ok(("", 41)));
        assert_eq!(parse_card_num("Card 27: "), Ok(("", 27)));
        assert_eq!(parse_num_list("13 59 32  5 47 | 26"), Ok((" | 26", vec![13, 59, 32, 5, 47])));
        assert_eq!(parse_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), Ok(("", Scratchcard {
            id: 1,
            winners: vec![41, 48, 83, 86, 17],
            num_list: vec![83, 86, 6, 31, 17, 9, 48, 53]
        })));
        assert_eq!(parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), vec![Scratchcard {
            id: 2,
            winners: vec![13, 32, 20, 16, 61],
            num_list: vec![61, 30, 68, 82, 17, 32, 24, 19]
        }, Scratchcard {
            id: 3,
            winners: vec![1, 21, 53, 59, 44],
            num_list: vec![69, 82, 63, 72, 16, 21, 14, 1]
        }]);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 30);
    }

}