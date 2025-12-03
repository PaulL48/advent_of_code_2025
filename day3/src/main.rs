use get_input::get_input;
use iter_first_max::IterFirstMaxExt;

fn main() {
    let input = match get_input(3) {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let input = format_input(&input);

    if let Err(err) = part1(&input) {
        println!("Could not complete part 1: {}", err);
    }

    if let Err(err) = part2(&input) {
        println!("Could not complete part 2: {}", err);
    }
}

fn format_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn part1(input: &[&str]) -> Result<(), String> {
    let mut sum = 0;
    for bank in input {
        let joltage = bank_maximum_joltage(bank, 2);
        sum += joltage;
    }

    println!("Part 1: Total joltage {}", sum);

    Ok(())
}

fn part2(input: &[&str]) -> Result<(), String> {
    let mut sum = 0;

    for bank in input {
        let joltage = bank_maximum_joltage(bank, 12);
        sum += joltage;
    }

    println!("Part 2: Total joltage {}", sum);

    Ok(())
}

fn bank_maximum_joltage(bank: &str, digits: u32) -> u64 {
    let mut collected_digits = String::new();
    let mut digits_remaining = digits as usize;
    let mut search_start = 0;
    while digits_remaining > 0 {
        let search_range_end = bank.len() - (digits_remaining - 1);
        let search_range = &bank[search_start..search_range_end];
        let (index, max) = search_range
            .chars()
            .enumerate()
            .first_max_by(|l, r| l.1.cmp(&r.1))
            .unwrap();
        collected_digits.push(max);
        digits_remaining -= 1;
        search_start += index + 1;
    }

    collected_digits.parse().unwrap()
}
