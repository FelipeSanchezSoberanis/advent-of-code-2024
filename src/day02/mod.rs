use std::fs;

pub fn main() {
    let mut num_safe_reports = 0;

    let input = fs::read_to_string("src/day02/input.txt").expect("could not read input file");

    input.lines().for_each(|line| {
        let numbers_strings = line.split_whitespace();
        let mut numbers = numbers_strings.map(|number_string| {
            number_string
                .parse::<u32>()
                .expect("could not parse number string")
        });

        let mut safe = true;
        let mut decreasing: Option<bool> = Option::None;
        let mut first_number = numbers.next().expect("Could not get first number");
        let mut second_number: u32;
        let mut next = numbers.next();

        while next.is_some() {
            second_number = next.unwrap();

            let diff = first_number.abs_diff(second_number);
            if diff == 0 || diff > 3 {
                safe = false;
                break;
            };

            if decreasing.is_none() {
                decreasing = Option::Some(second_number > first_number);
            } else {
                if decreasing.unwrap() != (second_number > first_number) {
                    safe = false;
                    break;
                };
            };

            first_number = second_number;
            next = numbers.next();
        }

        if safe {
            num_safe_reports += 1;
        }
    });

    println!("{}", num_safe_reports);
}
