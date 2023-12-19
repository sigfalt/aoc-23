
use std::collections::BTreeMap;

use aoc_runner_derive::aoc;
use nom::{IResult, combinator::{all_consuming, map_res, verify, map}, character::complete::{one_of, multispace1, space1, digit1, hex_digit1}, multi::separated_list1, sequence::tuple, bytes::complete::tag};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            'U' => Ok(Up),
            'D' => Ok(Down),
            'L' => Ok(Left),
            'R' => Ok(Right),
            _ => Err("invalid direction character"),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DigStep {
    dir: Direction,
    distance: u64,
    color: String
}

fn parse_line(input: &str) -> IResult<&str, DigStep> {
    map(tuple((
        map_res(one_of("UDLR"), |chr| chr.try_into()),
        space1,
        map_res(digit1, |num| u64::from_str_radix(num, 10)),
        space1,
        tag("(#"),
        verify(hex_digit1, |str: &str| str.len() == 6),
        tag(")")
    )), |(dir, _, distance, _, _, color, _)| DigStep {dir, distance, color: color.to_string()})(input)
}
fn parse(input: &str) -> Vec<DigStep> {
    let (_, output) = all_consuming(separated_list1(multispace1, parse_line))(input).unwrap();
    output
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HorizontalEdge {
    start: i64,
    end: i64
}
impl HorizontalEdge {
    fn new(points: [i64; 2]) -> Self {
        let start = points[0].min(points[1]);
        let end = points[0].max(points[1]);
        Self { start, end }
    }
    fn len(&self) -> u64 {
        (self.end - self.start) as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    row: i64,
    col: i64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct DigSpan {
    start: i64,
    end: i64
}
impl DigSpan {
    fn new(points: [i64; 2]) -> Self {
        let start = points[0].min(points[1]);
        let end = points[0].max(points[1]);
        Self { start, end }
    }
    fn len(&self) -> u64 {
        1 + (self.end - self.start) as u64
    }
    fn contains(&self, point: &i64) -> bool {
        (self.start..=self.end).contains(point)
    }
}


#[aoc(day18, part1)]
fn part1(input: &str) -> u64 {
    let input = parse(input);

    let mut horiz_edges: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut curr_pos = Point { row: 0, col: 0 };
    input.into_iter().for_each(|step| {
        match step.dir {
            Direction::Up => curr_pos.row = curr_pos.row.checked_sub_unsigned(step.distance).unwrap(),
            Direction::Down => curr_pos.row = curr_pos.row.checked_add_unsigned(step.distance).unwrap(),
            Direction::Left => {
                let new_col = curr_pos.col.checked_sub_unsigned(step.distance).unwrap();
                // horiz_edges.insert(curr_pos.row, HorizontalEdge::new([ curr_pos.col, new_col ]));
                horiz_edges.entry(curr_pos.row)
                    .and_modify(|curr| curr.push(HorizontalEdge::new([ curr_pos.col, new_col ])))
                    .or_insert_with(|| vec![HorizontalEdge::new([ curr_pos.col, new_col ])]);
                curr_pos.col = new_col;
            },
            Direction::Right => {
                let new_col = curr_pos.col.checked_add_unsigned(step.distance).unwrap();
                // horiz_edges.insert(curr_pos.row, HorizontalEdge::new([ curr_pos.col, new_col ]));
                horiz_edges.entry(curr_pos.row)
                    .and_modify(|curr| curr.push(HorizontalEdge::new([ curr_pos.col, new_col ])))
                    .or_insert_with(|| vec![HorizontalEdge::new([ curr_pos.col, new_col ])]);
                curr_pos.col = new_col;
            },
        };
    });
    if curr_pos != (Point { row: 0, col: 0 }) {
        panic!("Parsed steps did not complete a full loop! Final position: {curr_pos:?}");
    }

    // println!("edges: {horiz_edges:#?}");

    let (res, _, open_spans) = horiz_edges.range(..).fold((0, None, vec![]), |(acc, last_seen_row, dug_spans), (&curr_row, edges)| {
        // println!("curr spans: {dug_spans:?}");
        // println!("fold edges: ({curr_row}) {edges:?}");
        if let Some(last_seen_row) = last_seen_row {
            // area has increased by (curr_row - last_seen_row - 1) * (combined len of all dug_areas)
            let prev_dug_span_len = dug_spans.iter().fold(0, |acc, span: &DigSpan| {
                acc + span.len()
            });
            let skipped_area = (curr_row - last_seen_row - 1) as u64 * prev_dug_span_len;

            let (curr_span_len, next_spans) = edges.into_iter().fold((prev_dug_span_len, dug_spans), |(mut curr_span_len, next_spans), &edge| {
                let span_count = next_spans.len();

                // find the span that will be affected by this edge
                let mut modified_span = None;
                let mut new_spans = vec![];
                next_spans.into_iter().enumerate().for_each(|(span_ix, next_span)| {
                    if modified_span.is_some() { // already found and modified a span
                        new_spans.push(next_span);
                    } else if edge.start == next_span.start && edge.end == next_span.end { // edge is completely ending a span
                        modified_span = Some(span_ix);
                    } else if edge.end == next_span.start { // span is cutting out from left
                        modified_span = Some(span_ix);
                        curr_span_len += edge.len();
                        new_spans.push(DigSpan::new([edge.start, next_span.end]));
                    } else if edge.start == next_span.start { // span is cutting in from left
                        modified_span = Some(span_ix);
                        new_spans.push(DigSpan::new([edge.end, next_span.end]));
                    } else if edge.end == next_span.end { // span is cutting in from right
                        modified_span = Some(span_ix);
                        new_spans.push(DigSpan::new([next_span.start, edge.start]));
                    } else if edge.start == next_span.end { // span is cutting out from right
                        modified_span = Some(span_ix);
                        curr_span_len += edge.len();
                        new_spans.push(DigSpan::new([next_span.start, edge.end]));
                    } else if next_span.contains(&edge.start) && next_span.contains(&edge.end) { // span is splitting
                        modified_span = Some(span_ix);
                        new_spans.append(&mut vec![
                            DigSpan::new([next_span.start, edge.start]),
                            DigSpan::new([edge.end, next_span.end])
                        ]);
                    } else {
                        new_spans.push(next_span);
                    }
                });
                if let Some(modified_span_ix) = modified_span { // check if the span before and/or after should be merged with this one
                    if span_count == new_spans.len() { // can only happen if we didn't split an existing span
                        if let Some(span_after) = new_spans.get(modified_span_ix + 1).cloned() {
                            let modified_span = new_spans.get_mut(modified_span_ix).unwrap();
                            if modified_span.end == span_after.start {
                                modified_span.end = span_after.end;
                                new_spans.swap_remove(modified_span_ix + 1);
                                curr_span_len -= 1;
                            }
                        }
                        if let Some(span_before_ix) = modified_span_ix.checked_sub(1) {
                            if let Some(span_before) = new_spans.get(span_before_ix).cloned() {
                                let modified_span = new_spans.get_mut(modified_span_ix).unwrap();
                                if modified_span.end == span_before.start {
                                    modified_span.end = span_before.end;
                                    new_spans.swap_remove(span_before_ix);
                                    curr_span_len -= 1;
                                }
                            }
                        }
                    }
                } else { // edge is starting a new self-contained area
                    curr_span_len += edge.len() + 1;
                    new_spans.push(DigSpan::new([edge.start, edge.end]))
                }

                new_spans.sort_unstable();
                (curr_span_len, new_spans)
            });
            // println!("curr_span_len: {curr_span_len}");
            // println!("new spans: {next_spans:?}");

            (acc + skipped_area + curr_span_len, Some(curr_row), next_spans)
        } else {
            let (area, starting_spans) = edges.iter().fold((0, vec![]), |(area, mut dug_spans), &edge| {
                let new_span = DigSpan::new([edge.start, edge.end]);
                dug_spans.push(new_span);
                (area + new_span.len(), dug_spans)
            });
            (area, Some(curr_row), starting_spans)
        }
    });

    if open_spans.len() != 0 {
        panic!("not all spans closed? algorithm error!");
    }

    res
}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EncodedDigStep {
    dir: Direction,
    distance: u64,
}
impl From<DigStep> for EncodedDigStep {
    fn from(value: DigStep) -> Self {
        Self {
            distance: u64::from_str_radix(&value.color[..5], 16).unwrap(),
            dir: match value.color.chars().last().unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                x => x
            }.try_into().unwrap()
        }
    }
}


#[aoc(day18, part2)]
fn part2(input: &str) -> u64 {
    let input = parse(input);

    let mut horiz_edges: BTreeMap<_, Vec<_>> = BTreeMap::new();
    let mut curr_pos = Point { row: 0, col: 0 };
    input.into_iter().for_each(|step| {
        let enc_step: EncodedDigStep = step.into();
        match enc_step.dir {
            Direction::Up => curr_pos.row = curr_pos.row.checked_sub_unsigned(enc_step.distance).unwrap(),
            Direction::Down => curr_pos.row = curr_pos.row.checked_add_unsigned(enc_step.distance).unwrap(),
            Direction::Left => {
                let new_col = curr_pos.col.checked_sub_unsigned(enc_step.distance).unwrap();
                // horiz_edges.insert(curr_pos.row, HorizontalEdge::new([ curr_pos.col, new_col ]));
                horiz_edges.entry(curr_pos.row)
                    .and_modify(|curr| curr.push(HorizontalEdge::new([ curr_pos.col, new_col ])))
                    .or_insert_with(|| vec![HorizontalEdge::new([ curr_pos.col, new_col ])]);
                curr_pos.col = new_col;
            },
            Direction::Right => {
                let new_col = curr_pos.col.checked_add_unsigned(enc_step.distance).unwrap();
                // horiz_edges.insert(curr_pos.row, HorizontalEdge::new([ curr_pos.col, new_col ]));
                horiz_edges.entry(curr_pos.row)
                    .and_modify(|curr| curr.push(HorizontalEdge::new([ curr_pos.col, new_col ])))
                    .or_insert_with(|| vec![HorizontalEdge::new([ curr_pos.col, new_col ])]);
                curr_pos.col = new_col;
            },
        };
    });
    if curr_pos != (Point { row: 0, col: 0 }) {
        panic!("Parsed steps did not complete a full loop! Final position: {curr_pos:?}");
    }

    let (res, _, open_spans) = horiz_edges.range(..).fold((0, None, vec![]), |(acc, last_seen_row, dug_spans), (&curr_row, edges)| {
        if let Some(last_seen_row) = last_seen_row {
            // area has increased by (curr_row - last_seen_row - 1) * (combined len of all dug_areas)
            let prev_dug_span_len = dug_spans.iter().fold(0, |acc, span: &DigSpan| {
                acc + span.len()
            });
            let skipped_area = (curr_row - last_seen_row - 1) as u64 * prev_dug_span_len;

            let (curr_span_len, next_spans) = edges.into_iter().fold((prev_dug_span_len, dug_spans), |(mut curr_span_len, next_spans), &edge| {
                let span_count = next_spans.len();

                // find the span that will be affected by this edge
                let mut modified_span = None;
                let mut new_spans = vec![];
                next_spans.into_iter().enumerate().for_each(|(span_ix, next_span)| {
                    if modified_span.is_some() { // already found and modified a span
                        new_spans.push(next_span);
                    } else if edge.start == next_span.start && edge.end == next_span.end { // edge is completely ending a span
                        modified_span = Some(span_ix);
                    } else if edge.end == next_span.start { // span is cutting out from left
                        modified_span = Some(span_ix);
                        curr_span_len += edge.len();
                        new_spans.push(DigSpan::new([edge.start, next_span.end]));
                    } else if edge.start == next_span.start { // span is cutting in from left
                        modified_span = Some(span_ix);
                        new_spans.push(DigSpan::new([edge.end, next_span.end]));
                    } else if edge.end == next_span.end { // span is cutting in from right
                        modified_span = Some(span_ix);
                        new_spans.push(DigSpan::new([next_span.start, edge.start]));
                    } else if edge.start == next_span.end { // span is cutting out from right
                        modified_span = Some(span_ix);
                        curr_span_len += edge.len();
                        new_spans.push(DigSpan::new([next_span.start, edge.end]));
                    } else if next_span.contains(&edge.start) && next_span.contains(&edge.end) { // span is splitting
                        modified_span = Some(span_ix);
                        new_spans.append(&mut vec![
                            DigSpan::new([next_span.start, edge.start]),
                            DigSpan::new([edge.end, next_span.end])
                        ]);
                    } else {
                        new_spans.push(next_span);
                    }
                });
                if let Some(modified_span_ix) = modified_span { // check if the span before and/or after should be merged with this one
                    if span_count == new_spans.len() { // can only happen if we didn't split an existing span
                        if let Some(span_after) = new_spans.get(modified_span_ix + 1).cloned() {
                            let modified_span = new_spans.get_mut(modified_span_ix).unwrap();
                            if modified_span.end == span_after.start {
                                modified_span.end = span_after.end;
                                new_spans.swap_remove(modified_span_ix + 1);
                                curr_span_len -= 1;
                            }
                        }
                        if let Some(span_before_ix) = modified_span_ix.checked_sub(1) {
                            if let Some(span_before) = new_spans.get(span_before_ix).cloned() {
                                let modified_span = new_spans.get_mut(modified_span_ix).unwrap();
                                if modified_span.end == span_before.start {
                                    modified_span.end = span_before.end;
                                    new_spans.swap_remove(span_before_ix);
                                    curr_span_len -= 1;
                                }
                            }
                        }
                    }
                } else { // edge is starting a new self-contained area
                    curr_span_len += edge.len() + 1;
                    new_spans.push(DigSpan::new([edge.start, edge.end]))
                }

                new_spans.sort_unstable();
                (curr_span_len, new_spans)
            });

            (acc + skipped_area + curr_span_len, Some(curr_row), next_spans)
        } else {
            let (area, starting_spans) = edges.iter().fold((0, vec![]), |(area, mut dug_spans), &edge| {
                let new_span = DigSpan::new([edge.start, edge.end]);
                dug_spans.push(new_span);
                (area + new_span.len(), dug_spans)
            });
            (area, Some(curr_row), starting_spans)
        }
    });

    if open_spans.len() != 0 {
        panic!("not all spans closed? algorithm error!");
    }

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"), 62);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"), 952408144115);
    }

}