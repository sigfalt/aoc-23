use aoc_runner_derive::aoc;
use ascii::AsciiStr;
use itertools::Itertools;
use phf::phf_map;

fn parse(input: &str) -> Vec<&str> {
    input.lines().collect_vec()
}

fn parse_ascii(input: &str) -> Vec<&AsciiStr> {
    AsciiStr::from_ascii(input).unwrap().lines().collect_vec()
}

#[aoc(day1, part1, str)]
fn part1(input: &str) -> String {
    let input = parse(input);

    let mut sum = 0;
    for line in input {
        let first = line.find(|c: char| c.is_numeric()).unwrap();
        let last = line.rfind(|c: char| c.is_numeric()).unwrap();
        let line = line.as_bytes();
        let first = line[first];
        let last = line[last];
        let s = vec![first, last];
        let val = std::str::from_utf8(&s).unwrap();
        sum += val.parse::<u32>().unwrap();
    }

    sum.to_string()
}

#[aoc(day1, part1, ascii)]
fn part1_ascii(input: &str) -> String {
    let input = parse_ascii(input);

    let mut sum = 0;
    for line in input {
        let mut first = None;
        for i in 0..line.len() {
            let slice = &line[i..].first().unwrap();
            if slice.is_ascii_digit() {
                first = Some(slice.as_char().to_digit(10).unwrap());
                break;
            }
        };
        let first = first.unwrap();

        let mut last = None;
        for i in 0..line.len() {
            let slice = &line[..line.len() - i].last().unwrap();
            if slice.is_ascii_digit() {
                last = Some(slice.as_char().to_digit(10).unwrap());
                break;
            }
        };
        let last = last.unwrap();

        sum += (first * 10) + last;
    }

    sum.to_string()
}

static STR_SEARCH: phf::Map<&'static str, u32> = phf_map! {
    "0" => 0,
    "1" => 1,
    "2" => 2,
    "3" => 3,
    "4" => 4,
    "5" => 5,
    "6" => 6,
    "7" => 7,
    "8" => 8,
    "9" => 9,
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9
};

#[aoc(day1, part2, ascii)]
fn part2_ascii(input: &str) -> String {
    let input = parse_ascii(input);

    let mut sum = 0;
    for line in input {
        let mut first = None;
        for i in 0..line.len() {
            let slice = &line[i..];
            if let Some((_, &digit)) = STR_SEARCH.entries().find(|(&k, _)| slice.as_str().starts_with(k)) {
                first = Some(digit);
                break;
            }
        };
        let first = first.unwrap();

        let mut last = None;
        for i in 0..line.len() {
            let slice = &line[..line.len() - i];
            if let Some((_, &digit)) = STR_SEARCH.entries().find(|(&k, _)| slice.as_str().ends_with(k)) {
                last = Some(digit);
                break;
            }
        };
        let last = last.unwrap();

        sum += (first * 10) + last;
    }

    sum.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), "142");
    }

    #[test]
    fn part1_ascii_example() {
        assert_eq!(part1_ascii("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"), "142");
    }

    #[test]
    fn part2_ascii_example() {
        assert_eq!(part2_ascii("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"), "281");
    }
}