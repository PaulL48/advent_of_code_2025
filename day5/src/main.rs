use std::ops::{Range, RangeInclusive};

use get_input::get_input;

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn main() {
    let input = get_input(5).unwrap();

    let input = format_input(&input);

    part1();
    part2();
}

fn format_input(input: &str) -> Input {
    let mut input_iter = input.split("\n\n");
    let ranges = input_iter.next().unwrap();
    let ingredients = input_iter.next().unwrap();

    let mut endpoints = Vec::new();
    for (start, end) in ranges.lines().map(str_to_range) {
        endpoints.push(RangeEndpoint::Start(start));
        endpoints.push(RangeEndpoint::End(end));
    }

    endpoints.sort();

    let mut range_depth = 0;
    for endpoint in endpoints.iter() {
        match endpoint {
            RangeEndpoint::Start(s) => {


                range_depth += 1;
            },
            RangeEndpoint::End(e) => {

                
                range_depth -= 1;
            },
        }
    }

    // Input {
    //     ranges: r,
    //     ingredients: ingredients.lines().map(|l| l.parse().unwrap()).collect()
    // }

    todo!()
}

fn part1(input: &Input) {

}

fn part2(input: &Input) {

}

fn str_to_range(s: &str) -> (u64, u64) {
    let mut i = s.split("-");
    let start = i.next().unwrap().parse().unwrap();
    let end = i.next().unwrap().parse().unwrap();
    (start, end)
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum RangeEndpoint {
    Start(u64),
    End(u64)
}

fn accumulate_range(ranges: &mut Vec<RangeEndpoint>, v: RangeEndpoint) {

}
