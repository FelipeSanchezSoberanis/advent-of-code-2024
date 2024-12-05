use std::fs;

pub fn main() {
    let mut left_numbers: Vec<u32> = vec![];
    let mut right_numbers: Vec<u32> = vec![];
    let mut diff: u32 = 0;

    let input = match fs::read_to_string("src/day01/input.txt") {
        Ok(input) => input,
        Err(err) => panic!("Could not read input file: {err:?}"),
    };
    input
        .lines()
        .map(|line| line.split("   "))
        .for_each(|number_strings| {
            let mut numbers =
                number_strings.map(|number_string| match number_string.parse::<u32>() {
                    Ok(number) => number,
                    Err(err) => panic!("Could not parse number string: {err:?}"),
                });
            left_numbers.push(match numbers.next() {
                Some(number) => number,
                None => panic!("Could not get left number"),
            });
            right_numbers.push(match numbers.next() {
                Some(number) => number,
                None => panic!("Could not get right number"),
            });
        });
    left_numbers.sort();
    right_numbers.sort();
    for i in 0..left_numbers.len() {
        let left_number = left_numbers[i];
        let right_number = right_numbers[i];
        diff = diff + left_number.abs_diff(right_number);
    }
    println!("{}", diff);
}
