use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1};
use nom::character::complete::{digit1, line_ending};
use nom::character::is_newline;
use nom::combinator::{all_consuming, map, opt};
use nom::multi::separated_list1;
use nom::sequence::{pair, terminated};

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>
}

#[derive(Debug, Default, PartialEq)]
struct CubeSet {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>
}
impl CubeSet {
    fn new(colors: Vec<Color>) -> Self {
        let mut val = Self::default();
        colors.into_iter().for_each(|c| {
            match c {
                Color::RED(r) => {val.red = Some(r)}
                Color::GREEN(g) => {val.green = Some(g)}
                Color::BLUE(b) => {val.blue = Some(b)}
            };
        });
        val
    }
}

#[derive(Debug, PartialEq)]
enum Color {
    RED(u32),
    GREEN(u32),
    BLUE(u32)
}

fn parse(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, game_vec) = separated_list1(
        line_ending,
        parse_game
    )(input)?;

    Ok((input, game_vec))
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map(
        digit1,
        |id| u32::from_str_radix(id, 10).unwrap()
    )(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, set_vec) = separated_list1(
        tag("; "),
        take_till1(|c| c == ';' || is_newline(c as u8))
    )(input)?;
    let sets = set_vec.into_iter().map(|s| {
        all_consuming(parse_set)(s).unwrap().1
    }).collect_vec();

    Ok((input, Game {
        id,
        sets,
    }))
}

fn parse_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, color_vec) = separated_list1(
        tag(", "),
        take_till1(|c| c == ',' || c == ';')
            .and_then(alt((parse_red, parse_green, parse_blue))),
    )(input)?;

    Ok((input, CubeSet::new(color_vec)))
}

fn parse_red(input: &str) -> IResult<&str, Color> {
    let (input, num_cubes) = terminated(
        map(
            digit1,
            |cubes| u32::from_str_radix(cubes, 10).unwrap()
        ),
        pair(
            tag(" red"),
            opt(tag(","))
        )
    )(input)?;

    Ok((input, Color::RED(num_cubes)))
}

fn parse_green(input: &str) -> IResult<&str, Color> {
    let (input, num_cubes) = terminated(
        map(
            digit1,
            |cubes| u32::from_str_radix(cubes, 10).unwrap()
        ),
        pair(
            tag(" green"),
            opt(tag(","))
        )
    )(input)?;

    Ok((input, Color::GREEN(num_cubes)))
}

fn parse_blue(input: &str) -> IResult<&str, Color> {
    let (input, num_cubes) = terminated(
        map(
            digit1,
            |cubes| u32::from_str_radix(cubes, 10).unwrap()
        ),
        pair(
            tag(" blue"),
            opt(tag(","))
        )
    )(input)?;

    Ok((input, Color::BLUE(num_cubes)))
}

#[aoc(day2, part1)]
fn part1(input: &str) -> u32 {
    let limit = CubeSet {
        red: Some(12),
        green: Some(13),
        blue: Some(14)
    };
    let (_, games) = all_consuming(parse)(input).unwrap();
    games.into_iter().filter_map(|g| if g.sets.iter().all(|s|
        s.red.unwrap_or_default() <= limit.red.unwrap() &&
        s.green.unwrap_or_default() <= limit.green.unwrap() &&
        s.blue.unwrap_or_default() <= limit.blue.unwrap()
    ) { Some(g.id) } else { None }).sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> u32 {
    let (_, games) = all_consuming(parse)(input).unwrap();
    games.into_iter().map(|g| {
        let max = g.sets.into_iter().fold(CubeSet::default(), |acc, set| {
            CubeSet {
                red: Some(acc.red.unwrap_or_default().max(set.red.unwrap_or_default())),
                green: Some(acc.green.unwrap_or_default().max(set.green.unwrap_or_default())),
                blue: Some(acc.blue.unwrap_or_default().max(set.blue.unwrap_or_default()))
            }
        });
        max.red.unwrap() * max.green.unwrap() * max.blue.unwrap()
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tests() {
        // parsing colors
        assert_eq!(parse_red("4 red"), Ok(("", Color::RED(4))));
        assert_eq!(parse_blue("7 blue,"), Ok(("", Color::BLUE(7))));
        assert_eq!(parse_green("55 green"), Ok(("", Color::GREEN(55))));

        // parsing a set of colors
        assert_eq!(parse_set("1 red, 5 blue, 10 green"), Ok(("", CubeSet {
            red: Some(1),
            green: Some(10),
            blue: Some(5)
        })));
        assert_eq!(parse_set("5 green, 6 blue, 12 red;"), Ok((";", CubeSet {
            red: Some(12),
            green: Some(5),
            blue: Some(6)
        })));
        assert_eq!(parse_set("2 green, 1 blue"), Ok(("", CubeSet {
            red: None,
            green: Some(2),
            blue: Some(1)
        })));

        // parsing a game
        assert_eq!(parse_game("Game 1: 1 red, 5 blue, 10 green; 5 green, 6 blue, 12 red; 4 red, 10 blue, 4 green"),
        Ok(("", Game {
            id: 1,
            sets: vec![
                CubeSet {
                    red: Some(1),
                    blue: Some(5),
                    green: Some(10)
                },
                CubeSet {
                    green: Some(5),
                    blue: Some(6),
                    red: Some(12)
                },
                CubeSet {
                    red: Some(4),
                    blue: Some(10),
                    green: Some(4)
                }
            ]
        })));
        assert_eq!(parse_game("Game 2: 2 green, 1 blue; 1 red, 2 green; 3 red, 1 blue; 2 blue, 1 green, 8 red; 1 green, 10 red; 10 red"),
        Ok(("", Game {
            id: 2,
            sets: vec![
                CubeSet {
                    green: Some(2),
                    blue: Some(1),
                    ..Default::default()
                },
                CubeSet {
                    red: Some(1),
                    green: Some(2),
                    ..Default::default()
                },
                CubeSet {
                    red: Some(3),
                    blue: Some(1),
                    ..Default::default()
                },
                CubeSet {
                    blue: Some(2),
                    green: Some(1),
                    red: Some(8)
                },
                CubeSet {
                    green: Some(1),
                    red: Some(10),
                    ..Default::default()
                },
                CubeSet {
                    red: Some(10),
                    ..Default::default()
                }
            ]
        })));

        // put it all together
        assert_eq!(parse("Game 1: 1 red, 5 blue, 10 green; 5 green, 6 blue, 12 red; 4 red, 10 blue, 4 green
Game 2: 2 green, 1 blue; 1 red, 2 green; 3 red, 1 blue; 2 blue, 1 green, 8 red; 1 green, 10 red; 10 red
Game 3: 14 red, 9 green, 5 blue; 2 green, 5 red, 7 blue; 1 blue, 14 green; 6 green, 2 red"), Ok(("", vec![Game {
            id: 1,
            sets: vec![
                CubeSet {
                    red: Some(1),
                    blue: Some(5),
                    green: Some(10)
                },
                CubeSet {
                    green: Some(5),
                    blue: Some(6),
                    red: Some(12)
                },
                CubeSet {
                    red: Some(4),
                    blue: Some(10),
                    green: Some(4)
                }
            ]
        }, Game {
            id: 2,
            sets: vec![
                CubeSet {
                    green: Some(2),
                    blue: Some(1),
                    ..Default::default()
                },
                CubeSet {
                    red: Some(1),
                    green: Some(2),
                    ..Default::default()
                },
                CubeSet {
                    red: Some(3),
                    blue: Some(1),
                    ..Default::default()
                },
                CubeSet {
                    blue: Some(2),
                    green: Some(1),
                    red: Some(8)
                },
                CubeSet {
                    green: Some(1),
                    red: Some(10),
                    ..Default::default()
                },
                CubeSet {
                    red: Some(10),
                    ..Default::default()
                }
            ]
        }, Game {
            id: 3,
            sets: vec![
                CubeSet {
                    red: Some(14),
                    green: Some(9),
                    blue: Some(5)
                },
                CubeSet {
                    green: Some(2),
                    red: Some(5),
                    blue: Some(7)
                },
                CubeSet {
                    blue: Some(1),
                    green: Some(14),
                    ..Default::default()
                },
                CubeSet {
                    green: Some(6),
                    red: Some(2),
                    ..Default::default()
                }
            ]
        }])));
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"), 2286);
    }

}