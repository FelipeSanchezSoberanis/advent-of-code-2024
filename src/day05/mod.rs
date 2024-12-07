use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn main() {
    let input = fs::read_to_string("src/day05/input.txt").expect("could not read input file");
    let (rules, updates) = parse_input(input);
    let res = updates
        .iter()
        .filter(|update| is_update_valid(update, &rules))
        .map(|update| update[update.len() / 2])
        .sum::<u16>();
    println!("{res}");
}

fn make_update_valid(update: &Vec<u16>, rules: &HashMap<u16, HashSet<u16>>) -> Vec<u16> {
    vec![]
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
