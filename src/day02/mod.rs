use std::fs;

pub fn main() {
    let input = fs::read_to_string("src/day02/input.txt").expect("could not read input file");
    let reports = input.lines().map(|line| {
        let numbers_strings = line.split_whitespace();
        numbers_strings.map(|number_string| {
            number_string
                .parse::<u32>()
                .expect("could not parse number string")
        })
    });

    let mut num_valid_reports = 0;
    for report_iter in reports {
        let report = report_iter.collect::<Vec<_>>();

        if is_report_valid(&report) {
            num_valid_reports += 1;
            continue;
        };

        let mut has_valid_combination = false;
        (0..report.len()).for_each(|i| {
            let mut combination = report.clone();
            combination.remove(i);
            if is_report_valid(&combination) {
                has_valid_combination = true;
            };
        });

        if has_valid_combination {
            num_valid_reports += 1;
        };
    }

    println!("{}", num_valid_reports);
}

fn is_report_valid(report: &Vec<u32>) -> bool {
    let mut valid = true;
    let mut prev_num: Option<&u32> = Option::None;
    let mut asc = Option::None;

    report.iter().for_each(|curr_num| {
        if prev_num.is_none() {
            prev_num = Option::Some(curr_num);
            return;
        }

        let diff = curr_num.abs_diff(*prev_num.unwrap());
        if diff == 0 || diff > 3 {
            valid = false;
        };

        if asc.is_none() {
            asc = Option::Some(curr_num > &prev_num.unwrap());
        } else if asc.unwrap() != (curr_num > &prev_num.unwrap()) {
            valid = false;
        };

        prev_num = Option::Some(curr_num);
    });

    return valid;
}
