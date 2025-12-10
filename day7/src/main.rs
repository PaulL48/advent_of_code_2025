use std::collections::HashSet;

use get_input::get_input;
use grid::Grid;

fn main() {
    let input = get_input(7).unwrap();
    let first_line = input.lines().next().unwrap();
    let width = first_line.len();
    let start_coord = (first_line.find(|c| c == 'S').unwrap(), 0_usize);
    let input = input.chars().filter_map(|c| c.try_into().ok()).collect();
    let input = Grid::from_vec(input, width);
    let input = Board::new(input, start_coord);

    part1(input.clone());
    part2(input.clone());
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
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    grid: Grid<Space>,
    lasers: Vec<(usize, usize)>,
    laser_deduplicator: HashSet<(usize, usize)>,
}

impl Board {
    fn new(grid: Grid<Space>, start: (usize, usize)) -> Self {
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
            if new_coord.0 >= self.grid.size().0 || new_coord.1 >= self.grid.size().1 {
                continue;
            }

            // print!("new laser coord {:?} -> {:?}", new_coord, self.grid.get(new_coord.1, new_coord.0));

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
                },
            }
        }
        self.lasers = new_lasers;
        split_count
    }

    fn step_with_duplicates(&mut self) -> u64 {
        let mut split_count = 0;
        let mut new_lasers = Vec::new();
        println!("Laser count: {}", self.lasers.len());
        for laser in &self.lasers {
            let new_coord = (laser.0, laser.1 + 1);

            // Check if laser is out of bounds then we can remove it from the pool
            if new_coord.0 >= self.grid.size().0 || new_coord.1 >= self.grid.size().1 {
                continue;
            }

            match self.grid.get(new_coord.1, new_coord.0).unwrap() {
                Space::Blank => new_lasers.push(new_coord),
                Space::Splitter => {
                    split_count += 1;
                    new_lasers.push((new_coord.0 - 1, new_coord.1));
                    new_lasers.push((new_coord.0 + 1, new_coord.1));
                },
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

fn part2(mut board: Board) {
    let mut total_splits = 0;
    while !board.is_empty() {
        total_splits += board.step_with_duplicates();
    }
    println!("Part 1: Total splits {}", total_splits + 1)
}
