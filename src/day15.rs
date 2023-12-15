
use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{all_consuming, map_res, map, opt}, character::complete::{char, one_of, digit1, alpha1}, multi::separated_list1, sequence::tuple};


fn parse_step(input: &str) -> IResult<&str, LensInstruction> {
    map(tuple((
        map(alpha1, |str: &str| str.to_string()),
        map_res(one_of("-="), |chr| chr.try_into()),
        opt(map_res(digit1, |chr| u8::from_str_radix(chr, 10)))
    )), |(label, operation, focal_length)| LensInstruction { label, operation, focal_length })(input)
}
fn parse_line(input: &str) -> IResult<&str, Vec<LensInstruction>> {
    separated_list1(char(','), parse_step)(input)
}
fn parse(input: &str) -> Vec<LensInstruction> {
    let (_, output) = all_consuming(parse_line)(input).unwrap();
    output
}


fn hash_algorithm(input: impl Iterator<Item = impl Into<u8>>) -> u8 {
    input.fold(0, |hash_val, curr_byte| {
        hash_val.wrapping_add(curr_byte.into()).wrapping_mul(17)
    })
}


#[aoc(day15, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);

    input.into_iter().map(|st| hash_algorithm(st.bytes()) as u64).sum()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LensOperation {
    Remove,
    Insert
}
impl TryFrom<char> for LensOperation {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::Remove),
            '=' => Ok(Self::Insert),
            _ => Err("Invalid lens operation")
        }
    }
}
impl From<LensOperation> for char {
    fn from(value: LensOperation) -> Self {
        match value {
            LensOperation::Remove => '-',
            LensOperation::Insert => '='
        }
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct LensInstruction {
    label: String,
    operation: LensOperation,
    focal_length: Option<u8>
}
impl LensInstruction {
    fn bytes(&self) -> impl Iterator<Item = u8> {
        let mut output = self.label.clone();
        output.push(self.operation.into());
        if let Some(focal_length) = self.focal_length {
            output.push_str(&focal_length.to_string());
        }
        output.into_bytes().into_iter()
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    focal_length: u8,
}
impl TryFrom<LensInstruction> for Lens {
    type Error = &'static str;
    fn try_from(value: LensInstruction) -> Result<Self, Self::Error> {
        if let Some(focal_length) = value.focal_length {
            Ok(Self { label: value.label, focal_length })
        } else {
            Err("No focal length for lens instruction")
        }
    }
}


#[aoc(day15, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());
    input.into_iter().for_each(|step| {
        let box_ix: usize = hash_algorithm(step.label.bytes()).into();
        let selected_box = boxes.get_mut(box_ix).unwrap();
        match step.operation {
            LensOperation::Remove => {
                if let Some(lens_ix) = selected_box.iter().position(|lens| lens.label == step.label) {
                    selected_box.remove(lens_ix);
                }
            },
            LensOperation::Insert => {
                if let Some(lens) = selected_box.iter_mut().find(|lens| lens.label == step.label) {
                    lens.focal_length = step.focal_length.expect("Insertion operation without a focal length?!");
                } else {
                    selected_box.push(step.try_into().unwrap());
                }
            }
        }
    });

    boxes.into_iter().enumerate().fold(0, |sum, (box_ix, lens_box)| {
        sum + lens_box.into_iter().enumerate().fold(0, |box_sum, (lens_ix, lens)| {
            box_sum + ((box_ix as u64 + 1) * (lens_ix as u64 + 1) * lens.focal_length as u64)
        })
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(hash_algorithm("HASH".bytes()), 52);
        assert_eq!(part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 1320);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"), 145);
    }

}