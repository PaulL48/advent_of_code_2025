use std::iter::successors;

use get_input::get_input;
use itertools::izip;

fn main() {
    let input = get_input(6).unwrap();
    let p1_input = format_part_1_input(&input);

    part1(&p1_input);
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl TryFrom<&str> for Operation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "*" => Ok(Operation::Multiply),
            "+" => Ok(Operation::Add),
            s => Err(format!("Unexpected character {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Problem {
    values: [u64; 4],
    operation: Operation,
}

impl Problem {
    fn solve(&self) -> u64 {
        match self.operation {
            Operation::Add => self.values.iter().sum(),
            Operation::Multiply => self.values.iter().product(),
        }
    }

    fn width(&self) -> usize {
        self.values.iter().map(digit_width).max().unwrap()
    }
}

fn format_part_1_input(input: &str) -> Vec<Problem> {
    // Input is four lines of numbers
    // plus a line of operators
    let mut line_iter = input.lines();
    let line_1 = make_line_iter(line_iter.next().unwrap());
    let line_2 = make_line_iter(line_iter.next().unwrap());
    let line_3 = make_line_iter(line_iter.next().unwrap());
    let line_4 = make_line_iter(line_iter.next().unwrap());
    let ops_line = line_iter.next().unwrap().split_whitespace().collect::<Vec<_>>();

    let mut problems = Vec::new();
    for (v1, v2, v3, v4, op) in izip!(line_1, line_2, line_3, line_4, ops_line) {
        let p = Problem {
            values: [v1, v2, v3, v4],
            operation: op.try_into().unwrap(),
        };
        problems.push(p);
    }
    problems
}

fn format_part_2_input(input: &str, problem_widths: &[usize]) -> Vec<Problem> {
    let mut line_iter = input.lines();
    let line_1 = make_p2_line_iter(line_iter.next().unwrap(), problem_widths);
    let line_2 = make_p2_line_iter(line_iter.next().unwrap(), problem_widths);
    let line_3 = make_p2_line_iter(line_iter.next().unwrap(), problem_widths);
    let line_4 = make_p2_line_iter(line_iter.next().unwrap(), problem_widths);
    let ops_line = line_iter.next().unwrap().split_whitespace().collect::<Vec<_>>();
    Vec::new()
}

fn make_line_iter(l: &str) -> Vec<u64> {
    l.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>()
}

fn make_p2_line_iter(l: &str, widths: &[usize]) -> Vec<String> {
    let mut digit_strs = Vec::new();
    let mut cursor = 0;
    for width in widths {
        digit_strs.push(l[cursor..(cursor + width)].to_string());
        cursor += width;
    }
    digit_strs
}

fn part1(input: &[Problem]) {
    let total: u64 = input.iter().map(|p| p.solve()).sum();
    println!("Part 1: Total of solutions {}", total);
}

fn part2() {

}

fn digit_width(d: &u64) -> usize {
    successors(Some(*d), |&n| (n >= 10).then(|| n / 10)).count()
}
