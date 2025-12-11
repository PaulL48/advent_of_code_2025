use get_input::get_input;
use grid::Grid;
use std::collections::{HashMap, HashSet, hash_map::Keys};

fn main() {
    let input = get_input(7).unwrap();
    let first_line = input.lines().next().unwrap();
    let width = first_line.len();
    let start_coord = (first_line.find('S').unwrap() as u8, 0_u8);
    let input = input.chars().filter_map(|c| c.try_into().ok()).collect();
    let input = Grid::from_vec(input, width);
    let input1 = Board::new(input.clone(), start_coord);

    part1(input1.clone());

    let input2 = Board2::new(input, start_coord);
    part2(input2);
}

#[derive(Debug, Copy, Clone)]
enum Space {
    Blank,
    Splitter,
}

impl TryFrom<char> for Space {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' | 'S' => Ok(Space::Blank),
            '^' => Ok(Space::Splitter),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: Grid<Space>,
    lasers: Vec<(u8, u8)>,
    laser_deduplicator: HashSet<(u8, u8)>,
}

impl Board {
    fn new(grid: Grid<Space>, start: (u8, u8)) -> Self {
        let mut laser_deduplicator = HashSet::new();
        laser_deduplicator.insert(start);
        Self {
            grid,
            lasers: vec![start],
            laser_deduplicator,
        }
    }

    fn step(&mut self) -> u64 {
        let mut split_count = 0;
        let mut new_lasers = Vec::new();
        for laser in &self.lasers {
            let new_coord = (laser.0, laser.1 + 1);

            // Check if laser is out of bounds then we can remove it from the pool
            if new_coord.0 >= self.grid.size().0 as u8 || new_coord.1 >= self.grid.size().1 as u8 {
                continue;
            }

            if self.laser_deduplicator.contains(&new_coord) {
                continue;
            } else {
                self.laser_deduplicator.insert(new_coord);
            }

            match self.grid.get(new_coord.1, new_coord.0).unwrap() {
                Space::Blank => new_lasers.push(new_coord),
                Space::Splitter => {
                    split_count += 1;
                    new_lasers.push((new_coord.0 - 1, new_coord.1));
                    new_lasers.push((new_coord.0 + 1, new_coord.1));
                }
            }
        }
        self.lasers = new_lasers;
        split_count
    }

    fn is_empty(&self) -> bool {
        self.lasers.is_empty()
    }
}

fn part1(mut board: Board) {
    let mut total_splits = 0;
    while !board.is_empty() {
        total_splits += board.step();
    }
    println!("Part 1: Total splits {}", total_splits)
}

fn part2(mut board: Board2) {
    while board.step() {}

    println!("Part 2: Total splits {}", board.laser_count())
}

struct Board2 {
    grid: Grid<Space>,
    lasers: OverlapMap,
}

impl Board2 {
    pub fn new(grid: Grid<Space>, start: (u8, u8)) -> Self {
        let mut h = OverlapMap::new();
        h.add(start, 1);
        Self { grid, lasers: h }
    }

    pub fn step(&mut self) -> bool {
        if self.lasers.keys().next().unwrap().1 + 1 >= self.grid.size().0 as u8 {
            return false;
        }

        let mut new_lasers = OverlapMap::new();
        for (coord, overlap_count) in self.lasers.map() {
            let new_coord = (coord.0, coord.1 + 1);
            match self.grid.get(coord.1, coord.0).unwrap() {
                Space::Blank => new_lasers.add(new_coord, *overlap_count),
                Space::Splitter => {
                    new_lasers.add((new_coord.0 - 1, new_coord.1), *overlap_count);
                    new_lasers.add((new_coord.0 + 1, new_coord.1), *overlap_count);
                }
            }
        }
        self.lasers = new_lasers;

        true
    }

    fn laser_count(&self) -> u64 {
        self.lasers.map().values().sum()
    }
}

#[derive(Debug)]
struct OverlapMap {
    m: HashMap<(u8, u8), u64>,
}

impl OverlapMap {
    fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn add(&mut self, coord: (u8, u8), value: u64) {
        if let Some(entry) = self.m.get_mut(&coord) {
            *entry += value;
        } else {
            self.m.insert(coord, value);
        }
    }

    fn keys(&self) -> Keys<'_, (u8, u8), u64> {
        self.m.keys()
    }

    fn map(&self) -> &HashMap<(u8, u8), u64> {
        &self.m
    }
}
