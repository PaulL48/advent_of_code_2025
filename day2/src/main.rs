use get_input::get_input;
use std::iter::repeat_n;

fn main() {
    let input = match get_input(2) {
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

fn format_input(input: &str) -> Vec<(&str, &str)> {
    let mut output = Vec::new();
    for pair in input.split(",") {
        let mut split_iter = pair.split("-");
        let start = split_iter.next().unwrap().trim();
        let end = split_iter.next().unwrap().trim();
        output.push((start, end));
    }
    output
}

fn ceil_to_even_number_of_digits(input: &str) -> u64 {
    let input_str_len = input.len();

    if input_str_len.is_multiple_of(2) {
        input.parse().unwrap()
    } else {
        let new_length = input_str_len as u32;
        10_u64.pow(new_length)
    }
}

fn floor_to_even_number_of_digits(input: &str) -> u64 {
    let input_str_len = input.len();

    if input.len() == 1 || input_str_len.is_multiple_of(2) {
        input.parse().unwrap()
    } else {
        let new_length = input_str_len as u32 - 1;
        repeat_n('9', new_length as usize)
            .collect::<String>()
            .parse()
            .unwrap()
    }
}

fn split_number(input: u64) -> (u64, u64) {
    let input = input.to_string();
    let split = input.len() / 2;
    let first_half = &input[..split];
    let second_half = &input[split..];
    (first_half.parse().unwrap(), second_half.parse().unwrap())
}

fn part1(inputs: &[(&str, &str)]) -> Result<(), String> {
    let total: u64 = inputs
        .iter()
        .map(|(s, e)| total_of_mirror_codes(s, e))
        .sum();
    println!("Part 1: sum of invalid codes {}", total);
    Ok(())
}

fn part2(inputs: &[(&str, &str)]) -> Result<(), String> {
    let total: u64 = inputs
        .iter()
        .map(|(s, e)| total_of_repeat_codes(s, e))
        .sum();
    println!("Part 2: sum of invalid codes {}", total);
    Ok(())
}

fn total_of_mirror_codes(start_code: &str, end_code: &str) -> u64 {
    let start = ceil_to_even_number_of_digits(start_code);
    let end = floor_to_even_number_of_digits(end_code);

    // Now we can begin by auto rejecting any range that is not ordered
    if start > end {
        return 0;
    }

    let mut count = 0;
    for i in start..=end {
        if code_is_mirror(i) {
            count += i;
        }
    }

    count
}

fn code_is_mirror(i: u64) -> bool {
    let (left, right) = split_number(i);
    left == right
}

fn total_of_repeat_codes(start_code: &str, end_code: &str) -> u64 {
    let start: u64 = start_code.parse().unwrap();
    let end: u64 = end_code.parse().unwrap();

    let mut count = 0;
    for i in start..=end {
        if code_is_repeated(i) {
            count += i;
        }
    }

    count
}

fn code_is_repeated(i: u64) -> bool {
    let code = i.to_string();

    // So this iteration isn't over the string its over lengths of substrings
    // The highest we can go is half of the length rounded down since more than that
    // and we don't have enough length to reproduce the beginning substring
    // println!("Checking code {}", i);
    let pattern_length_limit = code.len() / 2;
    for i in (1..=pattern_length_limit).rev() {
        // We reverse to early exit on larger patterns

        // We can early reject the pattern if the code length isn't a multiple of the pattern size
        if !code.len().is_multiple_of(i) {
            continue;
        }
        // capture the potential pattern
        let pattern_candidate = &code[..i];

        // Step over the string starting at the end of the pattern with step size of the pattern
        let mut failed = false;
        let substring_start_endpoint = code.len() - i;
        for j in (i..=substring_start_endpoint).step_by(i) {
            if &code[j..(j + i)] != pattern_candidate {
                failed = true;
                break;
            }
        }

        if !failed {
            return true;
        }
    }

    false
}
