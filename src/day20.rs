
use std::{collections::{HashMap, VecDeque, HashSet}, iter::{empty, repeat}};

use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{all_consuming, map}, character::complete::{multispace1, char, alpha1}, multi::separated_list1, sequence::{separated_pair, preceded}, bytes::complete::tag, branch::alt};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High
}
impl Pulse {
    fn flip(&self) -> Self {
        use Pulse::*;
        match self {
            Low => High,
            High => Low,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleKind {
    FlipFlop(Pulse),
    Conjunction{ inputs: HashMap<String, Pulse> },
    Broadcast
}
impl ModuleKind {
    fn pulse(&mut self, pulse: Pulse, from: &String) -> Option<Pulse> {
        match (self, pulse) {
            (ModuleKind::FlipFlop( ref mut curr ), Pulse::Low) => {
                let output = curr.flip();
                *curr = output;
                Some(output)
            },
            (ModuleKind::Conjunction { ref mut inputs }, pulse) => {
                *inputs.get_mut(from).unwrap() = pulse;
                Some(
                    if inputs.values().all(|&p| p == Pulse::High) {Pulse::Low} else {Pulse::High}
                )
            },
            (ModuleKind::Broadcast, pulse) => Some(pulse),
            _ => None
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    kind: ModuleKind,
    children: Vec<String>
}
impl Module {
    fn pulse(&mut self, pulse: Pulse, from: &String) -> Box<dyn Iterator<Item = (Pulse, String)> + '_> {
        if let Some(output_pulse) = self.kind.pulse(pulse, from) {
            Box::new(repeat(output_pulse).zip(self.children.iter().cloned()))
        } else {
            Box::new(empty())
        }
    }
}


fn parse_flipflop_module(input: &str) -> IResult<&str, (String, ModuleKind)> {
    map(
        preceded(char('%'), alpha1),
        |str| (String::from(str), ModuleKind::FlipFlop(Pulse::Low))
    )(input)
}
fn parse_conjunction_module(input: &str) -> IResult<&str, (String, ModuleKind)> {
    map(
        preceded(char('&'), alpha1),
        |str| (String::from(str), ModuleKind::Conjunction { inputs: HashMap::new() })
    )(input)
}
fn parse_broadcast_module(input: &str) -> IResult<&str, (String, ModuleKind)> {
    map(
        tag("broadcaster"),
        |str| (String::from(str), ModuleKind::Broadcast)
    )(input)
}
fn parse_module(input: &str) -> IResult<&str, (String, ModuleKind)> {
    alt((
        parse_flipflop_module,
        parse_conjunction_module,
        parse_broadcast_module
    ))(input)
}
fn parse_children_modules(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(tag(", "), map(alpha1, |str| String::from(str)))(input)
}
fn parse_line(input: &str) -> IResult<&str, (String, Module)> {
    map(
        separated_pair(parse_module, tag(" -> "), parse_children_modules),
        |((name, kind), children)| (name, Module { kind, children })
    )(input)
}
fn parse(input: &str) -> Vec<(String, Module)> {
    let (_, output) = all_consuming(separated_list1(multispace1, parse_line))(input).unwrap();
    output
}


#[aoc(day20, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);

    let mut modules = HashMap::with_capacity(input.len());
    let mut module_inputs = HashMap::with_capacity(input.len());
    let mut conj_modules = HashSet::new();
    input.into_iter().for_each(|(name, module)| {
        if let ModuleKind::Conjunction { .. } = module.kind {
            conj_modules.insert(name.clone());
        }
        module.children.iter().for_each(|child_name| {
            module_inputs.entry(child_name.clone())
                .and_modify(|v: &mut Vec<_>| v.push(name.clone()))
                .or_insert(vec![name.clone()]);
        });

        modules.insert(name, module);
    });
    conj_modules.into_iter().for_each(|conj_module_name| {
        let conj_module_inputs = module_inputs.get(&conj_module_name).unwrap();
        modules.entry(conj_module_name).and_modify(|conj_module| {
            conj_module_inputs.into_iter().for_each(|input_module_name| {
                if let ModuleKind::Conjunction { ref mut inputs } = conj_module.kind {
                    inputs.insert(input_module_name.clone(), Pulse::Low);
                } else {
                    panic!("Conjunction module is not of ModuleKind::Conjunction?!");
                }
            })
        });
    });

    let mut pulses = VecDeque::new();
    let (mut low_pulses, mut high_pulses) = (0, 0);

    for _ in 0..1000 {
        pulses.push_back((String::from("button"), Pulse::Low, String::from("broadcaster")));
        while let Some((source_name, pulse, curr_name)) = pulses.pop_front() {
            match pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            };
            modules.entry(curr_name.clone()).and_modify(|module| {
                module.pulse(pulse, &source_name).into_iter().for_each(|(output_pulse, target_name)| {
                    pulses.push_back((curr_name.clone(), output_pulse, target_name));
                });
            });
        }
    }

    low_pulses * high_pulses
}


#[aoc(day20, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    let mut modules = HashMap::with_capacity(input.len());
    let mut module_inputs = HashMap::with_capacity(input.len());
    let mut conj_modules = HashSet::new();
    input.into_iter().for_each(|(name, module)| {
        if let ModuleKind::Conjunction { .. } = module.kind {
            conj_modules.insert(name.clone());
        }
        module.children.iter().for_each(|child_name| {
            module_inputs.entry(child_name.clone())
                .and_modify(|v: &mut Vec<_>| v.push(name.clone()))
                .or_insert(vec![name.clone()]);
        });

        modules.insert(name, module);
    });
    conj_modules.into_iter().for_each(|conj_module_name| {
        let conj_module_inputs = module_inputs.get(&conj_module_name).unwrap();
        modules.entry(conj_module_name.clone()).and_modify(|conj_module| {
            conj_module_inputs.into_iter().for_each(|input_module_name| {
                if let ModuleKind::Conjunction { ref mut inputs } = conj_module.kind {
                    inputs.insert(input_module_name.clone(), Pulse::Low);
                } else {
                    panic!("Conjunction module is not of ModuleKind::Conjunction?!");
                }
            })
        });
    });

    let mut pulses = VecDeque::new();
    let mut button_presses = 0;

    let mut rx_input_cycles = HashMap::new();
    let final_conj_name = modules.iter().find_map(|(module_name, module)| {
        if module.children.contains(&"rx".to_string()) {
            Some(module_name)
        } else {
            None
        }
    }).unwrap(); // this is the name of the conjunction node that outputs to "rx"
    if let ModuleKind::Conjunction { inputs } = &modules.get(final_conj_name).unwrap().kind {
        inputs.iter().for_each(|(input_name, _)| {
            rx_input_cycles.insert(input_name.clone(), None);
        });
    } // rx_input_cycles now contains all the inputs to the node that outputs to "rx" mapped to their cycle length

    loop {
        button_presses += 1;

        pulses.push_back((String::from("button"), Pulse::Low, String::from("broadcaster")));
        while let Some((source_name, pulse, curr_name)) = pulses.pop_front() {

            if let Some(conj_loop) = rx_input_cycles.get_mut(&source_name) {
                if conj_loop.is_none() && pulse == Pulse::High {
                    *conj_loop = Some(button_presses);
                }
            }

            modules.entry(curr_name.clone()).and_modify(|module| {
                module.pulse(pulse, &source_name).into_iter().for_each(|(output_pulse, target_name)| {
                    pulses.push_back((curr_name.clone(), output_pulse, target_name));
                });
            });
        }
        if rx_input_cycles.values().all(|cycle| cycle.is_some()) {
            return rx_input_cycles.into_values().map(|cycle| cycle.unwrap()).product()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"), 32000000);
        assert_eq!(part1("broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"), 11687500);
    }

}