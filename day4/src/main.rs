use get_input::get_input;
use grid::Grid;

fn main() {
    let input = match get_input(4) {
        Ok(input) => input,
        Err(err) => panic!("Could not get input: {}", err),
    };
    let column_count = input.lines().nth(0).unwrap().len();
    let input = input.chars().filter_map(char_to_boolean).collect();
    let input = Grid::from_vec(input, column_count);

    if let Err(err) = part1(&input) {
        println!("Could not complete part 1: {}", err);
    }

    if let Err(err) = part2(&input) {
        println!("Could not complete part 1: {}", err);
    }
}

fn part1(input: &Grid<bool>) -> Result<(), String> {
    println!("Part 1: number of accessible rolls {}", get_reachable_coordinates(input).len());
    Ok(())
}

fn part2(input: &Grid<bool>) -> Result<(), String> {
    let mut input = input.clone();
    let mut count = 0;

    loop {
        let reachable= get_reachable_coordinates(&input);
        count += reachable.len();
        if reachable.len() == 0 {
            break;
        }

        for c in reachable {
            *input.get_mut(c.0, c.1).unwrap() = false;
        }
    }

    println!("Part 2: reachable rolls with removals {}", count);

    Ok(())
}

fn char_to_boolean(c: char) -> Option<bool> {
    match c {
        '.' => Some(false),
        '@' => Some(true),
        _ => None,
    }
}

fn generate_neighbor_indices(row: i32, col: i32) -> [(i32, i32); 8] {
    [(row - 1, col - 1), (row - 1, col), (row - 1, col + 1),
     (row, col - 1),                     (row, col + 1),
     (row + 1, col - 1), (row + 1, col), (row + 1, col + 1)]
}

fn coordinate_occupied(coordinate: &(i32, i32), grid: &Grid<bool>) -> bool {
    grid.get(coordinate.0, coordinate.1).copied().unwrap_or(false)
}

fn coordinate_reachable(row: usize, col: usize, grid: &Grid<bool>) -> bool {
    if !grid.get(row, col).unwrap() {
        return false;
    }

    let neighbors = generate_neighbor_indices(row as i32, col as i32);
    let occupied_neighbors: u8 = neighbors.iter().map(|c| coordinate_occupied(c, grid) as u8).sum();
    
    occupied_neighbors < 4
}

fn get_reachable_coordinates(grid: &Grid<bool>) -> Vec<(usize, usize)> {
    let mut reachable = Vec::new();
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if coordinate_reachable(row, col, &grid) {
                reachable.push((row, col));
            }
        }
    }
    reachable
}