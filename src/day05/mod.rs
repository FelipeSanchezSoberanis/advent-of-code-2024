use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn main() {
    let input = fs::read_to_string("src/day05/input.txt").expect("could not read input file");
    let (rules, updates) = parse_input(input);
    let res = updates
        .iter()
        .filter(|update| !is_update_valid(update, &rules))
        .map(|invalid_update| make_update_valid(invalid_update, &rules))
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum::<u16>();
    println!("{res}");
}

fn make_update_valid(update: &Vec<u16>, rules: &HashMap<u16, HashSet<u16>>) -> Vec<u16> {
    let mut valid_update: Vec<u16> = Vec::new();
    update.iter().for_each(|num| valid_update.push(*num));

    let mut i = 0;
    while i < update.len() {
        let curr_num = &valid_update[i];
        let rem_nums = &valid_update.clone()[i + 1..valid_update.len()];
        let nums_to_move = rem_nums
            .iter()
            .filter(|rem_num| should_be_before(rem_num, curr_num, rules))
            .collect::<Vec<_>>();
        nums_to_move.iter().rev().for_each(|&&num_to_move| {
            if let Some(index_to_delete) = valid_update.iter().position(|&num| num == num_to_move) {
                valid_update.remove(index_to_delete);
            };
            valid_update.insert(i, num_to_move)
        });
        if !nums_to_move.is_empty() {
            if i <= 0 {
                continue;
            }
            i -= 1;
        }
        i += 1;
    }

    valid_update
}

fn should_be_before(a: &u16, b: &u16, rules: &HashMap<u16, HashSet<u16>>) -> bool {
    match rules.get(a) {
        Some(rule) => rule.contains(b),
        None => false,
    }
}

fn is_update_valid(update: &Vec<u16>, rules: &HashMap<u16, HashSet<u16>>) -> bool {
    for i in 0..update.len() {
        let number = &update[i];
        let remaining_numbers = &update[i + 1..update.len()];

        if remaining_numbers
            .iter()
            .any(|remaining_number| should_be_before(remaining_number, number, rules))
        {
            return false;
        }
    }
    true
}

fn parse_input(input: String) -> (HashMap<u16, HashSet<u16>>, Vec<Vec<u16>>) {
    let mut rules: HashMap<u16, HashSet<u16>> = HashMap::new();
    let mut updates: Vec<Vec<u16>> = Vec::new();

    let mut reading_rules = true;
    input.lines().for_each(|line| {
        if line.is_empty() {
            reading_rules = false;
            return;
        }

        if reading_rules {
            let (before, after) = parse_rule(line);
            if rules.contains_key(&before) {
                rules
                    .get_mut(&before)
                    .expect("could not get rule with key")
                    .insert(after);
            } else {
                let mut after_hash_set = HashSet::new();
                after_hash_set.insert(after);
                rules.insert(before, after_hash_set);
            }
        } else {
            let update = parse_update(line);
            updates.push(update);
        }
    });
    (rules, updates)
}

fn parse_update(line: &str) -> Vec<u16> {
    line.split(",")
        .map(|number_string| {
            number_string
                .parse::<u16>()
                .expect("could not parse update number")
        })
        .collect::<Vec<u16>>()
}

fn parse_rule(line: &str) -> (u16, u16) {
    let mut numbers = line.split("|");
    (
        numbers
            .next()
            .expect("could not get after number")
            .parse::<u16>()
            .expect("could not parse after number"),
        numbers
            .next()
            .expect("could not get after number")
            .parse::<u16>()
            .expect("could not parse after number"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rules() -> HashMap<u16, HashSet<u16>> {
        let input =
            fs::read_to_string("src/day05/input_tests.txt").expect("could not read input file");
        let (rules, _) = parse_input(input);
        rules
    }

    #[test]
    fn test_make_update_valid_one() {
        let rules = get_rules();
        let input = vec![75, 97, 47, 61, 53];
        let expected = vec![97, 75, 47, 61, 53];
        let actual = make_update_valid(&input, &rules);
        assert_eq!(expected, actual, "input: {:?}", input);
    }

    #[test]
    fn test_make_update_valid_two() {
        let rules = get_rules();
        let input = vec![61, 13, 29];
        let expected = vec![61, 29, 13];
        let actual = make_update_valid(&input, &rules);
        assert_eq!(expected, actual, "input: {:?}", input);
    }

    #[test]
    fn test_make_update_valid_three() {
        let rules = get_rules();
        let input = vec![97, 13, 75, 29, 47];
        let expected = vec![97, 75, 47, 29, 13];
        let actual = make_update_valid(&input, &rules);
        assert_eq!(expected, actual, "input: {:?}", input);
    }
}
