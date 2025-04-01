use std::{collections::HashMap, fs, u128};

fn parse_input(filepath: &str) -> Vec<u128> {
    fs::read_to_string(filepath)
        .expect("could not read input file")
        .split_whitespace()
        .map(|number_string| {
            number_string
                .parse::<u128>()
                .expect("could not parse number string")
        })
        .collect::<Vec<_>>()
}

fn solve(stone: u128, blinks: usize, memory: &mut HashMap<(u128, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    } else if let Some(value) = memory.get(&(stone, blinks)) {
        return *value;
    } else if stone == 0 {
        let value = solve(1, blinks - 1, memory);
        memory.insert((stone, blinks), value);
        return value;
    } else if stone.to_string().len() % 2 == 0 {
        let stone_string = stone.to_string();
        let value = solve(
            stone_string[0..stone_string.len() / 2]
                .parse()
                .expect("could not parse stone string"),
            blinks - 1,
            memory,
        ) + solve(
            stone_string[stone_string.len() / 2..stone_string.len()]
                .parse()
                .expect("could not parse stone string"),
            blinks - 1,
            memory,
        );
        memory.insert((stone, blinks), value);
        return value;
    } else {
        let value = solve(stone * 2024, blinks - 1, memory);
        memory.insert((stone, blinks), value);
        return value;
    }
}

fn part_one(filepath: &str) -> usize {
    let stones = parse_input(filepath);
    let mut memory = HashMap::new();
    stones
        .iter()
        .map(|stone| solve(*stone, 25, &mut memory))
        .sum()
}

fn part_two(filepath: &str) -> usize {
    let stones = parse_input(filepath);
    let mut memory = HashMap::new();
    stones
        .iter()
        .map(|stone| solve(*stone, 75, &mut memory))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_example() {
        let expected = 55312;
        let actual = part_one("./src/day11/input.example.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let expected = 185894;
        let actual = part_one("./src/day11/input.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_two() {
        let expected = 221632504974231;
        let actual = part_two("./src/day11/input.txt");
        assert_eq!(expected, actual);
    }
}
