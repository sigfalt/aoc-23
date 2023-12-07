use std::cmp::Ordering;

use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::{IResult, combinator::{map_res, all_consuming}, multi::separated_list1, character::complete::{digit1, space1, line_ending}, sequence::separated_pair, bytes::complete::take};


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2
}
impl TryFrom<char> for Card {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::Ace),
            'K' => Ok(Self::King),
            'Q' => Ok(Self::Queen),
            'J' => Ok(Self::Jack),
            'T' => Ok(Self::Ten),
            '9' => Ok(Self::Nine),
            '8' => Ok(Self::Eight),
            '7' => Ok(Self::Seven),
            '6' => Ok(Self::Six),
            '5' => Ok(Self::Five),
            '4' => Ok(Self::Four),
            '3' => Ok(Self::Three),
            '2' => Ok(Self::Two),
            _ => Err("bad card value")
        }
    }
}
impl TryFrom<u8> for Card {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(char::from(value))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bet: u32
}
impl Hand {
    fn get_hand_type(&self) -> HandType {
        let cards_map = self.cards.iter().counts();
        match cards_map.values().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                // do we have a pair to go along with the three matched cards?
                match cards_map.values().min().unwrap() {
                    2 => HandType::FullHouse,
                    1 => HandType::ThreeOfAKind,
                    _ => panic!("Expected between 1 and 2 additional cards to match with 3 already matching")
                }
            },
            2 => {
                // do we have two pairs or just one?
                match cards_map.values().filter(|&&c| c == 2).count() {
                    2 => HandType::TwoPair,
                    1 => HandType::OnePair,
                    _ => panic!("Expected between 1 and 2 pairs of matched cards")
                }
            },
            1 => HandType::HighCard,
            _ => panic!("Expected between 1 and 5 cards to match in a hand")
        }
    }
}
impl TryFrom<(&str, u32)> for Hand {
    type Error = &'static str;
    fn try_from((chars, bet): (&str, u32)) -> Result<Self, Self::Error> {
        if chars.len() != 5 {
            Err("expected five cards in a hand")
        } else {
            let cards = chars.bytes().map(|b|
                TryInto::<Card>::try_into(b)
            ).collect::<Result<_, _>>()?;
            Ok(Self {cards, bet})
        }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            Ordering::Equal => {},
            ord => return ord,
        };
        self.cards.iter().zip_eq(other.cards.iter()).map(|(card, other_card)| {
            card.cmp(other_card)
        }).find_or_last(|&order| {
            order != Ordering::Equal
        }).unwrap()
    }
}


fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(
        digit1,
        |num| u32::from_str_radix(num, 10)
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, Hand> {
    map_res(separated_pair(
        take(5u32),
        space1,
        parse_u32
    ), |h| h.try_into())(input)
}

fn parse(input: &str) -> Vec<Hand> {
    let (_, hands) = all_consuming(
        separated_list1(line_ending, parse_line)
    )(input).unwrap();

    hands
}


#[aoc(day7, part1)]
fn part1(input: &str) -> u32 {
    let mut hands = parse(input);
    
    hands.sort_unstable();

    hands.into_iter().enumerate().fold(0, |acc, (ix, hand)| {
        acc + (hand.bet * (1 + ix as u32))
    })
}

#[aoc(day7, part2)]
fn part2(input: &str) -> u32 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"), 5905);
    }

}