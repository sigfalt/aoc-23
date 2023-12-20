
use std::collections::HashMap;

use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{all_consuming, map_res, map, peek}, character::complete::{one_of, multispace1, digit1, char, alpha1}, multi::separated_list1, sequence::{tuple, terminated, separated_pair}, bytes::complete::{tag, take_till1}, branch::alt};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S
}
impl TryFrom<char> for Category {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Category::*;
        match value {
            'x' => Ok(X),
            'm' => Ok(M),
            'a' => Ok(A),
            's' => Ok(S),
            _ => Err("Invalid category character")
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64
}
impl Part {
    fn get(&self, category: Category) -> u64 {
        use Category::*;
        match category {
            X => self.x,
            M => self.m,
            A => self.a,
            S => self.s,
        }
    }
    fn accept(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Comparison {
    GreaterThan,
    LessThan
}
impl TryFrom<char> for Comparison {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Comparison::*;
        match value {
            '>' => Ok(GreaterThan),
            '<' => Ok(LessThan),
            _ => Err("Invalid comparison character")
        }
    }
}
impl Comparison {
    fn apply(&self, value_a: u64, value_b: u64) -> bool {
        use Comparison::*;
        match self {
            GreaterThan => value_a > value_b,
            LessThan => value_a < value_b
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    Unconditional,
    Conditional{category: Category, comp: Comparison, target: u64}
}
impl Condition {
    fn satisfied_by(&self, part: Part) -> bool {
        use Condition::*;
        match self {
            Unconditional => true,
            &Conditional { category, comp, target } => 
                comp.apply(part.get(category), target)
        }
    }
    fn constrain(&self, range: PartRange) -> (Option<PartRange>, Option<PartRange>) {
        use Condition::*;
        match self {
            Unconditional => (Some(range), None),
            &Conditional { category, comp, target } => range.constrain(category, comp, target)
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RuleResult {
    Accepted,
    Rejected,
    Redirect{name: String}
}
impl From<&str> for RuleResult {
    fn from(value: &str) -> Self {
        use RuleResult::*;
        match value {
            "A" => Accepted,
            "R" => Rejected,
            name => Redirect { name: name.to_string() }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    cond: Condition,
    result: RuleResult
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Workflow {
    name: String,
    rules: Vec<Rule>
}
impl Workflow {
    fn to_tuple(self) -> (String, Vec<Rule>) {
        (self.name, self.rules)
    }
}


fn parse_u64(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |num| u64::from_str_radix(num, 10))(input)
}
fn parse_conditional_workflow_rule(input: &str) -> IResult<&str, Rule> {
    map(tuple((
        map_res(one_of("xmas"), |chr| chr.try_into()),
        map_res(one_of("<>"), |chr| chr.try_into()),
        parse_u64,
        char(':'),
        map(alpha1, |str: &str| str.into())
    )), |(category, comp, target, _, result)|
        Rule {result, cond: Condition::Conditional { category, comp, target }}
    )(input)
}
fn parse_unconditional_workflow_rule(input: &str) -> IResult<&str, Rule> {
    map(terminated(alpha1, peek(char('}'))), |str: &str|
        Rule { cond: Condition::Unconditional, result: str.into() }
    )(input)
}
fn parse_workflow_rule(input: &str) -> IResult<&str, Rule> {
    alt((parse_conditional_workflow_rule, parse_unconditional_workflow_rule))(input)
}
fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    map(tuple((
        map(take_till1(|chr| chr == '{'), |str: &str| str.to_string()),
        char('{'),
        separated_list1(char(','), parse_workflow_rule),
        char('}')
    )), |(name, _, rules, _)| Workflow {name, rules})(input)
}
fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
    separated_list1(multispace1, parse_workflow)(input)
}
fn parse_part(input: &str) -> IResult<&str, Part> {
    map(tuple((
        tag("{x="), parse_u64,
        tag(",m="), parse_u64,
        tag(",a="), parse_u64,
        tag(",s="), parse_u64,
        char('}')
    )), |(_, x, _, m, _, a, _, s, _)| Part {x, m, a, s})(input)
}
fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(multispace1, parse_part)(input)
}
fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (_, output) = all_consuming(separated_pair(parse_workflows, multispace1, parse_parts))(input).unwrap();
    output
}


#[aoc(day19, part1)]
fn part1(input: &str) -> u64 {
    let (workflows, parts) = parse(input);
    let workflows = HashMap::<_, _>::from_iter(workflows.into_iter().map(Workflow::to_tuple));
    parts.into_iter().filter_map(|part| {

        let mut curr_ruleset = String::from("in");
        loop {
            match &workflows.get(&curr_ruleset).unwrap().iter().find_map(|rule| {
                if rule.cond.satisfied_by(part) { Some(rule) } else { None }
            }).unwrap().result {
                RuleResult::Accepted => return Some(part.accept()),
                RuleResult::Rejected => return None,
                RuleResult::Redirect { name } => curr_ruleset = name.clone(),
            }
        }

    }).sum()
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RatingRange {
    lower: u64,
    upper: u64
}
impl Default for RatingRange {
    fn default() -> Self {
        Self { lower: 1, upper: 4001 }
    }
}
impl RatingRange {
    fn accept(&self) -> u64 {
        self.upper - self.lower
    }
    fn constrain(self, comp: Comparison, value: u64) -> (Option<Self>, Option<Self>) {
        use Comparison::*;
        match comp {
            GreaterThan => self.constrain_min(value),
            LessThan => self.constrain_max(value),
        }
    }
    fn constrain_max(self, max: u64) -> (Option<Self>, Option<Self>) {
        // "x<max"
        // e.g. x = 1000..3001
        if self.upper <= max {
            // 3001 or higher would result in full acceptance
            (Some(self), None)
        } else if self.lower >= max {
            // 1000 or lower would result in full rejection
            (None, Some(self))
        } else {
            // 2000 would result in 1000..2000 accepted, 2000..3001 rejected
            (Some(Self { upper: max, ..self }), Some(Self { lower: max, ..self }))
        }
    }
    fn constrain_min(self, min: u64) -> (Option<Self>, Option<Self>) {
        // "x>min"
        // e.g. x = 1000..3001
        if self.lower > min {
            // 999 or lower would result in full acceptance
            (Some(self), None)
        } else if self.upper < min {
            // 3000 or higher would result in full rejection
            (None, Some(self))
        } else {
            // 2000 would result in 2001..3001 accepted, 1000..2001 rejected
            (Some(Self { lower: min + 1, ..self }), Some(Self { upper: min + 1, ..self }))
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct PartRange {
    x: RatingRange,
    m: RatingRange,
    a: RatingRange,
    s: RatingRange
}
impl PartRange {
    fn accept(&self) -> u64 {
        self.x.accept() * self.m.accept() * self.a.accept() * self.s.accept()
    }
    fn constrain(self, category: Category, comp: Comparison, value: u64) -> (Option<Self>, Option<Self>) {
        use Category::*;
        match category {
            X => {
                let (accepted, rejected) = self.x.constrain(comp, value);
                (accepted.map(|acc| Self { x: acc, ..self }), rejected.map(|rej| Self { x: rej, ..self }))
            },
            M => {
                let (accepted, rejected) = self.m.constrain(comp, value);
                (accepted.map(|acc| Self { m: acc, ..self }), rejected.map(|rej| Self { m: rej, ..self }))
            },
            A => {
                let (accepted, rejected) = self.a.constrain(comp, value);
                (accepted.map(|acc| Self { a: acc, ..self }), rejected.map(|rej| Self { a: rej, ..self }))
            },
            S => {
                let (accepted, rejected) = self.s.constrain(comp, value);
                (accepted.map(|acc| Self { s: acc, ..self }), rejected.map(|rej| Self { s: rej, ..self }))
            }
        }
    }
}


#[aoc(day19, part2)]
fn part2(input: &str) -> u64 {
    let (workflows, _) = parse(input);
    let workflows = HashMap::<_, _>::from_iter(workflows.into_iter().map(Workflow::to_tuple));
    let mut part_ranges_to_process = vec![(PartRange::default(), String::from("in"))];

    let mut sum = 0;
    while let Some((mut part_range, rule)) = part_ranges_to_process.pop() {
        let ruleset = workflows.get(&rule).unwrap();
        for rule in ruleset {
            let (matching, failed) = rule.cond.constrain(part_range);
            if let Some(matching) = matching {
                match &rule.result {
                    RuleResult::Accepted => sum += matching.accept(),
                    RuleResult::Rejected => {},
                    RuleResult::Redirect { name } => part_ranges_to_process.push((matching, name.clone())),
                }
            }
            if let Some(failed) = failed {
                part_range = failed;
            }
        }
    }
    sum
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"), 19114);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"), 167409079868000);
    }

}