use get_input::get_input;
use std::{collections::HashSet, ops::Sub};

fn main() {
    let input = get_input(8)
        .unwrap()
        .lines()
        .map(Vec3::from_str)
        .collect::<Vec<_>>();
    part1(&input, 1000);
    part2(&input);
}

fn part1(input: &[Vec3], take: usize) {
    let distances = compute_sorted_distances(input);
    let circuits = join_first_n_junctions(take, &distances);
    let mut circuit_sizes = circuits.iter().map(|c| c.len() as u64).collect::<Vec<_>>();
    circuit_sizes.sort_by(|l, r| r.cmp(l));
    // println!("Largest circuits: {:?}", circuit_sizes);
    println!(
        "Part 1: Product of three largest circuit sizes {}",
        circuit_sizes.iter().cloned().take(3).product::<u64>()
    );
}

fn part2(input: &[Vec3]) {
    let distances = compute_sorted_distances(input);
    let (j1, j2) = join_junctions_until_one_circuit(&distances, input.len());
    println!(
        "Part 2: Product of two last junction X coords {}",
        j1.x * j2.x
    );
}

fn compute_sorted_distances(input: &[Vec3]) -> Vec<(i64, &Vec3, &Vec3)> {
    let mut distances = Vec::new();
    for (i, entry) in input.iter().enumerate() {
        for j in input.iter().skip(i + 1) {
            distances.push(((*j - *entry).mag_sqrd(), entry, j));
        }
    }
    distances.sort_unstable_by(|l, r| l.0.cmp(&r.0));
    distances
}

fn join_first_n_junctions(n: usize, distances: &[(i64, &Vec3, &Vec3)]) -> Vec<HashSet<Vec3>> {
    let mut circuits = Vec::new();
    for (_distance, j1, j2) in distances.iter().take(n) {
        // print!("Checking {:?} <-> {:?} ", j1, j2);
        // Check that if one is a circuit that they both aren't in the same circuit
        if get_circuit(&mut circuits, j1).is_some()
            && get_circuit(&mut circuits, j1) == get_circuit(&mut circuits, j2)
        {
            // println!("Junctions are from the same circuit");
            continue;
        }

        merge_junctions(&mut circuits, j1, j2);
        // println!("Current circuits: {:?}", circuits);
    }
    circuits
}

fn join_junctions_until_one_circuit(
    distances: &[(i64, &Vec3, &Vec3)],
    junction_count: usize,
) -> (Vec3, Vec3) {
    let mut circuits = Vec::new();
    for (_distance, j1, j2) in distances.iter() {
        // print!("Checking {:?} <-> {:?} ", j1, j2);
        // Check that if one is a circuit that they both aren't in the same circuit
        if get_circuit(&mut circuits, j1).is_some()
            && get_circuit(&mut circuits, j1) == get_circuit(&mut circuits, j2)
        {
            // println!("Junctions are from the same circuit");
            continue;
        }

        merge_junctions(&mut circuits, j1, j2);
        if let Some(circuit) = circuits.first()
            && circuit.len() == junction_count
        {
            return (**j1, **j2);
        }
    }

    panic!("Mono circuit not formed");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn from_str(s: &str) -> Self {
        let values = s.split(",").collect::<Vec<_>>();
        Self {
            x: values[0].parse().unwrap(),
            y: values[1].parse().unwrap(),
            z: values[2].parse().unwrap(),
        }
    }

    fn mag_sqrd(&self) -> i64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn get_circuit(circuits: &mut [HashSet<Vec3>], j: &Vec3) -> Option<usize> {
    circuits.iter().enumerate().find_map(|(i, a)| {
        if a.contains(j) {
            return Some(i);
        }
        None
    })
}

fn merge_junctions(circuits: &mut Vec<HashSet<Vec3>>, j1: &Vec3, j2: &Vec3) {
    let j1_circuit = get_circuit(circuits, j1);
    let j2_circuit = get_circuit(circuits, j2);

    match (j1_circuit, j2_circuit) {
        (Some(c1), Some(c2)) => {
            let c = circuits[c2].clone();
            circuits[c1].extend(c);
            circuits.remove(c2);
        }
        (None, Some(c)) => {
            circuits[c].insert(*j1);
        }
        (Some(c), None) => {
            circuits[c].insert(*j2);
        }
        (None, None) => {
            let mut new_circuit = HashSet::new();
            new_circuit.insert(*j1);
            new_circuit.insert(*j2);
            circuits.push(new_circuit);
        }
    }
}
