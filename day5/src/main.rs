use get_input::get_input;
use std::ops::RangeInclusive;

struct Input {
    ranges: Vec<RangeInclusive<u64>>,
    ingredients: Vec<u64>,
}

fn main() {
    let input = get_input(5).unwrap();

    let input = Input::new(&input);

    part1(&input);
    part2(&input);
}

impl Input {
    fn new(input: &str) -> Self {
        let mut input_iter = input.split("\n\n");
        let ranges = input_iter.next().unwrap();
        let ingredients = input_iter
            .next()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();

        let mut original_ranges = Vec::new();
        let mut endpoints = Vec::new();
        for (start, end) in ranges.lines().map(str_to_range) {
            original_ranges.push(start..=end);
            endpoints.push(RangeEndpoint::Start(start));
            endpoints.push(RangeEndpoint::End(end));
        }
        endpoints.sort();

        let mut compressed_endpoints = Vec::new();
        let mut range_depth = 0;
        let mut current_range_start = None;
        for endpoint in &endpoints {
            match endpoint {
                RangeEndpoint::Start(s) => {
                    if range_depth == 0 {
                        current_range_start = Some(s);
                    }
                    range_depth += 1;
                }
                RangeEndpoint::End(e) => {
                    range_depth -= 1;
                    if let Some(start) = current_range_start
                        && range_depth == 0
                    {
                        compressed_endpoints.push(*start..=*e);
                    }
                }
            }
        }

        // Ranges can still be neighbors and 1 length ranges can overlap with neighbors
        let compressed_endpoints = fuse_neighboring_ranges(&compressed_endpoints);

        Self {
            ranges: compressed_endpoints,
            ingredients,
        }
    }

    fn contains(&self, v: u64) -> bool {
        !self.ranges.iter().all(|r| !r.contains(&v))
    }
}

fn part1(input: &Input) {
    let mut count = 0;
    for ingredient in &input.ingredients {
        if input.contains(*ingredient) {
            count += 1;
        }
    }
    println!("Part 1: {} ingredients are fresh", count);
}

fn part2(input: &Input) {
    for r in &input.ranges {
        println!("{:?}: {}", r, r.clone().count());
    }
    let s: usize = input.ranges.iter().cloned().map(|r| r.count()).sum();
    println!("Part 2: {} total ingredient IDs are fresh", s);
}

fn str_to_range(s: &str) -> (u64, u64) {
    let mut i = s.split("-");
    let start = i.next().unwrap().parse().unwrap();
    let end = i.next().unwrap().parse().unwrap();
    (start, end)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RangeEndpoint {
    Start(u64),
    End(u64),
}

impl Ord for RangeEndpoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let l = match self {
            Self::Start(s) => s,
            Self::End(e) => e,
        };

        let r = match other {
            Self::Start(s) => s,
            Self::End(e) => e,
        };

        l.cmp(r)
    }
}

impl PartialOrd for RangeEndpoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn fuse_neighboring_ranges(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let mut fused = Vec::new();
    let mut i = 0;
    while i < ranges.len() {
        let c = &ranges[i];
        let current_start = *c.start();
        let mut current_end = *c.end();
        for (j, n) in ranges.iter().enumerate().skip(i + 1) {
            if !(c.end() == n.start() || c.end() + 1 == *n.start()) {
                i = j - 1;
                break;
            }
            current_end = *ranges[j].end();
            i = j;
        }
        fused.push(current_start..=current_end);
        i += 1;
    }

    fused
}
