use std::{fs, u128};

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

fn part_one(filepath: &str) -> usize {
    let mut stones = parse_input(filepath);

    for _ in 0..25 {
        let mut updated_stones = Vec::<u128>::new();

        stones.iter().for_each(|stone| {
            if *stone == 0 {
                updated_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let number_string = stone.to_string();
                updated_stones.push(
                    number_string[0..number_string.len() / 2]
                        .parse()
                        .expect("could not parse number string"),
                );
                updated_stones.push(
                    number_string[number_string.len() / 2..number_string.len()]
                        .parse()
                        .expect("could not parse number string"),
                );
            } else {
                updated_stones.push(stone * 2024);
            }
        });

        stones = updated_stones;
    }

    stones.iter().len()
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
}
