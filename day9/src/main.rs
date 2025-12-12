use std::collections::HashSet;

use get_input::get_input;

fn main() {
    let input: Vec<Vec2> = get_input(9).unwrap().lines().map(|l| l.into()).collect();


    part1(&input);
    part2(&input);
}

fn part1(input: &[Vec2]) {
    let areas = compute_areas(input);
    println!("Part 1: largest area possible is {}", areas.first().unwrap().0)
}

fn part2(input: &[Vec2]) {
    // Here's the assumption
    // The largest rectangle will always be formed by three consecutive
    // edges especially since the constant axis alternates between every point

    let mut new_input = input.to_vec();
    
    // We clone the first point to the end to be able to check the wrapping behavior
    new_input.push(input[0]);

    let mut bounds = HashSet::new();
    for w in new_input.windows(2) {
        bounds.extend(line_between(&w[0], &w[1]));
    }

    // We augment the bounds with the interior area
    // let mut area = HashSet::new();
    // let mut in_bound = false;
    // for i in 0..100000 {
    //     for j in 0..100000 {
    //         if bounds.contains(&Vec2::new(i, j)) {
    //             in_bound = !in_bound;
    //         }
            
    //         if in_bound {
    //             area.insert(Vec2::new(i, j));
    //         }
    //     }
    //     if i % 1000 == 0 {
    //         println!("x {:?}", i);
    //     }
    // }

    let mut all_areas = compute_areas(input);
    let actual_areas = all_areas
        .iter()
        .filter(|v| points_in_bounds(&four_corners(v.1.0, v.1.1), &bounds))
        .collect::<Vec<_>>();
    println!("{:?}", actual_areas);
    // println!("Part 2: Constrained max area is {}", max_area);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: u64,
    y: u64
}

impl From<&str> for Vec2 {
    fn from(value: &str) -> Self {
        let values = value.split(",").map(|v| v.parse().unwrap()).collect::<Vec<_>>();
        Self {
            x: values[0],
            y: values[1],
        }
    }
}

impl Vec2 {
    fn new(x: u64, y: u64) -> Self {
        Self {
            x,
            y
        }
    }

    fn area_between(&self, rhs: &Vec2) -> u64 {
        let x_span = self.x.abs_diff(rhs.x) + 1;
        let y_span = self.y.abs_diff(rhs.y) + 1;

        x_span * y_span
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
                return (a.y..b.y).map(|v| Vec2::new(a.x, v)).collect();
            } else {
                return (b.y..a.y).map(|v| Vec2::new(a.x, v)).collect();
            }
        }, // Horizontal line
        (false, true) => {
            if a.x < b.x {
                return (a.x..b.x).map(|v| Vec2::new(v, a.y)).collect();
            } else {
                return (b.x..a.x).map(|v| Vec2::new(v, a.y)).collect();
            }
        },
        (false, false) => panic!("Cannot be connected with a line"),
        _ => panic!("")
    }
}

fn four_corners(a: &Vec2, b: &Vec2) -> [Vec2; 4] {
    [*a, *b, Vec2::new(a.x, b.y), Vec2::new(b.x, a.y)]
}

fn points_in_bounds(p: &[Vec2], bounds: &HashSet<Vec2>) -> bool {
    p.iter().all(|v| area.contains(v))
}
