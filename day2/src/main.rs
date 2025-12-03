use std::{collections::btree_map::Iter, iter::repeat_n};

use get_input::get_input;

fn main() {
    let input = match get_input(2) {
        Ok(input) => input,
        Err(err) => panic!("{}", err),
    };

    let input = format_input(&input);

    if let Err(err) = part1(&input) {
        println!("Could not complete part 1: {}", err);
    }

    if let Err(err) = part2() {
        println!("Could not complete part 2: {}", err);
    }
}

fn format_input(input: &str) -> Vec<(&str, &str)> {
    let mut output = Vec::new();
    for pair in input.split(",") {
        let mut split_iter = pair.split("-");
        let start = split_iter.next().unwrap();
        let end = split_iter.next().unwrap();
        output.push((start, end));
    }
    output
}

// Some notes
// there is an intuition about whether a range can contain a double-number
// 10-12 contains one

// We can early reject some candidates by their number of digits. an odd number of
// digits is impossible to have a double-number

// Lets just look at the double numbers in N

// 0,1,2,3,4,5,6,7,8,9
// 10
// 11  -----
// 12
// 13
// 14
// 15
// 16
// 17
// 18
// 19
// 20
// 21
// 22  -----
// ...
// 33  -----
// 44  -----
// ... skip many
// 1000
// 1010 -----
// 1111 -----

// So for the given ranges
// 0-9: Odd number of digits
// 10-99: 9 doubles 11, 22, 33, 44, 55, 66, 77, 88, 99
// 100-999: Odd number of digits
// 1000-1999: 1010,1111,1212,1313,1414,1515,1616,1717,1818,1919 (10)
// 2000-2999: 2020,2121,2222,2323,2424,2525,2626,2727,2828,2929 (10)
// 3000-3999: 10
// 4000-4999: 10
// 5000-5999: 10
// 6000-6999: 10
// 7000-7999: 10
// 8000-8999: 10
// 9000-9999: 9090,9191,9292,...,9999 (10)
// 1000-9999: 9 * 10 = 90
// 10000-99999: OND
// 100000: 100100, 101101, 102102, 103103, ..., 199199 (100)


// If we split the number in half, we can check how many increments it takes to roll
// over the most significant digit
// so for 2000-2999: 2020 -> (20 20) its just the number of numbers between 20 and 29 inclusive (10)

// lets check that logic against 10-99
// 11 -> (1 1) the number of numbers between 1 and 9 inclusively (9)

// we can extend the 2000-2999 into 1000-9999
// Four digit numbers, start with two digits and duplicate them
// (10 10) -> The number of numbers between 10 and 99 inclusively (90)

// For six digit numbers, start with three digit numbers
// (100 100) -> The number of numbers between 100 and 999 inclusively (900) 

// So the ranges are more of a digit range and start point for each digit range

// So given an arbitrary numerical start point and arbitrary numerical end point
// how do we emit the number of double numbers between them

// The question we're asking is given a number "how long is an uninturrepted sequences starting from"

fn sum_of_invalid_codes(start: &str, end: &str) -> u64 {
    let startn = start.parse().unwrap();
    let endn = end.parse().unwrap();
    let nstart: u64 = ceil_to_even_number_of_digits(start);
    let nend = floor_to_even_number_of_digits(end);

    // println!("{}-{} -> {}-{}", start, end, nstart, nend);

    // Now we can begin by auto rejecting any range that is not ordered
    if nstart > nend {
        return 0;
    }

    let mut count = 0;
    for code in startn..=endn {
        println!("{}", code);
        let (l, r) = split_number(code);
        if l == r {
            count += code;
        }
    }

    count
}

fn ceil_to_even_number_of_digits(input: &str) -> u64 {
    let input_str_len = input.len();

    if input_str_len.is_multiple_of(2) {
        input.parse().unwrap()
    } else {
        let new_length = input_str_len as u32;
        let new_output = 10_u64.pow(new_length);
        new_output
    }
}

fn floor_to_even_number_of_digits(input: &str) -> u64 {
    let input_str_len = input.len();

    if input.len() == 1 || input_str_len.is_multiple_of(2) {
        input.parse().unwrap()
    } else {
        let new_length = input_str_len as u32 - 1;
        let new_output = repeat_n('9', new_length as usize).collect::<String>().parse().unwrap();
        new_output
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
    let mut total = 0;
    for (start, end) in inputs {
        total += sum_of_invalid_codes(start, end);
    }

    println!("Part 1: sum of invalid codes {}", total);
    Ok(())
}

fn part2() -> Result<(), String> {
    Ok(())
}


// fn part1_2(input: &[]) {

// }

fn foo(start_code: &str, end_code: &str) -> u64 {
    let nstart = ceil_to_even_number_of_digits(start_code);
    let nend = floor_to_even_number_of_digits(end_code);

    // Now we can begin by auto rejecting any range that is not ordered
    if nstart > nend {
        return 0;
    }

    

    todo!()
}