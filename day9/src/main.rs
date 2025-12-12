use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    ops::RangeInclusive,
};

use get_input::get_input;
use serde::{Deserialize, Serialize};

fn main() {
    let input: Vec<Vec2> = get_input(9).unwrap().lines().map(|l| l.into()).collect();
    let min_x = input.iter().map(|v| v.x).min().unwrap();
    let min_y = input.iter().map(|v| v.y).min().unwrap();
    let max_x = input.iter().map(|v| v.x).max().unwrap();
    let max_y = input.iter().map(|v| v.y).max().unwrap();
    let min = Vec2::new(min_x, min_y);
    let max = Vec2::new(max_x, max_y);

    part1(&input);
    part2(&input, &min, &max);
}

fn part1(input: &[Vec2]) {
    let areas = compute_areas(input);
    println!(
        "Part 1: largest area possible is {}",
        areas.first().unwrap().0
    )
}

fn part2(input: &[Vec2], min: &Vec2, max: &Vec2) {
    let bounds = create_vertical_line_bounds(input);
    println!("Computed bounds");
    println!("Starting compression");
    let compressed_bounds =
        CompressedRowBounds::create_or_from_cache(&bounds, "./cache/crb.bin", min, max);
    println!("Finished compression");

    let areas = compute_areas(input);
    let mut i = 0;
    let largest_valid_area = areas
        .iter()
        .find(|v| {
            if i % 10 == 0 {
                println!("{}/{} areas checked", i, areas.len());
            }
            i += 1;
            points_in_bounds(&four_edges(v.1.0, v.1.1), &compressed_bounds)
        })
        .unwrap();
    println!("Part 2: Largest valid area is {}", largest_valid_area.0);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: u64,
    y: u64,
}

impl From<&str> for Vec2 {
    fn from(value: &str) -> Self {
        let values = value
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect::<Vec<_>>();
        Self {
            x: values[0],
            y: values[1],
        }
    }
}

impl Vec2 {
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    fn area_between(&self, rhs: &Vec2) -> u64 {
        let x_span = self.x.abs_diff(rhs.x) + 1;
        let y_span = self.y.abs_diff(rhs.y) + 1;

        x_span * y_span
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CompressedRowBounds {
    bounds: HashMap<u64, Vec<RangeInclusive<u64>>>,
}

impl CompressedRowBounds {
    fn from_line_bounds(bounds: &HashSet<Vec2>, min: &Vec2, max: &Vec2) -> Self {
        let mut all_range_bounds = HashMap::new();
        for y in min.y..=max.y {
            let mut range_start = 0;
            let mut row_bounds = Vec::new();
            let mut in_bound = bounds.contains(&Vec2::new(min.x, min.y));
            for x in min.x..=max.x {
                if bounds.contains(&Vec2::new(x, y)) {
                    if in_bound {
                        // Exiting a range
                        row_bounds.push(range_start..=x);
                    } else {
                        // Entering a range
                        range_start = x;
                    }
                    in_bound = !in_bound;
                }
            }
            all_range_bounds.insert(y, row_bounds);

            if y % 1000 == 0 {
                println!("Y {}", y);
            }
        }

        Self {
            bounds: all_range_bounds,
        }
    }

    fn point_in_bounds(&self, p: &Vec2) -> bool {
        for range in self.bounds.get(&p.y).unwrap() {
            if range.contains(&p.x) {
                return true;
            }
        }
        false
    }

    fn create_or_from_cache(bounds: &HashSet<Vec2>, path: &str, min: &Vec2, max: &Vec2) -> Self {
        match OpenOptions::new().read(true).open(path) {
            Ok(file) => {
                println!("Found cached CRB");
                // File exists deserialize it

                ciborium::from_reader(file).unwrap()
            }
            Err(_err) => {
                println!("No cached CRB. Creating now (3 minutes)");
                // File does not exist, create structure and serialize it
                let file = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .truncate(true)
                    .create_new(true)
                    .open(path)
                    .unwrap();
                let compressed_bound = Self::from_line_bounds(bounds, min, max);
                ciborium::into_writer(&compressed_bound, file).unwrap();
                compressed_bound
            }
        }
    }
}

fn compute_areas(input: &[Vec2]) -> Vec<(u64, (&Vec2, &Vec2))> {
    let mut areas = Vec::new();
    for (i, entry) in input.iter().enumerate() {
        for j in input.iter().skip(i + 1) {
            areas.push((entry.area_between(j), (entry, j)));
        }
    }
    areas.sort_by(|l, r| r.0.cmp(&l.0));
    areas
}

fn line_between(a: &Vec2, b: &Vec2) -> Vec<Vec2> {
    if *a == *b {
        return vec![*a];
    }

    match (a.x == b.x, a.y == b.y) {
        (true, false) => {
            if a.y < b.y {
                (a.y..=b.y).map(|v| Vec2::new(a.x, v)).collect()
            } else {
                (b.y..=a.y).map(|v| Vec2::new(a.x, v)).collect()
            }
        } // Vertical line
        (false, true) => {
            if a.x < b.x {
                (a.x..=b.x).map(|v| Vec2::new(v, a.y)).collect()
            } else {
                (b.x..=a.x).map(|v| Vec2::new(v, a.y)).collect()
            }
        }
        (false, false) => panic!("Cannot be connected with a line"),
        _ => panic!(""),
    }
}

/// Return the line of points between a and b only if they form a
/// vertical line. This line does not include the endpoints
fn line_between_vertical_only(
    a: &Vec2,
    b: &Vec2,
    exclude_endpoint: bool,
    exclude_start_point: bool,
) -> Vec<Vec2> {
    if *a == *b {
        return vec![*a];
    }

    let (start, end) = match (a.x == b.x, a.y == b.y) {
        (true, false) => {
            if a.y < b.y {
                let start = if exclude_start_point { a.y + 1 } else { a.y };

                let end = if exclude_endpoint { b.y } else { b.y + 1 };

                (start, end)
            } else {
                let end = if exclude_start_point { a.y } else { a.y + 1 };

                let start = if exclude_endpoint { b.y + 1 } else { b.y };

                (start, end)
            }
        }
        (false, true) => return Vec::new(),
        (false, false) => panic!("Cannot be connected with a line"),
        _ => panic!(""),
    };

    let out = (start..end).map(|v| Vec2::new(a.x, v)).collect();
    println!("{:?} -> {:?}: {:?}", a, b, out);
    out
}

fn four_edges(a: &Vec2, b: &Vec2) -> Vec<Vec2> {
    let other1 = Vec2::new(a.x, b.y);
    let other2 = Vec2::new(b.x, a.y);

    let mut lines = Vec::new();
    lines.extend(line_between(a, &other1));
    lines.extend(line_between(a, &other2));
    lines.extend(line_between(b, &other1));
    lines.extend(line_between(b, &other2));
    lines
}

fn points_in_bounds(p: &[Vec2], bounds: &CompressedRowBounds) -> bool {
    p.iter().all(|p| bounds.point_in_bounds(p))
}

fn create_vertical_line_bounds(points: &[Vec2]) -> HashSet<Vec2> {
    let mut wrapped_points = points.to_vec();
    wrapped_points.push(points[0]); // This closes the polygon
    wrapped_points.push(points[1]); // This allows our corner direction algorithm to see the last corner direction
    let mut bounds = HashSet::new();
    let mut exclude_start_point = false;
    for w in wrapped_points.windows(3) {
        // Here we can use the third point to get directional information
        let d1 = get_line_direction(&w[0], &w[1]).unwrap();
        let d2 = get_line_direction(&w[1], &w[2]).unwrap();
        let exclude_endpoint = matches!(
            (d1, d2),
            (LineDirection::Up, LineDirection::Left)
                | (LineDirection::Right, LineDirection::Up)
                | (LineDirection::Down, LineDirection::Right)
                | (LineDirection::Left, LineDirection::Down)
        );
        println!("{:?}-{:?}: inside? {}", d1, d2, exclude_endpoint);

        bounds.extend(line_between_vertical_only(
            &w[0],
            &w[1],
            exclude_endpoint,
            exclude_start_point,
        ));
        exclude_start_point = exclude_endpoint;
    }

    bounds
}

#[derive(Debug, Clone, Copy)]
enum LineDirection {
    Left,
    Right,
    Up,
    Down,
}

// Return the direction of the line or nothing if a and b are the same
fn get_line_direction(a: &Vec2, b: &Vec2) -> Option<LineDirection> {
    if a == b {
        return None;
    }

    match (a.x == b.x, a.y == b.y) {
        (true, false) => {
            // Vertical line
            if a.y < b.y {
                Some(LineDirection::Down)
            } else {
                Some(LineDirection::Up)
            }
        }
        (false, true) => {
            // Horizontal line
            if a.x < b.x {
                Some(LineDirection::Right)
            } else {
                Some(LineDirection::Left)
            }
        }
        (false, false) => panic!("Not a line"),
        (true, true) => panic!("Same point"),
    }
}
