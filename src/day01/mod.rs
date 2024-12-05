use std::collections::HashMap;
use std::fs;

pub fn main() {
    let mut left_numbers: HashMap<u32, u32> = HashMap::new();
    let mut right_numbers: Vec<u32> = Vec::new();

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
            let left_number = match numbers.next() {
                Some(number) => number,
                None => panic!("Could not get left number"),
            };
            let right_number = match numbers.next() {
                Some(number) => number,
                None => panic!("Could not get right number"),
            };
            if !left_numbers.contains_key(&left_number) {
                left_numbers.insert(left_number, 0);
            };
            right_numbers.push(right_number);
        });
    right_numbers.iter().for_each(|right_number| {
        if left_numbers.contains_key(right_number) {
            left_numbers.insert(*right_number, left_numbers.get(right_number).unwrap() + 1);
        };
    });
    let res = left_numbers
        .keys()
        .fold(0, |acc, key| acc + key * left_numbers.get(key).unwrap());
    println!("{}", res);
}
