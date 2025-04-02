use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn parse_input(filepath: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filepath)
        .expect("could not read input")
        .lines()
        .fold(Vec::new(), |mut acc, line| {
            acc.push(line.chars().collect());
            acc
        })
}

fn part_one(filepath: &str) -> u128 {
    let mut visited = HashSet::new();
    let mut tracker = Vec::new();

    let map = parse_input(filepath);
    map.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, _)| {
            if visited.contains(&(row, col)) {
                return;
            }

            let mut area = 0;
            let mut perimeter = 0;
            let char = map[row][col];

            let mut queue = VecDeque::new();

            queue.push_back((row, col));
            visited.insert((row, col));
            while let Some((y, x)) = queue.pop_front() {
                area += 1;

                if y > 0 && map[y - 1][x] == char {
                    if !visited.contains(&(y - 1, x)) {
                        queue.push_back((y - 1, x));
                        visited.insert((y - 1, x));
                    }
                } else {
                    perimeter += 1;
                }

                if x > 0 && map[y][x - 1] == char {
                    if !visited.contains(&(y, x - 1)) {
                        queue.push_back((y, x - 1));
                        visited.insert((y, x - 1));
                    }
                } else {
                    perimeter += 1;
                }

                if y < map.len() - 1 && map[y + 1][x] == char {
                    if !visited.contains(&(y + 1, x)) {
                        queue.push_back((y + 1, x));
                        visited.insert((y + 1, x));
                    }
                } else {
                    perimeter += 1;
                }

                if x < line.len() - 1 && map[y][x + 1] == char {
                    if !visited.contains(&(y, x + 1)) {
                        queue.push_back((y, x + 1));
                        visited.insert((y, x + 1));
                    }
                } else {
                    perimeter += 1;
                }
            }

            tracker.push((area, perimeter));
        });
    });

    tracker
        .iter()
        .map(|(area, perimeter)| area * perimeter)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day12::part_one;

    #[test]
    fn test_part_one_example_one() {
        let expected = 140;
        let actual = part_one("./src/day12/input.example.one.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one_example_two() {
        let expected = 772;
        let actual = part_one("./src/day12/input.example.two.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one_example_three() {
        let expected = 1930;
        let actual = part_one("./src/day12/input.example.three.txt");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let expected = 1473620;
        let actual = part_one("./src/day12/input.txt");
        assert_eq!(expected, actual);
    }
}
