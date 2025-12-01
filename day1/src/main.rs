use get_input::get_input;

fn main() {
    let day = 1;

    let input = match get_input(day) {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    if let Err(err) = part1(&input) {
        println!("Could not complete part 1: {}", err);
    }

    if let Err(err) = part2(&input) {
        println!("Could not complete part 2: {}", err);
    }
}

fn part1(input: &str) -> Result<(), String> {
    let mut dial_state = DialState::new();
    let mut zero_occurrences = 0;

    for rotation in input.lines().map(|l| Rotation::try_from(l).unwrap()) {
        dial_state.apply_rotation(&rotation);

        if dial_state.point() == 0 {
            zero_occurrences += 1;
        }
    }

    println!("Part 1: {}", zero_occurrences);
    Ok(())
}

fn part2(input: &str) -> Result<(), String> {
    let mut dial_state = DialState::new();
    let mut passing_zero = 0;

    for rotation in input.lines().map(|l| Rotation::try_from(l).unwrap()) {
        passing_zero += dial_state.apply_rotation(&rotation);

        if dial_state.point() == 0 {
            passing_zero += 1;
        }
    }

    println!("Part 2: {}", passing_zero);
    Ok(())
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let value = value.to_ascii_lowercase();
        match value {
            'l' => Ok(Direction::Left),
            'r' => Ok(Direction::Right),
            _ => Err(format!("Invalid direction \"{}\"", value)),
        }
    }
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    distance: i16,
}

impl TryFrom<&str> for Rotation {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let direction = value
            .chars()
            .nth(0)
            .ok_or("Invalid empty direction".to_string())?;
        let distance = &value[1..];

        Ok(Self {
            direction: direction.try_into()?,
            distance: distance
                .parse()
                .map_err(|e| format!("Invalid distance \"{}\": {}", distance, e))?,
        })
    }
}

#[derive(Debug)]
struct DialState {
    point: i16,
}

const MAX_DIAL: i16 = 100;
const DIAL_START: i16 = 50;

impl DialState {
    fn new() -> Self {
        Self { point: DIAL_START }
    }

    /// Apply the rotation to the dial and return the number of times
    /// it crosses zero during this rotation, not including if it stops
    /// at zero
    fn apply_rotation(&mut self, r: &Rotation) -> i16 {
        // Count the number of times the dial crosses zero

        // Because we only want to check if we step to 0 and not away from zero
        // we check if we start at zero and reject the reduced_distance crossing
        let started_at_zero = self.point == 0;

        // There is a number of zero crossings implicit in the distance
        let implicit_crossings = r.distance / MAX_DIAL;
        let reduced_distance = r.distance % MAX_DIAL;

        // With the reduced_distance the dial can only cross the zero zero or one more time
        let (new_point, extra_crossing): (i16, i16) = match r.direction {
            Direction::Left => (
                self.point - r.distance,
                (self.point < reduced_distance).into(),
            ),
            Direction::Right => (
                self.point + r.distance,
                (self.point + reduced_distance > MAX_DIAL).into(),
            ),
        };

        self.point = new_point.rem_euclid(MAX_DIAL);

        if started_at_zero {
            implicit_crossings
        } else {
            implicit_crossings + extra_crossing
        }
    }

    fn point(&self) -> i16 {
        self.point
    }
}
